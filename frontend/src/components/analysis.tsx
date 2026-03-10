"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Alert, AlertDescription } from "@/components/ui/alert";
import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";
import { useToast } from "@/hooks/use-toast";

import { useState } from "react";
import { AlertTriangle, BarChart, Music } from "lucide-react";
import Image from "next/image";
import Link from "next/link";
import { parseURL } from "@/lib/osu";
import { ScrollArea } from "./ui/scroll-area";

export interface BeatmapDetailsResult {
    title: string;
    artist: string;
    creator: string;
    creator_id: number;
    version: string;
    set_id: number;
    statistics: {
        ar: number;
        od: number;
        hp: number;
        cs: number;
        bpm: number;
        star_rating: number;
        total_objects: number;
    };
}

export interface BeatmapAnalysisResult {
    analysis_type: "jump" | "stream" | "slider"; // Added slider
    analysis: JumpAnalysis | StreamAnalysis | SliderAnalysis; // Added SliderAnalysis
}

interface JumpAnalysis {
    overall_confidence: number;
    total_jump: number;
    circle_diameter: number;
    max_jump_length: number;
    short_jumps: number;
    medium_jumps: number;
    long_jumps: number;
    jump_density: number;
    bpm_consistency: number;
    avg_spacing: number;
    // Distance Profile Fields
    narrow_count: number;
    moderate_count: number;
    wide_count: number;
    extreme_count: number;
    narrow_dens: number;
    moderate_dens: number;
    wide_dens: number;
    extreme_dens: number;
}

interface StreamAnalysis {
    overall_confidence: number;
    total_stream_patterns: number;
    circle_diameter: number;
    // Spacing Profile
    s_stacked_count: number;
    s_overlapping_count: number;
    s_spaced_count: number;
    s_extreme_count: number;
    avg_stream_spacing: number;
    s_stack_dens: number;
    s_over_dens: number;
    s_space_dens: number;
    s_extr_dens: number;
    // Variance Profile
    v_steady_count: number;
    v_variable_count: number;
    v_dynamic_count: number;
    // Length Profile
    // Length Profile
    bursts: number;
    short_streams: number;
    medium_streams: number;
    long_streams: number;
    death_streams: number;
    max_stream_length: number;
    stream_density: number;
    bpm_consistency: number;
}

interface SliderAnalysis {
    overall_confidence: number;
    avg_velocity: number;
    slider_ratio: number;
    // Length Profile (Relative to Map)
    l_short_count: number; l_short_dens: number;
    l_med_count: number;   l_med_dens: number;
    l_long_count: number;  l_long_dens: number;
    l_ext_count: number;   l_ext_dens: number;
    // Buzz Profile (Relative to Sliders)
    b_buzz_count: number;   b_buzz_dens: number;
    b_static_count: number; b_static_dens: number;
    // Artistic Profile (Relative to Sliders)
    a_simple_count: number; a_simple_dens: number;
    a_curved_count: number; a_curved_dens: number;
    a_complex_count: number; a_complex_dens: number;
    a_artistic_count: number; a_artistic_dens: number;
}

type AnalysisProps = {
    getBeatmapDetails(beatmapId: number): Promise<BeatmapDetailsResult>;
    getBeatmapAnalysis<T extends "stream" | "jump" | "all">(
        beatmapId: number,
        analysisType: T,
    ): Promise<
        T extends "all" ? BeatmapAnalysisResult[] : BeatmapAnalysisResult
    >;
};

export default function Analysis({
    getBeatmapAnalysis,
    getBeatmapDetails,
}: AnalysisProps) {
    const [beatmapUrl, setBeatmapUrl] = useState("");
    const [beatmapSetId, setBeatmapSetId] = useState(0);
    const [beatmapId, setBeatmapId] = useState(0);
    const { toast } = useToast();

    const [analysisResult, setAnalysisResult] = useState<
        BeatmapAnalysisResult[] | null
    >(null);
    const [detailsResult, setDetailsResult] =
        useState<BeatmapDetailsResult | null>(null);

    async function handleSubmit(e: React.FormEvent) {
        e.preventDefault();

        const urlMatch = parseURL(beatmapUrl);

        let beatmapId: string | null = null;

        if (urlMatch && "id" in urlMatch) beatmapId = urlMatch.id;
        else if (urlMatch && "setId" in urlMatch)
            beatmapId = urlMatch.difficultyId;

        if (beatmapId === null) {
            return;
        }

        try {
            const mapDetails = await getBeatmapDetails(+beatmapId);
            const mapAnalysis = await getBeatmapAnalysis(+beatmapId, "all");

            mapAnalysis.sort(
                (a, b) =>
                    b.analysis.overall_confidence -
                    a.analysis.overall_confidence,
            );

            setBeatmapSetId(mapDetails.set_id);
            setBeatmapId(+beatmapId);
            setDetailsResult(mapDetails);
            setAnalysisResult(mapAnalysis);
        } catch (e) {
            console.error(e);
            toast({
                variant: "destructive",
                title: "Oops!",
                description:
                    "Looks like there was an issue while processing your beatmap.\nPlease make sure you input a valid beatmap link.",
            });
        }
    }

    return (
        <div>
            <form onSubmit={handleSubmit} className="mb-8">
                <div className="flex gap-2">
                    <Input
                        type="text"
                        value={beatmapUrl}
                        onChange={(e) => setBeatmapUrl(e.target.value)}
                        placeholder="Enter beatmap ID or URL"
                        className="flex-grow"
                    />
                    <Button type="submit">Analyze</Button>
                </div>
            </form>

            {analysisResult && detailsResult && (
                <>
                    <div className="mb-2">
                        <Card className="mb-6">
                            <CardContent className="p-0">
                                <div className="relative aspect-[16/10] sm:aspect-[16/5] overflow-hidden">
                                    <Image
                                        alt="beatmap cover"
                                        fill
                                        src={`https://assets.ppy.sh/beatmaps/${beatmapSetId}/covers/cover.jpg`}
                                        className="object-cover"
                                    />
                                    <div className="absolute inset-0 bg-black bg-opacity-60 backdrop-blur-sm"></div>
                                    <div className="absolute inset-0 flex flex-col justify-center items-center text-white p-4">
                                        <h2 className="text-2xl font-bold mb-2 text-center">
                                            <Link
                                                href={`https://osu.ppy.sh/b/${beatmapId}`}
                                                className="underline text-pink-100"
                                                target="_blank"
                                            >
                                                {detailsResult.title}
                                            </Link>
                                        </h2>
                                        <p className="text-base mb-1 text-center">
                                            by{" "}
                                            <Link
                                                href={`https://osu.ppy.sh/beatmapsets?q=artist="${detailsResult.artist}"`}
                                                className="hover:underline text-pink-300"
                                                target="_blank"
                                            >
                                                {detailsResult.artist}
                                            </Link>
                                        </p>
                                        <p className="text-sm text-center">
                                            mapped by{" "}
                                            <Link
                                                href={`https://osu.ppy.sh/users/${detailsResult.creator_id}`}
                                                className="hover:underline text-pink-200"
                                                target="_blank"
                                            >
                                                {detailsResult.creator}
                                            </Link>
                                        </p>
                                        <p className="text-sm mt-1">
                                            [ {detailsResult.version} ]
                                        </p>
                                    </div>
                                </div>
                            </CardContent>
                        </Card>
                        <div className="grid gap-6 md:grid-cols-2">
                            <Card>
                                <CardHeader>
                                    <CardTitle className="flex items-center gap-2">
                                        <BarChart className="w-5 h-5" />
                                        Beatmap Stats
                                    </CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <div className="grid grid-cols-2 gap-2 text-sm">
                                        <div className="flex flex-row">
                                            <span className="font-semibold mr-1">
                                                AR:
                                            </span>
                                            <span>
                                                {detailsResult.statistics.ar}
                                            </span>
                                        </div>
                                        <div className="flex flex-row">
                                            <span className="font-semibold mr-1">
                                                OD:
                                            </span>
                                            <span>
                                                {detailsResult.statistics.od}
                                            </span>
                                        </div>
                                        <div className="flex flex-row">
                                            <span className="font-semibold mr-1">
                                                HP:
                                            </span>
                                            <span>
                                                {detailsResult.statistics.hp}
                                            </span>
                                        </div>
                                        <div className="flex flex-row">
                                            <span className="font-semibold mr-1">
                                                CS:
                                            </span>
                                            <span>
                                                {detailsResult.statistics.cs}
                                            </span>
                                        </div>
                                        <div className="flex flex-row">
                                            <span className="font-semibold mr-1">
                                                BPM:
                                            </span>
                                            <span>
                                                {detailsResult.statistics.bpm.toFixed()}
                                            </span>
                                        </div>
                                        <div className="col-span-2 flex flex-row">
                                            <span className="font-semibold mr-1">
                                                Star Rating:
                                            </span>
                                            <span>
                                                {detailsResult.statistics.star_rating.toFixed(
                                                    2,
                                                )}
                                            </span>
                                        </div>
                                    </div>
                                </CardContent>
                            </Card>

                            <Card>
                                <CardHeader>
                                    <CardTitle className="flex items-center gap-2">
                                        <Music className="w-5 h-5" />
                                        Classification
                                    </CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <ScrollArea className="h-56 pr-3">
                                        <div className="space-y-4">
                                            {analysisResult.map(
                                                (analysis, i) => (
                                                    <AnalysisCardClass
                                                        key={`class-${i}`}
                                                        index={i}
                                                        analysis={analysis}
                                                    />
                                                ),
                                            )}
                                            {analysisResult.map(
                                                (analysis, i) => (
                                                    <AnalysisCardDetails
                                                        key={`details-${i}`}
                                                        analysis={analysis}
                                                        totalObjects={detailsResult.statistics.total_objects}
                                                    />
                                                ),
                                            )}
                                        </div>
                                    </ScrollArea>
                                </CardContent>
                            </Card>
                        </div>
                    </div>
                    <Alert className="flex items-start">
                        <div className="flex items-center h-full pt-1">
                            <AlertTriangle className="h-4 w-4 flex-shrink-0" />
                        </div>
                        <AlertDescription className="ml-2">
                            This website is still early in development. Please{" "}
                            <Link
                                href="https://github.com/yorunoken/map-analyzer/issues"
                                className="underline"
                                target="_blank"
                            >
                                open an issue on GitHub
                            </Link>{" "}
                            if you have any recommendations or issues.
                        </AlertDescription>
                    </Alert>
                </>
            )}
        </div>
    );
}

function AnalysisCardClass({
    analysis,
    index,
}: {
    analysis: BeatmapAnalysisResult;
    index: number;
}) {
    const type = analysis.analysis_type;
    
    // Assign colors based on the analysis type
    const colors: Record<string, string> = {
        jump: "bg-pink-500",
        stream: "bg-blue-500",
        slider: "bg-green-500",
    };

    return (
        <div className="mb-4">
            <h3 className="font-bold text-lg uppercase tracking-tight">
                {type}
            </h3>
            <div className="w-full bg-gray-200 rounded-full h-3 dark:bg-gray-700 mt-1">
                <div
                    className={`${colors[type] || "bg-primary"} h-3 rounded-full transition-all duration-500`}
                    style={{
                        width: `${(analysis.analysis.overall_confidence || 0) * 100}%`,
                    }}
                ></div>
            </div>
            <p className="text-xs font-semibold mt-1 text-gray-400">
                Map Presence: {((analysis.analysis.overall_confidence || 0) * 100).toFixed(1)}%
            </p>
        </div>
    );
}

function AnalysisCardDetails({
    analysis,
    totalObjects,
}: {
    analysis: BeatmapAnalysisResult;
    totalObjects: number;
}) {
    const { analysis_type, analysis: details } = analysis;
    const capitalizedType =
        analysis_type.charAt(0).toUpperCase() + analysis_type.slice(1);

    return (
        <Accordion type="single" collapsible>
            <AccordionItem value="item-1">
                <AccordionTrigger className="font-semibold">
                    {capitalizedType} Details:
                </AccordionTrigger>
                <AccordionContent>
                    <ul className="list-disc list-inside text-sm">
                        {analysis_type === "stream" ? (
                            <StreamDetails
                                analysis={details as StreamAnalysis}
                                totalObjects={totalObjects} />
                        ) : analysis_type === "slider" ? (
                            <SliderDetails analysis={details as SliderAnalysis} />
                        ) : (
                            <JumpDetails analysis={details as JumpAnalysis} />
                        )}
                    </ul>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    );
}

function getSpacingTag(spacing: number, d: number) {
    if (spacing === 0) return "N/A";
    if (spacing < 2.0 * d) return "Narrow";
    if (spacing < 3.5 * d) return "Moderate";
    if (spacing < 5.0 * d) return "Wide";
    return "Cross-Screen (Extreme)";
}

function getStreamSpacingTag(spacing: number, d: number) {
    if (spacing === 0) return "N/A";
    if (spacing < 0.5 * d) return "Stacked";
    if (spacing < 1.0 * d) return "Overlapping";
    if (spacing < 2.0 * d) return "Spaced";
    return "Extreme (Jump-Stream)";
}

function getSliderTag(ratio: number) {
    if (ratio < 0.30) return "Mechanical Tech";
    if (ratio < 0.60) return "Technical";
    return "Slider Tech";
}

function StreamDetails({ analysis, totalObjects }: { analysis: StreamAnalysis; totalObjects: number; }) {
    const avg = analysis.avg_stream_spacing || 0;
    const d = analysis.circle_diameter || 73; // Fallback to CS4 if missing
    const totalPatterns = analysis.total_stream_patterns || 0;

    return (
        <>
            <li className="font-bold border-b border-blue-900 pb-1 mb-2">
                Type: {getStreamSpacingTag(avg, d)} ({avg.toFixed(1)} px)
            </li>

            <p className="text-xs font-semibold text-blue-400 uppercase mb-2">Distance Profile (Density by Notes)</p>
            <li className="flex justify-between">
                <span>Stacked (&lt; 0.5x D):</span>
                <span>{analysis.s_stacked_count || 0} ({((analysis.s_stack_dens || 0) * 100).toFixed(1)}%)</span>
            </li>
            <li className="flex justify-between">
                <span>Overlapping (0.5 - 1x D):</span>
                <span>{analysis.s_overlapping_count || 0} ({((analysis.s_over_dens || 0) * 100).toFixed(1)}%)</span>
            </li>
            <li className="flex justify-between">
                <span>Spaced (1 - 2x D):</span>
                <span>{analysis.s_spaced_count || 0} ({((analysis.s_space_dens || 0) * 100).toFixed(1)}%)</span>
            </li>
            <li className="flex justify-between mb-4">
                <span>Extreme (2 - 2.5x D):</span>
                <span>{analysis.s_extreme_count || 0} ({((analysis.s_extr_dens || 0) * 100).toFixed(1)}%)</span>
            </li>

            <p className="text-xs font-semibold text-blue-400 uppercase mb-2">Variance Profile (Relative to Streams)</p>
            <li className="flex justify-between"><span>Steady patterns:</span><span>{analysis.v_steady_count || 0} ({totalPatterns > 0 ? (((analysis.v_steady_count || 0) / totalPatterns) * 100).toFixed(1) : 0}%)</span></li>
            <li className="flex justify-between"><span>Variable patterns:</span><span>{analysis.v_variable_count || 0} ({totalPatterns > 0 ? (((analysis.v_variable_count || 0) / totalPatterns) * 100).toFixed(1) : 0}%)</span></li>
            <li className="flex justify-between mb-4"><span>Dynamic patterns:</span><span>{analysis.v_dynamic_count || 0} ({totalPatterns > 0 ? (((analysis.v_dynamic_count || 0) / totalPatterns) * 100).toFixed(1) : 0}%)</span></li>

            <p className="text-xs font-semibold text-blue-400 uppercase mb-2">Length Profile</p>
            <li>Bursts (3-4): {analysis.bursts || 0}</li>
            <li>Short streams (5-12): {analysis.short_streams || 0}</li>
            <li>Medium streams (13-24): {analysis.medium_streams || 0}</li>
            <li>Long streams (25-48): {analysis.long_streams || 0}</li>
            <li className="text-red-400 font-semibold mb-2">Deathstreams (49+): {analysis.death_streams || 0}</li>
            
            <li className="border-t border-blue-900 pt-2">Max stream: {analysis.max_stream_length} notes</li>
            <li>BPM Consistency: {(analysis.bpm_consistency * 100).toFixed(1)}%</li>
        </>
    );
}

function JumpDetails({ analysis }: { analysis: JumpAnalysis }) {
    const spacing = analysis.avg_spacing || 0;
    const d = analysis.circle_diameter || 73;
    const spacingTag = getSpacingTag(spacing, d);

    return (
        <>
            <li className="font-bold border-b border-gray-700 pb-1 mb-2">
                Spacing: {spacingTag} ({spacing.toFixed(1)} px)
            </li>
            
            <p className="text-xs font-semibold text-gray-500 uppercase mb-2">Distance Profile (Excluding Streams)</p>
            <li className="flex justify-between">
                <span>Narrow (&lt; 2.0x D):</span>
                <span>{analysis.narrow_count || 0} ({((analysis.narrow_dens || 0) * 100).toFixed(1)}%)</span>
            </li>
            <li className="flex justify-between">
                <span>Moderate (2 - 3.5x D):</span>
                <span>{analysis.moderate_count || 0} ({((analysis.moderate_dens || 0) * 100).toFixed(1)}%)</span>
            </li>
            <li className="flex justify-between">
                <span>Wide (3.5 - 5x D):</span>
                <span>{analysis.wide_count || 0} ({((analysis.wide_dens || 0) * 100).toFixed(1)}%)</span>
            </li>
            <li className="flex justify-between mb-3">
                <span>Extreme (5.0x+ D):</span>
                <span>{analysis.extreme_count || 0} ({((analysis.extreme_dens || 0) * 100).toFixed(1)}%)</span>
            </li>

            <li className="border-t border-gray-700 pt-2">Max jump chain: {analysis.max_jump_length} notes</li>
            <li>Short chain: {analysis.short_jumps}</li>
            <li>Medium chain: {analysis.medium_jumps}</li>
            <li>Long chain: {analysis.long_jumps}</li>
            <li>BPM Consistency: {(analysis.bpm_consistency * 100).toFixed(1)}%</li>
        </>
    );
}

function SliderDetails({ analysis }: { analysis: SliderAnalysis }) {
    const totalSliders = (analysis.l_short_count + analysis.l_med_count + analysis.l_long_count + analysis.l_ext_count) || 1;

    return (
        <>
            <li className="font-bold border-b border-green-900 pb-1 mb-2">
                Style: {getSliderTag(analysis.slider_ratio)} (Avg SV: {analysis.avg_velocity.toFixed(2)})
            </li>

            <p className="text-xs font-semibold text-green-400 uppercase mb-2">Slider Length Profile (Relative to Map)</p>
            <li className="flex justify-between"><span>Short (&lt;1.5x D):</span><span>{analysis.l_short_count} ({(analysis.l_short_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Medium (1.5-3x D):</span><span>{analysis.l_med_count} ({(analysis.l_med_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Long (3-4.5x D):</span><span>{analysis.l_long_count} ({(analysis.l_long_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between mb-4"><span>Extended (&gt;4.5x D):</span><span>{analysis.l_ext_count} ({(analysis.l_ext_dens * 100).toFixed(1)}%)</span></li>

            <p className="text-xs font-semibold text-green-400 uppercase mb-2">Buzz Slider Profile (Relative to Sliders)</p>
            <li className="flex justify-between"><span>Buzz Sliders:</span><span>{analysis.b_buzz_count} ({(analysis.b_buzz_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between mb-4"><span>Static Buzz:</span><span>{analysis.b_static_count} ({(analysis.b_static_dens * 100).toFixed(1)}%)</span></li>

            <p className="text-xs font-semibold text-green-400 uppercase mb-2">Artistic Profile (Relative to Sliders)</p>
            <li className="flex justify-between"><span>Simple (Linear):</span><span>{analysis.a_simple_count} ({(analysis.a_simple_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Curved:</span><span>{analysis.a_curved_count} ({(analysis.a_curved_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Complex:</span><span>{analysis.a_complex_count} ({(analysis.a_complex_dens * 100).toFixed(1)}%)</span></li>
            <li className="flex justify-between"><span>Artistic/Tech:</span><span>{analysis.a_artistic_count} ({(analysis.a_artistic_dens * 100).toFixed(1)}%)</span></li>
        </>
    );
}
