export function getSpacingTag(spacing: number, d: number) {
    if (spacing === 0) return "N/A";
    if (spacing < 2.0 * d) return "Narrow";
    if (spacing < 3.5 * d) return "Moderate";
    if (spacing < 5.0 * d) return "Wide";
    return "Cross-Screen (Extreme)";
}

export function getStreamSpacingTag(spacing: number, d: number) {
    if (spacing === 0) return "N/A";
    if (spacing < 0.5 * d) return "Stacked";
    if (spacing < 1.0 * d) return "Overlapping";
    if (spacing < 2.0 * d) return "Spaced";
    return "Extreme (Jump-Stream)";
}

export function getSliderTag(ratio: number) {
    if (ratio < 0.30) return "Mechanical Tech";
    if (ratio < 0.60) return "Technical";
    return "Slider Tech";
}