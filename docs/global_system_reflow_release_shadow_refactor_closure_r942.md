# r942 GlobalSystemReflowReleaseShadowRefactorClosure

## Parent und rejected evidence

Autoritativer Parent ist der akzeptierte r940 `RendererFeasibilityCompileKeepaliveRetirement`.
r941 `GlobalSystemReflowReleaseShadowRetirement` ist abgelehnte Failure-Evidence und niemals Parent: der Inhaltsschnitt war eng, aber `AuditOnly` war fuer den finalen Diff falsch, weil die geaenderte gemischte Release-Funktion semantisch als `ProofReentryReader` im Music-Domain erkannt wird.

## ChangeClass

`RefactorOnly`.

Diese Klasse wird nicht zum Gate-Passieren hochgestuft, sondern aus dem finalen Diff abgeleitet: Der Schnitt veraendert eine bestehende gemischte produktive Release-Funktion, deren Fingerprint als `ProofReentryReader` erfasst wird. Entfernt werden ausschliesslich redundante read-only Recompute-/Projection-Schritte. Keine MusicXML-Mutation, kein Writer, keine Selection-, Threshold-, Layout-Materialisierungs-, Renderer- oder Release-Decision-Policy wird hinzugefuegt oder geaendert.

## Ziel

Die nachgelagerte zweite Ausfuehrung von `build_global_horizontal_system_reflow` im Release-Owner und ihre reine `HorizontalLayoutGlobalSystemReflowShadow`-Konsolenprojektion werden physisch entfernt. Der produktive Global-System-Reflow im Final-Notation-Planer bleibt unveraendert erhalten.

## Warum der zweite Aufruf entbehrlich ist

Der committed Pre-Layout-Quelltext wird bereits im Metric-/FinalNotation-Owner mit `build_final_notation_plan` verarbeitet. Dort wird `build_global_horizontal_system_reflow(final_notation_xml, &base_plan)` produktiv ausgefuehrt und seine Demand-/Capacity-Evidence in die aktuelle Page/System-Preference-Authority eingebunden.

Der spaetere Release-Aufruf verwendet denselben committed Quelltext und den bereits ausgewaehlten finalen Horizontal-Layout-Plan. Sein erzeugter `GlobalHorizontalSystemReflowPlan` wird unmittelbar verworfen; ausschliesslich das Audit wird als `HorizontalLayoutGlobalSystemReflowShadow` in die Konsole projiziert. Kein Gate, Writer, Renderer-Entscheid, ReleaseDecision oder Report konsumiert diese Projektion.

## Failure-Semantik des entfernten `?`

Der zweite Aufruf besitzt syntaktisch `?`, aber keine eigenstaendige verbleibende Sicherheitsgrenze:

- Parse-/Final-Fact-Capture wurde fuer denselben committed Quelltext bereits im produktiven Final-Notation-Planer erfolgreich ausgefuehrt.
- Nichtleere Demand-Slots und Fact-/Plan-Slotgleichheit wurden dort bereits gebunden.
- `NotRequired` gibt den bereits validierten initialen Plan unveraendert zurueck.
- Ein `Confirmed` RendererFeasibility-Plan entsteht nur ueber den fail-closed Promote: nichtleere Page/System-Segmente, exakte Slotdeckung, Page-Grenzen innerhalb der Slotbindung, exakte Systemdeckung, Capacity-Bindung, No-Good-Ausschluss und vollstaendige Provenienz werden dort bereits geprueft.
- Ein `Rejected` Renderer-Oracle bricht bereits vor dem entfernten Shadow-Aufruf ab.
- Fuer jede nichtleere Page besitzt `global_reflow_page_partition` konstruktiv mindestens eine Singleton-Partition; die spaetere Reflow-Berechnung fuegt daher fuer den finalen Plan keine zusaetzliche externe Zulassungsbedingung hinzu.

Damit wiederholt der Release-Shadow nur bereits bewiesene Voraussetzungen und erzeugt keine eigene produktive Authority.

## Schnitt

Entfernt werden ausschliesslich:

- der zweite Release-stage-Aufruf von `build_global_horizontal_system_reflow`;
- die unmittelbar daran gebundene `HorizontalLayoutGlobalSystemReflowShadow`-Konsolenprojektion.

Nicht entfernt werden:

- `horizontal_layout_global_reflow.rs`;
- der produktive Reflow-Aufruf in `build_final_notation_plan`;
- Demand-/Capacity-Evidence;
- PageCountPreference / Joint PageSystem Planning;
- RendererFeasibility;
- Writer, Materialisierung, MusicXML, PDF oder Release-Gates.

## Erwarteter Beweis

Tree Integrity, Change-Control, Semantic Capability Diff, Strict Clippy, Build/All-Targets und ein realer Musiklauf muessen gruen bleiben. Im Lauf soll `HorizontalLayoutGlobalSystemReflowShadow` nicht mehr erscheinen; die produktiven `HorizontalLayoutGlobalPageCountPreferenceShadow`, `HorizontalLayoutGlobalPageSystemPreferenceCandidate` und `HorizontalLayoutGlobalPageSystemPreferenceCutover` bleiben dagegen erhalten.
