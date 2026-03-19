import React, { useState } from 'react';
import { Card } from "@/components/ui/card";
import { Clock } from "lucide-react";

const ProgressBar = ({ label, percentage, colorClass }: { label: string, percentage: number, colorClass: string }) => (
    <div className="flex items-center gap-4 mb-3">
        <span className="text-[11px] text-gray-400 w-32 shrink-0">{label}</span>
        <div className="flex-grow bg-gray-800 rounded-full h-2">
            <div className={`h-2 rounded-full ${colorClass} transition-all duration-700`} style={{ width: `${percentage}%` }}></div>
        </div>
        <span className="text-[11px] font-mono text-gray-300 w-10 text-right">{percentage.toFixed(1)}%</span>
    </div>
);

const formatTime = (ms: number) => {
    const totalSecs = Math.floor(ms / 1000);
    const mins = Math.floor(totalSecs / 60);
    const secs = totalSecs % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
};

export const ReadingProfile: React.FC<{ data: any }> = ({ data }) => {
    const [tooltip, setTooltip] = useState<{ x: number, y: number, k: any } | null>(null);

    if (!data || !data.topography) return <div className="p-4 text-gray-500 text-center">No reading data.</div>;

    const klines = data.topography.klines;
    const maxStrain = Math.max(...klines.map((k: any) => k.high), 10);
    const yMarkers = [maxStrain, maxStrain * 0.66, maxStrain * 0.33];

    const handleMouseMove = (e: React.MouseEvent, kline: any) => {
        const xOffset = e.clientX > window.innerWidth - 200 ? -160 : 20;
        const yOffset = e.clientY < 200 ? 20 : -140;

        setTooltip({
            x: e.clientX + xOffset,
            y: e.clientY + yOffset,
            k: kline
        });
    };

    return (
        <div className="flex flex-col gap-6 relative">
            {/* SMART DYNAMIC TOOLTIP */}
            {tooltip && (
                <div 
                    className="fixed z-[9999] bg-gray-950 border border-gray-700 p-3 rounded shadow-2xl font-mono min-w-[150px] pointer-events-none backdrop-blur-md ring-1 ring-white/10"
                    style={{ 
                        left: `${tooltip.x}px`, 
                        top: `${tooltip.y}px`,
                    }}
                >
                    <div className="text-blue-400 border-b border-gray-800 pb-1.5 mb-2 text-center font-bold text-xs">
                        {formatTime(tooltip.k.window_start)}
                    </div>
                    <div className="space-y-1.5 text-[11px]">
                        <div className="flex justify-between gap-4">
                            <span className="text-gray-400">Open:</span> 
                            <span className="text-gray-200">{tooltip.k.open.toFixed(2)}</span>
                        </div>
                        <div className="flex justify-between gap-4">
                            <span className="text-gray-400">High:</span> 
                            <span className="text-red-400 font-bold">{tooltip.k.high.toFixed(2)}</span>
                        </div>
                        <div className="flex justify-between gap-4">
                            <span className="text-gray-400">Low:</span> 
                            <span className="text-emerald-400 font-bold">{tooltip.k.low.toFixed(2)}</span>
                        </div>
                        <div className="flex justify-between gap-4">
                            <span className="text-gray-400">Close:</span> 
                            <span className="text-gray-200">{tooltip.k.close.toFixed(2)}</span>
                        </div>
                        <div className="text-gray-400 mt-2 pt-1.5 border-t border-gray-800 text-center text-[12px]">
                            Objects: {tooltip.k.volume}
                        </div>
                    </div>
                </div>
            )}

            {/* ROW 1: K-LINE CHART */}
            <Card className="border-gray-800 p-6 bg-gray-900/30">
                <div className="mb-6">
                    <h3 className="text-lg font-semibold text-blue-400">Cognitive Strain Topography</h3>
                    {/* RESTORED PEAK VALUE HERE */}
                    <p className="text-xs text-gray-400 font-mono italic">
                        Peak Reading Strain (95th): <span className="text-red-400 font-bold">{(data.summary.peak_strain || 0).toFixed(2)}</span>
                    </p>
                </div>
                
                <div className="relative w-full flex flex-col">
                    <div className="flex h-56">
                        {/* Y-Axis */}
                        <div className="flex flex-col justify-between text-[10px] text-gray-600 pr-3 border-r border-gray-800/50 h-48 font-mono">
                            {yMarkers.map(m => <span key={m}>{m.toFixed(1)}</span>)}
                            <span>0.0</span>
                        </div>

                        {/* Chart Area */}
                        <div className="relative flex-grow h-48 border-b border-gray-800 flex items-end gap-[1px] px-2 overflow-x-auto custom-scrollbar pb-1">
                            {klines.map((k: any, idx: number) => {
                                const highH = (k.high / maxStrain) * 100;
                                const lowH = (k.low / maxStrain) * 100;
                                const bTop = Math.max(k.open, k.close);
                                const bBot = Math.min(k.open, k.close);
                                const bH = Math.max(((bTop - bBot) / maxStrain) * 100, 2);
                                const bOff = (bBot / maxStrain) * 100;
                                const isUp = k.close >= k.open;

                                return (
                                    <div 
                                        key={idx} 
                                        className="relative w-[5px] shrink-0 h-48 cursor-crosshair group"
                                        onMouseMove={(e) => handleMouseMove(e, k)}
                                        onMouseLeave={() => setTooltip(null)}
                                    >
                                        {/* Wick */}
                                        <div className="absolute w-[1px] bg-gray-700 left-1/2 -translate-x-1/2" style={{ bottom: `${lowH}%`, height: `${highH - lowH}%` }}></div>
                                        {/* Candle Body */}
                                        <div className={`absolute w-full rounded-t-[1px] ${isUp ? 'bg-red-500' : 'bg-emerald-500'} group-hover:brightness-125`} 
                                             style={{ bottom: `${bOff}%`, height: `${bH}%` }}></div>

                                        {/* X-AXIS TIME MARKERS */}
                                        {idx % 12 === 0 && (
                                            <div className="absolute top-full pt-3 left-0 text-[10px] text-gray-600 whitespace-nowrap font-mono">
                                                {formatTime(k.window_start)}
                                            </div>
                                        )}
                                    </div>
                                );
                            })}
                        </div>
                    </div>
                    <div className="text-center text-[10px] text-gray-600 uppercase tracking-widest">Drain Time (mm:ss)</div>
                </div>
            </Card>

            {/* ROW 2 & 3 remain the same as previous version... */}
            <div className="space-y-4">
                <Card className="border-gray-800 p-6 bg-gray-900/30">
                    <h3 className="text-sm font-semibold text-purple-400 mb-4 uppercase tracking-widest">I. Visual Clutter</h3>
                    <ProgressBar label="Isolated (1-2)" percentage={data.density.isolated_pct} colorClass="bg-emerald-500" />
                    <ProgressBar label="Chunking (3-5)" percentage={data.density.chunking_pct} colorClass="bg-blue-500" />
                    <ProgressBar label="Clutter (6-8)" percentage={data.density.clutter_pct} colorClass="bg-yellow-500" />
                    <ProgressBar label="Overload (9+)" percentage={data.density.overload_pct} colorClass="bg-red-500" />
                </Card>

                <Card className="border-gray-800 p-6 bg-gray-900/30">
                    <h3 className="text-sm font-semibold text-orange-400 mb-4 uppercase tracking-widest">II. Trajectory Chaos</h3>
                    <ProgressBar label="Predictable Flow" percentage={data.trajectory.linear_pct} colorClass="bg-emerald-500" />
                    <ProgressBar label="Mild Shifts" percentage={data.trajectory.mild_shifts_pct} colorClass="bg-blue-500" />
                    <ProgressBar label="Sharp Kinks" percentage={data.trajectory.sharp_kinks_pct} colorClass="bg-orange-500" />
                    <ProgressBar label="Spaghetti / Overlap" percentage={data.trajectory.spaghetti_pct} colorClass="bg-red-500" />
                </Card>
            </div>

            <Card className="border-gray-800 p-6 bg-gray-900/30">
                <div className="flex justify-between items-start mb-6 text-xs text-gray-500 font-mono italic">
                    <div className="text-rose-400 font-bold uppercase not-italic tracking-tighter">III. Relational Deception (Traps)</div>
                    <div>Trap Index: {(data.traps.trap_index || 0).toFixed(1)} / 1k</div>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div className="bg-black/40 p-4 rounded border border-gray-800/50">
                        <div className="text-[10px] text-gray-500 mb-3 border-b border-gray-800 pb-1 flex justify-between uppercase">
                            <span>Notable Spikes</span>
                            <span>Magnitude</span>
                        </div>
                        <div className="space-y-2">
                            {(data.traps.notable_traps || []).slice(0, 5).map((trap: any, i: number) => (
                                <div key={i} className="flex justify-between items-center text-xs font-mono">
                                    <div className="flex items-center gap-2 text-gray-400">
                                        <Clock className="w-3 h-3 text-gray-600" />
                                        {formatTime(trap.time)}
                                    </div>
                                    <div className={`px-2 py-0.5 rounded text-[10px] font-bold ${trap.magnitude > 2.5 ? 'bg-red-900/40 text-red-400' : 'bg-gray-800 text-gray-300'}`}>
                                        {trap.magnitude.toFixed(2)}x
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>
                    <div className="flex flex-col justify-center items-center p-4 border border-dashed border-gray-800 rounded bg-gray-950/20">
                        <div className="text-3xl font-black text-rose-500 mb-1">{data.traps.count}</div>
                        <p className="text-[10px] text-gray-500 text-center leading-relaxed uppercase font-bold tracking-tighter">
                            Total Reading Traps Detected
                        </p>
                    </div>
                </div>
            </Card>
        </div>
    );
};