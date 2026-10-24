# r54 RendererSystemBreakReadbackFinalSelectedPlanDemandBindingConsolePrefixClosure

Parent: committed `0.10.1839 r53`.

## Target-system finding from r53

r53 is fully buildable and release-green:

- full corpus 10/10,
- `FinalExit 0`,
- DARK GREEN,
- renderer readback MATCH for all ten etudes,
- productive renderer-feasibility adoption remains correct.

However, the intended final diagnostic closure remained absent in the persisted
project reports for the five pieces whose final selected plan differs from the
initial Joint-Plan. They still showed the pre-r53
`RendererSystemBreakReadbackDemandBinding: Status UNAVAILABLE` result.

The r53 source snapshot and exported runtime evidence identify one exact cause.

`final_selected_plan_demand_binding_projection(...)` returned:

`ETUSCHLOPF: HorizontalLayoutFinalSelectedPlanDemandBinding: ...`

That string was passed to the existing `console_progress(...)` owner, whose
contract is to prepend `ETUSCHLOPF: ` and capture the resulting canonical
console line. Consequently the actual captured evidence became:

`ETUSCHLOPF: ETUSCHLOPF: HorizontalLayoutFinalSelectedPlanDemandBinding: ...`

The strict projection reader searches for exactly one canonical prefix:

`ETUSCHLOPF: HorizontalLayoutFinalSelectedPlanDemandBinding:`

and correctly ignored the malformed duplicate-prefix line.

## r54 closure

r54 changes only the producer-side message contract. The in-memory evidence
message now begins with:

`HorizontalLayoutFinalSelectedPlanDemandBinding:`

The existing `console_progress(...)` remains the sole owner of the external
`ETUSCHLOPF: ` prefix and of console-capture transport.

A regression test locks this ownership rule by asserting that the producer
message prefix contains no `ETUSCHLOPF:` owner prefix.

No readback parser is relaxed. No fallback is added. No initial Joint-Plan
evidence is removed or repurposed.

## Invariants

Unchanged from committed r53:

- productive renderer-feasibility adoption remains enabled;
- MuseScore remains feasibility-veto-only;
- EtuSchlopf remains candidate, plan and selection owner;
- no new music mutation;
- no new plan or selector;
- no new renderer-oracle behavior;
- no new demand model;
- no accepted-capacity or threshold change;
- no persistent No-Goods;
- no writer/read authority change;
- no identity special case;
- no report tolerance for ambiguous evidence.

Expected result after full-corpus acceptance:

- all ten pieces remain `RendererSystemBreakReadback: Status MATCH`;
- the five adopted pieces (2, 3, 6, 7, 8) report
  `RendererSystemBreakReadbackDemandBinding: Status BOUND`;
- source is `final_selected_plan_existing_joint_planner_evidence`;
- `MeasureDemandRecompose exact`;
- `ModelRecompute nein`;
- `ThresholdChange nein`;
- `PolicyWrite nein`.

ChangeClass: `RefactorOnly`.
