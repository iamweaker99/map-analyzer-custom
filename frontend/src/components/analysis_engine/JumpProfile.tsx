import { JumpAnalysis } from "./types";
import { getSpacingTag } from "./utils";
import { StatBar } from "./StatBar";

export function JumpProfile({ analysis }: { analysis: JumpAnalysis }) {
    const d = analysis.circle_diameter || 73;
    const spacing = analysis.avg_spacing || 0;

    const totalDurationChains =
        (analysis.duration_short_chains || 0) +
        (analysis.duration_medium_chains || 0) +
        (analysis.duration_long_chains || 0) +
        (analysis.duration_extreme_chains || 0);
    const totalAbsoluteDistances =
        (analysis.absolute_short_count || 0) +
        (analysis.absolute_medium_count || 0) +
        (analysis.absolute_long_count || 0) +
        (analysis.absolute_extreme_count || 0) +
        (analysis.absolute_cross_screen_count || 0);

    return (
        <div className="space-y-6">
            <li className="font-bold border-b border-gray-700 pb-1 mb-2">
                Spacing: {getSpacingTag(spacing, d)} ({spacing.toFixed(1)} px)
            </li>

            <h3 className="text-sm font-semibold text-pink-400 mb-4">
                Distance Profile (Excluding Streams)
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1">
                <StatBar
                    label="Narrow (&lt;20% / 76.8 px)"
                    value={analysis.absolute_short_count || 0}
                    total={totalAbsoluteDistances}
                    colorClass="bg-green-500"
                />
                <StatBar
                    label="Moderate (&lt;40% / 153.6 px)"
                    value={analysis.absolute_medium_count || 0}
                    total={totalAbsoluteDistances}
                    colorClass="bg-blue-500"
                />
                <StatBar
                    label="Wide (&lt;60% / 230.4 px)"
                    value={analysis.absolute_long_count || 0}
                    total={totalAbsoluteDistances}
                    colorClass="bg-orange-500"
                />
                <StatBar
                    label="Extreme (&lt;80% / 307.2 px)"
                    value={analysis.absolute_extreme_count || 0}
                    total={totalAbsoluteDistances}
                    colorClass="bg-red-500"
                />
                <StatBar
                    label="Cross-Screen (&ge;80% / 307.2 px)"
                    value={analysis.absolute_cross_screen_count || 0}
                    total={totalAbsoluteDistances}
                    colorClass="bg-red-500"
                />
            </div>

            <h3 className="text-sm font-semibold text-pink-400 mb-4">
                Jump Chain Profile
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1">
                <StatBar
                    label="Short (&lt;1s)"
                    value={analysis.duration_short_chains || 0}
                    total={totalDurationChains}
                    colorClass="bg-green-500"
                />
                <StatBar
                    label="Medium (&lt;2s)"
                    value={analysis.duration_medium_chains || 0}
                    total={totalDurationChains}
                    colorClass="bg-blue-500"
                />
                <StatBar
                    label="Long (&lt;4s)"
                    value={analysis.duration_long_chains || 0}
                    total={totalDurationChains}
                    colorClass="bg-orange-500"
                />
                <StatBar
                    label="Extreme (&ge;4s)"
                    value={analysis.duration_extreme_chains || 0}
                    total={totalDurationChains}
                    colorClass="bg-red-500"
                />
            </div>

            <li className="flex justify-between border-t border-pink-900 pt-2 font-semibold">
                <span>Max jump chain:</span>
                <span>
                    {analysis.max_jump_length} notes /{" "}
                    {(analysis.max_jump_duration || 0).toFixed(1)}s
                </span>
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
