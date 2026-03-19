import React from 'react';
import { Card } from "@/components/ui/card";
import { AimControlResult } from './types';
import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer } from 'recharts';

interface AimControlProfileProps {
    data: AimControlResult;
}

// Bypassing strict TS by accepting 'any' from Recharts axis
const formatTime = (ms: any) => {
    if (!ms || ms <= 0) return "0:00";
    const totalSeconds = Math.floor(Number(ms) / 1000);
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
};

// UPDATED: Now supports passing 'percentage' directly for our new Volatility metrics,
// while still supporting 'value' and 'total' for your old metrics.
const StatBar = ({ 
    label, value, total, percentage, colorClass = "bg-blue-500" 
}: { 
    label: string; value?: number; total?: number; percentage?: number; colorClass?: string; 
}) => {
    // If percentage is provided directly, use it. Otherwise calculate it.
    const displayPercentage = percentage !== undefined 
        ? percentage 
        : (total && total > 0 ? ((value || 0) / total) * 100 : 0);
    
    return (
        <div className="mb-2">
            <div className="flex justify-between text-xs mb-0.5">
                <span className="text-gray-300">{label}</span>
                <span className="font-mono text-gray-400">
                    {value !== undefined && `${value} `}
                    <span className="text-[10px]">({displayPercentage.toFixed(1)}%)</span>
                </span>
            </div>
            <div className="h-1 w-full bg-gray-800 rounded-full overflow-hidden">
                <div className={`h-full ${colorClass}`} style={{ width: `${displayPercentage}%` }} />
            </div>
        </div>
    );
};

// Custom Tooltip replaces the buggy formatter prop
const CustomTooltip = ({ active, payload, label }: any) => {
    if (active && payload && payload.length) {
        return (
            <div className="bg-gray-900 border border-gray-700 p-2 rounded-md shadow-md text-xs">
                <p className="text-gray-400 mb-1">Time: {formatTime(label)}</p>
                <p className="text-red-400 font-semibold">
                    Strain: {payload[0].value}
                </p>
            </div>
        );
    }
    return null;
};

export const AimControlProfile: React.FC<AimControlProfileProps> = ({ data }) => {
    if (!data?.spatial?.spacing_distribution) {
        return (
            <div className="p-4 bg-red-900/20 border border-red-900 rounded-md text-red-400 text-sm mt-4">
                <strong>Backend Version Mismatch:</strong> The UI expects the Stage 3 data shape, but the backend returned older data. Please run <code>cargo clean</code> and then <code>cargo run</code>.
            </div>
        );
    }

    const safeStrainCurve = data.endurance.strain_curve || [];
    const strainChartData = safeStrainCurve.map((point) => ({
        timeMs: point.time,
        strain: parseFloat(point.strain.toFixed(2))
    }));

    const totalSpacing = Object.values(data.spatial.spacing_distribution).reduce((a: any, b: any) => a + b, 0) as number;
    const totalAngles = Object.values(data.spatial.angle_distribution).reduce((a: any, b: any) => a + b, 0) as number;
    const totalAlignment = Object.values(data.vectors.alignment).reduce((a: any, b: any) => a + b, 0) as number;

    // Helper to map volatility distribution into the 5 colored StatBars
    const renderDistribution = (dist: any) => {
        if (!dist) return null;
        return (
            <>
                {/* Note: switches_0 becomes switches0, switches_more_than_3 becomes switchesMoreThan3 */}
                <StatBar label="0 Switches (Stable)" percentage={dist.switches0} colorClass="bg-emerald-400" />
                <StatBar label="1 - 2 Switches" percentage={dist.switches12} colorClass="bg-blue-400" />
                <StatBar label="3 - 4 Switches" percentage={dist.switches34} colorClass="bg-yellow-400" />
                <StatBar label="5 - 6 Switches" percentage={dist.switches56} colorClass="bg-orange-400" />
                <StatBar label="7 Switches (Chaotic)" percentage={dist.switches7} colorClass="bg-red-500" />
            </>
        );
    };

    // Safely extract aim_volatility if it exists on the data object
    const aimVolatility = (data as any).aimVolatility;

    return (
        <div className="space-y-4 mt-2">
            <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
                <Card className="bg-gray-900/50 border-gray-800 p-3">
                    <p className="text-[10px] font-medium text-gray-400 uppercase tracking-wider mb-1">Avg Spacing</p>
                    <p className="text-lg font-semibold text-gray-100">{(data.spatial.avg_spacing_d || 0).toFixed(2)} <span className="text-xs text-gray-500 font-normal">D</span></p>
                </Card>
                <Card className="bg-gray-900/50 border-gray-800 p-3">
                    <p className="text-[10px] font-medium text-gray-400 uppercase tracking-wider mb-1">Avg Velocity</p>
                    <p className="text-lg font-semibold text-gray-100">{(data.kinematics.avg_velocity || 0).toFixed(2)} <span className="text-xs text-gray-500 font-normal">px/ms</span></p>
                </Card>
                <Card className="bg-gray-900/50 border-gray-800 p-3">
                    <p className="text-[10px] font-medium text-gray-400 uppercase tracking-wider mb-1">Dir Flips</p>
                    <p className="text-lg font-semibold text-orange-400">{data.vectors.directional_flips || 0}</p>
                </Card>
                <Card className="bg-gray-900/50 border-gray-800 p-3">
                    <p className="text-[10px] font-medium text-gray-400 uppercase tracking-wider mb-1">Peak Strain</p>
                    <p className="text-lg font-semibold text-red-400">{(data.endurance.peak_strain || 0).toFixed(0)}</p>
                </Card>
            </div>

            <Card className="border-gray-800 p-4">
                <h3 className="text-sm font-semibold mb-3">Spacing Profile</h3>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1">
                    <StatBar label="Stacked / Overlap" value={data.spatial.spacing_distribution.stacked} total={totalSpacing} colorClass="bg-gray-500" />
                    <StatBar label="Micro (Wiggles)" value={data.spatial.spacing_distribution.micro} total={totalSpacing} colorClass="bg-blue-400" />
                    <StatBar label="Flow Aim" value={data.spatial.spacing_distribution.flow} total={totalSpacing} colorClass="bg-emerald-400" />
                    <StatBar label="Standard Jumps" value={data.spatial.spacing_distribution.standard} total={totalSpacing} colorClass="bg-orange-400" />
                    <StatBar label="Fullscreen Jumps" value={data.spatial.spacing_distribution.large} total={totalSpacing} colorClass="bg-red-500" />
                </div>
            </Card>

            <Card className="border-gray-800 p-4">
                <h3 className="text-sm font-semibold mb-4">Deflection & Vectors</h3>
                <div className="space-y-6">
                    <div>
                        <h4 className="text-[10px] font-semibold text-gray-500 uppercase tracking-wider mb-2 border-b border-gray-800 pb-1">Angles (Pathing)</h4>
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1 mt-2">
                            <StatBar label="Linear (Straight)" value={data.spatial.angle_distribution.linear} total={totalAngles} colorClass="bg-emerald-400" />
                            <StatBar label="Wide (Flow)" value={data.spatial.angle_distribution.wide} total={totalAngles} colorClass="bg-blue-400" />
                            <StatBar label="Acute (Sharp Tech)" value={data.spatial.angle_distribution.acute} total={totalAngles} colorClass="bg-orange-400" />
                            <StatBar label="Snap-Backs (180s)" value={data.spatial.angle_distribution.snap_backs} total={totalAngles} colorClass="bg-red-400" />
                        </div>
                    </div>
                    <div>
                        <h4 className="text-[10px] font-semibold text-gray-500 uppercase tracking-wider mb-2 border-b border-gray-800 pb-1">Alignment (Pattern Logic)</h4>
                        <div className="mb-2 mt-2">
                            <div className="flex justify-between text-xs mb-0.5">
                                <span className="text-gray-300">Chirps (Rotation Resets)</span>
                                <span className="font-mono text-gray-400">{data.vectors.directional_chirps || 0}</span>
                            </div>
                        </div>
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1">
                            <StatBar label="Parallel (Stars/Farm)" value={data.vectors.alignment.parallel} total={totalAlignment} colorClass="bg-emerald-400" />
                            <StatBar label="Orthogonal (Squares/Tech)" value={data.vectors.alignment.orthogonal} total={totalAlignment} colorClass="bg-orange-400" />
                            <StatBar label="Anti-Symmetric (Overlaps)" value={data.vectors.alignment.anti_symmetric} total={totalAlignment} colorClass="bg-red-400" />
                        </div>
                    </div>
                </div>
            </Card>

            {/* NEW SECTION: Volatility & Complexity */}
            {aimVolatility && (
                <Card className="border-gray-800 p-4">
                    <h3 className="text-sm font-semibold mb-4">Aim Complexity & Volatility</h3>
                    <div className="space-y-6">
                         {/* NEW: Relative Velocity Distribution */}
                        <div>
                            <h4 className="text-[10px] font-semibold text-gray-500 uppercase tracking-wider mb-2 border-b border-gray-800 pb-1">Relative Velocity Distribution</h4>
                            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1 mt-2">
                                <StatBar label="Significantly Slower" percentage={aimVolatility.velocityBuckets.switches0} colorClass="bg-blue-500" />
                                <StatBar label="Slower" percentage={aimVolatility.velocityBuckets.switches12} colorClass="bg-blue-300" />
                                <StatBar label="Standard (Mean)" percentage={aimVolatility.velocityBuckets.switches34} colorClass="bg-emerald-400" />
                                <StatBar label="Faster" percentage={aimVolatility.velocityBuckets.switches56} colorClass="bg-orange-300" />
                                <StatBar label="Significantly Faster" percentage={aimVolatility.velocityBuckets.switches7} colorClass="bg-red-500" />
                            </div>
                        </div>

                        {/* NEW: Velocity Switch Intensity */}
                        <div>
                            <h4 className="text-[10px] font-semibold text-gray-500 uppercase tracking-wider mb-2 border-b border-gray-800 pb-1">Velocity Switch Intensity</h4>
                            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1 mt-2">
                                <StatBar label="Major Adjustment (2+ Step)" percentage={aimVolatility.velocityIntensity.majorAdjustment} colorClass="bg-red-500" />
                                <StatBar label="Minor Adjustment (1 Step)" percentage={aimVolatility.velocityIntensity.minorAdjustment} colorClass="bg-orange-400" />
                                <StatBar label="Steady Velocity" percentage={aimVolatility.velocityIntensity.steady} colorClass="bg-emerald-400" />
                            </div>
                        </div>

                        {/* NEW: Aim Style (Snap vs Flow) */}
                        <div>
                            <h4 className="text-[10px] font-semibold text-gray-500 uppercase tracking-wider mb-2 border-b border-gray-800 pb-1">Aim Style Balance</h4>
                            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1 mt-2">
                                <StatBar label="Snap Aim Bias" percentage={aimVolatility.snapFlow.snapAim} colorClass="bg-red-400" />
                                <StatBar label="Flow Aim Bias" percentage={aimVolatility.snapFlow.flowAim} colorClass="bg-emerald-400" />
                            </div>
                        </div>
                        <div>
                            <h4 className="text-[10px] font-semibold text-gray-500 uppercase tracking-wider mb-2 border-b border-gray-800 pb-1">Angle Switches</h4>
                            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1 mt-2">
                                {renderDistribution(aimVolatility.angle)}
                            </div>
                        </div>
                        <div>
                            <h4 className="text-[10px] font-semibold text-gray-500 uppercase tracking-wider mb-2 border-b border-gray-800 pb-1">Direction Switches</h4>
                            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1 mt-2">
                                {/* Access key updated to direction */}
                                {renderDistribution(aimVolatility.direction)}
                            </div>
                        </div>
                        <div>
                            <h4 className="text-[10px] font-semibold text-gray-500 uppercase tracking-wider mb-2 border-b border-gray-800 pb-1">Aim Texture Matrix</h4>
                            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-1 mt-2">
                                <StatBar label="Consistent (Flow/Farm)" percentage={0.0} colorClass="bg-emerald-400" />
                                <StatBar label="Flow Tech" percentage={0.0} colorClass="bg-blue-400" />
                                <StatBar label="Rhythmic Tech" percentage={0.0} colorClass="bg-purple-400" />
                                <StatBar label="Chaotic Tech" percentage={0.0} colorClass="bg-pink-500" />
                            </div>
                        </div>
                    </div>
                </Card>
            )}

            {/* NEW SECTION: Burst Aim Physicality */}
            {data?.burst_aim && (
                <Card className="border-gray-800 p-4">
                    <h3 className="text-sm font-semibold mb-4">Burst Aim Profile (2n - 6n)</h3>
                    <div className="space-y-6">
                        <div>
                            <h4 className="text-[10px] font-semibold text-gray-500 uppercase tracking-wider mb-2 border-b border-gray-800 pb-1">Average Burst Spacing</h4>
                            <div className="grid grid-cols-1 md:grid-cols-3 gap-x-4 gap-y-1 mt-2">
                                <StatBar label="Stacked (<0.5D)" percentage={data.burst_aim.avgSpacing.low} colorClass="bg-blue-400" />
                                <StatBar label="Flow (0.5-2D)" percentage={data.burst_aim.avgSpacing.mid} colorClass="bg-emerald-400" />
                                <StatBar label="Jump (>2D)" percentage={data.burst_aim.avgSpacing.high} colorClass="bg-red-400" />
                            </div>
                        </div>
                        <div>
                            <h4 className="text-[10px] font-semibold text-gray-500 uppercase tracking-wider mb-2 border-b border-gray-800 pb-1">Burst Spacing Variance</h4>
                            <div className="grid grid-cols-1 md:grid-cols-3 gap-x-4 gap-y-1 mt-2">
                                <StatBar label="Constant" percentage={data.burst_aim.variance.low} colorClass="bg-emerald-400" />
                                <StatBar label="Adaptive" percentage={data.burst_aim.variance.mid} colorClass="bg-orange-400" />
                                <StatBar label="Erratic" percentage={data.burst_aim.variance.high} colorClass="bg-red-500" />
                            </div>
                        </div>
                        <div>
                            <h4 className="text-[10px] font-semibold text-gray-500 uppercase tracking-wider mb-2 border-b border-gray-800 pb-1">Burst Spacing Spikes</h4>
                            <div className="grid grid-cols-1 md:grid-cols-3 gap-x-4 gap-y-1 mt-2">
                                <StatBar label="Flat" percentage={data.burst_aim.spikes.low} colorClass="bg-emerald-400" />
                                <StatBar label="Accented" percentage={data.burst_aim.spikes.mid} colorClass="bg-orange-400" />
                                <StatBar label="Kick-Gap" percentage={data.burst_aim.spikes.high} colorClass="bg-red-500" />
                            </div>
                        </div>
                    </div>
                </Card>
            )}

            <Card className="border-gray-800 p-4">
                <div className="flex flex-row items-center justify-between mb-3">
                    <div>
                        <h3 className="text-sm font-semibold">Sustained Aim Strain</h3>
                        <p className="text-[10px] text-gray-500">Kinematic speed & angular tension (EMA).</p>
                    </div>
                    <div className="text-right">
                        <p className="text-[10px] text-gray-500 uppercase tracking-wider">Time Under Tension</p>
                        <p className="text-md font-mono font-semibold text-red-400">{formatTime(data.endurance.time_under_tension_ms)}</p>
                    </div>
                </div>
                <div className="h-48">
                    <ResponsiveContainer width="100%" height="100%">
                        <LineChart data={strainChartData} margin={{ top: 5, right: 20, left: 0, bottom: 5 }}>
                            <XAxis 
                                dataKey="timeMs" 
                                stroke="#4b5563" 
                                fontSize={10} 
                                tickLine={false} 
                                axisLine={false} 
                                tickFormatter={formatTime} 
                                minTickGap={30}
                            />
                            <YAxis stroke="#4b5563" fontSize={10} tickLine={false} axisLine={false} />
                            <Tooltip content={<CustomTooltip />} />
                            <Line type="monotone" dataKey="strain" stroke="#f87171" strokeWidth={1.5} dot={false} activeDot={{ r: 3, fill: '#f87171', stroke: '#111827', strokeWidth: 1.5 }} />
                        </LineChart>
                    </ResponsiveContainer>
                </div>
            </Card>
        </div>
    );
};