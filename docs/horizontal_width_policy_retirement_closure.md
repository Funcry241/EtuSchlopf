# HorizontalWidthPolicyRetirementClosure

## Scope

Take 16 retires the now-proven-dead explicit `measure/@width` decision paths after v0.10.1837-r01 completed the full RendererAuto cutover. The v0.10.1837 Windows run was real DARK GREEN and reported 44 slots, 43 width removals, 0 system-break changes, 0 page-break changes, 0 explicit widths after promotion and 44 RendererAuto slots. A direct PDF comparison against the preceding global Page+System plan showed 6/6 pages pixel-identical and 0 changed pixels.

## Architecture closure

The final horizontal plan must now be natively RendererAuto for width rather than generating generic width decisions and demoting them afterwards. Therefore:

- the structural legacy seed no longer carries inherited `measure/@width`;
- DenseRegisterNotation no longer owns a productive width decision path; its old score/width projection remains test-only only for historical fact-extraction parity;
- VisualLayoutPressure keeps its system-break decision role and vertical polish inputs, but no longer plans a width;
- PianisticReadability keeps its target selection for vertical staff-distance / octave-y polish, but no longer plans a width;
- the Take-14 width-demotion audit module and the Take-15 runtime demotion transaction are retired from the productive graph;
- the final planning owner fails closed if any explicit width or width owner nevertheless survives.

The plan/materializer contract deliberately retains the optional `explicit_width` representation so a future evidence-backed exceptional width constraint could be introduced explicitly. There is simply no current generic productive policy that writes it.

## Invariants

- final plan: `ExplicitWidths == 0`;
- final plan: `RendererAutoSlots == Slots`;
- final plan: `WidthPolicyWriters == 0`;
- page/system decisions unchanged;
- One Planning Authority and One Materializer unchanged;
- no repair/recovery layer;
- no Etude/measure-specific production logic;
- no security weakening.

This take is `RefactorOnly`: it removes policy debt proven behaviorally inert by the preceding GREEN cutover and pixel-identical renderer evidence.
