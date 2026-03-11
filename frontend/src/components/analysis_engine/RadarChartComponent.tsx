import React from 'react';
import {
  Radar, RadarChart, PolarGrid, PolarAngleAxis, PolarRadiusAxis, ResponsiveContainer
} from 'recharts';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { JumpAnalysis, StreamAnalysis, FingerControlAnalysis } from "./types";

interface Props {
  jump?: JumpAnalysis;
  stream?: StreamAnalysis;
  fingerControl?: FingerControlAnalysis;
}

export const RadarChartComponent: React.FC<Props> = ({ jump, stream, fingerControl }) => {
  // Mapping the technical data to a 0-10 scale for the chart
  const data = [
    { name: 'Aim (Jumps)', value: (jump?.overall_confidence || 0) * 10 },
    { name: 'Speed (Streams)', value: (stream?.overall_confidence || 0) * 10 },
    { name: 'Complexity', value: fingerControl?.complexityScore || 0 },
    { name: 'Switching', value: fingerControl?.morphologyIndex || 0 },
    { name: 'Rhythm', value: (fingerControl?.overall_confidence || 0) * 10 },
  ];

  return (
    <Card className="w-full bg-secondary/10 border-none shadow-none">
      <CardHeader className="pb-2">
        <CardTitle className="text-sm font-medium text-center uppercase tracking-widest text-muted-foreground">
          Technical Profile
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="h-[250px] w-full">
          <ResponsiveContainer width="100%" height="100%">
            <RadarChart cx="50%" cy="50%" outerRadius="80%" data={data}>
              <PolarGrid stroke="#444" />
              <PolarAngleAxis dataKey="name" tick={{ fill: '#888', fontSize: 10 }} />
              <PolarRadiusAxis angle={30} domain={[0, 10]} tick={false} axisLine={false} />
              <Radar
                name="Map Profile"
                dataKey="value"
                stroke="#8b5cf6"
                fill="#8b5cf6"
                fillOpacity={0.5}
              />
            </RadarChart>
          </ResponsiveContainer>
        </div>
      </CardContent>
    </Card>
  );
};