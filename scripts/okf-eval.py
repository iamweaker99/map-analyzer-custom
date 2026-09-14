#!/usr/bin/env python3
"""Evaluate the narrow OKF retriever and the Stage 1 context retriever."""

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
CASES_PATH = ROOT / "okf" / "eval" / "retrieval_cases.json"
BASELINE_JSON_PATH = ROOT / "okf" / "generated" / "retrieval-baseline.json"
BASELINE_MARKDOWN_PATH = ROOT / "okf" / "generated" / "retrieval-baseline.md"
STAGE1_JSON_PATH = ROOT / "okf" / "generated" / "retrieval-stage1.json"
STAGE1_MARKDOWN_PATH = ROOT / "okf" / "generated" / "retrieval-stage1.md"
ID_PATTERNS = {
    "retrieve": re.compile(r"^([a-z]+\.[a-z0-9-]+) \["),
    "context": re.compile(r"^OKF_CONTEXT_OBJECT ([a-z]+\.[a-z0-9-]+) "),
}
COVERAGE_PATTERN = re.compile(r"^coverage: (semantic|source_only|none)$")
ALLOWED_COVERAGE = {"semantic", "source_only", "none"}


def run_git(*args: str) -> str:
    result = subprocess.run(
        ["git", *args], cwd=ROOT, capture_output=True, text=True, encoding="utf-8"
    )
    if result.returncode != 0:
        raise RuntimeError(
            f"git {' '.join(args)} failed: {result.stderr.strip() or f'exit {result.returncode}'}"
        )
    return result.stdout.strip()


def load_cases() -> list[dict]:
    try:
        value = json.loads(CASES_PATH.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise RuntimeError(f"unable to read case definition: {error}") from error
    if not isinstance(value, list) or not value:
        raise RuntimeError("case definition must be a non-empty JSON array")
    required = {"id", "class", "query", "expected_ids", "expected_coverage", "notes"}
    seen = set()
    for index, case in enumerate(value):
        if not isinstance(case, dict) or set(case) != required:
            raise RuntimeError(f"case {index} must contain exactly {sorted(required)}")
        if not all(isinstance(case[field], str) and case[field] for field in ("id", "class", "query", "notes")):
            raise RuntimeError(f"case {index} has an invalid string field")
        if case["id"] in seen:
            raise RuntimeError(f"duplicate case id: {case['id']}")
        seen.add(case["id"])
        if not isinstance(case["expected_ids"], list) or not all(
            isinstance(item, str) and item for item in case["expected_ids"]
        ):
            raise RuntimeError(f"case {case['id']} has invalid expected_ids")
        if case["expected_coverage"] not in ALLOWED_COVERAGE:
            raise RuntimeError(f"case {case['id']} has invalid expected_coverage")
    return value


def command_for(operation: str, query: str) -> list[str]:
    command = [operation, "--query", query]
    if os.name == "nt":
        return [
            "powershell", "-NoProfile", "-ExecutionPolicy", "Bypass",
            "-File", str(ROOT / "scripts" / "okf.ps1"), *command,
        ]
    return [str(ROOT / "scripts" / "okf"), *command]


def run_case(case: dict, operation: str) -> dict:
    try:
        completed = subprocess.run(
            command_for(operation, case["query"]), cwd=ROOT, capture_output=True,
            text=True, encoding="utf-8", errors="replace",
        )
    except OSError as error:
        raise RuntimeError(f"unable to invoke OKF for {case['id']}: {error}") from error
    stdout = completed.stdout
    returned_ids = [
        match.group(1)
        for line in stdout.splitlines()
        if (match := ID_PATTERNS[operation].match(line))
    ]
    ranks = {object_id: index for index, object_id in enumerate(returned_ids, start=1)}
    expected_ranks = [ranks[object_id] for object_id in case["expected_ids"] if object_id in ranks]
    positive = bool(case["expected_ids"])
    retrieval_pass = bool(expected_ranks) if positive else not returned_ids
    actual_coverage = None
    coverage_match = None
    if operation == "context":
        coverage_values = [
            match.group(1)
            for line in stdout.splitlines()
            if (match := COVERAGE_PATTERN.match(line))
        ]
        actual_coverage = coverage_values[0] if len(coverage_values) == 1 else None
        coverage_match = actual_coverage == case["expected_coverage"]
    return {
        **case,
        "operation": operation,
        "returned_ids": returned_ids,
        "expected_found": bool(expected_ranks),
        "negative_control_pass": not positive and not returned_ids,
        "actual_coverage": actual_coverage,
        "coverage_match": coverage_match,
        "pass": retrieval_pass and coverage_match is not False,
        "best_expected_rank": min(expected_ranks) if expected_ranks else None,
        "process_exit_status": completed.returncode,
        "output_size": len(stdout.encode("utf-8")),
        "stdout": stdout,
        "stderr": completed.stderr,
    }


def metric(rank: int, selected: list[dict]) -> dict:
    hits = sum(
        result["best_expected_rank"] is not None and result["best_expected_rank"] <= rank
        for result in selected
    )
    return {"hits": hits, "cases": len(selected), "value": hits / len(selected) if selected else None}


def negative_pass_rate(results: list[dict]) -> dict:
    passed = sum(result["negative_control_pass"] for result in results)
    return {"passed": passed, "cases": len(results), "value": passed / len(results) if results else None}


def output_summary(results: list[dict]) -> dict:
    values = [result["output_size"] for result in results]
    return {"average_bytes": sum(values) / len(values), "max_bytes": max(values)}


def aggregate(results: list[dict]) -> dict:
    positive = [result for result in results if result["expected_ids"]]
    negative = [result for result in results if not result["expected_ids"]]
    output = output_summary(results)
    by_class = {}
    for test_class in sorted({result["class"] for result in results}):
        selected = [result for result in results if result["class"] == test_class]
        expected = [result for result in selected if result["expected_ids"]]
        negative_selected = [result for result in selected if not result["expected_ids"]]
        by_class[test_class] = {
            "total": len(selected),
            "expected_hit_count": sum(result["expected_found"] for result in selected),
            "recall_at_1": metric(1, expected),
            "recall_at_3": metric(3, expected),
            "recall_at_5": metric(5, expected),
            "negative_control_pass_rate": negative_pass_rate(negative_selected),
        }
    return {
        "total_cases": len(results),
        "positive_case_count": len(positive),
        "expected_hit_count": sum(result["expected_found"] for result in positive),
        "recall_at_1": metric(1, positive),
        "recall_at_3": metric(3, positive),
        "recall_at_5": metric(5, positive),
        "negative_case_count": len(negative),
        "negative_cases_correctly_empty": sum(result["negative_control_pass"] for result in negative),
        "negative_control_pass_rate": negative_pass_rate(negative),
        "false_positive_negative_cases": [result["id"] for result in negative if result["returned_ids"]],
        "output_size": output,
        "by_class": by_class,
    }


def format_metric(value: dict) -> str:
    return "n/a" if value["value"] is None else f"{value['value']:.3f} ({value['hits']}/{value['cases']})"


def format_negative(value: dict) -> str:
    return "n/a" if value["value"] is None else f"{value['value']:.3f} ({value['passed']}/{value['cases']})"


def format_optional(value: float | None, suffix: str = "") -> str:
    return "n/a" if value is None else f"{value:.1f}{suffix}"


def render_case_results(lines: list[str], results: list[dict]) -> None:
    for result in results:
        expected = ", ".join(f"`{item}`" for item in result["expected_ids"]) or "(none)"
        returned = ", ".join(
            f"`{object_id}` (#{index})" for index, object_id in enumerate(result["returned_ids"], start=1)
        ) or "(none)"
        status = "PASS" if result["pass"] else "FAIL"
        lines.extend([
            f"### `{result['id']}` — {status}", "",
            f"- Class: `{result['class']}`", f"- Query: {result['query']}",
            f"- Expected: {expected}", f"- Returned: {returned}",
            f"- Best expected rank: {result['best_expected_rank'] if result['best_expected_rank'] is not None else 'null'}",
        ])
        if result["operation"] == "context":
            lines.append(
                f"- Coverage: expected `{result['expected_coverage']}`, "
                f"actual `{result['actual_coverage'] or 'missing'}`"
            )
        lines.extend([
            f"- Output: {result['output_size']} bytes",
            f"- Notes: {result['notes']}", "",
        ])


def render_summary(lines: list[str], data: dict) -> None:
    lines.extend([
        f"- Positive cases: {data['positive_case_count']}",
        f"- Expected-object hits: {data['expected_hit_count']}/{data['positive_case_count']}",
        f"- Recall@1/@3/@5: {format_metric(data['recall_at_1'])} / {format_metric(data['recall_at_3'])} / {format_metric(data['recall_at_5'])}",
        f"- Negative-control pass rate: {format_negative(data['negative_control_pass_rate'])}",
        f"- False-positive negative cases: {', '.join(data['false_positive_negative_cases']) or 'none'}",
        f"- Average/max output: {format_optional(data['output_size']['average_bytes'], ' bytes')} / {format_optional(data['output_size']['max_bytes'], ' bytes')}",
    ])


def render_baseline(metadata: dict, data: dict, results: list[dict]) -> str:
    lines = ["# OKF Retrieval Baseline", "", "## Summary", ""]
    render_summary(lines, data)
    lines.extend([
        f"- Git branch: `{metadata['branch']}`",
        f"- Git HEAD: `{metadata['head']}`", "",
        "## Results by test class", "",
        "| Class | Cases | Hits | Recall@1 | Recall@3 | Recall@5 | Negative pass rate |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: |",
    ])
    for test_class, class_data in data["by_class"].items():
        lines.append(
            f"| `{test_class}` | {class_data['total']} | {class_data['expected_hit_count']} | "
            f"{format_metric(class_data['recall_at_1'])} | {format_metric(class_data['recall_at_3'])} | "
            f"{format_metric(class_data['recall_at_5'])} | {format_negative(class_data['negative_control_pass_rate'])} |"
        )
    lines.extend(["", "## Case results", ""])
    render_case_results(lines, results)
    lines.extend(["The baseline uses the original narrow `retrieve()` operation; negative controls are excluded from recall denominators.", ""])
    return "\n".join(lines)


def render_stage1(metadata: dict, baseline: dict, context_data: dict, results: list[dict]) -> str:
    lines = [
        "# OKF Routing Stage 1 Comparison", "", "## Summary", "",
        "| Metric | Stage 0 retrieve | Stage 1 context |",
        "| --- | ---: | ---: |",
    ]
    for label, key in (("Recall@1", "recall_at_1"), ("Recall@3", "recall_at_3"), ("Recall@5", "recall_at_5")):
        lines.append(f"| {label} | {format_metric(baseline[key])} | {format_metric(context_data[key])} |")
    lines.extend([
        f"| Negative-control pass rate | {format_negative(baseline['negative_control_pass_rate'])} | {format_negative(context_data['negative_control_pass_rate'])} |",
        f"| Average output | {format_optional(baseline['output_size']['average_bytes'], ' bytes')} | {format_optional(context_data['output_size']['average_bytes'], ' bytes')} |",
        "", f"Git branch: `{metadata['branch']}`; HEAD: `{metadata['head']}`.", "",
        "## Results by test class", "",
        "| Class | Stage 0 hits | Stage 1 hits | Stage 0 recall@1 | Stage 1 recall@1 | Stage 1 negative pass |",
        "| --- | ---: | ---: | ---: | ---: | ---: |",
    ])
    for test_class in sorted(context_data["by_class"]):
        before = baseline["by_class"][test_class]
        after = context_data["by_class"][test_class]
        lines.append(
            f"| `{test_class}` | {before['expected_hit_count']} | {after['expected_hit_count']} | "
            f"{format_metric(before['recall_at_1'])} | {format_metric(after['recall_at_1'])} | "
            f"{format_negative(after['negative_control_pass_rate'])} |"
        )
    lines.extend(["", "## Stage 1 case results", ""])
    render_case_results(lines, results)
    lines.extend([
        "## Interpretation", "",
        "Context retrieval uses deterministic lexical matching over stored IDs, keys, titles, descriptions, claims, headings, and body text.",
        "Source-only misses remain coverage observations; no semantic objects were added to make them pass.", "",
    ])
    return "\n".join(lines)


def main() -> int:
    try:
        cases = load_cases()
        baseline_results = [run_case(case, "retrieve") for case in cases]
        context_results = [run_case(case, "context") for case in cases]
        failed = [result for result in baseline_results + context_results if result["process_exit_status"] != 0]
        metadata = {
            "branch": run_git("rev-parse", "--abbrev-ref", "HEAD"),
            "head": run_git("rev-parse", "HEAD"),
        }
        baseline = aggregate(baseline_results)
        context_data = aggregate(context_results)
        BASELINE_JSON_PATH.parent.mkdir(parents=True, exist_ok=True)
        BASELINE_JSON_PATH.write_text(json.dumps({"metadata": metadata, "aggregate": baseline, "results": baseline_results}, indent=2) + "\n", encoding="utf-8")
        BASELINE_MARKDOWN_PATH.write_text(render_baseline(metadata, baseline, baseline_results), encoding="utf-8")
        STAGE1_JSON_PATH.write_text(json.dumps({"metadata": metadata, "baseline": baseline, "context": context_data, "results": context_results}, indent=2) + "\n", encoding="utf-8")
        STAGE1_MARKDOWN_PATH.write_text(render_stage1(metadata, baseline, context_data, context_results), encoding="utf-8")
        if failed:
            raise RuntimeError("OKF command failed for: " + ", ".join(result["id"] for result in failed))
        coverage_mismatches = [result["id"] for result in context_results if not result["coverage_match"]]
        if coverage_mismatches:
            raise RuntimeError("coverage mismatch for: " + ", ".join(coverage_mismatches))
    except RuntimeError as error:
        print(f"okf-eval: {error}", file=sys.stderr)
        return 1
    print(f"OKF evaluation: {len(cases)} cases for retrieve and context; reports written")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
