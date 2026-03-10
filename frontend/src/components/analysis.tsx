"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { AnalysisCardDetails } from "./analysis_engine";
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
    analysis_type: "jump" | "stream" | "slider" | "fingercontrol";
    analysis: JumpAnalysis | StreamAnalysis | SliderAnalysis | FingerControlAnalysis;
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

interface FingerControlAnalysis {
    overall_confidence: number; // Matches the progress bar logic
    complexityScore: number;
    morphologyIndex: number;
    snapDistribution: { label: string, percentage: number }[];
    evenBurstRatio: number;
}

type AnalysisProps = {
    getBeatmapDetails(beatmapId: number): Promise<BeatmapDetailsResult>;
    getBeatmapAnalysis<T extends "stream" | "jump" | "slider" | "fingercontrol" | "all">(
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
        fingercontrol: "bg-purple-500", // Finger control is Purple
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