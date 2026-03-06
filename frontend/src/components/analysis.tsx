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
    };
}

export interface BeatmapAnalysisResult {
    analysis_type: "jump" | "stream";
    analysis: JumpAnalysis | StreamAnalysis;
}

interface JumpAnalysis {
    overall_confidence: number;
    total_jump: number;
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
    short_streams: number;
    medium_streams: number;
    long_streams: number;
    max_stream_length: number;
    stream_density: number;
    bpm_consistency: number;
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
    let indexString = "";
    switch (index) {
        case 0:
            indexString = "Primary";
            break;
        case 1:
            indexString = "Secondary";
            break;
    }

    return (
        <div className="mb-4">
            <h3 className="font-semibold">
                {indexString}: {analysis.analysis_type.charAt(0).toUpperCase()}
                {analysis.analysis_type.slice(1)}
            </h3>
            <div className="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700 mt-2">
                <div
                    className="bg-primary h-2.5 rounded-full"
                    style={{
                        width: `${analysis.analysis.overall_confidence * 100}%`,
                    }}
                ></div>
            </div>
            <p className="text-sm mt-1">
                Confidence:{" "}
                {(analysis.analysis.overall_confidence * 100).toFixed(1)}%
            </p>
        </div>
    );
}

function AnalysisCardDetails({
    analysis,
}: {
    analysis: BeatmapAnalysisResult;
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
                            />
                        ) : (
                            <JumpDetails analysis={details as JumpAnalysis} />
                        )}
                    </ul>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    );
}

function getSpacingTag(spacing: number) {
    if (spacing === 0) return "N/A";
    if (spacing < 120) return "Narrow (Close)";
    if (spacing < 190) return "Moderate";
    if (spacing < 280) return "Wide (Far)";
    return "Cross-Screen (Extreme)";
}

function StreamDetails({ analysis }: { analysis: StreamAnalysis }) {
    return (
        <>
            <li title="The longest string of unbroken stream notes in the map">
                Longest stream: {analysis.max_stream_length} notes
            </li>
            <li title="Bursts of 3 to 5 notes">
                Short streams: {analysis.short_streams}
            </li>
            <li title="Streams of 6 to 12 notes">
                Medium streams: {analysis.medium_streams}
            </li>
            <li title="Streams of 13 or more notes">
                Long streams: {analysis.long_streams}
            </li>
            <li title="Ratio of stream notes to total objects. Higher = more constant rapid clicking.">
                Stream Density: {analysis.stream_density.toFixed(3)}
            </li>
            <li title="Rhythmic regularity. 100% is perfectly steady; lower means complex rhythms or variable spacing.">
                BPM Consistency: {(analysis.bpm_consistency * 100).toFixed(1)}%
            </li>
        </>
    );
}

function JumpDetails({ analysis }: { analysis: JumpAnalysis }) {
    // We use (analysis.avg_spacing || 0) so that if the backend 
    // hasn't sent the data yet, it just uses 0 instead of crashing.
    const spacing = analysis.avg_spacing || 0;
    const spacingTag = getSpacingTag(spacing);

    return (
        <>
            <li className="font-bold">Spacing: {spacingTag}</li>
            <li>Avg. Jump Distance: {spacing.toFixed(1)} px</li>

            <p className="text-xs font-semibold text-gray-500 uppercase mb-2">Distance Profile (Individual Jumps)</p>
            
            <li className="flex justify-between">
                <span>Narrow (&lt;120px):</span>
                <span className="font-mono">{analysis.narrow_count || 0} ({((analysis.narrow_dens || 0) * 100).toFixed(1)}%)</span>
            </li>
            <li className="flex justify-between">
                <span>Moderate (120-190px):</span>
                <span className="font-mono">{analysis.moderate_count || 0} ({((analysis.moderate_dens || 0) * 100).toFixed(1)}%)</span>
            </li>
            <li className="flex justify-between">
                <span>Wide (190-280px):</span>
                <span className="font-mono">{analysis.wide_count || 0} ({((analysis.wide_dens || 0) * 100).toFixed(1)}%)</span>
            </li>
            <li className="flex justify-between mb-3">
                <span>Extreme (280px+):</span>
                <span className="font-mono">{analysis.extreme_count || 0} ({((analysis.extreme_dens || 0) * 100).toFixed(1)}%)</span>
            </li>

            <li title="The longest string of unbroken jumps in the map">
                Max jump chain: {analysis.max_jump_length} notes
            </li>
            <li title="Patterns with 3 to 5 jumps">
                Short chain: {analysis.short_jumps}
            </li>
            <li title="Patterns with 6 to 11 jumps">
                Medium chain: {analysis.medium_jumps}
            </li>
            <li title="Patterns with 12+ jumps">
                Long chain: {analysis.long_jumps}
            </li>
            <li title="Ratio of jumps to total objects. Higher = more constant jumping, less rest.">
                Jump Density: {analysis.jump_density.toFixed(3)}
            </li>
            <li title="Rhythmic regularity. 100% is perfectly steady; lower means complex/changing rhythms.">
                BPM Consistency: {(analysis.bpm_consistency * 100).toFixed(1)}%
            </li>
        </>
    );
}
