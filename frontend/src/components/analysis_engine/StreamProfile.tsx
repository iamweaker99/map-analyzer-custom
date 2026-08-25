import { StreamAnalysis } from "./types";
import { getStreamSpacingTag } from "./utils";
import { StatBar } from "./StatBar";

export function StreamProfile({
    analysis,
    totalObjects,
}: {
    analysis: StreamAnalysis;
    totalObjects: number;
}) {
    const avg = analysis.avg_stream_spacing || 0;
    const d = analysis.circle_diameter || 73;
    const totalPatterns = analysis.total_stream_patterns || 0;

    const totalLength =
        (analysis.bursts || 0) +
        (analysis.short_streams || 0) +
        (analysis.medium_streams || 0) +
        (analysis.long_streams || 0) +
        (analysis.death_streams || 0);

    return (
        <div className="space-y-6">
            <li className="font-bold border-b border-blue-900 pb-1 mb-2">
                Type: {getStreamSpacingTag(avg, d)} ({avg.toFixed(1)} px)
            </li>

            <h3 className="text-sm font-semibold text-blue-400 mb-4">
                Distance Profile (Density by Notes)
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1">
                <StatBar
                    label="Stacked (&lt;0.5x D)"
                    value={analysis.s_stacked_count || 0}
                    percentage={(analysis.s_stack_dens || 0) * 100}
                    colorClass="bg-green-500"
                />
                <StatBar
                    label="Overlapping (0.5-1x D)"
                    value={analysis.s_overlapping_count || 0}
                    percentage={(analysis.s_over_dens || 0) * 100}
                    colorClass="bg-blue-500"
                />
                <StatBar
                    label="Spaced (1-2x D)"
                    value={analysis.s_spaced_count || 0}
                    percentage={(analysis.s_space_dens || 0) * 100}
                    colorClass="bg-orange-500"
                />
                <StatBar
                    label="Extreme (2-2.5x D)"
                    value={analysis.s_extreme_count || 0}
                    percentage={(analysis.s_extr_dens || 0) * 100}
                    colorClass="bg-red-500"
                />
            </div>

            <h3 className="text-sm font-semibold text-blue-400 mb-4">
                Variance Profile
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1">
                <StatBar
                    label="Steady"
                    value={analysis.v_steady_count || 0}
                    total={totalPatterns}
                    colorClass="bg-green-500"
                />
                <StatBar
                    label="Variable"
                    value={analysis.v_variable_count || 0}
                    total={totalPatterns}
                    colorClass="bg-blue-500"
                />
                <StatBar
                    label="Dynamic"
                    value={analysis.v_dynamic_count || 0}
                    total={totalPatterns}
                    colorClass="bg-red-500"
                />
            </div>

            <h3 className="text-sm font-semibold text-blue-400 mb-4">
                Length Profile
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1">
                <StatBar
                    label="Burst (0.5-1s)"
                    value={analysis.bursts || 0}
                    total={totalLength}
                    colorClass="bg-green-500"
                />
                <StatBar
                    label="Short (1-2s)"
                    value={analysis.short_streams || 0}
                    total={totalLength}
                    colorClass="bg-blue-500"
                />
                <StatBar
                    label="Medium (2-4s)"
                    value={analysis.medium_streams || 0}
                    total={totalLength}
                    colorClass="bg-orange-500"
                />
                <StatBar
                    label="Long (4-6s)"
                    value={analysis.long_streams || 0}
                    total={totalLength}
                    colorClass="bg-red-500"
                />
                <StatBar
                    label="Deathstream (6s+)"
                    value={analysis.death_streams || 0}
                    total={totalLength}
                    colorClass="bg-red-500"
                />
            </div>

            <li className="flex justify-between border-t border-blue-900 pt-2 font-semibold">
                <span>Max stream:</span>
                <span>{(analysis.max_stream_duration || 0).toFixed(1)}s / {analysis.max_stream_length} notes</span>
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
