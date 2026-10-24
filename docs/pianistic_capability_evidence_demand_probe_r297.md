# r297 PianisticCapabilityEvidenceDemandProbe

## Parent und Ziel

r297 basiert direkt auf dem real installierten und dunkelgruenen r296 `PianisticExecutionPlanWitness` mit 1036/1036 Tests.

r296 hat den konstruktiven Such- und Independent-Replay-Apparat geschlossen, bleibt fuer den realen Score aber bewusst bei `MODEL_INSUFFICIENT`, weil noch kein produktives Pianisten-/Transitionsmodell gebunden ist. r297 schliesst diese Luecke noch nicht durch erfundene Physiologie. Stattdessen erzeugt der neue read-only Owner `PianisticCapabilityEvidenceDemandProbe` erstmals ein endliches, fingerprint-gebundenes Inventar der Capability-Fragen, die ein spaeteres Referenz- oder kalibriertes Modell beantworten muss.

Der Take bleibt `AuditOnly`: kein MusicXML-Writer, kein Render-Writer, kein MasterApply, keine Musikmutation und kein Playability-Claim.

## Warum zuerst Evidence Demand statt Referenzprofil

Ein direkt erfundenes Referenzprofil wuerde sofort numerische Reichweiten- oder Zeitannahmen zur scheinbaren Koerperwahrheit machen. Genau das soll der neue Strang vermeiden.

r297 dreht deshalb die Richtung um:

`Score -> ExecutionEvents -> PhysicalKeyObligations -> Probe -> CapabilityEvidenceDemand`

Erst danach darf ein Modell sagen, welche dieser konkreten Konfigurationen oder Transitionen es wirklich abdeckt.

## Source-Staff-Affinity nur als Probe

Staff 1 und Staff 2 bleiben ausdruecklich keine Haende.

r297 darf die vorhandene Staff-Affinitaet jedoch als bewusst nicht-autoritativen Probeweg benutzen:

- Staff 1 -> Right **nur fuer den Probeweg**
- Staff 2 -> Left **nur fuer den Probeweg**
- gemischte oder unbekannte Affinitaet -> unresolved

Der Probeweg ist weder Suchconstraint noch Witness noch CapabilityEvidence. Er dient nur dazu, eine konkrete Kalibrationsfrage zu materialisieren.

Ein besonders wichtiger realer Gegenfall ist bereits vor dem Windows-Lauf sichtbar: Der aktuelle Etuede-6-Master enthaelt eine Boundary mit sieben gleichzeitig aktiven Tasten, davon sechs mit Staff-1-Affinitaet. Im v1-Technikraum `one-key-per-finger` kann eine einzelne Hand hoechstens fuenf gleichzeitige Tasten halten. Damit liefert r297 einen konstruktiven Beweis dafuer, dass `StaffIsHand` falsch waere und ein spaeterer Witness Handuebernahmen beziehungsweise eine von Staff unabhaengige Aufteilung finden muss.

Diese Beobachtung wird nicht hardcodiert und nicht als Etuedenregel verwendet.

## CanonicalProbeFingering

Damit aus einem Staff-Affinity-Pfad konkrete statische Signaturen und Transitionen als Fragen entstehen, verwendet r297 eine deterministische CanonicalProbeFingering-Projektion.

Sie ist ausdruecklich:

- keine Fingersatzempfehlung
- keine CapabilityEvidence
- keine statische Freigabe
- kein Witness
- keine Reparatur

Wenn sich fuer eine waehrend eines Holds aktive Obligation die kanonische Probe-Fingerzuordnung zwischen Boundaries aendert, wird das als `HoldFingerReassignmentConflict` gezaehlt. Der Konflikt wird nicht still durch eine Fingersubstitution geheilt, weil Fingersubstitution im v1-Witness noch nicht modelliert ist.

## Fail-closed Probe-Semantik

Der Probeweg wird `INCOMPLETE`, sobald mindestens einer der folgenden Faelle auftritt:

- gemischte oder unbekannte Staff-Affinitaet einer physischen Obligation
- mehr als fuenf gleichzeitig aktive Tasten auf einer Affinity-Hand
- Hold-Finger-Reassignment im CanonicalProbeFingering
- gebrochene TransitionDemand-Kette durch einen unaufloesbaren Probe-Handzustand

Wichtig:

`INCOMPLETE Probe != unplayable`

und

`COMPLETE Probe != playable`

Der Status bewertet nur die Verwendbarkeit des Probewegs als endliches Kalibrationsinventar.

## EvidenceDemand

r297 sammelt zwei Arten von Fragen.

### StaticConfigurationDemand

Jede nichtleere kanonische Probe-Handkonfiguration wird durch die bereits autoritative r295-`StaticConfigurationSignature` beschrieben. Identische relative Ganzkonfigurationen werden dedupliziert.

r297 sagt nicht `SUPPORTED` oder `REJECTED`. Es zaehlt nur:

- eindeutige statische Signaturen
- Auftretenshaeufigkeit

### TransitionDemand

Zwischen zwei bekannten nichtleeren Probe-Handkonfigurationen wird die bereits r296-autoritative `TransitionSignature` verwendet.

Pro eindeutiger Transition wird gesammelt:

- Auftretenshaeufigkeit
- kleinste im Score beobachtete verfuegbare physische Zeit

Auch hier entsteht noch keine Mindestzeit und keine Freigabe. Die kleinste beobachtete Zeit ist nur die konkrete Frage, die ein spaeteres Capability-Modell beantworten muss.

## Fingerprint

Das komplette EvidenceDemand-Inventar erhaelt einen SHA-256-Fingerprint.

Gebunden werden:

- MusicalExecutionFingerprint
- statische Signaturen und Auftretenszahlen
- TransitionSignaturen und Auftretenszahlen
- minimale beobachtete Transition-Zeit
- unresolved Affinities
- OverFive-Probe-Grenzen
- Hold-Finger-Konflikte
- Transition-Chain-Breaks

Source-Voice-IDs gehoeren nicht in diese Identitaet. Ein Regressionstest prueft daher, dass eine reine Voice-Umetikettierung den EvidenceDemand-Fingerprint nicht veraendert.

## Voraberwartung Etuede 6

Aus dem realen r296-Master werden vor dem Windows-Lauf fuer den neuen Probeweg erwartet:

- SourceExecutionEvents: 1237
- PhysicalKeyObligations: 1234
- BoundaryCount: 645
- AffinityLeftObligations: 476
- AffinityRightObligations: 758
- UnresolvedAffinityObligations: 0
- OverFiveKeyHandBoundaries: 1
- HoldFingerReassignmentConflicts: 41
- TransitionChainBreaks: 1
- UniqueStaticConfigurationDemands: 105
- StaticConfigurationOccurrences: 1121
- UniqueTransitionDemands: 509
- TransitionOccurrences: 857
- ProbeStatus: INCOMPLETE
- EvidenceDemandSha256: `3c9c2d64fcf8ed9a371d51869c8263c8159de552d90818883c6a5084c444b8da`
- PlayabilityClaim: NOT_EVALUATED

Diese Werte werden nicht im Produktcode hardcodiert.

## Regressionen

r297 fuegt acht allgemeine Regressionen hinzu:

1. Staff 1/2 werden nur als rechte/linke Probe-Affinitaet aufgeloest.
2. Ein physisch koalesziertes Ereignis mit gemischter Staff-Affinitaet bleibt unresolved.
3. Transponierte Einzelnoten teilen sich dieselbe relative statische Demand-Signatur.
4. Eine sequentielle Transposition erzeugt genau eine TransitionDemand.
5. Sechs gleichzeitige Tasten derselben Affinity-Hand machen den Probeweg fail-closed `INCOMPLETE`.
6. Eine kanonische Fingerneuzuordnung waehrend eines Holds wird als Konflikt sichtbar.
7. Reine Voice-Umetikettierung veraendert den EvidenceDemand-Fingerprint nicht.
8. Der Report trennt Probe, Capability und Playability explizit.

Ausgehend von 1036/1036 werden 1044/1044 integrierte Tests erwartet.

## Naechster Schritt

Nach real gruener r297-Verifikation ist die Capability-Luecke erstmals endlich und scoregebunden beschrieben.

Der naechste fachliche Schritt ist dann nicht sofort Obstruction/Tempo, sondern ein eigener r298-Owner fuer ein explizit benanntes, fingerprint-gebundenes Referenz-/Capability-Modell. Dieses Modell muss seine Annahmen offenlegen und darf nur modellrelative Aussagen erzeugen. Erst wenn r296 unter diesem gebundenen Modell entweder einen replay-validierten Witness oder einen ehrlichen Modellrest liefert, ist r299 `PianisticObstructionAndTempoFrontier` belastbar.
