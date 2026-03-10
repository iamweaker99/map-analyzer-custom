import { StreamAnalysis } from "./types";
import { getStreamSpacingTag } from "./utils";

export function StreamProfile({ analysis, totalObjects }: { analysis: StreamAnalysis; totalObjects: number; }) {
    const avg = analysis.avg_stream_spacing || 0;
    const d = analysis.circle_diameter || 73;
    const totalPatterns = analysis.total_stream_patterns || 0;

    return (
        <div className="space-y-4">
            <li className="font-bold border-b border-blue-900 pb-1 mb-2">
                Type: {getStreamSpacingTag(avg, d)} ({avg.toFixed(1)} px)
            </li>

            <p className="text-xs font-semibold text-blue-400 uppercase mb-2">Distance Profile (Density by Notes)</p>
            <li className="flex justify-between"><span>Stacked (&lt;0.5x D):</span><span >{analysis.s_stacked_count} ({((analysis.s_stack_dens || 0) * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Overlapping (0.5-1x D):</span><span >{analysis.s_overlapping_count} ({((analysis.s_over_dens || 0) * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Spaced (1-2x D):</span><span >{analysis.s_spaced_count} ({((analysis.s_space_dens || 0) * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between mb-4"><span>Extreme (2-2.5x D):</span><span >{analysis.s_extreme_count} ({((analysis.s_extr_dens || 0) * 100).toFixed(1)}%)</span></li>

            <p className="text-xs font-semibold text-blue-400 uppercase mb-2">Variance Profile</p>
            <li className="flex justify-between"><span>Steady:</span><span >{analysis.v_steady_count} ({totalPatterns > 0 ? ((analysis.v_steady_count / totalPatterns) * 100).toFixed(1) : 0}%)</span></li>
            <li className="flex justify-between"><span>Variable:</span><span >{analysis.v_variable_count} ({totalPatterns > 0 ? ((analysis.v_variable_count / totalPatterns) * 100).toFixed(1) : 0}%)</span></li>
            <li className="flex justify-between mb-4"><span>Dynamic:</span><span >{analysis.v_dynamic_count} ({totalPatterns > 0 ? ((analysis.v_dynamic_count / totalPatterns) * 100).toFixed(1) : 0}%)</span></li>

            <p className="text-xs font-semibold text-blue-400 uppercase mb-2">Length Profile</p>
            <li className="flex justify-between"><span>Bursts (3-4):</span><span >{analysis.bursts}</span></li>
            <li className="flex justify-between"><span>Short (5-12):</span><span >{analysis.short_streams}</span></li>
            <li className="flex justify-between"><span>Medium (13-24):</span><span >{analysis.medium_streams}</span></li>
            <li className="flex justify-between"><span>Long (25-48):</span><span >{analysis.long_streams}</span></li>
            <li className="flex justify-between text-blue-300 font-semibold mb-2"><span>Deathstream (49+):</span><span >{analysis.death_streams}</span></li>
            
            <li className="flex justify-between border-t border-blue-900 pt-2 font-semibold">
                <span>Max stream:</span><span >{analysis.max_stream_length} notes</span>
            </li>
            <li className="flex justify-between">
                <span>BPM Consistency:</span><span >{((analysis.bpm_consistency || 0) * 100).toFixed(1)}%</span>
            </li>
        </div>
    );
}