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

const StatBar = ({ 
    label, value = 0, total = 0, colorClass = "bg-blue-500" 
}: { 
    label: string; value?: number; total?: number; colorClass?: string; 
}) => {
    const safeValue = value || 0;
    const safeTotal = total || 0;
    const percentage = safeTotal > 0 ? (safeValue / safeTotal) * 100 : 0;
    
    return (
        <div className="mb-2">
            <div className="flex justify-between text-xs mb-0.5">
                <span className="text-gray-300">{label}</span>
                <span className="font-mono text-gray-400">
                    {safeValue} <span className="text-[10px]">({percentage.toFixed(1)}%)</span>
                </span>
            </div>
            <div className="h-1 w-full bg-gray-800 rounded-full overflow-hidden">
                <div className={`h-full ${colorClass}`} style={{ width: `${percentage}%` }} />
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

            {/* NEW: ACCV Complexity Dashboard */}
            {data.accv && (
                <Card className="border-gray-800 p-4 bg-gray-900/30">
                    <div className="flex flex-row items-center justify-between mb-3">
                        <div>
                            <h3 className="text-sm font-semibold text-purple-400">Aim Control Complexity (ACCV)</h3>
                            <p className="text-[10px] text-gray-500">Multi-dimensional mechanical variance</p>
                        </div>
                    </div>
                    <div className="grid grid-cols-2 md:grid-cols-5 gap-3">
                        <div className="bg-gray-900/80 p-2 rounded border border-gray-800">
                            <p className="text-[10px] text-gray-400 uppercase">Peak (95%)</p>
                            <p className="text-lg font-semibold text-red-400">{data.accv.peak_complexity.toFixed(2)}</p>
                        </div>
                        <div className="bg-gray-900/80 p-2 rounded border border-gray-800">
                            <p className="text-[10px] text-gray-400 uppercase">Sustained (50%)</p>
                            <p className="text-lg font-semibold text-blue-400">{data.accv.sustained_complexity.toFixed(2)}</p>
                        </div>
                        <div className="bg-gray-900/80 p-2 rounded border border-gray-800">
                            <p className="text-[10px] text-gray-400 uppercase" title="Spacing Variance">Spatial Var</p>
                            <p className="text-lg font-semibold text-gray-200">{data.accv.peak_spatial_cv.toFixed(2)}</p>
                        </div>
                        <div className="bg-gray-900/80 p-2 rounded border border-gray-800">
                            <p className="text-[10px] text-gray-400 uppercase" title="Rhythm Variance">Temporal Var</p>
                            <p className="text-lg font-semibold text-gray-200">{data.accv.peak_temporal_cv.toFixed(2)}</p>
                        </div>
                        <div className="bg-gray-900/80 p-2 rounded border border-gray-800">
                            <p className="text-[10px] text-gray-400 uppercase" title="Angle Variance">Kinetic Var</p>
                            <p className="text-lg font-semibold text-gray-200">{data.accv.peak_kinetic_var.toFixed(2)}</p>
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
                        {/* Margin added to fix graph cut-offs */}
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