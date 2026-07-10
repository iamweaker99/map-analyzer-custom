"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { useToast } from "@/hooks/use-toast";
import { ScrollArea } from "./ui/scroll-area";

import { useState } from "react";
import { AlertTriangle, BarChart, Music } from "lucide-react";
import Image from "next/image";
import Link from "next/link";
import { parseURL } from "@/lib/osu";

import {
    BeatmapDetailsResult,
    BeatmapAnalysisResult,
    JumpAnalysis,
    StreamAnalysis,
    SliderAnalysis,
    FingerControlAnalysis,
    AimControlResult,
    ReadingResult,
} from "./analysis_engine/types";

import { JumpProfile } from "./analysis_engine/JumpProfile";
import { StreamProfile } from "./analysis_engine/StreamProfile";
import { SliderProfile } from "./analysis_engine/SliderProfile";
import { FingerControlProfile } from "./analysis_engine/FingerControlProfile";
import { AimControlProfile } from "./analysis_engine/AimControlProfile";
import { ReadingProfile } from "./analysis_engine/ReadingProfile";

type AnalysisProps = {
    getBeatmapDetails(beatmapId: number): Promise<BeatmapDetailsResult>;
    getBeatmapAnalysis<T extends "stream" | "jump" | "slider" | "all">(
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

    const jumpResult = analysisResult?.find(
        (a) => a.analysis_type === "jump",
    );
    const streamResult = analysisResult?.find(
        (a) => a.analysis_type === "stream",
    );
    const sliderResult = analysisResult?.find(
        (a) => a.analysis_type === "slider",
    );
    const fingerResult = analysisResult?.find(
        (a) => a.analysis_type === "fingercontrol",
    );
    const aimResult = analysisResult?.find(
        (a) => a.analysis_type === "aimcontrol",
    );
    const readingResult = analysisResult?.find(
        (a) => a.analysis_type === "reading",
    );

    const classificationTypes = [jumpResult, streamResult, sliderResult]
        .filter(
            (r): r is BeatmapAnalysisResult => r !== null && r !== undefined,
        )
        .sort(
            (a, b) =>
                ((b.analysis as any).overall_confidence ?? 0) -
                ((a.analysis as any).overall_confidence ?? 0),
        );

    const stats = detailsResult?.statistics;
    const totalObjects = stats?.total_objects ?? 0;

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
                    {/* Beatmap Banner */}
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

                    {/* Row 1: Stats | Classification */}
                    <div className="grid gap-4 md:grid-cols-2 mb-6">
                        {/* Beatmap Stats Card */}
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
                                            {stats?.ar.toFixed(2)}
                                        </span>
                                    </div>
                                    <div className="flex flex-row">
                                        <span className="font-semibold mr-1">
                                            OD:
                                        </span>
                                        <span>
                                            {stats?.od.toFixed(1)}
                                        </span>
                                    </div>
                                    <div className="flex flex-row">
                                        <span className="font-semibold mr-1">
                                            HP:
                                        </span>
                                        <span>
                                            {stats?.hp.toFixed(1)}
                                        </span>
                                    </div>
                                    <div className="flex flex-row">
                                        <span className="font-semibold mr-1">
                                            CS:
                                        </span>
                                        <span>
                                            {stats?.cs.toFixed(1)}
                                        </span>
                                    </div>
                                    <div className="flex flex-row">
                                        <span className="font-semibold mr-1">
                                            BPM:
                                        </span>
                                        <span>
                                            {stats?.bpm.toFixed(1)}
                                        </span>
                                    </div>
                                    <div className="col-span-2 flex flex-row">
                                        <span className="font-semibold mr-1">
                                            Star Rating:
                                        </span>
                                        <span>
                                            {stats?.star_rating.toFixed(2)}
                                        </span>
                                    </div>
                                </div>
                            </CardContent>
                        </Card>

                        {/* Classification Card */}
                        <Card>
                            <CardHeader>
                                <CardTitle className="flex items-center gap-2">
                                    <Music className="w-5 h-5" />
                                    Classification
                                </CardTitle>
                            </CardHeader>
                            <CardContent>
                                {classificationTypes.length > 0 ? (
                                    classificationTypes.map((analysis, i) => (
                                        <AnalysisCardClass
                                            key={`class-${i}`}
                                            analysis={analysis}
                                        />
                                    ))
                                ) : (
                                    <p className="text-sm text-gray-500">
                                        No classification data available.
                                    </p>
                                )}
                            </CardContent>
                        </Card>
                    </div>

                    {/* Row 2: Jump | Stream | Slider */}
                    <div className="grid gap-4 grid-cols-1 md:grid-cols-2 xl:grid-cols-3 mb-6">
                        {jumpResult && (
                            <Card className="border-t-2 border-t-pink-500/50">
                                <CardHeader>
                                    <CardTitle>Jumps</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <ScrollArea className="h-72 pr-3">
                                        <JumpProfile
                                            analysis={
                                                jumpResult
                                                    .analysis as JumpAnalysis
                                            }
                                        />
                                    </ScrollArea>
                                </CardContent>
                            </Card>
                        )}

                        {streamResult && (
                            <Card className="border-t-2 border-t-blue-500/50">
                                <CardHeader>
                                    <CardTitle>Streams</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <ScrollArea className="h-72 pr-3">
                                        <StreamProfile
                                            analysis={
                                                streamResult
                                                    .analysis as StreamAnalysis
                                            }
                                            totalObjects={totalObjects}
                                        />
                                    </ScrollArea>
                                </CardContent>
                            </Card>
                        )}

                        {sliderResult && (
                            <Card className="border-t-2 border-t-green-500/50">
                                <CardHeader>
                                    <CardTitle>Sliders</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <ScrollArea className="h-72 pr-3">
                                        <SliderProfile
                                            analysis={
                                                sliderResult
                                                    .analysis as SliderAnalysis
                                            }
                                        />
                                    </ScrollArea>
                                </CardContent>
                            </Card>
                        )}
                    </div>

                    {/* Row 3: Finger Control | Aim Control | Reading */}
                    <div className="grid gap-4 grid-cols-1 md:grid-cols-2 xl:grid-cols-3 mb-6">
                        {fingerResult && (
                            <Card className="border-t-2 border-t-purple-500/50">
                                <CardHeader>
                                    <CardTitle>Finger Control</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <ScrollArea className="h-72 pr-3">
                                        <FingerControlProfile
                                            analysis={
                                                fingerResult
                                                    .analysis as FingerControlAnalysis
                                            }
                                        />
                                    </ScrollArea>
                                </CardContent>
                            </Card>
                        )}

                        {aimResult && (
                            <Card className="border-t-2 border-t-cyan-500/50">
                                <CardHeader>
                                    <CardTitle>Aim Control</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <ScrollArea className="h-72 pr-3">
                                        <AimControlProfile
                                            data={
                                                aimResult
                                                    .analysis as AimControlResult
                                            }
                                        />
                                    </ScrollArea>
                                </CardContent>
                            </Card>
                        )}

                        {readingResult && (
                            <Card className="border-t-2 border-t-amber-500/50">
                                <CardHeader>
                                    <CardTitle>Reading</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <ScrollArea className="h-72 pr-3">
                                        <ReadingProfile
                                            data={
                                                readingResult
                                                    .analysis as ReadingResult
                                            }
                                        />
                                    </ScrollArea>
                                </CardContent>
                            </Card>
                        )}
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
}: {
    analysis: BeatmapAnalysisResult;
}) {
    const type = analysis.analysis_type;

    const colors: Record<string, string> = {
        jump: "bg-pink-500",
        stream: "bg-blue-500",
        slider: "bg-green-500",
    };

    const confidence = (analysis.analysis as any).overall_confidence ?? 0;

    return (
        <div className="mb-4 last:mb-0">
            <h3 className="font-bold text-lg uppercase tracking-tight">
                {type}
            </h3>
            <div className="w-full bg-gray-200 rounded-full h-3 dark:bg-gray-700 mt-1">
                <div
                    className={`${colors[type] || "bg-primary"} h-3 rounded-full transition-all duration-500`}
                    style={{
                        width: `${confidence * 100}%`,
                    }}
                ></div>
            </div>
            <p className="text-xs font-semibold mt-1 text-gray-400">
                Map Presence: {(confidence * 100).toFixed(1)}%
            </p>
        </div>
    );
}

export type { BeatmapDetailsResult, BeatmapAnalysisResult };
