import { SliderAnalysis } from "./types";
import { getSliderTag } from "./utils";

export function SliderProfile({ analysis }: { analysis: SliderAnalysis }) {
    return (
        <div className="space-y-4">
            <li className="font-bold border-b border-green-900 pb-1 mb-2">
                Style: {getSliderTag(analysis.slider_ratio)} (Avg SV: {analysis.avg_velocity.toFixed(2)})
            </li>

            <p className="text-xs font-semibold text-green-400 uppercase mb-2">Slider Length Profile (Rel. to Map)</p>
            <li className="flex justify-between"><span>Short (&lt;1.5x D):</span><span >{analysis.l_short_count} ({(analysis.l_short_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Medium (1.5-3x D):</span><span >{analysis.l_med_count} ({(analysis.l_med_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Long (3-4.5x D):</span><span >{analysis.l_long_count} ({(analysis.l_long_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between mb-4"><span>Extended (&gt;4.5x D):</span><span >{analysis.l_ext_count} ({(analysis.l_ext_dens * 100).toFixed(1)}%)</span></li>

            <p className="text-xs font-semibold text-green-400 uppercase mb-2">Buzz Profile (Rel. to Sliders)</p>
            <li className="flex justify-between"><span>Buzz Sliders:</span><span >{analysis.b_buzz_count} ({(analysis.b_buzz_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between mb-4"><span>Static Buzz:</span><span >{analysis.b_static_count} ({(analysis.b_static_dens * 100).toFixed(1)}%)</span></li>

            <p className="text-xs font-semibold text-green-400 uppercase mb-2">Artistic Profile (Rel. to Sliders)</p>
            <li className="flex justify-between"><span>Simple (Linear):</span><span >{analysis.a_simple_count} ({(analysis.a_simple_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Curved:</span><span >{analysis.a_curved_count} ({(analysis.a_curved_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Complex:</span><span >{analysis.a_complex_count} ({(analysis.a_complex_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Artistic/Tech:</span><span >{analysis.a_artistic_count} ({(analysis.a_artistic_dens * 100).toFixed(1)}%)</span></li>
        </div>
    );
}