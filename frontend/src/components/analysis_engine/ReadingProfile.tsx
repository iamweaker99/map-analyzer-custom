import React from 'react';
import { Card } from "@/components/ui/card";
import { ReadingResult } from './types';

interface ReadingProfileProps {
    data: ReadingResult;
}

const ProgressBar = ({ label, percentage, colorClass }: { label: string, percentage: number, colorClass: string }) => (
    <div className="mb-2">
        <div className="flex justify-between text-xs text-gray-400 mb-1">
            <span>{label}</span>
            <span>{percentage.toFixed(1)}%</span>
        </div>
        <div className="w-full bg-gray-800 rounded-full h-1.5">
            <div className={`h-1.5 rounded-full ${colorClass}`} style={{ width: `${percentage}%` }}></div>
        </div>
    </div>
);

export const ReadingProfile: React.FC<ReadingProfileProps> = ({ data }) => {
    if (!data || !data.topography) return <div className="p-4 text-gray-500">No reading data available.</div>;

    // Calculate dynamic scaling for the K-Line chart
    const maxStrain = Math.max(...data.topography.klines.map(k => k.high), 10); // Floor of 10 for visual scale

    return (
        <div className="flex flex-col gap-4">
            {/* 1. TOPOGRAPHY (The K-Line Chart) */}
            <Card className="border-gray-800 p-4 bg-gray-900/30">
                <div className="mb-4">
                    <h3 className="text-lg font-semibold text-blue-400">Cognitive Strain Topography</h3>
                    <p className="text-xs text-gray-500">Working memory overload and pattern residue (5s windows)</p>
                </div>
                
                {/* Pure Tailwind K-Line Rendering */}
                <div className="relative h-48 w-full border-b border-gray-800 flex items-end overflow-x-auto gap-1 pb-1 custom-scrollbar">
                    {data.topography.klines.map((kline, idx) => {
                        // Normalize values between 0 and 100% based on maxStrain
                        const heightPct = (kline.high - kline.low) / maxStrain * 100;
                        const bottomPct = kline.low / maxStrain * 100;
                        
                        const isBullish = kline.close >= kline.open; // Strain went UP (Red for danger in our context)
                        const candleColor = isBullish ? "bg-red-500" : "bg-emerald-500";
                        
                        const bodyTop = Math.max(kline.open, kline.close);
                        const bodyBottom = Math.min(kline.open, kline.close);
                        const bodyHeightPct = Math.max((bodyTop - bodyBottom) / maxStrain * 100, 1); // Min 1% height
                        const bodyOffsetPct = bodyBottom / maxStrain * 100;

                        return (
                            <div key={idx} className="relative w-3 shrink-0 flex flex-col justify-end group" style={{ height: '100%' }}>
                                {/* Tooltip on hover */}
                                <div className="opacity-0 group-hover:opacity-100 absolute -top-10 left-1/2 -translate-x-1/2 bg-gray-950 border border-gray-800 text-[10px] p-1 rounded z-10 whitespace-nowrap pointer-events-none">
                                    Peak: {kline.high.toFixed(1)} | Vol: {kline.volume}
                                </div>
                                {/* Upper/Lower Wick */}
                                <div className="absolute w-px bg-gray-600 left-1/2 -translate-x-1/2" 
                                     style={{ bottom: `${bottomPct}%`, height: `${heightPct}%` }}></div>
                                {/* Candle Body */}
                                <div className={`absolute w-full rounded-sm ${candleColor}`} 
                                     style={{ bottom: `${bodyOffsetPct}%`, height: `${bodyHeightPct}%` }}></div>
                            </div>
                        );
                    })}
                </div>
                <div className="mt-2 text-[10px] text-gray-500 flex justify-between">
                    <span>Peak Load: {data.summary.peak_strain.toFixed(2)}</span>
                    <span>Time &rarr;</span>
                </div>
            </Card>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                {/* 2. DENSITY (Visual Clutter) */}
                <Card className="border-gray-800 p-4 bg-gray-900/30">
                    <h3 className="text-sm font-semibold text-purple-400 mb-1">Visual Clutter</h3>
                    <p className="text-[10px] text-gray-500 mb-4">Screen object density (AR window: {data.summary.ar_preempt_ms.toFixed(0)}ms)</p>
                    <ProgressBar label="Isolated (1-2 notes)" percentage={data.density.isolated_pct} colorClass="bg-emerald-500" />
                    <ProgressBar label="Chunking (3-5 notes)" percentage={data.density.chunking_pct} colorClass="bg-blue-500" />
                    <ProgressBar label="Clutter (6-8 notes)" percentage={data.density.clutter_pct} colorClass="bg-yellow-500" />
                    <ProgressBar label="Overload (9+ notes)" percentage={data.density.overload_pct} colorClass="bg-red-500" />
                </Card>

                {/* 3. TRAJECTORY (Geometric Chaos) */}
                <Card className="border-gray-800 p-4 bg-gray-900/30">
                    <h3 className="text-sm font-semibold text-orange-400 mb-1">Trajectory Chaos</h3>
                    <p className="text-[10px] text-gray-500 mb-4">Predictability of visual paths</p>
                    <ProgressBar label="Linear / Predictable" percentage={data.trajectory.linear_pct} colorClass="bg-emerald-500" />
                    <ProgressBar label="Mild Shifts" percentage={data.trajectory.mild_shifts_pct} colorClass="bg-blue-500" />
                    <ProgressBar label="Sharp Kinks" percentage={data.trajectory.sharp_kinks_pct} colorClass="bg-orange-500" />
                    <ProgressBar label="Overlapping / Spaghetti" percentage={data.trajectory.spaghetti_pct} colorClass="bg-red-500" />
                </Card>

                {/* 4. TRAPS & OUTLIERS */}
                <Card className="border-gray-800 p-4 bg-gray-900/30 flex flex-col justify-center items-center text-center">
                    <h3 className="text-sm font-semibold text-rose-400 mb-2">Relational Deception</h3>
                    <p className="text-xs text-gray-400 mb-4">Moments where physical spacing misrepresents rhythmic timing.</p>
                    
                    <div className="bg-gray-950 border border-gray-800 p-4 rounded-lg w-full">
                        <span className="block text-3xl font-bold text-rose-500 mb-1">
                            {data.traps.total_deceleration_traps}
                        </span>
                        <span className="text-[10px] text-gray-500 uppercase tracking-wider">
                            Deceleration Traps
                        </span>
                    </div>
                </Card>
            </div>
        </div>
    );
};