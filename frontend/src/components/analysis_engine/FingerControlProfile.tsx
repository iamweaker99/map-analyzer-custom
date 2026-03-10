import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { FingerControlAnalysis as FingerControlAnalysisType } from "./types";

interface Props {
  analysis: FingerControlAnalysisType; // Updated the type name here
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
  return (
    <Card className="w-full border-none shadow-none bg-transparent">
      <CardHeader className="px-0">
        <CardTitle className="text-lg flex justify-between items-center">
          Finger Control / Tapping Profile
          <div className={`px-2 py-1 rounded text-xs font-bold ${analysis.complexityScore > 7 ? 'bg-red-500 text-white' : 'bg-secondary text-secondary-foreground'}`}>
            Complexity: {analysis.complexityScore.toFixed(2)}
          </div>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6 px-0">
        <div className="grid grid-cols-2 gap-4">
          <div className="p-3 bg-secondary/30 rounded-lg">
            <p className="text-sm text-muted-foreground">Action Switching</p>
            <p className="text-2xl font-bold">{analysis.morphologyIndex.toFixed(1)}</p>
            <p className="text-xs text-muted-foreground">Circle ↔ Slider transitions</p>
          </div>
          <div className="p-3 bg-secondary/30 rounded-lg">
            <p className="text-sm text-muted-foreground">Even Burst Ratio</p>
            <p className="text-2xl font-bold">{(analysis.evenBurstRatio * 100).toFixed(0)}%</p>
            <p className="text-xs text-muted-foreground">Percentage of 2, 4, 6... patterns</p>
          </div>
        </div>

        <div className="space-y-2">
          <p className="text-sm font-medium">Rhythmic Signature (Snap Distribution)</p>
          <div className="h-8 w-full flex rounded-full overflow-hidden bg-secondary">
            {analysis.snapDistribution.map((snap) => (
                <div
                    key={snap.label}
                    className={`${snapColors[snap.label] || 'bg-gray-400'} h-full transition-all hover:opacity-80 cursor-help`}
                    style={{ width: `${snap.percentage * 100}%` }}
                    title={`${snap.label} Snap: ${(snap.percentage * 100).toFixed(1)}% of the map`}
                />
            ))}
          </div>
          <div className="flex flex-wrap gap-2 mt-2">
            {analysis.snapDistribution.map((snap) => (
              <div key={snap.label} className="flex items-center gap-1 text-xs">
                <div className={`w-2 h-2 rounded-full ${snapColors[snap.label] || 'bg-gray-400'}`} />
                <span>{snap.label}</span>
              </div>
            ))}
          </div>
        </div>
      </CardContent>
    </Card>
  );
};