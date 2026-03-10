import { 
    Accordion, 
    AccordionContent, 
    AccordionItem, 
    AccordionTrigger 
} from "@/components/ui/accordion";

import { JumpProfile } from "./JumpProfile";
import { StreamProfile } from "./StreamProfile";
import { SliderProfile } from "./SliderProfile";
import { FingerControlProfile } from "./FingerControlProfile"; // This is the COMPONENT

import { 
    BeatmapAnalysisResult, 
    JumpAnalysis, 
    StreamAnalysis, 
    SliderAnalysis,
    FingerControlAnalysis,
} from "./types";

export function AnalysisCardDetails({
    analysis,
    totalObjects,
}: {
    analysis: BeatmapAnalysisResult;
    totalObjects: number;
}) {
    const { analysis_type, analysis: details } = analysis;
    
    // Custom label logic for the Accordion
    const displayType = analysis_type === "fingercontrol" 
        ? "Finger Control Analysis" 
        : analysis_type.charAt(0).toUpperCase() + analysis_type.slice(1);

    return (
        <Accordion type="single" collapsible>
            <AccordionItem value="item-1">
                <AccordionTrigger className="font-semibold">
                    {displayType}:
                </AccordionTrigger>
                <AccordionContent>
                    {/* Removed <ul> for better layout nesting */}
                    <div className="text-sm">
                        {analysis_type === "stream" && (
                            <StreamProfile
                                analysis={details as StreamAnalysis}
                                totalObjects={totalObjects} 
                            />
                        )}
                        {analysis_type === "slider" && (
                            <SliderProfile analysis={details as SliderAnalysis} />
                        )}
                        {analysis_type === "jump" && (
                            <JumpProfile analysis={details as JumpAnalysis} />
                        )}
                        {analysis_type === "fingercontrol" && (
                            <FingerControlProfile analysis={details as FingerControlAnalysis} />
                        )}
                    </div>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    );
}