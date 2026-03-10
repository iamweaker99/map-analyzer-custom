import { 
    Accordion, 
    AccordionContent, 
    AccordionItem, 
    AccordionTrigger 
} from "@/components/ui/accordion"; // Fixes Accordion errors

import { JumpProfile } from "./JumpProfile"; // Matches your file name
import { StreamProfile } from "./StreamProfile"; // Matches your file name
import { SliderProfile } from "./SliderProfile"; // Matches your file name

import { 
    BeatmapAnalysisResult, 
    BeatmapDetailsResult, 
    JumpAnalysis, 
    StreamAnalysis, 
    SliderAnalysis 
} from "./types"; // Fixes Interface errors

// ... rest of your code ...

export function AnalysisCardDetails({
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
                            <StreamProfile
                                analysis={details as StreamAnalysis}
                                totalObjects={totalObjects} 
                            />
                        ) : analysis_type === "slider" ? (
                            <SliderProfile analysis={details as SliderAnalysis} />
                        ) : (
                            <JumpProfile analysis={details as JumpAnalysis} />
                        )}
                    </ul>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    );
}