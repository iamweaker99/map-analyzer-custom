import React from 'react';

interface StatBarProps {
  label: string;
  percentage: number; // 0 to 100
  count?: number; // Optional: If we ever want to show raw count later
  colorClass: string; // Tailwind bg color class (e.g., 'bg-green-400')
}

export const StatBar: React.FC<StatBarProps> = ({ 
  label, 
  percentage, 
  count, 
  colorClass 
}) => {
  // Format percentage to 1 decimal place (e.g., 24.0%)
  const formattedPercentage = percentage.toFixed(1);

  return (
    <div className="flex flex-col mb-4 w-full px-2">
      {/* Top Row: Label & Values */}
      <div className="flex justify-between items-end mb-1">
        <span className="text-gray-200 font-medium text-sm md:text-base leading-tight">
          {label}
        </span>
        <div className="flex items-baseline space-x-3 text-sm md:text-base">
          {count !== undefined && (
            <span className="text-gray-300">{count}</span>
          )}
          <span className="text-gray-400 font-mono text-xs md:text-sm">
            ({formattedPercentage}%)
          </span>
        </div>
      </div>
      
      {/* Bottom Row: Progress Bar */}
      <div className="w-full h-1.5 bg-gray-800 rounded-full overflow-hidden flex">
        <div 
          className={`h-full ${colorClass} rounded-full`} 
          style={{ width: `${Math.max(0, Math.min(100, percentage))}%` }} 
        />
      </div>
    </div>
  );
};