import React from 'react';
import { Card, CardContent } from "@/components/ui/card";
import { FingerControlAnalysis as FingerControlInterface } from "./types";

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
  
  // Calculate max count for scaling the bars relative to each other
  // We extract values from the burstHistogram object and find the highest one
  const histogramValues = Object.values(analysis.burstHistogram || {});
  const maxCount = Math.max(...(histogramValues.length > 0 ? histogramValues : [0]), 1);
  const maxBucket = Math.max(...analysis.offGridBuckets, 1);

  // Helper to format time to mm:ss:ms
    const formatTime = (ms: number) => {
    const mins = Math.floor(ms / 60000);
    const secs = Math.floor((ms % 60000) / 1000);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <Card className="w-full border-none bg-transparent shadow-none">
      <CardContent className="space-y-6 px-0">
        
        {/* 1. Burst Length Profile (List Format) */}
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

        {/* Snap Distribution Bar */}
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

{/* 3. Off-Grid Section Analysis (10 Sections) */}
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
          <p className="text-[10px] text-muted-foreground italic px-1">
            * Total off-grid notes: {analysis.offGridDetails.length}
          </p>
        </div>

      </CardContent>
    </Card>
  );
};