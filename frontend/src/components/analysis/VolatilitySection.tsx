import React from 'react';

interface VolatilitySectionProps {
  title: string;
  icon?: React.ReactNode; // Optional icon like the music note in your screenshot
  children: React.ReactNode;
}

export const VolatilitySection: React.FC<VolatilitySectionProps> = ({ title, icon, children }) => {
  return (
    <div className="mb-8 border border-gray-800 bg-[#121212] rounded-lg p-4 md:p-6 shadow-md">
      {/* Section Header */}
      <div className="flex items-center mb-5 border-b border-gray-800 pb-2">
        {icon && <span className="mr-2 text-xl text-gray-300">{icon}</span>}
        <h3 className="text-lg font-semibold text-white tracking-wide">
          {title}
        </h3>
      </div>

      {/* Grid for the StatBars (Matches the Spacing Profile layout) */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-2">
        {children}
      </div>
    </div>
  );
};