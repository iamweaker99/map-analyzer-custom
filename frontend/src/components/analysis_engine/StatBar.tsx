interface StatBarProps {
    label: string;
    /** Raw count (shown alongside percentage when provided) */
    value?: number;
    /** Total for the category group (value/total*100 = percentage) */
    total?: number;
    /** Direct percentage (0-100). Takes priority over value/total if provided */
    percentage?: number;
    /** Tailwind bg color class, e.g. "bg-pink-500" */
    colorClass?: string;
}

export function StatBar({
    label,
    value,
    total = 0,
    percentage: directPct,
    colorClass = "bg-blue-500",
}: StatBarProps) {
    const safeValue = value ?? 0;
    const safeTotal = total || 0;
    const percentage =
        directPct !== undefined
            ? directPct
            : safeTotal > 0
              ? (safeValue / safeTotal) * 100
              : 0;

    return (
        <div className="mb-2">
            <div className="flex justify-between text-xs mb-0.5">
                <span className="text-gray-300">{label}</span>
                <span className="font-mono text-gray-400">
                    {value !== undefined ? (
                        <>
                            {safeValue}{" "}
                            <span className="text-[10px]">
                                ({percentage.toFixed(1)}%)
                            </span>
                        </>
                    ) : (
                        <span>{percentage.toFixed(1)}%</span>
                    )}
                </span>
            </div>
            <div className="h-1 w-full bg-gray-800 rounded-full overflow-hidden">
                <div
                    className={`h-full ${colorClass}`}
                    style={{ width: `${percentage}%` }}
                />
            </div>
        </div>
    );
}
