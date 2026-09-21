Experiment Reasoning Framework

From the experiment, evidence, and discussion provided, produce four separate sections: Observation, Inference, Assumption, Hypothesis.
The four categories must remain strictly distinct.

1. Observation
Definition: A statement directly supported by what was observed, measured, recorded, or explicitly reported.
Include only facts or directly recorded evidence.
Do not include: explanations, causes, interpretations, predictions or beliefs about what should happen.
Test: “Could I point to the experiment data, transcript, event, or direct record and show exactly where this came from?”. If not, it is probably not an observation.
Format:
- Observation: [directly observed fact]
- Evidence: [measurement, event, quote, or record supporting it]
- Source
Example:
O1: 62% of users who started onboarding did not complete it.
Evidence: 620 of 1,000 users exited before reaching the completion screen.
Source: Experiment A, onboarding completion data.

O2: 41% of users who abandoned onboarding exited on Step 5.
Evidence: Product analytics event log.
Source: Experiment A.

2. Inference
Definition: A reasoned interpretation or possible explanation derived from one or more observations.
Inferences explain what the observations might mean, but they are not directly observed facts.
Use uncertainty language when appropriate: “may,” “might,” “suggests,” “is consistent with.”
Do not present an inference as a fact.
Test: “Am I interpreting the evidence rather than merely reporting it?”
Format:
- Inference: [interpretation]
- Confidence: [low / medium / high]
Example:
I1: Step 5 may be creating substantial friction during onboarding.
Confidence: Medium.

I2: Users may have difficulty understanding the information requested at Step 5.
Confidence: Medium.

3. Assumption
Definition: A proposition that the reasoning, decision, or experiment is relying on as though it were true, but which has not been sufficiently established by the current evidence.
An assumption is a premise, not an observation and not a prediction.
Ask: “What must we believe to be true for this reasoning, interpretation, or decision to hold?”
An assumption may be implicit in the discussion. Make it explicit.
Do not call something an assumption merely because it is uncertain. It must be something the reasoning is actually depending on.
Format:
- Assumption: [premise being relied upon]
- Why it matters: [what reasoning or decision depends on it]
- Evidence status: [supported / partially supported / unsupported / unknown]
Example:
A1: Users who abandon Step 5 would otherwise continue onboarding if the step were easier to complete.
Why it matters: The proposed intervention assumes that Step 5 friction is a meaningful cause of abandonment rather than merely correlated with it.
Evidence status: Partially supported.

4. Hypothesis
Definition: A specific, testable prediction or proposition derived from an inference and/or assumptions.
A hypothesis must be capable of being supported or contradicted by future evidence.
Prefer the structure:
If [condition/intervention], then [measurable outcome], under [relevant conditions].
A hypothesis is not merely an explanation such as “users dislike the product.” It should specify what we predict will happen and what evidence would distinguish the hypothesis from alternatives.
Format:
- Hypothesis: [testable prediction]
- Prediction: [specific expected outcome]
- Test: [experiment or evidence needed]
- Falsifier: [what observation would count against it]
Example:
H1: If Step 5 is reduced from five required fields to two, onboarding completion will increase by at least 10 percentage points.
Based on: I1, A1.
Prediction: Completion rate will increase from 38% to ≥48%.
Test: Run an A/B test comparing the existing Step 5 with the simplified version.
Falsifier: If completion does not increase meaningfully, or increases by less than the predefined threshold, the hypothesis is not supported.

Classification rules
If a statement could belong to more than one category, explain the distinction and place it in the category that best matches its epistemic role.
Never silently upgrade an epistemic status.
Important: Do not force every statement into all four categories. It is acceptable for one category to have no strong candidate. Do not invent an assumption, inference, or hypothesis merely to fill the template.
When uncertain, downgrade the claim rather than upgrade it. For example, write “The data are consistent with X” rather than “X caused Y” unless causality was actually established.

Output Rules
Do not produce a linear narrative. Construct a traceable many-to-many reasoning graph. Preserve alternative branches, shared dependencies, converging evidence, and competing hypotheses. 
Every inference, assumption, and hypothesis must be traceable through explicit links to one or more original observations. 
Never invent upstream evidence to complete a chain.
Example:
O1 + O2 + O5 -> I1 + I3 -> A2 + A4 -> H3