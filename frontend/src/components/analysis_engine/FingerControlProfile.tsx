import React from 'react';
import { Card, CardContent } from "@/components/ui/card";
import { FingerControlAnalysis as FingerControlInterface, TransitionOccurrence } from "./types";
import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer, CartesianGrid, Legend } from 'recharts';

interface Props {
  analysis: FingerControlInterface;
}

const snapColors: Record<string, string> = {
  "1/1": "bg-slate-500",
  "1/2": "bg-blue-500",
  "1/4": "bg-red-500",
  "1/3": "bg-yellow-500",
  "1/6": "bg-purple-500",
  "1/8": "bg-pink-500",
  "1/12": "bg-orange-500",
};

export const FingerControlProfile: React.FC<Props> = ({ analysis }) => {
  const burstSizes = [2, 3, 4, 5, 6];

  // --- ADD THIS LINE HERE ---
  // This unique key will force all charts to destroy and recreate when the map changes
  const chartResetKey = analysis.beatmapMd5 || "default-key";

  // Add this block back! It formats the X-Axis labels on the graph
  const formatTime = (ms: number) => {
    const mins = Math.floor(ms / 60000);
    const secs = Math.floor((ms % 60000) / 1000);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  // Helper to render the categorized tables
  const renderTransitionTable = (title: string, data: TransitionOccurrence[]) => (
    <div className="space-y-2">
      <h4 className="text-[10px] font-bold text-muted-foreground uppercase px-1">{title}</h4>
      <div className="max-h-32 overflow-y-auto rounded border border-muted/10 bg-black/10">
        <table className="w-full text-[10px] text-left">
          <tbody className="divide-y divide-muted/10">
            {data && data.length > 0 ? data.map((t, i) => (
              <tr key={i} className="hover:bg-white/5 transition-colors">
                <td className="p-2 text-muted-foreground">{t.label}</td>
                <td className="p-2 text-right font-mono font-bold text-purple-400">
                  {t.percentage.toFixed(1)}%
                </td>
              </tr>
            )) : (
              <tr><td className="p-2 italic text-muted-foreground">No transitions recorded</td></tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );

  return (
    <Card className="w-full border-none bg-transparent shadow-none">
      <CardContent className="space-y-8 px-0">
        
        {/* 1. Burst Length Profile */}
        <div className="space-y-2">
          <h3 className="text-xs font-bold uppercase tracking-wider text-muted-foreground">Numbered Burst Profile</h3>
          <div className="grid grid-cols-5 gap-2">
            {burstSizes.map(size => (
              <div key={size} className="bg-secondary/30 p-2 rounded text-center border border-muted/10">
                <div className="text-[10px] text-muted-foreground font-medium">{size}n</div>
                <div className="text-sm font-bold text-purple-400">
                  {analysis.burstHistogram[size] || 0}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* 2. Snap Distribution */}
        <div className="space-y-2">
          <h3 className="text-xs font-bold uppercase tracking-wider text-muted-foreground">Rhythmic Signature</h3>
          <div className="h-8 w-full flex rounded-full overflow-hidden bg-secondary">
            {analysis.snapDistribution.map((snap) => (
              <div
                key={snap.label}
                className={`${snapColors[snap.label] || 'bg-gray-400'} h-full transition-all hover:opacity-80 cursor-help`}
                style={{ width: `${snap.percentage * 100}%` }}
                title={`${snap.label} Snap: ${(snap.percentage * 100).toFixed(1)}%`}
              />
            ))}
          </div>
          <div className="flex flex-wrap gap-2 mt-2">
            {analysis.snapDistribution.map((snap) => (
              <div key={snap.label} className="flex items-center gap-1 text-xs">
                <div className={`w-2 h-2 rounded-full ${snapColors[snap.label] || 'bg-gray-400'}`} />
                <span className="text-muted-foreground">{snap.label}</span>
              </div>
            ))}
          </div>
        </div>

        {/* 3. Off-Grid Section Analysis */}
        <div className="space-y-3">
          <h3 className="text-xs font-bold uppercase tracking-wider text-muted-foreground">
            Rhythmic Instability by Map Section
          </h3>
          {/* SCROLLABLE WRAPPER */}
          <div className="max-h-48 overflow-y-auto rounded border border-muted/20 bg-black/10 custom-scrollbar">
            <table className="w-full text-[10px] text-left border-collapse">
              <thead className="bg-secondary text-muted-foreground sticky top-0 z-20 shadow-sm">
                <tr>
                  <th className="p-2 border-r border-muted/10">Map Section</th>
                  <th className="p-2">Unstable Notes</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-muted/10">
                {analysis.offGridBuckets.map((count, i) => (
                  <tr key={i} className={count > 0 ? "bg-yellow-500/5 hover:bg-yellow-500/10 transition-colors" : "hover:bg-white/5 transition-colors"}>
                    <td className="p-2 border-r border-muted/10 font-mono text-muted-foreground">
                      Section {i + 1} ({i * 10}% - {(i + 1) * 10}%)
                    </td>
                    <td className="p-2 flex items-center gap-2">
                      <span className={count > 0 ? "font-bold text-yellow-500" : "text-muted-foreground"}>
                        {count}
                      </span>
                      {count > 0 && (
                        <div 
                          className="h-1 bg-yellow-500 rounded-full" 
                          style={{ width: `${Math.min(count * 2, 100)}px` }} 
                        />
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        {/* 4. Morphology & Transitions */}
        <div className="space-y-6">
          <div className="space-y-3">
            <h3 className="text-xs font-bold uppercase tracking-wider text-muted-foreground border-b border-muted/20 pb-1">
                Transition between Numbered Bursts
            </h3>
            <div className="grid grid-cols-4 gap-2">
                <div className="bg-secondary/20 p-2 rounded text-center border border-muted/10">
                    <div className="text-[9px] text-muted-foreground font-medium uppercase">Odd - Odd</div>
                    <div className="text-sm font-bold text-green-400">{analysis.transitionMatrix.categoryCounts.oddToOdd}</div>
                </div>
                <div className="bg-secondary/20 p-2 rounded text-center border border-muted/10">
                    <div className="text-[9px] text-muted-foreground font-medium uppercase">Even - Even</div>
                    <div className="text-sm font-bold text-blue-400">{analysis.transitionMatrix.categoryCounts.evenToEven}</div>
                </div>
                <div className="bg-secondary/20 p-2 rounded text-center border border-muted/10">
                    <div className="text-[9px] text-muted-foreground font-medium uppercase">Odd - Even</div>
                    <div className="text-sm font-bold text-red-400">{analysis.transitionMatrix.categoryCounts.oddToEven}</div>
                </div>
                <div className="bg-orange-500/10 p-2 rounded text-center border border-orange-500/20">
                    <div className="text-[9px] text-orange-500 font-medium uppercase">Rhythmic Resets</div>
                    <div className="text-sm font-bold text-orange-400">{analysis.transitionMatrix.categoryCounts.rhythmicResets}</div>
                </div>
            </div>
          </div>

          <div className="space-y-4">
            <h3 className="text-xs font-bold uppercase tracking-wider text-muted-foreground">BPM / Snap Switching Profile</h3>
            {renderTransitionTable("Top 10 Snap-to-Snap Transitions", analysis.transitionMatrix.bpmTransitions)}
            
            <div className="space-y-3 pt-2">
                <h4 className="text-[11px] font-bold text-muted-foreground border-l-2 border-yellow-500 pl-2">Transition Based on Δ of BPM/Snap</h4>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {renderTransitionTable("Ordinary Switch", analysis.transitionMatrix.bpmOrdinary || [])}
                    {renderTransitionTable("Minor Switch", analysis.transitionMatrix.bpmMinor || [])}
                    {renderTransitionTable("Major Switch", analysis.transitionMatrix.bpmMajor || [])}
                </div>
            </div>
          </div>

          <div className="space-y-4 pt-4 border-t border-muted/20">
            <h3 className="text-xs font-bold uppercase tracking-wider text-muted-foreground">Pattern Morphology</h3>
            {renderTransitionTable("Top 10 Pattern Transitions", analysis.transitionMatrix.topTransitions)}
            
            <div className="space-y-3 pt-2">
                <h4 className="text-[11px] font-bold text-muted-foreground border-l-2 border-purple-500 pl-2">Transition Based on Δ of Notes</h4>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {renderTransitionTable("Consistency (Δ0 / Same Snap)", analysis.transitionMatrix.deltaGroups[0] || [])}
                    {renderTransitionTable("Rhythmic Resets (Δ0 / Speed Shift)", analysis.transitionMatrix.rhythmicResets || [])}
                    {renderTransitionTable("Compound Friction (Δ1)", analysis.transitionMatrix.deltaGroups[1] || [])}
                    {renderTransitionTable("Compound Friction (Δ2)", analysis.transitionMatrix.deltaGroups[2] || [])}
                    {renderTransitionTable("Compound Friction (Δ3)", analysis.transitionMatrix.deltaGroups[3] || [])}
                </div>
            </div>
          </div>
        </div>

        {/* 5. Triple Timeline SMA Curves */}
        <div className="space-y-6 pt-4 border-t border-muted/20" key={chartResetKey}>
        {/* Adding the key here forces the entire graph section to re-render from scratch on map change */}
        <h3 className="text-xs font-bold uppercase tracking-wider text-muted-foreground">Technical Density Curves (SMA)</h3>
          
          {/* Graph 1: Overall */}
          <div className="space-y-2">
            <h4 className="text-[11px] font-bold text-muted-foreground">Overall Switching</h4>
            <div className="h-48 w-full bg-secondary/10 rounded-lg p-2 border border-muted/10">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={analysis.timeline} syncId="fingerControl">
                  <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.05)" vertical={false} />
                  <XAxis dataKey="time" type="number" domain={['dataMin', 'dataMax']} tickFormatter={formatTime} stroke="rgba(255,255,255,0.2)" tick={{ fontSize: 10 }} />
                  <YAxis stroke="rgba(255,255,255,0.2)" tick={{ fontSize: 10 }} width={30} />
                  <Tooltip labelFormatter={(l) => formatTime(l as number)} contentStyle={{ backgroundColor: '#0f172a', fontSize: '12px' }} />
                  <Legend iconType="circle" wrapperStyle={{ fontSize: '11px' }} />
                  <Line type="monotone" name="Pattern Switches" dataKey="patternSma" stroke="#a855f7" strokeWidth={2} dot={false} isAnimationActive={false} connectNulls={true} animateNewValues={false}/>
                  <Line type="monotone" name="BPM/Snap Switches" dataKey="bpmSma" stroke="#eab308" strokeWidth={2} dot={false} isAnimationActive={false} connectNulls={true} animateNewValues={false}/>
                </LineChart>
              </ResponsiveContainer>
            </div>
          </div>

          {/* Graph 2: BPM Delta */}
          <div className="space-y-2">
            <h4 className="text-[11px] font-bold text-muted-foreground">Transition based on Δ of BPM/Snap</h4>
            <div className="h-48 w-full bg-secondary/10 rounded-lg p-2 border border-muted/10">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={analysis.timeline} syncId="fingerControl">
                  <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.05)" vertical={false} />
                  <XAxis dataKey="time" type="number" domain={['dataMin', 'dataMax']} tickFormatter={formatTime} stroke="rgba(255,255,255,0.2)" tick={{ fontSize: 10 }} />
                  <YAxis stroke="rgba(255,255,255,0.2)" tick={{ fontSize: 10 }} width={30} />
                  <Tooltip labelFormatter={(l) => formatTime(l as number)} contentStyle={{ backgroundColor: '#0f172a', fontSize: '12px' }} />
                  <Legend iconType="circle" wrapperStyle={{ fontSize: '11px' }} />
                  <Line type="monotone" name="Ordinary" dataKey="bpmOrdinarySma" stroke="#22c55e" strokeWidth={2} dot={false} isAnimationActive={false} connectNulls={true} animateNewValues={false}/>
                  <Line type="monotone" name="Minor" dataKey="bpmMinorSma" stroke="#eab308" strokeWidth={2} dot={false} isAnimationActive={false} connectNulls={true} animateNewValues={false}/>
                  <Line type="monotone" name="Major" dataKey="bpmMajorSma" stroke="#ef4444" strokeWidth={2} dot={false} isAnimationActive={false} connectNulls={true} animateNewValues={false}/>
                </LineChart>
              </ResponsiveContainer>
            </div>
          </div>

          {/* Graph 3: Notes Delta */}
          <div className="space-y-2">
            <h4 className="text-[11px] font-bold text-muted-foreground">Transition based on Δ of Notes</h4>
            <div className="h-48 w-full bg-secondary/10 rounded-lg p-2 border border-muted/10">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={analysis.timeline} syncId="fingerControl">
                  <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.05)" vertical={false} />
                  <XAxis dataKey="time" type="number" domain={['dataMin', 'dataMax']} tickFormatter={formatTime} stroke="rgba(255,255,255,0.2)" tick={{ fontSize: 10 }} />
                  <YAxis stroke="rgba(255,255,255,0.2)" tick={{ fontSize: 10 }} width={30} />
                  <Tooltip labelFormatter={(l) => formatTime(l as number)} contentStyle={{ backgroundColor: '#0f172a', fontSize: '12px' }} />
                  <Legend iconType="circle" wrapperStyle={{ fontSize: '11px' }} />
                  <Line type="monotone" name="Consistency (Δ0)" dataKey="noteDelta0ConsSma" stroke="#94a3b8" strokeWidth={1.5} dot={false} isAnimationActive={false} connectNulls={true} animateNewValues={false}/>
                  <Line type="monotone" name="Rhythmic Reset (Δ0)" dataKey="noteDelta0ResetSma" stroke="#f97316" strokeWidth={2} dot={false} isAnimationActive={false} connectNulls={true} animateNewValues={false}/>
                  <Line type="monotone" name="Compound Friction (Δ1)" dataKey="noteDelta1Sma" stroke="#3b82f6" strokeWidth={2} dot={false} isAnimationActive={false} connectNulls={true} animateNewValues={false}/>
                  <Line type="monotone" name="Compound Friction (Δ2)" dataKey="noteDelta2Sma" stroke="#8b5cf6" strokeWidth={2} dot={false} isAnimationActive={false} connectNulls={true} animateNewValues={false}/>
                  <Line type="monotone" name="Compound Friction (Δ3)" dataKey="noteDelta3Sma" stroke="#ec4899" strokeWidth={2} dot={false} isAnimationActive={false} connectNulls={true} animateNewValues={false}/>
                </LineChart>
              </ResponsiveContainer>
            </div>
          </div>
        </div>

      </CardContent>
    </Card>
  );
};