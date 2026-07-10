import { SliderAnalysis } from "./types";
import { getSliderTag } from "./utils";
import { StatBar } from "./StatBar";

export function SliderProfile({ analysis }: { analysis: SliderAnalysis }) {
    return (
        <div className="space-y-6">
            <li className="font-bold border-b border-green-900 pb-1 mb-2">
                Style: {getSliderTag(analysis.slider_ratio)} (Avg SV:{" "}
                {analysis.avg_velocity.toFixed(2)})
            </li>

            <h3 className="text-sm font-semibold text-green-400 mb-4">
                Length Profile (Rel. to Map)
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1">
                <StatBar
                    label="Short (&lt;1.5x D)"
                    value={analysis.l_short_count || 0}
                    percentage={(analysis.l_short_dens || 0) * 100}
                    colorClass="bg-green-500"
                />
                <StatBar
                    label="Medium (1.5-3x D)"
                    value={analysis.l_med_count || 0}
                    percentage={(analysis.l_med_dens || 0) * 100}
                    colorClass="bg-blue-500"
                />
                <StatBar
                    label="Long (3-4.5x D)"
                    value={analysis.l_long_count || 0}
                    percentage={(analysis.l_long_dens || 0) * 100}
                    colorClass="bg-orange-500"
                />
                <StatBar
                    label="Extended (&gt;4.5x D)"
                    value={analysis.l_ext_count || 0}
                    percentage={(analysis.l_ext_dens || 0) * 100}
                    colorClass="bg-red-500"
                />
            </div>

            <h3 className="text-sm font-semibold text-green-400 mb-4">
                Buzz Profile (Rel. to Sliders)
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1">
                <StatBar
                    label="Buzz Sliders"
                    value={analysis.b_buzz_count || 0}
                    percentage={(analysis.b_buzz_dens || 0) * 100}
                    colorClass="bg-blue-500"
                />
                <StatBar
                    label="Static Buzz"
                    value={analysis.b_static_count || 0}
                    percentage={(analysis.b_static_dens || 0) * 100}
                    colorClass="bg-green-500"
                />
            </div>

            <h3 className="text-sm font-semibold text-green-400 mb-4">
                Artistic Profile (Rel. to Sliders)
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1">
                <StatBar
                    label="Simple (Linear)"
                    value={analysis.a_simple_count || 0}
                    percentage={(analysis.a_simple_dens || 0) * 100}
                    colorClass="bg-green-500"
                />
                <StatBar
                    label="Curved"
                    value={analysis.a_curved_count || 0}
                    percentage={(analysis.a_curved_dens || 0) * 100}
                    colorClass="bg-blue-500"
                />
                <StatBar
                    label="Complex"
                    value={analysis.a_complex_count || 0}
                    percentage={(analysis.a_complex_dens || 0) * 100}
                    colorClass="bg-orange-500"
                />
                <StatBar
                    label="Artistic/Tech"
                    value={analysis.a_artistic_count || 0}
                    percentage={(analysis.a_artistic_dens || 0) * 100}
                    colorClass="bg-red-500"
                />
            </div>
        </div>
    );
}
