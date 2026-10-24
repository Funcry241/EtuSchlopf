# r560 – PhraseNotationInternalHierarchyAudit

## Parent

Autoritativer Elternstand ist EtuSchlopf v0.10.1839 r559
`CrossStaffDottedEighthSixteenthBeamRule`, im uebergebenen 10er-Korpus formal 10/10 gruen.

## Zweck

r560 ist ausschliesslich AuditOnly. Der Take untersucht die bereits vorhandenen
`allow_structural_pending_visual`-Phrasierungsintents, ohne ihre Klassifikation, Projektion oder
produktive Schreibberechtigung zu veraendern.

Ausgangspunkt ist der historische Befund, dass r146 Phrase-Slurs produktiv materialisierte und r421
diese Persistenz wegen falscher grosser Render-Boegen fail-closed schloss. Die heutige r559-Analyse
besitzt weiterhin die vollstaendige Phrase-Evidence-/Structure-/Intent-Pipeline, laesst den
Phrase-Slur-Kandidaten aber nur als Shadow-Proof entstehen.

Die offene Frage lautet daher nicht, ob lange analytische Phrasen verboten werden sollen, sondern
welche interne Hierarchie die heutigen `pending_visual`-Projektionen tatsaechlich ueberspannen.

## Beobachteter Architekturspalt

Der Phrase-Evidence-Owner unterscheidet bereits unter anderem:

- `phrase_local_joint`;
- `phrase_closure_structure_gate`;
- `closure_supported`;
- `structural_joint`;
- `continuation_supported`;
- `conflict`;
- `unresolved`.

Die historisch als `GroupingLevel0LocalGroups` bezeichnete Segmentierung verwendet jedoch nur
`phrase_closure_structure_gate`. Ein `phrase_local_joint` kann deshalb innerhalb eines spaeteren
`closure_ending_local_group` liegen, ohne den Projektionsanfang neu zu setzen.

r560 aendert diese Logik ausdrücklich nicht. Der Take macht nur sichtbar, wie viele solcher bereits
erkannten lokalen Gelenke innerhalb jeder realen `allow_structural_pending_visual`-Projektion liegen.

## Neuer Audit-Nachweis pro Pending-Visual-Projektion

Fuer jeden bereits autorisierten Pending-Visual-Intent wird im Resultat eine
`PhraseNotationInternalHierarchy[...]`-Zeile ausgegeben mit:

- realem Projection-Start und Projection-Ende;
- realem `projection_attack_count`;
- historischem `SlurIntentSample`-Endpunkt und dessen Attack-Zahl;
- explizitem Nachweis, ob die produktive Projektion die Closure-Ankunft zusaetzlich umfasst;
- inklusiver Taktspanne;
- Anzahl interner Boundary-Positionen;
- Anzahl interner `structural_joint`-Rollen;
- Anzahl interner `phrase_closure_structure_gate`-Positionen;
- Anzahl interner `continuation_supported`-, `conflict`- und `unresolved`-Rollen;
- Anzahl interner unerklärter Parallelismus-Rollendivergenzen;
- terminaler Boundary-Rolle.

Zusätzlich erscheint eine aggregierte `PhraseNotationInternalHierarchyAudit`-Zeile mit Anzahl der
Pending-Visual-Zeilen, Anzahl der Projektionen mit internen StructuralJoints, Gesamt- und Maximalzahl
dieser Gelenke, maximaler Projection-Attack-Zahl, maximaler Taktspanne und Anzahl der Projektionen,
bei denen die reale Projektion gegenueber dem historischen Sample die Closure-Ankunft ergaenzt.

## Bewusst nicht behauptete Evidenz

Der Phrase-Evidence-Owner arbeitet vor dem finalen Layout. Deshalb wird `SystemSpan` in r560
explizit als `unavailable_pre_layout_owner` ausgewiesen. Es wird keine Systemgrenzen-Wahrheit
nachgebildet oder aus Taktzahlen erraten.

Die heutige Lane-Identitaet bleibt unveraendert `(part, staff, voice)`. Der Audit weist deshalb
`StaffBoundIdentity` sichtbar aus, ohne diese Identitaet in diesem Take zu aendern oder VoiceRuns
staffuebergreifend zusammenzufuehren.

Ebenso bleiben `PhraseMarkSemanticCandidate`, `LegatoSemanticCandidate` und die eigentliche
NotationPolicy absichtlich `unclassified` bzw. `unchanged_pending_visual`.

## Historischer Report-/Projection-Versatz

Seit r139 endet der reale `PhraseNotationIntentProjection` auf der rechten, ankommenden Note der
Closure und besitzt damit einen Anschlag mehr als der historische `SlurIntentSample`. Der Sample-
Report blieb aus Stabilitaetsgruenden beim linken Closure-Anschlag.

r560 veraendert keinen der beiden historischen Vertraege. Es stellt beide Werte erstmals gemeinsam
gegenueber, damit die spaetere NotationPolicy auf dem tatsaechlich an den Writer projizierten Span
arbeitet.

## Keine neue NotationPolicy

r560 fuehrt insbesondere **keine** der folgenden Heuristiken ein:

- keine maximale Notenzahl;
- keine maximale Taktzahl;
- kein `structural_joint` als automatischer harter Slur-Trenner;
- keine gewichtete PhraseConfidence;
- keine neue Schwelle;
- keine Promotion `pending_visual -> visible`;
- keine Cross-Staff-Phrase-Rekonstruktion;
- keine Layout-/Systembruchentscheidung.

Die neuen Werte sind reine Beobachtung. Sie besitzen kein Schreibrecht.

## Keine Mutation

Unveraendert bleiben:

- `PhraseNotationIntentProjection`-Felder und deren Semantik;
- Anzahl und Auswahl von `allow_structural_pending_visual`;
- alle Withhold-Gruende;
- PhraseBoundary-/PhraseStructure-Klassifikation;
- Phrase-Slur-Shadow-Writer;
- Render-MusicXML;
- Master-MusicXML;
- Pitch, Onset, Duration, Staff und Voice;
- Pass-Reihenfolge;
- StructuralWriteAuthority;
- Writer-Reachability;
- alle produktiven Musikregeln.

`RenderXmlWrite nein`, `SoundingEventMutation nein`, `MasterMusicXmlMutation nein` bleiben harte
Abnahmebedingungen.

## Regression

Die bestehenden Phrase-Tests werden innerhalb desselben Audit-Owners erweitert:

1. der vorhandene einfache Closure-Fall behaelt exakt einen Pending-Visual-Intent mit unveraendertem
   Projection-Start, Projection-Ende und AttackCount;
2. der neue Audit zeigt fuer diesen Fall den historischen Sample-/Projection-`+1`-Versatz;
3. ein synthetischer Fall mit einem off-beat rhythmischen `structural_joint` innerhalb eines spaeter
   closure-gestuetzten Spans behaelt denselben Pending-Visual-Intent, meldet aber mindestens ein
   internes StructuralJoint;
4. innerhalb dieser Projektion liegt kein zusaetzliches `phrase_closure_structure_gate`, womit der
   Unterschied zwischen lokaler Boundary-Evidence und der historischen Level-0-Segmentierung
   explizit belegt wird.

## Erwartete Korpusabnahme

Nach erfolgreichem Windows-Take zuerst eine Etuede mit mehreren Pending-Visual-Intents ausfuehren und
pruefen:

- `PhraseNotationInternalHierarchyAudit` erscheint;
- jede `allow_structural_pending_visual`-Projektion besitzt genau eine
  `PhraseNotationInternalHierarchy[...]`-Zeile;
- `ProjectionAddsClosureArrivalRows` entspricht dem historischen r139-Vertrag;
- `WriterOpen nein` und `MutationAuthority none` bleiben bestehen.

Danach Vollkorpus. Der Take ist nur akzeptabel, wenn die bisherigen 10 Projekte formal unveraendert
gruen bleiben und kein MusicXML-/PDF-/Sounding-Delta durch diese Audit-Erweiterung entsteht.

Die Korpuszahlen aus r560 sind anschliessend die Evidenz fuer einen separaten spaeteren
NotationPolicy-Take. r560 selbst entscheidet keine sichtbaren Phrasierungsboegen.
