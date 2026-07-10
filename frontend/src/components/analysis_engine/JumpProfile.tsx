import { JumpAnalysis } from "./types";
import { getSpacingTag } from "./utils";
import { StatBar } from "./StatBar";

export function JumpProfile({ analysis }: { analysis: JumpAnalysis }) {
    const d = analysis.circle_diameter || 73;
    const spacing = analysis.avg_spacing || 0;

    const totalChains =
        (analysis.short_jumps || 0) +
        (analysis.medium_jumps || 0) +
        (analysis.long_jumps || 0);

    return (
        <div className="space-y-4">
            <li className="font-bold border-b border-gray-700 pb-1 mb-2">
                Spacing: {getSpacingTag(spacing, d)} ({spacing.toFixed(1)} px)
            </li>

            <p className="text-xs font-semibold text-pink-400 uppercase mb-2">
                Distance Profile (Excluding Streams)
            </p>
            <StatBar
                label="Narrow (&lt;2.0x D)"
                value={analysis.narrow_count || 0}
                percentage={(analysis.narrow_dens || 0) * 100}
                colorClass="bg-pink-500"
            />
            <StatBar
                label="Moderate (2-3.5x D)"
                value={analysis.moderate_count || 0}
                percentage={(analysis.moderate_dens || 0) * 100}
                colorClass="bg-pink-500"
            />
            <StatBar
                label="Wide (3.5-5x D)"
                value={analysis.wide_count || 0}
                percentage={(analysis.wide_dens || 0) * 100}
                colorClass="bg-pink-500"
            />
            <StatBar
                label="Extreme (5.0x+ D)"
                value={analysis.extreme_count || 0}
                percentage={(analysis.extreme_dens || 0) * 100}
                colorClass="bg-pink-500"
            />

            <p className="text-xs font-semibold text-pink-400 uppercase mb-2">
                Jump Chain Profile
            </p>
            <StatBar
                label="Short chains (3-5)"
                value={analysis.short_jumps || 0}
                total={totalChains}
                colorClass="bg-pink-500"
            />
            <StatBar
                label="Medium chains (6-11)"
                value={analysis.medium_jumps || 0}
                total={totalChains}
                colorClass="bg-pink-500"
            />
            <StatBar
                label="Long chains (12+)"
                value={analysis.long_jumps || 0}
                total={totalChains}
                colorClass="bg-pink-500"
            />

            <li className="flex justify-between border-t border-pink-900 pt-2 font-semibold">
                <span>Max jump chain:</span>
                <span>{analysis.max_jump_length} notes</span>
            </li>
            <li className="flex justify-between">
                <span>BPM Consistency:</span>
                <span>
                    {((analysis.bpm_consistency || 0) * 100).toFixed(1)}%
                </span>
            </li>
        </div>
    );
}
