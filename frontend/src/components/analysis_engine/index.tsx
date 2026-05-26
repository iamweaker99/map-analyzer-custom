import { 
    Accordion, 
    AccordionContent, 
    AccordionItem, 
    AccordionTrigger 
} from "@/components/ui/accordion";

import { JumpProfile } from "./JumpProfile";
import { StreamProfile } from "./StreamProfile";
import { SliderProfile } from "./SliderProfile";
import { FingerControlProfile } from "./FingerControlProfile";
import { AimControlProfile } from './AimControlProfile';
import { ReadingProfile } from './ReadingProfile'; // <-- INJECTED HERE

import { 
    BeatmapAnalysisResult, 
    JumpAnalysis, 
    StreamAnalysis, 
    SliderAnalysis,
    FingerControlAnalysis,
    AimControlResult,
    ReadingResult // <-- INJECTED HERE
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
    let displayType = analysis_type.charAt(0).toUpperCase() + analysis_type.slice(1);
    if (analysis_type === "fingercontrol") displayType = "Finger Control Analysis";
    if (analysis_type === "aimcontrol") displayType = "Aim Control Analysis";
    if (analysis_type === "reading") displayType = "Reading Analysis"; // <-- INJECTED HERE

    return (
        <Accordion type="single" collapsible>
            <AccordionItem value="item-1">
                <AccordionTrigger className="font-semibold">
                    {displayType}:
                </AccordionTrigger>
                <AccordionContent>
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
                        {analysis_type === "aimcontrol" && (
                            <AimControlProfile data={details as AimControlResult} />
                        )}
                        {analysis_type === "reading" && (
                            <ReadingProfile data={details as ReadingResult} /> // <-- INJECTED HERE
                        )}
                    </div>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    );
} 