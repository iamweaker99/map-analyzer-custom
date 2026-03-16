import React from 'react';
import { Card } from "@/components/ui/card";
import { AimControlResult } from './types';
import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer } from 'recharts';

interface AimControlProfileProps {
    data: AimControlResult;
}

const formatTime = (ms: number) => {
    if (!ms) return "0:00";
    const totalSeconds = Math.floor(ms / 1000);
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

export const AimControlProfile: React.FC<AimControlProfileProps> = ({ data }) => {
    const safeStrainCurve = data?.endurance?.strain_curve || [];
    const safeSpacingDist = data?.spatial?.spacing_distribution || {};
    const safeAngleDist = data?.spatial?.angle_distribution || {};
    const safeAlignment = data?.vectors?.alignment || {};

    const strainChartData = safeStrainCurve.map((point) => ({
        timeMs: point.time,
        strain: parseFloat(point.strain.toFixed(2))
    }));

    const totalSpacing = Object.values(safeSpacingDist).reduce((a: any, b: any) => a + b, 0) as number;
    const totalAngles = Object.values(safeAngleDist).reduce((a: any, b: any) => a + b, 0) as number;
    const totalAlignment = Object.values(safeAlignment).reduce((a: any, b: any) => a + b, 0) as number;

    return (
        <div className="space-y-4 mt-2">
            <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
                <Card className="bg-gray-900/50 border-gray-800 p-3">
                    <p className="text-[10px] font-medium text-gray-400 uppercase tracking-wider mb-1">Avg Spacing</p>
                    <p className="text-lg font-semibold text-gray-100">{(data?.spatial?.avg_spacing_d || 0).toFixed(2)} <span className="text-xs text-gray-500 font-normal">D</span></p>
                </Card>
                <Card className="bg-gray-900/50 border-gray-800 p-3">
                    <p className="text-[10px] font-medium text-gray-400 uppercase tracking-wider mb-1">Avg Velocity</p>
                    <p className="text-lg font-semibold text-gray-100">{(data?.kinematics?.avg_velocity || 0).toFixed(2)} <span className="text-xs text-gray-500 font-normal">px/ms</span></p>
                </Card>
                <Card className="bg-gray-900/50 border-gray-800 p-3">
                    <p className="text-[10px] font-medium text-gray-400 uppercase tracking-wider mb-1">Dir Flips</p>
                    <p className="text-lg font-semibold text-orange-400">{data?.vectors?.directional_flips || 0}</p>
                </Card>
                <Card className="bg-gray-900/50 border-gray-800 p-3">
                    <p className="text-[10px] font-medium text-gray-400 uppercase tracking-wider mb-1">Peak Strain</p>
                    <p className="text-lg font-semibold text-red-400">{(data?.endurance?.peak_strain || 0).toFixed(0)}</p>
                </Card>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <Card className="border-gray-800 p-4">
                    <h3 className="text-sm font-semibold mb-3">Spacing Profile</h3>
                    <div>
                        <StatBar label="Stacked / Overlap" value={safeSpacingDist.stacked} total={totalSpacing} colorClass="bg-gray-500" />
                        <StatBar label="Micro (Wiggles)" value={safeSpacingDist.micro} total={totalSpacing} colorClass="bg-blue-400" />
                        <StatBar label="Flow Aim" value={safeSpacingDist.flow} total={totalSpacing} colorClass="bg-emerald-400" />
                        <StatBar label="Standard Jumps" value={safeSpacingDist.standard} total={totalSpacing} colorClass="bg-orange-400" />
                        <StatBar label="Fullscreen Jumps" value={safeSpacingDist.large} total={totalSpacing} colorClass="bg-red-500" />
                    </div>
                </Card>

                <Card className="border-gray-800 p-4">
                    <h3 className="text-sm font-semibold mb-3">Deflection & Vectors</h3>
                    <div className="grid grid-cols-2 gap-4">
                        <div>
                            <h4 className="text-[10px] font-semibold text-gray-500 uppercase tracking-wider mb-2">Angles</h4>
                            <StatBar label="Snap-Backs" value={safeAngleDist.snap_backs} total={totalAngles} colorClass="bg-red-400" />
                            <StatBar label="Acute" value={safeAngleDist.acute} total={totalAngles} colorClass="bg-orange-400" />
                            <StatBar label="Wide" value={safeAngleDist.wide} total={totalAngles} colorClass="bg-blue-400" />
                            <StatBar label="Linear" value={safeAngleDist.linear} total={totalAngles} colorClass="bg-emerald-400" />
                        </div>
                        <div>
                            <h4 className="text-[10px] font-semibold text-gray-500 uppercase tracking-wider mb-2">Alignment</h4>
                            <div className="mb-2">
                                <div className="flex justify-between text-xs mb-0.5">
                                    <span className="text-gray-300">Chirps (Resets)</span>
                                    <span className="font-mono text-gray-400">{data?.vectors?.directional_chirps || 0}</span>
                                </div>
                            </div>
                            <StatBar label="Parallel (Farm)" value={safeAlignment.parallel} total={totalAlignment} colorClass="bg-emerald-400" />
                            <StatBar label="Orthogonal (Tech)" value={safeAlignment.orthogonal} total={totalAlignment} colorClass="bg-orange-400" />
                            <StatBar label="Anti-Symmetric" value={safeAlignment.anti_symmetric} total={totalAlignment} colorClass="bg-red-400" />
                        </div>
                    </div>
                </Card>
            </div>

            <Card className="border-gray-800 p-4">
                <div className="flex flex-row items-center justify-between mb-3">
                    <div>
                        <h3 className="text-sm font-semibold">Sustained Aim Strain</h3>
                        <p className="text-[10px] text-gray-500">Kinematic speed & angular tension (EMA).</p>
                    </div>
                    <div className="text-right">
                        <p className="text-[10px] text-gray-500 uppercase tracking-wider">Time Under Tension</p>
                        <p className="text-md font-mono font-semibold text-red-400">{formatTime(data?.endurance?.time_under_tension_ms || 0)}</p>
                    </div>
                </div>
                <div className="h-48">
                    <ResponsiveContainer width="100%" height="100%">
                        <LineChart data={strainChartData}>
                            <XAxis 
                                dataKey="timeMs" 
                                stroke="#4b5563" 
                                fontSize={10} 
                                tickLine={false} 
                                axisLine={false} 
                                tickFormatter={(val) => formatTime(val)}
                                minTickGap={30}
                            />
                            <YAxis stroke="#4b5563" fontSize={10} tickLine={false} axisLine={false} />
                            <Tooltip 
                                contentStyle={{ backgroundColor: '#111827', border: '1px solid #374151', borderRadius: '6px', fontSize: '12px' }}
                                labelStyle={{ color: '#9ca3af', marginBottom: '2px' }}
                                itemStyle={{ color: '#f87171', fontWeight: 600 }}
                                formatter={(value: number) => [value, 'Strain']}
                                labelFormatter={(label) => `Time: ${formatTime(label as number)}`}
                            />
                            <Line type="monotone" dataKey="strain" stroke="#f87171" strokeWidth={1.5} dot={false} activeDot={{ r: 3, fill: '#f87171', stroke: '#111827', strokeWidth: 1.5 }} />
                        </LineChart>
                    </ResponsiveContainer>
                </div>
            </Card>
        </div>
    );
};