use serde_yaml::{Mapping, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const TYPES: [&str; 6] = [
    "entity",
    "concept",
    "system",
    "research",
    "decision",
    "reference",
];
const SEMANTIC_FIELDS: [&str; 6] = [
    "entities",
    "concepts",
    "systems",
    "research",
    "decisions",
    "references",
];
const SESSION_HANDOFF_PATH: &str = "Keep_LOCAL/SESSION_HANDOFF.md";
const SESSION_HANDOFF_TEMPLATE: &str = r#"# Workstream

## Objective

## Current phase

## Current repository state
- branch:
- relevant commit:
- working-tree changes:
- tests/status:

## Durable artifacts produced/updated

## Draft durable knowledge awaiting review

## Operational remainder

## Immediate next action

## Relevant primary artifacts

## Verification required before proceeding
"#;

#[derive(Debug, Clone)]
struct SemanticDocument {
    path: PathBuf,
    frontmatter: Mapping,
    body: String,
}

#[derive(Debug, Clone)]
struct Diagnostic {
    path: PathBuf,
    message: String,
}

impl Diagnostic {
    fn severity(&self) -> &'static str {
        if self.message.contains("drift") {
            "ERROR"
        } else {
            "ERROR"
        }
    }

    fn code(&self) -> &'static str {
        let message = self.message.to_ascii_lowercase();
        for code in ["OKF-E420", "OKF-E421", "OKF-E422", "OKF-E423", "OKF-E424"] {
            if self.message.contains(code) {
                return code;
            }
        }
        if message.contains("missing semantic object directory") {
            return "OKF-E002";
        }
        if message.contains("directly inside") {
            return "OKF-E027";
        }
        if message.contains("duplicate") {
            return "OKF-E007";
        }
        if message.contains("illegal lifecycle") {
            return "OKF-E012";
        }
        if message.contains("verified identity mutated") {
            return "OKF-E025";
        }
        if message.contains("durable ID") {
            return "OKF-E008";
        }
        if message.contains("unknown object") || message.contains("unknown claim") {
            return "OKF-E104";
        }
        if message.contains("cycle") {
            return "OKF-E111";
        }
        if message.contains("decision_key") {
            return "OKF-E308";
        }
        if message.contains("semantic_hash mismatch") {
            return "OKF-E017";
        }
        "OKF-E001"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandOutcome {
    Clean,
    Errors,
}

#[derive(Debug, Clone)]
struct SourceResult {
    path: PathBuf,
    severity: &'static str,
    message: String,
}

const REFERENCE_KINDS: [&str; 8] = [
    "repository_file",
    "repository_tree",
    "git_commit",
    "git_ref",
    "github_issue",
    "github_pull_request",
    "artifact",
    "external_url",
];

#[derive(Debug, Clone)]
struct RelationRule {
    inverse: String,
    source_claim: bool,
    target_claim: bool,
    source_types: Vec<String>,
    target_types: Vec<String>,
    selectors: bool,
    self_edges_forbidden: bool,
    acyclic: bool,
}

#[derive(Debug, Clone)]
struct TypeRule {
    required_fields: Vec<String>,
    forbidden_fields: Vec<String>,
    canonical_claim: Option<String>,
    canonical_claims: bool,
    active_sections: Vec<String>,
}

#[derive(Debug, Clone)]
struct Registries {
    relations: BTreeMap<String, RelationRule>,
    types: BTreeMap<String, TypeRule>,
}

fn main() {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "help".to_string());

    let result = match command.as_str() {
        "lint" => lint(&args.collect::<Vec<_>>()),
        "source-check" => source_check(&args.collect::<Vec<_>>()),
        "generate" => generate(&args.collect::<Vec<_>>()),
        "review" => review(&args.collect::<Vec<_>>()),
        "check" => check(&args.collect::<Vec<_>>()),
        "rehash" => rehash(&args.collect::<Vec<_>>()),
        "retrieve" => retrieve(&args.collect::<Vec<_>>()),
        "session-init" => session_init(&args.collect::<Vec<_>>()),
        "session-check" => session_check(&args.collect::<Vec<_>>()),
        "draft-init" => draft_init(&args.collect::<Vec<_>>()),
        "help" | "--help" | "-h" => {
            print_help();
            Ok(CommandOutcome::Clean)
        }
        other => Err(format!("unsupported command: {other}")),
    };

    match result {
        Ok(CommandOutcome::Clean) => {}
        Ok(CommandOutcome::Errors) => std::process::exit(1),
        Err(message) => {
            eprintln!("okf: {message}");
            std::process::exit(2);
        }
    }
}

fn print_help() {
    println!("Usage: scripts/okf <lint|generate|source-check|review|check|rehash|retrieve|session-init|session-check|draft-init>");
    println!("  lint         validate semantic objects (use --history in CI)");
    println!("  generate     write deterministic views (use --check in CI)");
    println!("  source-check check repository-local References");
    println!("  review       display review conditions");
    println!("  check        run the read-only CI validation sequence");
    println!("  rehash       recompute derived hashes (use --write to modify objects)");
    println!("  session-init create the operational handoff template if absent");
    println!("  session-check validate the handoff and run the read-only OKF checks");
    println!("  draft-init   create a non-active draft semantic object");
}

fn session_init(_args: &[String]) -> Result<CommandOutcome, String> {
    let path = Path::new(SESSION_HANDOFF_PATH);
    if path.exists() {
        return Err(format!(
            "{SESSION_HANDOFF_PATH} already exists; edit it instead of replacing it"
        ));
    }
    fs::write(path, SESSION_HANDOFF_TEMPLATE)
        .map_err(|error| format!("{SESSION_HANDOFF_PATH}: {error}"))?;
    println!("OKF: created {SESSION_HANDOFF_PATH}");
    Ok(CommandOutcome::Clean)
}

fn session_check(_args: &[String]) -> Result<CommandOutcome, String> {
    let okf_result = check(&[])?;
    let path = Path::new(SESSION_HANDOFF_PATH);
    let content =
        fs::read_to_string(path).map_err(|error| format!("{SESSION_HANDOFF_PATH}: {error}"))?;
    let (documents, _) = load_documents()?;
    let diagnostics = handoff_diagnostics(&content, &documents);
    for diagnostic in &diagnostics {
        println!("ERROR OKF-E501 {SESSION_HANDOFF_PATH}: {diagnostic}");
    }
    if diagnostics.is_empty() {
        println!("OKF: session handoff is valid");
    }
    Ok(
        if okf_result == CommandOutcome::Errors || !diagnostics.is_empty() {
            CommandOutcome::Errors
        } else {
            CommandOutcome::Clean
        },
    )
}

fn draft_init(args: &[String]) -> Result<CommandOutcome, String> {
    if args.len() != 2 {
        return Err("draft-init requires <type> <slug>".to_string());
    }
    let object_type = args[0].as_str();
    if !["entity", "concept", "system", "research", "decision"].contains(&object_type) {
        return Err(
            "draft-init type must be entity, concept, system, research, or decision".to_string(),
        );
    }
    validate_id(&format!("{object_type}.{}", args[1]), object_type)?;
    let directory = match object_type {
        "entity" => "entities",
        "concept" => "concepts",
        "system" => "systems",
        "research" => "research",
        "decision" => "decisions",
        _ => unreachable!(),
    };
    let path = Path::new("okf")
        .join(directory)
        .join(format!("{}.md", args[1]));
    if path.exists() {
        return Err(format!(
            "{} already exists; refusing to overwrite it",
            path.display()
        ));
    }
    let now = current_utc_timestamp();
    let claim_id = "candidate";
    let mut frontmatter: Mapping = serde_yaml::from_str(&format!(
        "id: {object_type}.{}\ntype: {object_type}\ntitle: Draft {}\ndescription: Unverified draft knowledge; replace this description before review.\nlifecycle: draft\ncreated_at: \"{now}\"\nupdated_at: \"{now}\"\ngenerated:\n  by: process:okf-draft/v1\n  at: \"{now}\"\nclaims:\n- id: {claim_id}\n  lifecycle: draft\n  statement: Replace this candidate proposition with the load-bearing conclusion under review.\n  load_bearing: true\n  semantic_hash: sha256:{}\n",
        args[1],
        args[1].replace('-', " "),
        "0".repeat(64)
    ))
    .map_err(|error| error.to_string())?;
    let body = match object_type {
        "entity" => "## Definition\n\nReplace with the concrete thing being captured.\n\n## Identity and boundaries\n\nDescribe what is and is not included.\n",
        "concept" => {
            frontmatter.insert(Value::String("definition_claim".into()), Value::String(claim_id.into()));
            "## Definition\n\nReplace with the durable definition.\n\n## Boundaries\n\nDescribe neighboring concepts and exclusions.\n"
        }
        "system" => {
            frontmatter.insert(Value::String("behavior_claims".into()), Value::Sequence(vec![Value::String(claim_id.into())]));
            "## Purpose\n\nReplace with the purpose.\n\n## Boundary\n\nReplace with the system boundary.\n\n## Current behavior\n\nReplace with observed behavior.\n\n## Interfaces and dependencies\n\nReplace with relevant interfaces and dependencies.\n\n## Known limitations\n\nRecord known limitations.\n"
        }
        "research" => {
            frontmatter.insert(Value::String("research_question".into()), Value::String("Replace with the research question.".into()));
            "## Question\n\nReplace with the question.\n\n## Method and evidence\n\nRecord the method and primary evidence.\n\n## Findings\n\nRecord findings without promoting them.\n\n## Conclusions\n\nRecord the draft conclusion.\n\n## Limitations and uncertainty\n\nRecord uncertainty and missing evidence.\n"
        }
        "decision" => {
            frontmatter.insert(Value::String("decision_key".into()), Value::String(format!("draft.{}", args[1])));
            frontmatter.insert(Value::String("decision_claim".into()), Value::String(claim_id.into()));
            "## Context\n\nRecord the context.\n\n## Decision\n\nThis is a draft choice and is not active.\n\n## Rationale\n\nRecord available rationale.\n\n## Alternatives considered\n\nRecord alternatives.\n\n## Consequences\n\nRecord expected consequences.\n"
        }
        _ => unreachable!(),
    };
    let claim_hash_value = frontmatter
        .get_mut("claims")
        .and_then(Value::as_sequence_mut)
        .and_then(|claims| claims.first_mut())
        .and_then(Value::as_mapping_mut)
        .ok_or_else(|| "draft claim construction failed".to_string())?;
    let claim_hash_value = claim_hash(claim_hash_value)?;
    if let Some(claim) = frontmatter
        .get_mut("claims")
        .and_then(Value::as_sequence_mut)
        .and_then(|claims| claims.first_mut())
        .and_then(Value::as_mapping_mut)
    {
        claim.insert(
            Value::String("semantic_hash".into()),
            Value::String(claim_hash_value),
        );
    }
    let object_hash_value = object_hash(&frontmatter, body)?;
    frontmatter.insert(
        Value::String("semantic_hash".into()),
        Value::String(object_hash_value),
    );
    let yaml = serde_yaml::to_string(&frontmatter).map_err(|error| error.to_string())?;
    fs::write(&path, format!("---\n{yaml}---\n{body}"))
        .map_err(|error| format!("{}: {error}", path.display()))?;
    println!("OKF: created draft {}", path.display());
    Ok(CommandOutcome::Clean)
}

fn handoff_diagnostics(content: &str, documents: &[SemanticDocument]) -> Vec<String> {
    let required_headings = [
        "# Workstream",
        "## Objective",
        "## Current phase",
        "## Current repository state",
        "## Durable artifacts produced/updated",
        "## Draft durable knowledge awaiting review",
        "## Operational remainder",
        "## Immediate next action",
        "## Relevant primary artifacts",
        "## Verification required before proceeding",
    ];
    let mut diagnostics = required_headings
        .iter()
        .filter(|heading| !content.contains(**heading))
        .map(|heading| format!("missing required heading: {heading}"))
        .collect::<Vec<_>>();

    let objects = documents
        .iter()
        .filter_map(|document| {
            document
                .frontmatter
                .get("id")
                .and_then(Value::as_str)
                .map(|id| (id.to_string(), document))
        })
        .collect::<HashMap<_, _>>();
    for token in content.split_whitespace() {
        let Some((object_id, claim_id)) = parse_okf_address(token) else {
            continue;
        };
        let Some(document) = objects.get(&object_id) else {
            diagnostics.push(format!(
                "handoff references unknown OKF object: {object_id}"
            ));
            continue;
        };
        if let Some(claim_id) = claim_id {
            let found = document
                .frontmatter
                .get("claims")
                .and_then(Value::as_sequence)
                .into_iter()
                .flatten()
                .filter_map(Value::as_mapping)
                .filter_map(|claim| claim.get("id").and_then(Value::as_str))
                .any(|id| id == claim_id);
            if !found {
                diagnostics.push(format!(
                    "handoff references unknown claim: {object_id}#{claim_id}"
                ));
            }
        }
    }
    diagnostics
}

fn parse_okf_address(token: &str) -> Option<(String, Option<String>)> {
    let token = token.trim_matches(|character: char| "`[](){}<>,.;:'\"".contains(character));
    let (object_id, claim_id) = token
        .split_once('#')
        .map_or((token, None), |(object, claim)| (object, Some(claim)));
    let (type_name, slug) = object_id.split_once('.')?;
    if !TYPES.contains(&type_name)
        || slug.is_empty()
        || !slug.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
    {
        return None;
    }
    if let Some(claim_id) = claim_id {
        if claim_id.is_empty()
            || !claim_id.chars().all(|character| {
                character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
            })
        {
            return None;
        }
    }
    Some((object_id.to_string(), claim_id.map(str::to_string)))
}

fn lint(args: &[String]) -> Result<CommandOutcome, String> {
    let format = option_value(args, "--format")?.unwrap_or_else(|| "text".to_string());
    if format != "text" && format != "json" {
        return Err("--format must be text or json".to_string());
    }

    let diagnostics = collect_lint_diagnostics(args)?;
    print_diagnostics(&diagnostics, &format);
    Ok(
        if diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() == "ERROR")
        {
            CommandOutcome::Errors
        } else {
            CommandOutcome::Clean
        },
    )
}

fn collect_lint_diagnostics(args: &[String]) -> Result<Vec<Diagnostic>, String> {
    let registries = load_registries(Path::new("okf/schema"))?;
    let root = Path::new("okf");
    let mut diagnostics = Vec::new();
    let mut documents = Vec::new();
    for directory in SEMANTIC_FIELDS {
        let directory_path = root.join(directory);
        if !directory_path.exists() {
            diagnostics.push(Diagnostic {
                path: directory_path,
                message: "missing semantic object directory".to_string(),
            });
            continue;
        }
        collect_markdown_files(
            &directory_path,
            &registries,
            &mut documents,
            &mut diagnostics,
        )?;
    }
    validate_graph(&documents, &registries, &mut diagnostics);

    if args.iter().any(|arg| arg == "--history") {
        diagnostics.extend(history_diagnostics()?);
    }
    Ok(diagnostics)
}

fn print_diagnostics(diagnostics: &[Diagnostic], format: &str) {
    if format == "json" {
        let output: Vec<_> = diagnostics
            .iter()
            .map(|diagnostic| {
                serde_json::json!({
                    "code": diagnostic.code(),
                    "severity": diagnostic.severity(),
                    "path": diagnostic.path.to_string_lossy(),
                    "message": diagnostic.message,
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&output).unwrap_or_else(|_| "[]".to_string())
        );
    } else if diagnostics.is_empty() {
        println!("OKF: no diagnostics");
    } else {
        for diagnostic in diagnostics {
            println!(
                "{} {} {}: {}",
                diagnostic.severity(),
                diagnostic.code(),
                diagnostic.path.display(),
                diagnostic.message
            );
        }
    }
}

fn print_source_results(results: &[SourceResult], format: &str) -> Result<(), String> {
    if format == "json" {
        println!(
            "{}",
            serde_json::to_string_pretty(
                &results
                    .iter()
                    .map(|result| serde_json::json!({
                        "code": source_code(result), "severity": result.severity,
                        "path": result.path.to_string_lossy(), "message": result.message,
                    }))
                    .collect::<Vec<_>>()
            )
            .map_err(|error| error.to_string())?
        );
    } else if results.is_empty() {
        println!("OKF: source snapshots match");
    } else {
        for result in results {
            println!(
                "{} {} {}: {}",
                result.severity,
                source_code(result),
                result.path.display(),
                result.message
            );
        }
    }
    Ok(())
}

fn source_code(result: &SourceResult) -> &'static str {
    let message = result.message.to_ascii_lowercase();
    if message.contains("drift") {
        "OKF-W013"
    } else if message.contains("unreachable") {
        "OKF-W014"
    } else {
        "OKF-E200"
    }
}

fn history_diagnostics() -> Result<Vec<Diagnostic>, String> {
    if git_output(&["rev-parse", "--is-shallow-repository"]).as_deref() == Ok("true") {
        return Ok(vec![Diagnostic {
            path: PathBuf::from(".git"),
            message: "history-aware checks require a non-shallow checkout".to_string(),
        }]);
    }
    let commits = git_output(&["rev-list", "--reverse", "--all", "--", "okf"])?;
    if commits.is_empty() {
        return Ok(Vec::new());
    }

    let mut ledger: BTreeMap<String, (String, String, String, bool, String)> = BTreeMap::new();
    let mut previously_observed = HashSet::new();
    let mut deleted_durable_ids = HashSet::new();
    let mut diagnostics = Vec::new();
    for commit in commits.lines() {
        let files = git_output(&["ls-tree", "-r", "--name-only", commit, "--", "okf"])?;
        let mut observed = HashSet::new();
        for file in files.lines().filter(|file| file.ends_with(".md")) {
            let Some(object_type) = file
                .strip_prefix("okf/")
                .and_then(|path| path.split('/').next())
                .and_then(|directory| match directory {
                    "entities" => Some("entity"),
                    "concepts" => Some("concept"),
                    "systems" => Some("system"),
                    "research" => Some("research"),
                    "decisions" => Some("decision"),
                    "references" => Some("reference"),
                    _ => None,
                })
            else {
                continue;
            };
            let content = git_output_bytes(&["show", &format!("{commit}:{file}")])?;
            let document = match parse_document_content(Path::new(file), &content) {
                Ok(document) => document,
                Err(_) => continue,
            };
            let map = &document.frontmatter;
            let Some(id) = map.get("id").and_then(Value::as_str) else {
                continue;
            };
            let lifecycle = map
                .get("lifecycle")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let hash = map
                .get("semantic_hash")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let verified = map.get("verified").is_some();
            observed.insert(id.to_string());
            if deleted_durable_ids.contains(id) {
                diagnostics.push(Diagnostic {
                    path: PathBuf::from(file),
                    message: format!("durable ID {id} was reused after deletion"),
                });
            }
            if let Some((
                previous_type,
                previous_lifecycle,
                previous_path,
                previous_verified,
                previous_hash,
            )) = ledger.get(id)
            {
                if previous_type != object_type {
                    diagnostics.push(Diagnostic {
                        path: PathBuf::from(file),
                        message: format!(
                            "ID {id} changed type from {previous_type} to {object_type}"
                        ),
                    });
                }
                let legal = (previous_lifecycle == "draft"
                    && ["draft", "active", "deprecated"].contains(&lifecycle))
                    || (previous_lifecycle == "active"
                        && ["active", "deprecated"].contains(&lifecycle))
                    || (previous_lifecycle == "deprecated" && lifecycle == "deprecated");
                if !legal {
                    diagnostics.push(Diagnostic { path: PathBuf::from(file), message: format!("illegal lifecycle transition for {id}: {previous_lifecycle} -> {lifecycle}") });
                }
                if *previous_verified && previous_hash != hash {
                    diagnostics.push(Diagnostic {
                        path: PathBuf::from(file),
                        message: format!("verified identity mutated for {id}"),
                    });
                }
                if previous_lifecycle != "draft" && previous_path != file {
                    diagnostics.push(Diagnostic {
                        path: PathBuf::from(file),
                        message: format!("durable ID {id} moved from {previous_path} to {file}"),
                    });
                }
                ledger.insert(
                    id.to_string(),
                    (
                        object_type.to_string(),
                        lifecycle.to_string(),
                        file.to_string(),
                        *previous_verified || verified,
                        hash.to_string(),
                    ),
                );
            } else {
                ledger.insert(
                    id.to_string(),
                    (
                        object_type.to_string(),
                        lifecycle.to_string(),
                        file.to_string(),
                        verified,
                        hash.to_string(),
                    ),
                );
            }
        }
        for id in previously_observed.difference(&observed) {
            if let Some((_, lifecycle, _, verified, _)) = ledger.get(id) {
                if lifecycle != "draft" || *verified {
                    deleted_durable_ids.insert(id.clone());
                    diagnostics.push(Diagnostic {
                        path: PathBuf::from("okf"),
                        message: format!("durable ID {id} was deleted from repository history"),
                    });
                }
            }
        }
        previously_observed = observed;
    }
    Ok(diagnostics)
}

fn git_output_bytes(args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn review(_args: &[String]) -> Result<CommandOutcome, String> {
    let (documents, _) = load_documents()?;
    let review = render_review(&documents);
    let items = review
        .get("items")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for item in &items {
        let kind = item
            .get("kind")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("review");
        let id = item
            .get("id")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default();
        let severity = if kind == "draft" { "NOTICE" } else { "WARNING" };
        let code = if kind == "draft" {
            "OKF-N001"
        } else {
            "OKF-W003"
        };
        println!("{severity} {code} {id}: review condition");
    }
    if items.is_empty() {
        println!("OKF: no review conditions");
    }
    Ok(CommandOutcome::Clean)
}

fn check(_args: &[String]) -> Result<CommandOutcome, String> {
    let lint_result = lint(&[
        "--history".to_string(),
        "--format".to_string(),
        "text".to_string(),
    ])?;
    let generate_result = generate(&["--check".to_string()])?;
    let source_result = source_check(&[
        "--scope".to_string(),
        "repo".to_string(),
        "--format".to_string(),
        "text".to_string(),
    ])?;
    let review_result = review(&[])?;
    Ok(
        if [lint_result, generate_result, source_result, review_result]
            .contains(&CommandOutcome::Errors)
        {
            CommandOutcome::Errors
        } else {
            CommandOutcome::Clean
        },
    )
}

fn rehash(args: &[String]) -> Result<CommandOutcome, String> {
    let write = args.iter().any(|arg| arg == "--write");
    let mut changed = 0;
    for directory in SEMANTIC_FIELDS {
        let path = Path::new("okf").join(directory);
        if !path.exists() {
            continue;
        }
        for entry in fs::read_dir(&path).map_err(|error| error.to_string())? {
            let file = entry.map_err(|error| error.to_string())?.path();
            if file.extension().and_then(|value| value.to_str()) != Some("md") {
                continue;
            }
            let document = parse_document(&file)?;
            let mut frontmatter = document.frontmatter.clone();
            if let Some(claims) = frontmatter
                .get_mut("claims")
                .and_then(Value::as_sequence_mut)
            {
                for claim in claims {
                    let hash =
                        claim_hash(claim.as_mapping().ok_or_else(|| {
                            format!("{}: claim must be a mapping", file.display())
                        })?)?;
                    if let Some(map) = claim.as_mapping_mut() {
                        map.insert(Value::String("semantic_hash".into()), Value::String(hash));
                    }
                }
            }
            let hash = object_hash(&frontmatter, &document.body)?;
            frontmatter.insert(Value::String("semantic_hash".into()), Value::String(hash));
            changed += 1;
            if write {
                let yaml =
                    serde_yaml::to_string(&frontmatter).map_err(|error| error.to_string())?;
                fs::write(&file, format!("---\n{yaml}---\n{}", document.body))
                    .map_err(|error| format!("{}: {error}", file.display()))?;
            }
        }
    }
    if write {
        println!("OKF: recomputed hashes for {changed} objects");
    } else {
        println!("OKF: {changed} objects would be rehashed (use --write)");
    }
    Ok(CommandOutcome::Clean)
}

fn source_check(args: &[String]) -> Result<CommandOutcome, String> {
    let format = option_value(args, "--format")?.unwrap_or_else(|| "text".to_string());
    if format != "text" && format != "json" {
        return Err("--format must be text or json".to_string());
    }
    let scope = option_value(args, "--scope")?.unwrap_or_else(|| "repo".to_string());
    let write = args.iter().any(|arg| arg == "--write");
    if !["repo", "remote", "all"].contains(&scope.as_str()) {
        return Err("--scope must be repo, remote, or all".to_string());
    }
    let registries = load_registries(Path::new("okf/schema"))?;
    let mut documents = Vec::new();
    let mut diagnostics = Vec::new();
    for directory in SEMANTIC_FIELDS {
        let directory_path = Path::new("okf").join(directory);
        if directory_path.exists() {
            collect_markdown_files(
                &directory_path,
                &registries,
                &mut documents,
                &mut diagnostics,
            )?;
        }
    }
    let mut results = Vec::new();
    for document in &documents {
        if document.frontmatter.get("type").and_then(Value::as_str) != Some("reference") {
            continue;
        }
        if let Some(source) = document.frontmatter.get("source") {
            let kind = source
                .get("kind")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let is_remote = ["github_issue", "github_pull_request", "external_url"].contains(&kind);
            if (scope == "repo" && !is_remote) || (scope != "repo") {
                results.extend(check_reference_source(source, &document.path));
            }
        }
    }
    if write {
        write_source_freshness(&documents, &results)?;
    }
    print_source_results(&results, &format)?;
    Ok(if results.iter().any(|result| result.severity == "ERROR") {
        CommandOutcome::Errors
    } else {
        CommandOutcome::Clean
    })
}

fn write_source_freshness(
    documents: &[SemanticDocument],
    results: &[SourceResult],
) -> Result<(), String> {
    let checked_at = current_utc_timestamp();
    for document in documents {
        if document.frontmatter.get("type").and_then(Value::as_str) != Some("reference") {
            continue;
        }
        let kind = document
            .frontmatter
            .get("source")
            .and_then(|source| source.get("kind"))
            .and_then(Value::as_str)
            .unwrap_or_default();
        if ["github_issue", "github_pull_request", "external_url"].contains(&kind) {
            continue;
        }
        let result = results.iter().find(|result| result.path == document.path);
        let mut frontmatter = document.frontmatter.clone();
        let mut freshness = Mapping::new();
        match result {
            Some(result) if result.severity == "ERROR" => {
                freshness.insert(Value::String("state".into()), Value::String("stale".into()));
                freshness.insert(
                    Value::String("detected_at".into()),
                    Value::String(checked_at.clone()),
                );
                freshness.insert(
                    Value::String("reason".into()),
                    Value::String(result.message.clone()),
                );
            }
            _ => {
                freshness.insert(Value::String("state".into()), Value::String("fresh".into()));
                freshness.insert(
                    Value::String("checked_at".into()),
                    Value::String(checked_at.clone()),
                );
            }
        }
        frontmatter.insert(Value::String("freshness".into()), Value::Mapping(freshness));
        frontmatter.insert(
            Value::String("updated_at".into()),
            Value::String(checked_at.clone()),
        );
        let yaml = serde_yaml::to_string(&frontmatter).map_err(|error| error.to_string())?;
        let content = format!("---\n{yaml}---\n{}", document.body);
        fs::write(&document.path, content)
            .map_err(|error| format!("{}: {error}", document.path.display()))?;
    }
    Ok(())
}

fn current_utc_timestamp() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let days = seconds.div_euclid(86_400);
    let day_seconds = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        day_seconds / 3_600,
        (day_seconds % 3_600) / 60,
        day_seconds % 60
    )
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 }.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096).div_euclid(365);
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2).div_euclid(153);
    let day = doy - (153 * mp + 2).div_euclid(5) + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    (year + if month <= 2 { 1 } else { 0 }, month, day)
}

fn generate(args: &[String]) -> Result<CommandOutcome, String> {
    let (documents, registries) = load_documents()?;
    let catalog = render_catalog(&documents);
    let graph = render_graph(&documents, &registries);
    let review = render_review(&documents);
    let index = render_index(&documents, &review);
    let outputs = [
        (Path::new("okf/index.md"), index.into_bytes(), "OKF-E420"),
        (
            Path::new("okf/generated/catalog.json"),
            json_bytes(&catalog)?,
            "OKF-E421",
        ),
        (
            Path::new("okf/generated/graph.json"),
            json_bytes(&graph)?,
            "OKF-E422",
        ),
        (
            Path::new("okf/generated/review.json"),
            json_bytes(&review)?,
            "OKF-E423",
        ),
        (
            Path::new("okf/generated/review.md"),
            render_review_markdown(&review).into_bytes(),
            "OKF-E424",
        ),
    ];
    if args.iter().any(|arg| arg == "--check") {
        let mut drift = Vec::new();
        for (path, expected, code) in outputs {
            if fs::read(path)
                .map(|actual| actual != expected)
                .unwrap_or(true)
            {
                drift.push(Diagnostic {
                    path: path.to_path_buf(),
                    message: format!("generated output drift ({code})"),
                });
            }
        }
        print_diagnostics(&drift, "text");
        return Ok(if drift.is_empty() {
            CommandOutcome::Clean
        } else {
            CommandOutcome::Errors
        });
    }
    for (path, bytes, _) in outputs {
        fs::write(path, bytes).map_err(|error| error.to_string())?;
    }
    println!("OKF: generated deterministic views");
    Ok(CommandOutcome::Clean)
}

fn retrieve(args: &[String]) -> Result<CommandOutcome, String> {
    let (documents, _) = load_documents()?;
    if let Some(key) = option_value(args, "--decision-key")? {
        let decisions: Vec<&SemanticDocument> = documents
            .iter()
            .filter(|document| {
                document.frontmatter.get("type").and_then(Value::as_str) == Some("decision")
                    && document
                        .frontmatter
                        .get("decision_key")
                        .and_then(Value::as_str)
                        == Some(key.as_str())
                    && document
                        .frontmatter
                        .get("lifecycle")
                        .and_then(Value::as_str)
                        == Some("active")
            })
            .collect();
        match decisions.as_slice() {
            [] => println!("No active Decision for decision_key {key}"),
            [document] => {
                let map = &document.frontmatter;
                let id = map.get("id").and_then(Value::as_str).unwrap_or_default();
                println!(
                    "{id} [active; effective_freshness={}] {}",
                    effective_freshness(id, &documents),
                    map.get("title").and_then(Value::as_str).unwrap_or_default()
                );
            }
            _ => println!("Structural error: multiple active Decisions for decision_key {key}"),
        }
        return Ok(CommandOutcome::Clean);
    }
    let query = option_value(args, "--id")?
        .or(option_value(args, "--query")?)
        .ok_or_else(|| "retrieve requires --id, --decision-key, or --query".to_string())?;
    let mut matches: Vec<&SemanticDocument> = documents
        .iter()
        .filter(|document| {
            let map = &document.frontmatter;
            map.get("id").and_then(Value::as_str) == Some(query.as_str())
                || map.get("decision_key").and_then(Value::as_str) == Some(query.as_str())
                || map
                    .get("title")
                    .and_then(Value::as_str)
                    .is_some_and(|title| title.eq_ignore_ascii_case(&query))
                || map
                    .get("description")
                    .and_then(Value::as_str)
                    .is_some_and(|description| {
                        description.to_lowercase().contains(&query.to_lowercase())
                    })
        })
        .collect();
    matches.sort_by_key(|document| {
        (
            document
                .frontmatter
                .get("type")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            document
                .frontmatter
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
        )
    });
    if matches.is_empty() {
        println!("No OKF match for {query}");
    } else {
        for document in matches {
            let map = &document.frontmatter;
            let id = map.get("id").and_then(Value::as_str).unwrap_or_default();
            let lifecycle = map
                .get("lifecycle")
                .and_then(Value::as_str)
                .unwrap_or_default();
            println!(
                "{id} [{lifecycle}; effective_freshness={}] {}",
                effective_freshness(id, &documents),
                map.get("title").and_then(Value::as_str).unwrap_or_default()
            );
        }
    }
    Ok(CommandOutcome::Clean)
}

fn load_documents() -> Result<(Vec<SemanticDocument>, Registries), String> {
    let registries = load_registries(Path::new("okf/schema"))?;
    let mut documents = Vec::new();
    let mut diagnostics = Vec::new();
    for directory in SEMANTIC_FIELDS {
        let directory_path = Path::new("okf").join(directory);
        if directory_path.exists() {
            collect_markdown_files(
                &directory_path,
                &registries,
                &mut documents,
                &mut diagnostics,
            )?;
        }
    }
    if let Some(diagnostic) = diagnostics.first() {
        return Err(format!(
            "{}: {}",
            diagnostic.path.display(),
            diagnostic.message
        ));
    }
    Ok((documents, registries))
}

fn json_bytes(value: &serde_json::Value) -> Result<Vec<u8>, String> {
    let text = serde_json::to_string_pretty(value).map_err(|error| error.to_string())?;
    Ok(format!("{text}\n").into_bytes())
}

fn render_catalog(documents: &[SemanticDocument]) -> serde_json::Value {
    let mut entries: Vec<serde_json::Value> = documents
        .iter()
        .map(|document| {
            let map = &document.frontmatter;
            let mut entry = serde_json::Map::new();
            for field in [
                "id",
                "type",
                "title",
                "description",
                "lifecycle",
                "semantic_hash",
            ] {
                if let Some(value) = map.get(field) {
                    entry.insert(field.to_string(), yaml_to_json_lossless(value));
                }
            }
            entry.insert(
                "path".to_string(),
                serde_json::Value::String(document.path.to_string_lossy().replace('\\', "/")),
            );
            for field in [
                "aliases",
                "definition_claim",
                "behavior_claims",
                "conclusion_claims",
                "decision_claim",
                "decision_key",
                "research_question",
            ] {
                if let Some(value) = map.get(field) {
                    entry.insert(field.to_string(), yaml_to_json_lossless(value));
                }
            }
            if let Some(kind) = map.get("source").and_then(|source| source.get("kind")) {
                entry.insert("source_kind".to_string(), yaml_to_json_lossless(kind));
            }
            entry.insert(
                "effective_freshness".to_string(),
                serde_json::Value::String(effective_freshness(
                    map.get("id").and_then(Value::as_str).unwrap_or_default(),
                    documents,
                )),
            );
            serde_json::Value::Object(entry)
        })
        .collect();
    entries.sort_by_key(|entry| {
        (
            entry
                .get("type")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default()
                .to_string(),
            entry
                .get("id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default()
                .to_string(),
        )
    });
    serde_json::json!({ "objects": entries })
}

fn render_graph(documents: &[SemanticDocument], registries: &Registries) -> serde_json::Value {
    let mut authored = Vec::new();
    let mut generated = Vec::new();
    for document in documents {
        let source = document
            .frontmatter
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        collect_graph_edges(
            document.frontmatter.get("relations"),
            source,
            &document.path,
            registries,
            &mut authored,
            &mut generated,
        );
        if let Some(claims) = document
            .frontmatter
            .get("claims")
            .and_then(Value::as_sequence)
        {
            for claim in claims {
                let claim_id = claim.get("id").and_then(Value::as_str).unwrap_or_default();
                collect_graph_edges(
                    claim.get("relations"),
                    &format!("{source}#{claim_id}"),
                    &document.path,
                    registries,
                    &mut authored,
                    &mut generated,
                );
            }
        }
    }
    authored.sort_by_key(edge_sort_key);
    generated.sort_by_key(edge_sort_key);
    serde_json::json!({ "authored": authored, "generated": generated })
}

fn collect_graph_edges(
    value: Option<&Value>,
    source: &str,
    path: &Path,
    registries: &Registries,
    authored: &mut Vec<serde_json::Value>,
    generated: &mut Vec<serde_json::Value>,
) {
    let Some(relations) = value.and_then(Value::as_sequence) else {
        return;
    };
    for relation in relations {
        let Some(map) = relation.as_mapping() else {
            continue;
        };
        let Some(name) = map.get("type").and_then(Value::as_str) else {
            continue;
        };
        let Some(target_map) = map.get("target").and_then(Value::as_mapping) else {
            continue;
        };
        let Some(target_object) = target_map.get("object").and_then(Value::as_str) else {
            continue;
        };
        let target = if let Some(claim) = target_map.get("claim").and_then(Value::as_str) {
            format!("{target_object}#{claim}")
        } else {
            target_object.to_string()
        };
        authored.push(serde_json::json!({
            "source": source,
            "relation": name,
            "target": target,
            "selectors": map.get("selectors").map(yaml_to_json_lossless).unwrap_or(serde_json::Value::Null),
            "note": map.get("note").map(yaml_to_json_lossless).unwrap_or(serde_json::Value::Null),
            "path": path.to_string_lossy().replace('\\', "/"),
        }));
        if let Some(rule) = registries.relations.get(name) {
            generated.push(serde_json::json!({
                "source": target,
                "relation": rule.inverse,
                "target": source,
                "generated_from": name,
                "path": path.to_string_lossy().replace('\\', "/"),
            }));
        }
    }
}

fn edge_sort_key(edge: &serde_json::Value) -> (String, String, String) {
    (
        edge.get("source")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_string(),
        edge.get("relation")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_string(),
        edge.get("target")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_string(),
    )
}

fn render_review(documents: &[SemanticDocument]) -> serde_json::Value {
    let mut items = Vec::new();
    for document in documents {
        let map = &document.frontmatter;
        let id = map.get("id").and_then(Value::as_str).unwrap_or_default();
        let lifecycle = map
            .get("lifecycle")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let effective = effective_freshness(id, documents);
        if lifecycle == "draft" {
            items.push(serde_json::json!({"kind": "draft", "id": id, "message": "draft knowledge is not settled current truth"}));
        }
        if effective == "stale" || effective == "unknown" {
            items.push(serde_json::json!({"kind": "freshness", "id": id, "effective_freshness": effective}));
        }
    }
    items.sort_by_key(|item| {
        (
            item.get("kind")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default()
                .to_string(),
            item.get("id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default()
                .to_string(),
        )
    });
    serde_json::json!({ "items": items })
}

fn render_review_markdown(review: &serde_json::Value) -> String {
    let mut output = String::from("# OKF Review\n\n> Generated. Do not edit directly.\n\n");
    let items = review
        .get("items")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    if items.is_empty() {
        output.push_str("No review items.\n");
    } else {
        for item in items {
            output.push_str(&format!(
                "- {} {}\n",
                item.get("kind")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or_default(),
                item.get("id")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or_default()
            ));
        }
    }
    output
}

fn render_index(documents: &[SemanticDocument], review: &serde_json::Value) -> String {
    let mut output = String::from(
        "# map-analyzer-custom OKF Project Brain\n\n> Generated. Do not edit directly.\n\n",
    );
    output.push_str("## Curated routes\n\n");
    output.push_str(&render_curated_routes());
    for (heading, lifecycle, type_name) in [
        ("Active systems", "active", "system"),
        ("Current decisions", "active", "decision"),
        ("Active concepts", "active", "concept"),
        ("Active entities", "active", "entity"),
        ("Active research", "active", "research"),
        ("Active references", "active", "reference"),
        ("Historical / deprecated", "deprecated", ""),
        ("Draft knowledge", "draft", ""),
    ] {
        output.push_str(&format!("## {heading}\n\n"));
        let mut matches: Vec<&SemanticDocument> = documents
            .iter()
            .filter(|document| {
                let map = &document.frontmatter;
                map.get("lifecycle").and_then(Value::as_str) == Some(lifecycle)
                    && (type_name.is_empty()
                        || map.get("type").and_then(Value::as_str) == Some(type_name))
            })
            .collect();
        matches.sort_by_key(|document| {
            document
                .frontmatter
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string()
        });
        if matches.is_empty() {
            output.push_str("- None.\n\n");
        } else {
            for document in matches {
                let map = &document.frontmatter;
                output.push_str(&format!(
                    "- {} — {} ({})\n",
                    map.get("id").and_then(Value::as_str).unwrap_or_default(),
                    map.get("title").and_then(Value::as_str).unwrap_or_default(),
                    map.get("description")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                ));
            }
            output.push('\n');
        }
    }
    output.push_str("## Needs review\n\n");
    if review
        .get("items")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|items| !items.is_empty())
    {
        output.push_str("- See generated/review.md.\n");
    } else {
        output.push_str("- None.\n");
    }
    output
}

fn render_curated_routes() -> String {
    let directory = Path::new("okf/routes");
    let Ok(entries) = fs::read_dir(directory) else {
        return "- No curated routes.\n\n".to_string();
    };
    let mut routes = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        let normalized = content.replace("\r\n", "\n");
        let Some(end) = normalized[4..].find("\n---\n") else {
            continue;
        };
        let yaml = &normalized[4..4 + end];
        let Ok(value) = serde_yaml::from_str::<Value>(yaml) else {
            continue;
        };
        let Some(map) = value.as_mapping() else {
            continue;
        };
        let route_id = map
            .get("route_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let title = map.get("title").and_then(Value::as_str).unwrap_or(route_id);
        let description = map
            .get("description")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if route_id.is_empty() {
            continue;
        }
        routes.push((
            route_id.to_string(),
            title.to_string(),
            description.to_string(),
            path,
        ));
    }
    routes.sort_by_key(|route| route.0.clone());
    if routes.is_empty() {
        return "- No curated routes.\n\n".to_string();
    }
    let mut output = String::new();
    for (_, title, description, path) in routes {
        let route_path = path.to_string_lossy().replace('\\', "/");
        let relative = route_path.strip_prefix("okf/").unwrap_or(&route_path);
        output.push_str(&format!("- [{title}]({relative}) — {description}\n"));
    }
    output.push('\n');
    output
}

fn yaml_to_json_lossless(value: &Value) -> serde_json::Value {
    yaml_to_canonical_json(value).unwrap_or(serde_json::Value::Null)
}

fn effective_freshness(id: &str, documents: &[SemanticDocument]) -> String {
    let mut visiting = HashSet::new();
    effective_freshness_inner(id, documents, &mut visiting)
}

fn effective_freshness_inner(
    id: &str,
    documents: &[SemanticDocument],
    visiting: &mut HashSet<String>,
) -> String {
    if !visiting.insert(id.to_string()) {
        return "unknown".to_string();
    }
    let Some(document) = documents
        .iter()
        .find(|document| document.frontmatter.get("id").and_then(Value::as_str) == Some(id))
    else {
        return "unknown".to_string();
    };
    let local = document
        .frontmatter
        .get("freshness")
        .and_then(|value| value.get("state"))
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    if local == "stale" {
        return "stale".to_string();
    }
    let mut dependency_unknown = local == "unknown";
    let mut dependency_present = false;
    for relation in all_relations(&document.frontmatter) {
        let name = relation
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if !["derived_from", "supported_by", "depends_on"].contains(&name) {
            continue;
        }
        let Some(target) = relation
            .get("target")
            .and_then(|target| target.get("object"))
            .and_then(Value::as_str)
        else {
            continue;
        };
        dependency_present = true;
        match effective_freshness_inner(target, documents, visiting).as_str() {
            "stale" => return "stale".to_string(),
            "unknown" => dependency_unknown = true,
            _ => {}
        }
    }
    if dependency_unknown {
        "unknown".to_string()
    } else if local == "not_applicable" && !dependency_present {
        "not_applicable".to_string()
    } else {
        "fresh".to_string()
    }
}

fn all_relations(map: &Mapping) -> Vec<&Mapping> {
    map.get("relations")
        .and_then(Value::as_sequence)
        .map(|items| items.iter().filter_map(Value::as_mapping).collect())
        .unwrap_or_default()
}

fn collect_markdown_files(
    directory: &Path,
    registries: &Registries,
    documents: &mut Vec<SemanticDocument>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(), String> {
    for entry in
        fs::read_dir(directory).map_err(|error| format!("{}: {error}", directory.display()))?
    {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            diagnostics.push(Diagnostic {
                path,
                message: "semantic objects must be directly inside their type directory"
                    .to_string(),
            });
        } else if path.extension().is_some_and(|extension| extension == "md") {
            match parse_document(&path).and_then(|document| {
                validate_document_with_registries(&document, registries)?;
                Ok(document)
            }) {
                Ok(document) => documents.push(document),
                Err(message) => diagnostics.push(Diagnostic { path, message }),
            }
        }
    }
    Ok(())
}

fn load_registries(directory: &Path) -> Result<Registries, String> {
    let relations = load_yaml_file(&directory.join("relations.yaml"))?;
    let types = load_yaml_file(&directory.join("types.yaml"))?;
    let relations = parse_relation_registry(&relations)?;
    let types = parse_type_registry(&types)?;
    validate_registries(&relations, &types)?;
    Ok(Registries { relations, types })
}

fn load_yaml_file(path: &Path) -> Result<Value, String> {
    let content =
        fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_yaml::from_str(&content)
        .map_err(|error| format!("{}: invalid YAML: {error}", path.display()))
}

fn parse_relation_registry(value: &Value) -> Result<BTreeMap<String, RelationRule>, String> {
    let entries = value
        .get("relations")
        .and_then(Value::as_sequence)
        .ok_or_else(|| "relations registry must contain a relations sequence".to_string())?;
    let mut result = BTreeMap::new();
    for entry in entries {
        let map = entry
            .as_mapping()
            .ok_or_else(|| "each relation registry entry must be a mapping".to_string())?;
        let name = value_string(map, "name")?;
        if !valid_slug(&name) || result.contains_key(&name) {
            return Err(format!("invalid or duplicate relation name: {name}"));
        }
        let inverse = value_string(map, "inverse")?;
        let rule = RelationRule {
            inverse,
            source_claim: value_bool(map, "source_claim")?,
            target_claim: value_bool(map, "target_claim")?,
            source_types: value_strings(map, "source_types")?,
            target_types: value_strings(map, "target_types")?,
            selectors: value_bool(map, "selectors")?,
            self_edges_forbidden: value_string(map, "self_edges")? == "forbidden",
            acyclic: value_bool(map, "acyclic")?,
        };
        result.insert(name, rule);
    }
    Ok(result)
}

fn parse_type_registry(value: &Value) -> Result<BTreeMap<String, TypeRule>, String> {
    let entries = value
        .get("types")
        .and_then(Value::as_sequence)
        .ok_or_else(|| "types registry must contain a types sequence".to_string())?;
    let mut result = BTreeMap::new();
    for entry in entries {
        let map = entry
            .as_mapping()
            .ok_or_else(|| "each type registry entry must be a mapping".to_string())?;
        let name = value_string(map, "name")?;
        if !TYPES.contains(&name.as_str()) || result.contains_key(&name) {
            return Err(format!("invalid or duplicate semantic type: {name}"));
        }
        result.insert(
            name,
            TypeRule {
                required_fields: value_strings(map, "required_fields")?,
                forbidden_fields: value_strings(map, "forbidden_fields")?,
                canonical_claim: value_optional_string(map, "canonical_claim")?,
                canonical_claims: value_bool(map, "canonical_claims")?,
                active_sections: value_strings(map, "active_sections")?,
            },
        );
    }
    Ok(result)
}

fn validate_registries(
    relations: &BTreeMap<String, RelationRule>,
    types: &BTreeMap<String, TypeRule>,
) -> Result<(), String> {
    if types.len() != TYPES.len() || TYPES.iter().any(|name| !types.contains_key(*name)) {
        return Err(
            "types registry must define each of the six semantic types exactly once".to_string(),
        );
    }
    let names: HashSet<_> = relations.keys().cloned().collect();
    let mut inverses = HashSet::new();
    for (name, rule) in relations {
        if name == &rule.inverse || !valid_slug(&rule.inverse) || names.contains(&rule.inverse) {
            return Err(format!(
                "relation {name} has an invalid or authored inverse"
            ));
        }
        if !inverses.insert(&rule.inverse) {
            return Err(format!(
                "relation inverse names must be unique: {}",
                rule.inverse
            ));
        }
        if !rule.self_edges_forbidden {
            return Err(format!("relation {name} must forbid exact self-edges"));
        }
        if rule
            .source_types
            .iter()
            .chain(rule.target_types.iter())
            .any(|ty| !TYPES.contains(&ty.as_str()))
        {
            return Err(format!("relation {name} contains an unknown endpoint type"));
        }
    }
    Ok(())
}

fn parse_document(path: &Path) -> Result<SemanticDocument, String> {
    let content = fs::read_to_string(path).map_err(|error| error.to_string())?;
    parse_document_content(path, &content)
}

fn parse_document_content(path: &Path, content: &str) -> Result<SemanticDocument, String> {
    let normalized = content.replace("\r\n", "\n");
    if !normalized.starts_with("---\n") {
        return Err("semantic object must start with YAML frontmatter delimiter".to_string());
    }

    let remaining = &normalized[4..];
    let delimiter = remaining
        .match_indices("\n---\n")
        .next()
        .map(|(index, _)| index)
        .ok_or_else(|| {
            "semantic object is missing closing YAML frontmatter delimiter".to_string()
        })?;
    let yaml = &remaining[..delimiter];
    if yaml
        .lines()
        .any(|line| line.trim_start().starts_with('&') || line.trim_start().starts_with('*'))
    {
        return Err("YAML anchors and aliases are not supported".to_string());
    }
    let frontmatter: Value =
        serde_yaml::from_str(yaml).map_err(|error| format!("invalid YAML: {error}"))?;
    let frontmatter = frontmatter
        .as_mapping()
        .cloned()
        .ok_or_else(|| "frontmatter must be a YAML mapping".to_string())?;
    let body_start = 4 + delimiter + "\n---\n".len();

    Ok(SemanticDocument {
        path: path.to_path_buf(),
        frontmatter,
        body: normalized[body_start..].to_string(),
    })
}

fn validate_document(document: &SemanticDocument) -> Result<(), String> {
    let map = &document.frontmatter;
    for field in [
        "id",
        "type",
        "title",
        "description",
        "lifecycle",
        "created_at",
        "updated_at",
        "generated",
        "semantic_hash",
    ] {
        require(map, field)?;
    }

    let object_type = string_field(map, "type")?;
    if !TYPES.contains(&object_type.as_str()) {
        return Err(format!("type must be one of: {}", TYPES.join(", ")));
    }
    let id = string_field(map, "id")?;
    validate_id(&id, &object_type)?;
    validate_filename(&document.path)?;
    validate_directory_type(&document.path, &object_type)?;

    for field in ["title", "description"] {
        if string_field(map, field)?.trim().is_empty() {
            return Err(format!("{field} must not be empty"));
        }
    }
    validate_lifecycle(string_field(map, "lifecycle")?.as_str())?;
    validate_timestamp(string_field(map, "created_at")?.as_str())?;
    validate_timestamp(string_field(map, "updated_at")?.as_str())?;
    validate_generated(map.get(Value::String("generated".to_string())))?;
    validate_hash(string_field(map, "semantic_hash")?.as_str())?;
    validate_freshness(map.get(Value::String("freshness".to_string())))?;

    if object_type == "reference" && map.contains_key(Value::String("claims".to_string())) {
        return Err("Reference objects must not contain claims".to_string());
    }
    if let Some(claims) = map.get(Value::String("claims".to_string())) {
        validate_claims(claims)?;
    }
    if let Some(verified) = map.get(Value::String("verified".to_string())) {
        validate_verified(verified)?;
    }

    let expected_hash = string_field(map, "semantic_hash")?;
    let actual_hash = object_hash(map, &document.body)?;
    if expected_hash != actual_hash {
        return Err(format!(
            "semantic_hash mismatch: expected {expected_hash}, computed {actual_hash}"
        ));
    }
    Ok(())
}

fn validate_document_with_registries(
    document: &SemanticDocument,
    registries: &Registries,
) -> Result<(), String> {
    validate_document(document)?;
    let map = &document.frontmatter;
    let object_type = string_field(map, "type")?;
    let rule = registries
        .types
        .get(&object_type)
        .ok_or_else(|| format!("type registry has no rule for {object_type}"))?;
    for field in &rule.required_fields {
        require(map, field)?;
    }
    for field in &rule.forbidden_fields {
        if map.contains_key(Value::String(field.clone())) {
            return Err(format!("{object_type} objects must not contain {field}"));
        }
    }
    validate_type_contract(document, rule)?;
    if object_type == "reference" {
        validate_reference_source(map, &string_field(map, "lifecycle")?)?;
    }
    validate_relations_shape(map, &object_type, registries)?;
    Ok(())
}

fn validate_type_contract(document: &SemanticDocument, rule: &TypeRule) -> Result<(), String> {
    let map = &document.frontmatter;
    let lifecycle = string_field(map, "lifecycle")?;
    let claims = map
        .get(Value::String("claims".to_string()))
        .and_then(Value::as_sequence);
    let claim_ids: HashSet<String> = claims
        .into_iter()
        .flatten()
        .filter_map(|claim| {
            claim
                .get("id")
                .and_then(Value::as_str)
                .map(ToString::to_string)
        })
        .collect();
    if let Some(field) = &rule.canonical_claim {
        if let Some(pointer) = map.get(Value::String(field.clone())) {
            let pointer = pointer
                .as_str()
                .ok_or_else(|| format!("{field} must be a string"))?;
            if !claim_ids.contains(pointer) {
                return Err(format!("{field} must point to an existing claim"));
            }
            if lifecycle == "active" && !claim_is_active(claims, pointer) {
                return Err(format!("active {field} must point to an active claim"));
            }
        } else if lifecycle == "active" {
            return Err(format!("active object requires {field}"));
        }
    }
    if rule.canonical_claims {
        let field = if map.contains_key(Value::String("behavior_claims".to_string())) {
            "behavior_claims"
        } else {
            "conclusion_claims"
        };
        if let Some(pointer) = map.get(Value::String(field.to_string())) {
            let pointers = pointer
                .as_sequence()
                .ok_or_else(|| format!("{field} must be a sequence"))?;
            if lifecycle == "active" && pointers.is_empty() {
                return Err(format!("active {field} must not be empty"));
            }
            for pointer in pointers {
                let pointer = pointer
                    .as_str()
                    .ok_or_else(|| format!("{field} entries must be strings"))?;
                if !claim_ids.contains(pointer)
                    || (lifecycle == "active" && !claim_is_active(claims, pointer))
                {
                    return Err(format!("{field} must point to active existing claims"));
                }
            }
        } else if lifecycle == "active" {
            return Err(format!("active object requires {field}"));
        }
    }
    if lifecycle == "active" {
        for section in &rule.active_sections {
            if !markdown_has_heading(&document.body, section) {
                return Err(format!(
                    "active object is missing required section: {section}"
                ));
            }
        }
        if map.get("type") == Some(&Value::String("decision".to_string())) {
            let decision_key = string_field(map, "decision_key")?;
            if !valid_decision_key(&decision_key) {
                return Err(
                    "decision_key must use lowercase dot- or hyphen-separated segments".to_string(),
                );
            }
            let claim_id = map
                .get("decision_claim")
                .and_then(Value::as_str)
                .ok_or_else(|| "active Decision requires decision_claim".to_string())?;
            let claim = claims
                .into_iter()
                .flatten()
                .find(|claim| claim.get("id").and_then(Value::as_str) == Some(claim_id))
                .ok_or_else(|| "decision_claim must point to an existing claim".to_string())?;
            if claim.get("load_bearing") != Some(&Value::Bool(true)) {
                return Err("active Decision decision_claim must be load-bearing".to_string());
            }
            require(map, "decided_at")?;
            validate_timestamp(string_field(map, "decided_at")?.as_str())?;
        }
    }
    Ok(())
}

fn validate_reference_source(map: &Mapping, lifecycle: &str) -> Result<(), String> {
    let source = map
        .get("source")
        .and_then(Value::as_mapping)
        .ok_or_else(|| "Reference source must be a mapping".to_string())?;
    let kind = value_string(source, "kind")?;
    if !REFERENCE_KINDS.contains(&kind.as_str()) {
        return Err(format!(
            "Reference source.kind must be one of: {}",
            REFERENCE_KINDS.join(", ")
        ));
    }
    match kind.as_str() {
        "repository_file" => {
            validate_repository_locator(source)?;
            validate_snapshot_commit(source, "blob_sha")?;
        }
        "repository_tree" => {
            validate_repository_locator(source)?;
            validate_snapshot_commit(source, "tree_sha")?;
        }
        "git_commit" => {
            validate_repository(source)?;
            validate_sha(value_string(source, "commit")?.as_str(), "commit")?;
        }
        "git_ref" => {
            validate_repository(source)?;
            let ref_type = value_string(source, "ref_type")?;
            if !["branch", "tag"].contains(&ref_type.as_str()) {
                return Err("git_ref ref_type must be branch or tag".to_string());
            }
            value_string(source, "ref")?;
            if lifecycle == "active" {
                validate_snapshot_resolved_commit(source)?;
            }
        }
        "github_issue" | "github_pull_request" => {
            validate_repository(source)?;
            let number = source
                .get("number")
                .and_then(Value::as_i64)
                .ok_or_else(|| "GitHub Reference number must be an integer".to_string())?;
            if number <= 0 {
                return Err("GitHub Reference number must be positive".to_string());
            }
            validate_snapshot_hash(source, "content_hash")?;
            if kind == "github_pull_request" {
                if let Some(snapshot) = source.get("snapshot").and_then(Value::as_mapping) {
                    if let Some(commit) = snapshot.get("head_commit").and_then(Value::as_str) {
                        validate_sha(commit, "head_commit")?;
                    }
                    if let Some(commit) = snapshot.get("merge_commit").and_then(Value::as_str) {
                        validate_sha(commit, "merge_commit")?;
                    }
                }
            }
        }
        "artifact" => {
            value_string(source, "locator")?;
            value_string(source, "media_type")?;
            validate_snapshot_hash(source, "content_hash")?;
        }
        "external_url" => {
            let url = value_string(source, "url")?;
            if !(url.starts_with("http://") || url.starts_with("https://")) {
                return Err("external_url url must use http:// or https://".to_string());
            }
            validate_snapshot_hash(source, "content_hash")?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn validate_repository_locator(source: &Mapping) -> Result<(), String> {
    validate_repository(source)?;
    let path = value_string(source, "path")?;
    if path.starts_with('/')
        || path.contains('\\')
        || path.split('/').any(|part| part == ".." || part.is_empty())
    {
        return Err(
            "Reference path must be repository-relative, use / separators, and contain no .."
                .to_string(),
        );
    }
    Ok(())
}

fn validate_repository(source: &Mapping) -> Result<(), String> {
    let repository = value_string(source, "repository")?;
    if repository.is_empty()
        || repository.split('/').count() != 2
        || repository.contains(char::is_whitespace)
    {
        return Err("Reference repository must use owner/name form".to_string());
    }
    Ok(())
}

fn validate_snapshot_commit(source: &Mapping, hash_field: &str) -> Result<(), String> {
    let snapshot = source
        .get("snapshot")
        .and_then(Value::as_mapping)
        .ok_or_else(|| "anchored Reference requires a snapshot mapping".to_string())?;
    validate_sha(
        value_string(snapshot, "commit")?.as_str(),
        "snapshot.commit",
    )?;
    validate_sha(
        value_string(snapshot, hash_field)?.as_str(),
        &format!("snapshot.{hash_field}"),
    )
}

fn validate_snapshot_resolved_commit(source: &Mapping) -> Result<(), String> {
    let snapshot = source
        .get("snapshot")
        .and_then(Value::as_mapping)
        .ok_or_else(|| "active git_ref requires snapshot.resolved_commit".to_string())?;
    validate_sha(
        value_string(snapshot, "resolved_commit")?.as_str(),
        "snapshot.resolved_commit",
    )
}

fn validate_snapshot_hash(source: &Mapping, hash_field: &str) -> Result<(), String> {
    let snapshot = source
        .get("snapshot")
        .and_then(Value::as_mapping)
        .ok_or_else(|| "Reference requires a snapshot mapping".to_string())?;
    validate_timestamp(value_string(snapshot, "captured_at")?.as_str())?;
    value_string(snapshot, "method")?;
    validate_hash(value_string(snapshot, hash_field)?.as_str())
}

fn validate_sha(value: &str, field: &str) -> Result<(), String> {
    if value.len() == 40
        && value
            .chars()
            .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase())
    {
        Ok(())
    } else {
        Err(format!(
            "{field} must be a 40-character lowercase hexadecimal Git SHA-1"
        ))
    }
}

fn check_reference_source(value: &Value, path: &Path) -> Vec<SourceResult> {
    let Some(source) = value.as_mapping() else {
        return vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: "source is not a mapping".to_string(),
        }];
    };
    let Some(kind) = source.get("kind").and_then(Value::as_str) else {
        return vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: "source.kind is missing".to_string(),
        }];
    };
    match kind {
        "repository_file" | "repository_tree" => check_git_path(source, path, kind),
        "git_commit" => check_git_commit(source, path),
        "git_ref" => check_git_ref(source, path),
        "artifact" => check_artifact(source, path),
        "github_issue" | "github_pull_request" | "external_url" => {
            check_remote_source(source, path, kind)
        }
        _ => vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: format!("unsupported source kind: {kind}"),
        }],
    }
}

fn check_git_path(source: &Mapping, path: &Path, kind: &str) -> Vec<SourceResult> {
    let commit = source
        .get("snapshot")
        .and_then(Value::as_mapping)
        .and_then(|snapshot| snapshot.get("commit"))
        .and_then(Value::as_str);
    let locator = source.get("path").and_then(Value::as_str);
    let expected = source
        .get("snapshot")
        .and_then(Value::as_mapping)
        .and_then(|snapshot| {
            snapshot.get(if kind == "repository_file" {
                "blob_sha"
            } else {
                "tree_sha"
            })
        })
        .and_then(Value::as_str);
    let Some((commit, locator, expected)) = commit
        .zip(locator)
        .zip(expected)
        .map(|((commit, locator), expected)| (commit, locator, expected))
    else {
        return vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: "local Git Reference is missing its anchor".to_string(),
        }];
    };
    match git_output(&["rev-parse", &format!("{commit}:{locator}")]) {
        Ok(actual) if actual == expected => Vec::new(),
        Ok(actual) => vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: format!("{kind} anchor drifted: expected {expected}, found {actual}"),
        }],
        Err(error) => vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: format!("{kind} source is unreachable: {error}"),
        }],
    }
}

fn check_git_commit(source: &Mapping, path: &Path) -> Vec<SourceResult> {
    let Some(commit) = source.get("commit").and_then(Value::as_str) else {
        return vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: "git_commit is missing commit".to_string(),
        }];
    };
    match git_output(&["cat-file", "-e", &format!("{commit}^{{commit}}")]) {
        Ok(_) => Vec::new(),
        Err(error) => vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: format!("git_commit is unreachable: {error}"),
        }],
    }
}

fn check_git_ref(source: &Mapping, path: &Path) -> Vec<SourceResult> {
    let Some(reference) = source.get("ref").and_then(Value::as_str) else {
        return vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: "git_ref is missing ref".to_string(),
        }];
    };
    let expected = source
        .get("snapshot")
        .and_then(Value::as_mapping)
        .and_then(|snapshot| snapshot.get("resolved_commit"))
        .and_then(Value::as_str);
    let Some(expected) = expected else {
        return vec![SourceResult {
            path: path.to_path_buf(),
            severity: "WARNING",
            message: "floating git_ref was not checked because it has no resolved snapshot"
                .to_string(),
        }];
    };
    match git_output(&["rev-parse", reference]) {
        Ok(actual) if actual == expected => Vec::new(),
        Ok(actual) => vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: format!("git_ref drifted: expected {expected}, found {actual}"),
        }],
        Err(error) => vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: format!("git_ref is unreachable: {error}"),
        }],
    }
}

fn check_artifact(source: &Mapping, path: &Path) -> Vec<SourceResult> {
    let Some(locator) = source.get("locator").and_then(Value::as_str) else {
        return vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: "artifact is missing locator".to_string(),
        }];
    };
    let expected = source
        .get("snapshot")
        .and_then(Value::as_mapping)
        .and_then(|snapshot| snapshot.get("content_hash"))
        .and_then(Value::as_str);
    let Some(expected) = expected else {
        return vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: "artifact is missing content_hash".to_string(),
        }];
    };
    match fs::read(locator) {
        Ok(bytes) => {
            let actual = hash_bytes(&bytes);
            if actual == expected {
                Vec::new()
            } else {
                vec![SourceResult {
                    path: path.to_path_buf(),
                    severity: "ERROR",
                    message: format!(
                        "artifact content drifted: expected {expected}, found {actual}"
                    ),
                }]
            }
        }
        Err(error) => vec![SourceResult {
            path: path.to_path_buf(),
            severity: "ERROR",
            message: format!("artifact is unreachable: {error}"),
        }],
    }
}

fn check_remote_source(source: &Mapping, path: &Path, kind: &str) -> Vec<SourceResult> {
    let expected = source
        .get("snapshot")
        .and_then(Value::as_mapping)
        .and_then(|snapshot| snapshot.get("content_hash"))
        .and_then(Value::as_str);
    let Some(expected) = expected else {
        return vec![SourceResult {
            path: path.to_path_buf(),
            severity: "WARNING",
            message: format!("{kind} source has no persisted content_hash"),
        }];
    };
    let actual = match kind {
        "github_issue" => fetch_github_issue_hash(source),
        "github_pull_request" => fetch_github_pull_request_hash(source),
        "external_url" => fetch_external_url_hash(source),
        _ => unreachable!(),
    };
    match actual {
        Ok(actual) if actual == expected => Vec::new(),
        Ok(actual) => vec![SourceResult {
            path: path.to_path_buf(),
            severity: "WARNING",
            message: format!("{kind} source drifted: expected {expected}, found {actual}"),
        }],
        Err(error) => vec![SourceResult {
            path: path.to_path_buf(),
            severity: "WARNING",
            message: format!("{kind} source could not be rechecked: {error}"),
        }],
    }
}

fn fetch_github_issue_hash(source: &Mapping) -> Result<String, String> {
    let repository = value_string(source, "repository")?;
    let number = source
        .get("number")
        .and_then(Value::as_i64)
        .ok_or_else(|| "GitHub issue number must be an integer".to_string())?;
    let output = remote_command(
        "gh",
        &[
            "issue".to_string(),
            "view".to_string(),
            number.to_string(),
            "--repo".to_string(),
            repository.clone(),
            "--comments".to_string(),
            "--json".to_string(),
            "number,title,body,state,stateReason,labels,assignees,milestone,comments".to_string(),
        ],
    )?;
    let value: serde_json::Value =
        serde_json::from_slice(&output).map_err(|error| error.to_string())?;
    let payload = github_issue_payload(&repository, &value)?;
    let bytes = serde_json::to_vec(&payload).map_err(|error| error.to_string())?;
    Ok(hash_bytes(&bytes))
}

fn github_issue_payload(
    repository: &str,
    issue: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    let labels = sorted_json_strings(issue.get("labels"), "name")?;
    let assignees = sorted_json_strings(issue.get("assignees"), "login")?;
    let milestone = issue
        .get("milestone")
        .and_then(|value| value.get("title"))
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let mut comments = issue
        .get("comments")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "GitHub issue response is missing comments".to_string())?
        .iter()
        .map(|comment| {
            serde_json::json!({
                "id": comment.get("id").cloned().unwrap_or(serde_json::Value::Null),
                "author": comment.get("author").and_then(|author| author.get("login")).cloned().unwrap_or(serde_json::Value::Null),
                "body": comment.get("body").cloned().unwrap_or(serde_json::Value::Null),
                "created_at": comment.get("createdAt").cloned().unwrap_or(serde_json::Value::Null),
            })
        })
        .collect::<Vec<_>>();
    comments.sort_by_key(|comment| {
        comment
            .get("id")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_string()
    });
    Ok(serde_json::json!({
        "repository": repository,
        "number": issue.get("number").cloned().unwrap_or(serde_json::Value::Null),
        "title": issue.get("title").cloned().unwrap_or(serde_json::Value::Null),
        "body": issue.get("body").cloned().unwrap_or(serde_json::Value::Null),
        "state": issue.get("state").cloned().unwrap_or(serde_json::Value::Null),
        "state_reason": issue.get("stateReason").cloned().unwrap_or(serde_json::Value::Null),
        "labels": labels,
        "assignees": assignees,
        "milestone": milestone,
        "comments": comments,
    }))
}

fn sorted_json_strings(
    value: Option<&serde_json::Value>,
    field: &str,
) -> Result<Vec<String>, String> {
    let values = value
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| format!("GitHub response is missing {field} collection"))?;
    let mut strings = values
        .iter()
        .filter_map(|value| value.get(field).and_then(serde_json::Value::as_str))
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    strings.sort();
    Ok(strings)
}

fn fetch_github_pull_request_hash(source: &Mapping) -> Result<String, String> {
    let repository = value_string(source, "repository")?;
    let number = source
        .get("number")
        .and_then(Value::as_i64)
        .ok_or_else(|| "GitHub pull request number must be an integer".to_string())?;
    let output = remote_command(
        "gh",
        &[
            "pr".to_string(),
            "view".to_string(),
            number.to_string(),
            "--repo".to_string(),
            repository,
            "--json".to_string(),
            "number,title,body,state,baseRefName,headRefName,headRefOid,mergeCommit,comments,reviews".to_string(),
        ],
    )?;
    let value: serde_json::Value =
        serde_json::from_slice(&output).map_err(|error| error.to_string())?;
    let bytes = serde_json::to_vec(&value).map_err(|error| error.to_string())?;
    Ok(hash_bytes(&bytes))
}

fn fetch_external_url_hash(source: &Mapping) -> Result<String, String> {
    let url = value_string(source, "url")?;
    let output = remote_command(
        "curl",
        &[
            "-L".to_string(),
            "--fail".to_string(),
            "--silent".to_string(),
            "--show-error".to_string(),
            "--max-time".to_string(),
            "20".to_string(),
            url,
        ],
    )?;
    Ok(hash_bytes(&output))
}

fn remote_command(program: &str, args: &[String]) -> Result<Vec<u8>, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn git_output(args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn claim_is_active(claims: Option<&Vec<Value>>, id: &str) -> bool {
    claims.into_iter().flatten().any(|claim| {
        claim.get("id").and_then(Value::as_str) == Some(id)
            && claim.get("lifecycle").and_then(Value::as_str) == Some("active")
    })
}

fn markdown_has_heading(body: &str, heading: &str) -> bool {
    let expected = format!("## {heading}");
    body.lines().any(|line| line.trim() == expected)
}

fn validate_relations_shape(
    map: &Mapping,
    object_type: &str,
    registries: &Registries,
) -> Result<(), String> {
    if let Some(relations) = map.get("relations") {
        validate_relation_sequence(relations, object_type, false, registries)?;
    }
    if let Some(claims) = map.get("claims").and_then(Value::as_sequence) {
        for claim in claims {
            if let Some(relations) = claim.get("relations") {
                validate_relation_sequence(relations, object_type, true, registries)?;
            }
        }
    }
    Ok(())
}

fn validate_relation_sequence(
    value: &Value,
    source_type: &str,
    source_claim: bool,
    registries: &Registries,
) -> Result<(), String> {
    let relations = value
        .as_sequence()
        .ok_or_else(|| "relations must be a sequence".to_string())?;
    for relation in relations {
        let relation = relation
            .as_mapping()
            .ok_or_else(|| "each relation must be a mapping".to_string())?;
        let name = value_string(relation, "type")?;
        let rule = registries
            .relations
            .get(&name)
            .ok_or_else(|| format!("unknown authored relation: {name}"))?;
        if source_claim && !rule.source_claim {
            return Err(format!("relation {name} cannot originate from a claim"));
        }
        if !rule.source_types.iter().any(|ty| ty == source_type) {
            return Err(format!(
                "relation {name} cannot originate from {source_type}"
            ));
        }
        let target = relation
            .get("target")
            .and_then(Value::as_mapping)
            .ok_or_else(|| "relation target must be a mapping".to_string())?;
        let target_object = value_string(target, "object")?;
        let target_type = target_object.split('.').next().unwrap_or_default();
        if !rule.target_types.iter().any(|ty| ty == target_type) {
            return Err(format!("relation {name} cannot target {target_type}"));
        }
        if name == "supersedes" && target_type != source_type {
            return Err("supersedes must connect the same semantic type".to_string());
        }
        let target_claim = target.contains_key(Value::String("claim".to_string()));
        if target_claim && !rule.target_claim {
            return Err(format!("relation {name} cannot target a claim"));
        }
        if target_claim && target_type == "reference" {
            return Err("Reference claims are forbidden".to_string());
        }
        if relation.contains_key(Value::String("selectors".to_string())) && !rule.selectors {
            return Err(format!("relation {name} does not allow selectors"));
        }
        if relation.contains_key(Value::String("selectors".to_string()))
            && (name != "supported_by" && name != "derived_from")
        {
            return Err("selectors are only allowed on supported_by or derived_from".to_string());
        }
        if relation.contains_key(Value::String("selectors".to_string())) {
            if target_claim || target_type != "reference" {
                return Err("selectors require a whole Reference target".to_string());
            }
            validate_selectors(relation.get("selectors").unwrap())?;
        }
        if name == "supersedes" && target_claim != source_claim {
            return Err("supersedes must connect matching address kinds".to_string());
        }
        if name == "part_of" && (source_claim || target_claim) {
            return Err("part_of cannot target claims".to_string());
        }
    }
    Ok(())
}

fn validate_selectors(value: &Value) -> Result<(), String> {
    let selectors = value
        .as_sequence()
        .ok_or_else(|| "selectors must be a sequence".to_string())?;
    for selector in selectors {
        let selector = selector
            .as_mapping()
            .ok_or_else(|| "each selector must be a mapping".to_string())?;
        let kind = value_string(selector, "kind")?;
        match kind.as_str() {
            "lines" => {
                let start = selector
                    .get("start")
                    .and_then(Value::as_i64)
                    .ok_or_else(|| "line selector start must be an integer".to_string())?;
                let end = selector
                    .get("end")
                    .and_then(Value::as_i64)
                    .ok_or_else(|| "line selector end must be an integer".to_string())?;
                if start < 1 || end < start {
                    return Err("line selector must be 1-based with end >= start".to_string());
                }
            }
            "section" | "symbol" | "json_pointer" => {
                if value_string(selector, "value")?.is_empty() {
                    return Err("selector value must not be empty".to_string());
                }
            }
            _ => {
                return Err(
                    "selector kind must be lines, section, symbol, or json_pointer".to_string(),
                )
            }
        }
    }
    Ok(())
}

fn validate_graph(
    documents: &[SemanticDocument],
    registries: &Registries,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut objects = HashMap::new();
    let mut decision_keys = HashMap::<String, Vec<String>>::new();
    for document in documents {
        let id = document
            .frontmatter
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let object_type = document
            .frontmatter
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        objects.insert(id.clone(), (object_type.clone(), document));
        if object_type == "decision"
            && document
                .frontmatter
                .get("lifecycle")
                .and_then(Value::as_str)
                == Some("active")
        {
            if let Some(key) = document
                .frontmatter
                .get("decision_key")
                .and_then(Value::as_str)
            {
                decision_keys.entry(key.to_string()).or_default().push(id);
            }
        }
    }
    for (key, ids) in decision_keys {
        if ids.len() > 1 {
            diagnostics.push(Diagnostic {
                path: PathBuf::from("okf/decisions"),
                message: format!(
                    "at most one active Decision is allowed for decision_key {key}: {}",
                    ids.join(", ")
                ),
            });
        }
    }
    let mut seen = BTreeSet::new();
    let mut contradictions = HashSet::new();
    let mut cycles = HashMap::<String, Vec<String>>::new();
    for document in documents {
        let source = document
            .frontmatter
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        collect_edges(
            document.frontmatter.get("relations"),
            source,
            &objects,
            registries,
            &mut seen,
            &mut contradictions,
            &mut cycles,
            diagnostics,
        );
        if let Some(claims) = document
            .frontmatter
            .get("claims")
            .and_then(Value::as_sequence)
        {
            for claim in claims {
                let claim_id = claim.get("id").and_then(Value::as_str).unwrap_or_default();
                let source_address = format!("{source}#{claim_id}");
                collect_edges(
                    claim.get("relations"),
                    &source_address,
                    &objects,
                    registries,
                    &mut seen,
                    &mut contradictions,
                    &mut cycles,
                    diagnostics,
                );
            }
        }
    }
    for (relation, edges) in cycles {
        if has_cycle(&edges) {
            diagnostics.push(Diagnostic {
                path: PathBuf::from("okf"),
                message: format!("{relation} cycle detected"),
            });
        }
    }
}

fn collect_edges(
    value: Option<&Value>,
    source: &str,
    objects: &HashMap<String, (String, &SemanticDocument)>,
    registries: &Registries,
    seen: &mut BTreeSet<String>,
    contradictions: &mut HashSet<String>,
    cycles: &mut HashMap<String, Vec<String>>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(relations) = value.and_then(Value::as_sequence) else {
        return;
    };
    for relation in relations {
        let Some(map) = relation.as_mapping() else {
            continue;
        };
        let Some(name) = map.get("type").and_then(Value::as_str) else {
            continue;
        };
        let Some(target_map) = map.get("target").and_then(Value::as_mapping) else {
            continue;
        };
        let Some(target_object) = target_map.get("object").and_then(Value::as_str) else {
            continue;
        };
        let target = if let Some(claim) = target_map.get("claim").and_then(Value::as_str) {
            format!("{target_object}#{claim}")
        } else {
            target_object.to_string()
        };
        if !objects.contains_key(target.split('#').next().unwrap_or_default()) {
            diagnostics.push(Diagnostic {
                path: PathBuf::from("okf"),
                message: format!("relation {name} targets unknown object {target}"),
            });
        } else if let Some(claim) = target.split('#').nth(1) {
            let object_id = target.split('#').next().unwrap_or_default();
            let has_claim = objects
                .get(object_id)
                .and_then(|(_, document)| document.frontmatter.get("claims"))
                .and_then(Value::as_sequence)
                .into_iter()
                .flatten()
                .any(|candidate| candidate.get("id").and_then(Value::as_str) == Some(claim));
            if !has_claim {
                diagnostics.push(Diagnostic {
                    path: PathBuf::from("okf"),
                    message: format!("relation {name} targets unknown claim {target}"),
                });
            }
        }
        let selectors =
            serde_json::to_string(map.get("selectors").unwrap_or(&Value::Null)).unwrap_or_default();
        let identity = format!("{source}|{name}|{target}|{selectors}");
        if !seen.insert(identity) {
            diagnostics.push(Diagnostic {
                path: PathBuf::from("okf"),
                message: format!("duplicate {name} edge from {source} to {target}"),
            });
        }
        if name == "contradicts" {
            let pair = format!("{source}|{target}");
            let reverse = format!("{target}|{source}");
            if contradictions.contains(&reverse) {
                diagnostics.push(Diagnostic {
                    path: PathBuf::from("okf"),
                    message: format!("reciprocal authored contradicts edges are forbidden between {source} and {target}"),
                });
            }
            contradictions.insert(pair);
        }
        if source == target {
            diagnostics.push(Diagnostic {
                path: PathBuf::from("okf"),
                message: format!("exact self-edge is forbidden: {source}"),
            });
        }
        if registries
            .relations
            .get(name)
            .is_some_and(|rule| rule.acyclic)
        {
            cycles
                .entry(name.to_string())
                .or_default()
                .push(format!("{source}>{target}"));
        }
    }
}

fn has_cycle(edges: &[String]) -> bool {
    let mut graph = HashMap::<&str, Vec<&str>>::new();
    for edge in edges {
        if let Some((source, target)) = edge.split_once('>') {
            graph.entry(source).or_default().push(target);
        }
    }
    fn visit<'a>(
        node: &'a str,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        visiting: &mut HashSet<&'a str>,
        visited: &mut HashSet<&'a str>,
    ) -> bool {
        if visiting.contains(node) {
            return true;
        }
        if !visited.insert(node) {
            return false;
        }
        visiting.insert(node);
        let found = graph
            .get(node)
            .into_iter()
            .flatten()
            .any(|next| visit(next, graph, visiting, visited));
        visiting.remove(node);
        found
    }
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    graph
        .keys()
        .any(|node| visit(node, &graph, &mut visiting, &mut visited))
}

fn value_string(map: &Mapping, field: &str) -> Result<String, String> {
    map.get(Value::String(field.to_string()))
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| format!("{field} must be a string"))
}

fn value_optional_string(map: &Mapping, field: &str) -> Result<Option<String>, String> {
    match map.get(Value::String(field.to_string())) {
        None | Some(Value::Null) => Ok(None),
        Some(_) => value_string(map, field).map(Some),
    }
}

fn value_bool(map: &Mapping, field: &str) -> Result<bool, String> {
    map.get(Value::String(field.to_string()))
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("{field} must be a boolean"))
}

fn value_strings(map: &Mapping, field: &str) -> Result<Vec<String>, String> {
    map.get(Value::String(field.to_string()))
        .and_then(Value::as_sequence)
        .ok_or_else(|| format!("{field} must be a sequence"))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(ToString::to_string)
                .ok_or_else(|| format!("{field} entries must be strings"))
        })
        .collect()
}

fn valid_slug(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        && !value.starts_with('_')
        && !value.ends_with('_')
}

fn valid_decision_key(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '-')
        && !value.starts_with(['.', '-'])
        && !value.ends_with(['.', '-'])
        && !value.contains("..")
        && !value.contains("--")
        && !value.contains(".-")
        && !value.contains("-.")
}

fn validate_id(id: &str, object_type: &str) -> Result<(), String> {
    let Some((prefix, slug)) = id.split_once('.') else {
        return Err("id must have the form <type>.<slug>".to_string());
    };
    if prefix != object_type
        || slug.is_empty()
        || !slug.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
        || slug.starts_with('-')
        || slug.ends_with('-')
        || slug.contains("--")
    {
        return Err("id must match its type and use a lowercase kebab-case slug".to_string());
    }
    Ok(())
}

fn validate_filename(path: &Path) -> Result<(), String> {
    let name = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if name.is_empty()
        || !name.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
        || name.starts_with('-')
        || name.ends_with('-')
        || name.contains("--")
    {
        return Err("filename must be lowercase ASCII kebab-case".to_string());
    }
    Ok(())
}

fn validate_directory_type(path: &Path, object_type: &str) -> Result<(), String> {
    let expected = match object_type {
        "entity" => "entities",
        "concept" => "concepts",
        "system" => "systems",
        "research" => "research",
        "decision" => "decisions",
        "reference" => "references",
        _ => unreachable!("object type is validated before directory mapping"),
    };
    let parent = path
        .parent()
        .and_then(|value| value.file_name())
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if parent != expected {
        return Err(format!(
            "object of type {object_type} must be inside okf/{expected}"
        ));
    }
    Ok(())
}

fn validate_lifecycle(value: &str) -> Result<(), String> {
    if ["draft", "active", "deprecated"].contains(&value) {
        Ok(())
    } else {
        Err("lifecycle must be draft, active, or deprecated".to_string())
    }
}

fn validate_timestamp(value: &str) -> Result<(), String> {
    let bytes = value.as_bytes();
    let separators = [4, 7, 10, 13, 16];
    let valid = bytes.len() == 20
        && value.ends_with('Z')
        && separators
            .iter()
            .all(|index| matches!(bytes[*index], b'-' | b'T' | b':'))
        && bytes.iter().enumerate().all(|(index, byte)| {
            index == 19 || separators.contains(&index) || byte.is_ascii_digit()
        });
    if valid {
        Ok(())
    } else {
        Err("timestamp must be quoted UTC RFC3339 in YYYY-MM-DDTHH:MM:SSZ form".to_string())
    }
}

fn validate_generated(value: Option<&Value>) -> Result<(), String> {
    let mapping = value
        .and_then(Value::as_mapping)
        .ok_or_else(|| "generated must be a mapping".to_string())?;
    let by = mapping
        .get(Value::String("by".to_string()))
        .and_then(Value::as_str)
        .ok_or_else(|| "generated.by must be a string".to_string())?;
    let at = mapping
        .get(Value::String("at".to_string()))
        .and_then(Value::as_str)
        .ok_or_else(|| "generated.at must be a string".to_string())?;
    if !["human:", "agent:", "process:"]
        .iter()
        .any(|prefix| by.starts_with(prefix))
    {
        return Err("generated.by must use a human:, agent:, or process: actor".to_string());
    }
    validate_timestamp(at)
}

fn validate_freshness(value: Option<&Value>) -> Result<(), String> {
    let Some(value) = value else {
        return Ok(());
    };
    let mapping = value
        .as_mapping()
        .ok_or_else(|| "freshness must be a mapping".to_string())?;
    let state = mapping
        .get(Value::String("state".to_string()))
        .and_then(Value::as_str)
        .ok_or_else(|| "freshness.state is required".to_string())?;
    if !["fresh", "stale", "unknown", "not_applicable"].contains(&state) {
        return Err("freshness.state is invalid".to_string());
    }
    if state == "fresh" {
        validate_timestamp(
            mapping
                .get(Value::String("checked_at".to_string()))
                .and_then(Value::as_str)
                .ok_or_else(|| "fresh freshness requires checked_at".to_string())?,
        )?;
    }
    if state == "stale" {
        validate_timestamp(
            mapping
                .get(Value::String("detected_at".to_string()))
                .and_then(Value::as_str)
                .ok_or_else(|| "stale freshness requires detected_at".to_string())?,
        )?;
        if mapping
            .get(Value::String("reason".to_string()))
            .and_then(Value::as_str)
            .is_none()
        {
            return Err("stale freshness requires reason".to_string());
        }
    }
    Ok(())
}

fn validate_claims(value: &Value) -> Result<(), String> {
    let claims = value
        .as_sequence()
        .ok_or_else(|| "claims must be a sequence".to_string())?;
    let mut ids = std::collections::BTreeSet::new();
    for claim in claims {
        let map = claim
            .as_mapping()
            .ok_or_else(|| "each claim must be a mapping".to_string())?;
        for field in [
            "id",
            "lifecycle",
            "statement",
            "load_bearing",
            "semantic_hash",
        ] {
            require(map, field)?;
        }
        let id = string_field(map, "id")?;
        if id.is_empty()
            || !id.chars().all(|character| {
                character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
            })
            || !ids.insert(id.clone())
        {
            return Err("claim IDs must be unique lowercase kebab-case values".to_string());
        }
        validate_lifecycle(string_field(map, "lifecycle")?.as_str())?;
        if string_field(map, "statement")?.trim().is_empty() {
            return Err("claim statement must not be empty".to_string());
        }
        if !map
            .get(Value::String("load_bearing".to_string()))
            .is_some_and(Value::is_bool)
        {
            return Err("claim load_bearing must be boolean".to_string());
        }
        validate_hash(string_field(map, "semantic_hash")?.as_str())?;
        validate_freshness(map.get(Value::String("freshness".to_string())))?;
        let expected = string_field(map, "semantic_hash")?;
        let actual = claim_hash(map)?;
        if expected != actual {
            return Err(format!("claim {id} semantic_hash mismatch"));
        }
    }
    Ok(())
}

fn validate_verified(value: &Value) -> Result<(), String> {
    let attestations = value
        .as_sequence()
        .ok_or_else(|| "verified must be a list".to_string())?;
    for attestation in attestations {
        let map = attestation
            .as_mapping()
            .ok_or_else(|| "each verification attestation must be a mapping".to_string())?;
        for field in ["by", "at", "subject", "revision"] {
            require(map, field)?;
        }
        let by = string_field(map, "by")?;
        if !["human:", "agent:", "process:"]
            .iter()
            .any(|prefix| by.starts_with(prefix))
        {
            return Err("verification by must use an actor ID".to_string());
        }
        validate_timestamp(string_field(map, "at")?.as_str())?;
        validate_hash(string_field(map, "revision")?.as_str())?;
        if map
            .get(Value::String("subject".to_string()))
            .and_then(Value::as_mapping)
            .is_none()
        {
            return Err("verification subject must be a mapping".to_string());
        }
    }
    Ok(())
}

fn object_hash(map: &Mapping, body: &str) -> Result<String, String> {
    let mut projection = map.clone();
    for key in [
        "created_at",
        "updated_at",
        "generated",
        "verified",
        "semantic_hash",
        "lifecycle",
        "freshness",
    ] {
        projection.remove(Value::String(key.to_string()));
    }
    if let Some(Value::Sequence(claims)) = projection.get_mut(Value::String("claims".to_string())) {
        for claim in claims {
            if let Some(claim) = claim.as_mapping_mut() {
                for key in ["semantic_hash", "lifecycle", "freshness"] {
                    claim.remove(Value::String(key.to_string()));
                }
            }
        }
    }
    let json = yaml_to_canonical_json(&Value::Mapping(projection))?;
    let payload = format!(
        "{}\n---BODY---\n{}",
        serde_json::to_string(&json).map_err(|error| error.to_string())?,
        normalize_body(body)
    );
    Ok(hash_bytes(payload.as_bytes()))
}

fn claim_hash(map: &Mapping) -> Result<String, String> {
    let mut projection = map.clone();
    for key in ["semantic_hash", "lifecycle", "freshness"] {
        projection.remove(Value::String(key.to_string()));
    }
    let json = yaml_to_canonical_json(&Value::Mapping(projection))?;
    Ok(hash_bytes(
        serde_json::to_string(&json)
            .map_err(|error| error.to_string())?
            .as_bytes(),
    ))
}

fn yaml_to_canonical_json(value: &Value) -> Result<serde_json::Value, String> {
    match value {
        Value::Null => Ok(serde_json::Value::Null),
        Value::Bool(value) => Ok(serde_json::Value::Bool(*value)),
        Value::Number(value) => serde_json::to_value(value).map_err(|error| error.to_string()),
        Value::String(value) => Ok(serde_json::Value::String(value.clone())),
        Value::Sequence(values) => values
            .iter()
            .map(yaml_to_canonical_json)
            .collect::<Result<Vec<_>, _>>()
            .map(serde_json::Value::Array),
        Value::Mapping(values) => {
            let mut sorted = BTreeMap::new();
            for (key, value) in values {
                sorted.insert(
                    key.as_str()
                        .ok_or_else(|| "semantic YAML mapping keys must be strings".to_string())?
                        .to_string(),
                    yaml_to_canonical_json(value)?,
                );
            }
            Ok(serde_json::Value::Object(sorted.into_iter().collect()))
        }
        Value::Tagged(_) => Err("tagged YAML values are not supported".to_string()),
    }
}

fn normalize_body(body: &str) -> String {
    let mut lines: Vec<String> = body
        .lines()
        .map(|line| line.trim_end_matches([' ', '\t']).to_string())
        .collect();
    while lines.last().is_some_and(String::is_empty) {
        lines.pop();
    }
    if lines.is_empty() {
        String::new()
    } else {
        format!("{}\n", lines.join("\n"))
    }
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("sha256:{:x}", hasher.finalize())
}

fn validate_hash(value: &str) -> Result<(), String> {
    if value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .chars()
            .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase())
    {
        Ok(())
    } else {
        Err("hash must be sha256:<64 lowercase hexadecimal characters>".to_string())
    }
}

fn require(map: &Mapping, field: &str) -> Result<(), String> {
    if map.contains_key(Value::String(field.to_string())) {
        Ok(())
    } else {
        Err(format!("missing required field: {field}"))
    }
}

fn string_field(map: &Mapping, field: &str) -> Result<String, String> {
    map.get(Value::String(field.to_string()))
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| format!("{field} must be a string"))
}

fn option_value(args: &[String], option: &str) -> Result<Option<String>, String> {
    let Some(index) = args.iter().position(|arg| arg == option) else {
        return Ok(None);
    };
    args.get(index + 1)
        .cloned()
        .ok_or_else(|| format!("{option} requires a value"))
        .map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> SemanticDocument {
        let body = "## Definition\n\nA beatmap entity.\n".to_string();
        let mut map = Mapping::new();
        map.insert(
            Value::String("id".into()),
            Value::String("entity.beatmap".into()),
        );
        map.insert(Value::String("type".into()), Value::String("entity".into()));
        map.insert(
            Value::String("title".into()),
            Value::String("Beatmap".into()),
        );
        map.insert(
            Value::String("description".into()),
            Value::String("A map.".into()),
        );
        map.insert(
            Value::String("lifecycle".into()),
            Value::String("draft".into()),
        );
        map.insert(
            Value::String("created_at".into()),
            Value::String("2026-08-25T00:00:00Z".into()),
        );
        map.insert(
            Value::String("updated_at".into()),
            Value::String("2026-08-25T00:00:00Z".into()),
        );
        map.insert(
            Value::String("generated".into()),
            serde_yaml::from_str("by: agent:test\nat: '2026-08-25T00:00:00Z'").unwrap(),
        );
        map.insert(
            Value::String("semantic_hash".into()),
            Value::String("sha256:".to_string() + &"0".repeat(64)),
        );
        SemanticDocument {
            path: PathBuf::from("okf/entities/beatmap.md"),
            frontmatter: map,
            body,
        }
    }

    #[test]
    fn body_normalization_is_deterministic() {
        assert_eq!(normalize_body("a  \r\n\r\n"), "a\n");
        assert_eq!(normalize_body(""), "");
    }

    #[test]
    fn parser_splits_frontmatter_from_body() {
        let path = std::env::temp_dir().join(format!("okf-parser-test-{}.md", std::process::id()));
        std::fs::write(
            &path,
            "---\nid: entity.beatmap\ntype: entity\n---\n\n## Definition\n",
        )
        .unwrap();
        let document = parse_document(&path).unwrap();
        std::fs::remove_file(path).unwrap();
        assert_eq!(
            document
                .frontmatter
                .get(Value::String("id".into()))
                .and_then(Value::as_str),
            Some("entity.beatmap")
        );
        assert_eq!(document.body, "\n## Definition\n");
    }

    #[test]
    fn object_projection_excludes_administrative_fields() {
        let mut document = sample();
        let first = object_hash(&document.frontmatter, &document.body).unwrap();
        document.frontmatter.insert(
            Value::String("updated_at".into()),
            Value::String("2026-08-26T00:00:00Z".into()),
        );
        document.frontmatter.insert(
            Value::String("generated".into()),
            serde_yaml::from_str("by: human:test\nat: '2026-08-26T00:00:00Z'").unwrap(),
        );
        assert_eq!(
            first,
            object_hash(&document.frontmatter, &document.body).unwrap()
        );
    }

    #[test]
    fn valid_document_passes_validation() {
        let mut document = sample();
        let hash = object_hash(&document.frontmatter, &document.body).unwrap();
        document
            .frontmatter
            .insert(Value::String("semantic_hash".into()), Value::String(hash));
        assert!(
            validate_document(&document).is_ok(),
            "{:?}",
            validate_document(&document)
        );
    }

    #[test]
    fn references_cannot_contain_claims() {
        let mut document = sample();
        document.frontmatter.insert(
            Value::String("type".into()),
            Value::String("reference".into()),
        );
        document.frontmatter.insert(
            Value::String("id".into()),
            Value::String("reference.source".into()),
        );
        document.path = PathBuf::from("okf/references/source.md");
        document
            .frontmatter
            .insert(Value::String("claims".into()), Value::Sequence(Vec::new()));
        assert_eq!(
            validate_document(&document).unwrap_err(),
            "Reference objects must not contain claims"
        );
    }

    #[test]
    fn claim_hash_excludes_administrative_fields() {
        let claim: Mapping = serde_yaml::from_str("id: finding\nlifecycle: draft\nstatement: A finding\nload_bearing: true\nsemantic_hash: sha256:0000000000000000000000000000000000000000000000000000000000000000\nfreshness:\n  state: unknown").unwrap();
        let first = claim_hash(&claim).unwrap();
        let mut changed = claim.clone();
        changed.insert(
            Value::String("lifecycle".into()),
            Value::String("active".into()),
        );
        changed.insert(
            Value::String("semantic_hash".into()),
            Value::String("sha256:".to_string() + &"1".repeat(64)),
        );
        assert_eq!(first, claim_hash(&changed).unwrap());
    }

    #[test]
    fn phase_two_registries_are_complete_and_self_consistent() {
        let schema = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../okf/schema");
        let registries = load_registries(&schema).expect("schema registries should load");
        assert_eq!(registries.relations.len(), 9);
        assert_eq!(registries.types.len(), 6);
        assert_eq!(registries.relations["depends_on"].inverse, "required_by");
    }

    #[test]
    fn relation_rules_reject_invalid_endpoints_and_selectors() {
        let schema = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../okf/schema");
        let registries = load_registries(&schema).unwrap();
        let relation: Value =
            serde_yaml::from_str("- type: depends_on\n  target:\n    object: reference.source\n")
                .unwrap();
        assert!(
            validate_relation_sequence(&relation, "concept", false, &registries)
                .unwrap_err()
                .contains("cannot target reference")
        );

        let relation: Value = serde_yaml::from_str(
            "- type: depends_on\n  target:\n    object: concept.other\n  selectors:\n    - kind: lines\n      start: 1\n      end: 2\n",
        ).unwrap();
        assert!(
            validate_relation_sequence(&relation, "concept", false, &registries)
                .unwrap_err()
                .contains("does not allow selectors")
        );
    }

    #[test]
    fn reference_source_contracts_require_immutable_anchors() {
        let source: Value = serde_yaml::from_str(
            "source:\n  kind: repository_file\n  repository: iamweaker99/map-analyzer-custom\n  path: scripts/okf-tool/src/main.rs\n  snapshot:\n    commit: 0000000000000000000000000000000000000000\n    blob_sha: 0000000000000000000000000000000000000000\n",
        )
        .unwrap();
        assert!(validate_reference_source(source.as_mapping().unwrap(), "active").is_ok());

        let floating: Value = serde_yaml::from_str(
            "source:\n  kind: git_ref\n  repository: iamweaker99/map-analyzer-custom\n  ref_type: branch\n  ref: main\n",
        )
        .unwrap();
        assert!(validate_reference_source(floating.as_mapping().unwrap(), "draft").is_ok());
        assert!(
            validate_reference_source(floating.as_mapping().unwrap(), "active")
                .unwrap_err()
                .contains("resolved_commit")
        );
    }

    #[test]
    fn selectors_are_validated_at_the_reference_seam() {
        let schema = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../okf/schema");
        let registries = load_registries(&schema).unwrap();
        let relation: Value = serde_yaml::from_str(
            "- type: supported_by\n  target:\n    object: reference.source\n  selectors:\n    - kind: lines\n      start: 4\n      end: 9\n",
        )
        .unwrap();
        assert!(validate_relation_sequence(&relation, "research", true, &registries).is_ok());
    }

    #[test]
    fn artifact_snapshot_checker_detects_drift() {
        let path = std::env::temp_dir().join(format!("okf-artifact-{}.json", std::process::id()));
        std::fs::write(&path, b"{\"value\":1}").unwrap();
        let source: Value = serde_yaml::from_str(&format!(
            "kind: artifact\nlocator: {}\nmedia_type: application/json\nsnapshot:\n  method: raw-bytes-v1\n  content_hash: {}\n",
            path.display(),
            hash_bytes(b"{\"value\":1}")
        ))
        .unwrap();
        assert!(check_artifact(source.as_mapping().unwrap(), Path::new("test")).is_empty());
        std::fs::write(&path, b"{\"value\":2}").unwrap();
        assert_eq!(
            check_artifact(source.as_mapping().unwrap(), Path::new("test"))[0].severity,
            "ERROR"
        );
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn github_issue_projection_excludes_transport_fields() {
        let first = serde_json::json!({
            "number": 3,
            "title": "Example",
            "body": "Body",
            "state": "OPEN",
            "stateReason": "",
            "labels": [{"name": "enhancement", "description": "one", "url": "transport-a"}],
            "assignees": [],
            "milestone": null,
            "comments": []
        });
        let second = serde_json::json!({
            "number": 3,
            "title": "Example",
            "body": "Body",
            "state": "OPEN",
            "stateReason": "",
            "labels": [{"name": "enhancement", "description": "changed", "url": "transport-b"}],
            "assignees": [],
            "milestone": null,
            "comments": []
        });
        assert_eq!(
            github_issue_payload("iamweaker99/map-analyzer-custom", &first).unwrap(),
            github_issue_payload("iamweaker99/map-analyzer-custom", &second).unwrap()
        );
    }

    #[test]
    fn draft_init_rejects_reference_capture() {
        assert!(draft_init(&["reference".into(), "source".into()]).is_err());
    }

    #[test]
    fn generated_views_are_deterministic_for_the_same_documents() {
        let schema = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../okf/schema");
        let registries = load_registries(&schema).unwrap();
        let documents = Vec::new();
        let first = (
            render_catalog(&documents),
            render_graph(&documents, &registries),
            render_review(&documents),
            render_index(&documents, &render_review(&documents)),
        );
        let second = (
            render_catalog(&documents),
            render_graph(&documents, &registries),
            render_review(&documents),
            render_index(&documents, &render_review(&documents)),
        );
        assert_eq!(
            serde_json::to_string(&first.0).unwrap(),
            serde_json::to_string(&second.0).unwrap()
        );
        assert_eq!(
            serde_json::to_string(&first.1).unwrap(),
            serde_json::to_string(&second.1).unwrap()
        );
        assert_eq!(
            serde_json::to_string(&first.2).unwrap(),
            serde_json::to_string(&second.2).unwrap()
        );
        assert_eq!(first.3, second.3);
    }

    #[test]
    fn session_handoff_accepts_existing_object_and_claim_addresses() {
        let frontmatter: Mapping = serde_yaml::from_str(
            "id: research.example\ntype: research\nclaims:\n  - id: finding\n",
        )
        .unwrap();
        let document = SemanticDocument {
            path: PathBuf::from("okf/research/example.md"),
            frontmatter,
            body: String::new(),
        };
        let content = format!("{SESSION_HANDOFF_TEMPLATE}\n- research.example#finding\n");
        assert!(handoff_diagnostics(&content, &[document]).is_empty());
    }

    #[test]
    fn session_handoff_reports_unknown_claim_addresses() {
        let frontmatter: Mapping =
            serde_yaml::from_str("id: research.example\ntype: research\nclaims: []\n").unwrap();
        let document = SemanticDocument {
            path: PathBuf::from("okf/research/example.md"),
            frontmatter,
            body: String::new(),
        };
        let content = format!("{SESSION_HANDOFF_TEMPLATE}\n- research.example#missing\n");
        let diagnostics = handoff_diagnostics(&content, &[document]);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.contains("unknown claim")));
    }

    #[test]
    fn hard_cycle_detection_is_limited_to_registered_acyclic_relations() {
        assert!(has_cycle(&[
            "a>b".to_string(),
            "b>c".to_string(),
            "c>a".to_string()
        ]));
        assert!(!has_cycle(&["a>b".to_string(), "b>c".to_string()]));
    }

    #[test]
    fn phase_five_diagnostics_have_stable_machine_codes() {
        let catalog = [
            ("OKF-E420", "okf/index.md"),
            ("OKF-E421", "okf/generated/catalog.json"),
            ("OKF-E422", "okf/generated/graph.json"),
            ("OKF-E423", "okf/generated/review.json"),
            ("OKF-E424", "okf/generated/review.md"),
        ];
        for (code, path) in catalog {
            let diagnostic = Diagnostic {
                path: PathBuf::from(path),
                message: format!("generated output drift ({code})"),
            };
            assert_eq!(diagnostic.severity(), "ERROR");
            assert_eq!(diagnostic.code(), code);
        }
    }
}
