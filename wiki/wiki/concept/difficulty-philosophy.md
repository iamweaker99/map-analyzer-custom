---
type: concept
updated: 2026-08-12
---
# Difficulty Philosophy — Trajectory × Velocity, Local Spikes, Direction-Agnostic

*User-dictated design philosophy (2026-08-12) for the motor/reading difficulty model. The model evaluates how the real car performs on the track — not the track as a pure 2D drawing.*

## The race-track axiom

Difficulty is a property of the **(trajectory, velocity) pair**, not of the trajectory alone. A beatmap pattern is a perfect time-attack record on a race track:

- go off the track → you fail;
- can't keep up with the speed → you fail;
- failures and errors reduce the score — in osu! terms, Accuracy;
- 100% accuracy = perfectly following how the track is meant to be played.

Physics quantities (velocity, centripetal, jerk) are welcome when they fit the case. Real-world physics does not have to be very complex to induce difficulty — don't over-abstract into math.

## Momentum disruption = motor adjustment

A note sequence carries motion and momentum. Anything that **breaks the momentum** causes disruption and therefore introduces a **motor adjustment** — a cursor reset to get back on track under the same hit window. This tests how well the player controls and keeps themselves on track under disruption. "Abrupt" joins between minor sections (see [[motor-model-requirements]] R2) are the dominant instance: more motor adjustment under the same hit window, delivered suddenly.

## Local and relative spikes, not global values

Reading difficulty is due to a **relative or local spike**, not a global one — "else every jump is hard, as most of them have high spacing & direction change" (yet the 51-section ground truth flags ~1 jump section, for self-overlap). The object of analysis is the **trajectory with the velocity travelling on it**: at every corner the trajectory signals a velocity + directional change. The same trajectory under a different speed has different difficulty. A corner is not "an angle of 70°" — it is a momentum change under a time budget.

## Direction-agnostic difficulty

"Harder" is not always "higher". By design:

- the value can be **high** = harder (e.g. speed: higher = faster = harder to control);
- or **low** = harder (e.g. consistency: lower = more chaos = harder to control).

Every metric must state which direction (or both) makes difficulty grow.

## Scope boundaries

- **Sliders:** ignored by the reading/motor model — the slider analysis already covers them well; only improved someday in the future, not soon. (Open: slider heads/ends still appear as trajectory waypoints in tagged sections.)
- **Mods:** excluded (no Hidden etc.) — base game only, keep it simple.

## No fused score

Multi-perspective analysis; no single weighted difficulty score (consistent with [[Data-Philosophy]]).

## Geometry via low-level descriptors

Do not build a database of geometry types ("square", "spiral" — infinite possibilities). Describe geometry with **low-level descriptors the way CAD software does**: "four points in orthogonal directions with equal distance" instead of "square". The shape vocabulary (line / curve / wiggle / zig-zag / V / diagonal / spiral) is *derived* from the descriptors, and difficulty is studied as correlation/causation against them.

## Difficulty features appear at high star ratings

Join severity, self-overlap, dirty placement are intentional design on high-star maps and will be seen frequently there — that is where the model matters.

## Sources
User statements 2026-08-12 (quoted in [[2026-08-12-handoff]] + log); ground truth: `Prototyping/51_test_run_sample.json` (51 tagged sections of the YOASOBI collab extra map).
