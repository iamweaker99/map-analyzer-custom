import { JumpAnalysis } from "./types";
import { getSpacingTag } from "./utils";

export function JumpProfile({ analysis }: { analysis: JumpAnalysis }) {
    const d = analysis.circle_diameter || 73;
    const spacing = analysis.avg_spacing || 0;

    return (
        <div className="space-y-4">
            <li className="font-bold border-b border-gray-700 pb-1 mb-2">
                Spacing: {getSpacingTag(spacing, d)} ({spacing.toFixed(1)} px)
            </li>
            
            <p className="text-xs font-semibold text-pink-400 uppercase mb-2">Distance Profile (Excluding Streams)</p>
            <li className="flex justify-between"><span>Narrow (&lt; 2.0x D):</span><span >{analysis.narrow_count || 0} ({((analysis.narrow_dens || 0) * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Moderate (2-3.5x D):</span><span >{analysis.moderate_count || 0} ({((analysis.moderate_dens || 0) * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Wide (3.5-5x D):</span><span >{analysis.wide_count || 0} ({((analysis.wide_dens || 0) * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between mb-3"><span>Extreme (5.0x+ D):</span><span >{analysis.extreme_count || 0} ({((analysis.extreme_dens || 0) * 100).toFixed(1)}%)</span></li>

            <p className="text-xs font-semibold text-pink-400 uppercase mb-2">Jump Chain Profile</p>
            <li className="flex justify-between"><span>Short chains (3-5):</span><span >{analysis.short_jumps || 0}</span></li>
            <li className="flex justify-between"><span>Medium chains (6-11):</span><span >{analysis.medium_jumps || 0}</span></li>
            <li className="flex justify-between mb-2"><span>Long chains (12+):</span><span >{analysis.long_jumps || 0}</span></li>
            
            <li className="flex justify-between border-t border-pink-900 pt-2 font-semibold">
                <span>Max jump chain:</span><span >{analysis.max_jump_length} notes</span>
            </li>
            <li className="flex justify-between">
                <span>BPM Consistency:</span><span >{((analysis.bpm_consistency || 0) * 100).toFixed(1)}%</span>
            </li>
        </div>
    );
}