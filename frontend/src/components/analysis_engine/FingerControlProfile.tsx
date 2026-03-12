import React from 'react';
import { Card, CardContent } from "@/components/ui/card";
import { 
  FingerControlAnalysis as FingerControlInterface, 
  TransitionOccurrence // Import the type we just added
} from "./types";

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
          <div className="rounded border border-muted/20 overflow-hidden">
            <table className="w-full text-[10px] text-left border-collapse">
              <thead className="bg-secondary text-muted-foreground">
                <tr>
                  <th className="p-2 border-r border-muted/10">Map Section</th>
                  <th className="p-2">Unstable Notes</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-muted/10">
                {analysis.offGridBuckets.map((count, i) => (
                  <tr key={i} className={count > 0 ? "bg-yellow-500/5" : ""}>
                    <td className="p-2 border-r border-muted/10 font-mono text-muted-foreground">
                      Section {i + 1} ({i * 10}% - {(i + 1) * 10}%)
                    </td>
                    <td className="p-2 flex items-center gap-2">
                      <span className={count > 0 ? "font-bold text-yellow-500" : "text-muted-foreground"}>
                        {count}
                      </span>
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
            <div className="grid grid-cols-3 gap-2">
                <div className="bg-secondary/20 p-2 rounded text-center border border-muted/10">
                    <div className="text-[9px] text-muted-foreground font-medium">Odd - Odd Bursts</div>
                    <div className="text-sm font-bold text-green-400">{analysis.transitionMatrix.categoryCounts.oddToOdd}</div>
                </div>
                <div className="bg-secondary/20 p-2 rounded text-center border border-muted/10">
                    <div className="text-[9px] text-muted-foreground font-medium">Even - Even Bursts</div>
                    <div className="text-sm font-bold text-blue-400">{analysis.transitionMatrix.categoryCounts.evenToEven}</div>
                </div>
                <div className="bg-secondary/20 p-2 rounded text-center border border-muted/10">
                    <div className="text-[9px] text-muted-foreground font-medium">Odd - Even Bursts</div>
                    <div className="text-sm font-bold text-red-400">{analysis.transitionMatrix.categoryCounts.oddToEven}</div>
                </div>
            </div>
          </div>

          {/* NEW: BPM / Snap Switching Profile */}
          <div className="space-y-4">
            <h3 className="text-xs font-bold uppercase tracking-wider text-muted-foreground">BPM / Snap Switching Profile</h3>
            {renderTransitionTable("Top 10 Snap-to-Snap Transitions", analysis.transitionMatrix.bpmTransitions)}
          </div>

          <div className="space-y-4">
            <h3 className="text-xs font-bold uppercase tracking-wider text-muted-foreground">Pattern Morphology</h3>
            {renderTransitionTable("Overall Top Transitions", analysis.transitionMatrix.topTransitions)}
            
            <div className="space-y-3 pt-2">
                <h4 className="text-[11px] font-bold text-muted-foreground border-l-2 border-purple-500 pl-2">Transition based on Δ</h4>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {renderTransitionTable("Transition with Δ0", analysis.transitionMatrix.deltaGroups[0] || [])}
                    {renderTransitionTable("Transition with Δ1", analysis.transitionMatrix.deltaGroups[1] || [])}
                    {renderTransitionTable("Transition with Δ2", analysis.transitionMatrix.deltaGroups[2] || [])}
                    {renderTransitionTable("Transition with Δ3", analysis.transitionMatrix.deltaGroups[3] || [])}
                </div>
            </div>
          </div>
        </div>

      </CardContent>
    </Card>
  );
};