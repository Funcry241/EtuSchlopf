# r559 – CrossStaffDottedEighthSixteenthBeamRule

## Parent

Autoritativer Elternstand ist EtuSchlopf v0.10.1839 r556
`VisibleRegisterBoundaryHiddenChordMaskingClosure`, im uebergebenen 10er-Korpus formal
10/10 gruen.

## Revisionshinweis

r558 `MusicXml31OfflineXsd10ValidatorQualificationPackagingRecovery` wurde in einem
parallelen Arbeitsordner als Kandidat installiert, scheiterte im integrierten Rust-Testgate
und wurde vom Take-Bootstrap vollstaendig auf den vorherigen Quellstand zurueckgerollt.
Der Kandidat ist fail-closed rejected und wird nicht wiederverwendet. r559 basiert daher
fachlich und quellseitig weiterhin direkt auf dem autoritativen r556-Elternstand; es werden
keine Inhalte aus r558 uebernommen.

## Partiturbefund

In Etuede 3, Takt 19, Zaehlzeit 2 bilden eine punktierte Achtel und die folgende
Sechzehntel derselben musikalischen Voice gemeinsam genau einen Viertelschlag.
Die erste Note liegt auf Staff 2, die zweite auf Staff 1. Im r556-Render fehlen
beiden Noten die primaeren Beam-Tags, obwohl die benachbarten Zaehlzeiten derselben
Cross-Staff-Geste regulaer gebalkt sind.

Die fachliche Regel ist nicht fallidentitaetsgebunden:

> Ein Staff-Wechsel allein unterbricht eine nach dem Metrum zusammengehoerige
> Balkengruppe derselben musikalischen Voice nicht.

Fuer diesen Take wird ausschliesslich die bisher fehlende vollstaendige
`3/4 Beat + 1/4 Beat`- beziehungsweise `1/4 Beat + 3/4 Beat`-Klasse aus
punktierter Achtel und Sechzehntel geschlossen.

## Root Cause

Der bestehende Owner `structural_cross_staff_voice_beam_groups` qualifiziert seine
historische Cross-Staff-Klasse nur aus beamfaehigen Ereignissen mit
`duration <= beat_unit / 2`. Dadurch wird die punktierte Achtel vor der
Gruppenbildung ausgeschlossen. Die verbleibende Sechzehntel kann allein keine
Cross-Staff-Beamgruppe bilden.

Der Renderer ist nicht Ursache. Die fehlenden Beam-Tags liegen bereits im
Render-MusicXML vor.

## Fachlicher Cut

r559 bleibt im bestehenden Metric-Beam-DecisionOwner und in der bestehenden
Committed-Cross-Staff-Materialisierung. Es entsteht kein neuer produktiver Caller,
kein neuer Writer und kein neuer Pass.

Die historische `<= half beat`-Klasse bleibt unveraendert. Daneben wird nur eine
zweite Klasse zugelassen, wenn alle folgenden Bedingungen gleichzeitig gelten:

- gleiche Voice und gleicher metrischer Beat;
- genau zwei sichtbare, klingende, beamfaehige Ereignisse;
- realer Staff-Wechsel zwischen den beiden Ereignissen;
- lueckenlose Onset-Folge;
- die beiden Ereignisse fuellen den gesamten Beat exakt vom Beat-Anfang bis zum
  Beat-Ende;
- Typen sind `eighth` und `16th`;
- Dauern entsprechen exakt `3/4 + 1/4` oder spiegelbildlich `1/4 + 3/4` des
  `beat_unit`.

Die Dauerpruefung ist divisions-unabhaengig und enthaelt keine Etueden-, Takt-,
Positions-, Pitch- oder `96/72/24`-Sonderidentitaet.

Die bereits bestehende `structural_metric_beam_hierarchy_plan(..., 2)`-Entscheidung
bleibt alleiniger Beam-State-Planer. Fuer diese Zweiergruppe erzeugt sie nur den
gemeinsamen primaeren `begin/end`-Balken. Es wird keine neue Beam-Level-2- oder
Hook-Logik eingefuehrt.

## Keine Reparatur-Schicht

Dieser Take fuehrt ausdruecklich keine Reparatur- oder Recovery-Schicht ein:

- kein nachgeschalteter Beam-Repair;
- kein Render-Patch ausserhalb des vorhandenen Owners;
- kein zweiter Beam-Writer;
- keine neue WriterReachability;
- keine Takt- oder Etueden-Sonderregel;
- keine nachtraegliche Korrektur eines bereits falsch entschiedenen Kandidaten.

Die fehlende regulare Rhythmusklasse wird unmittelbar im bestehenden fachlichen
Cross-Staff-Selector zugelassen und danach vom bereits vorhandenen gemeinsamen
BeamPlan materialisiert.

## Regression

Der bereits writer-erreichende Cross-Staff-Regressionstest wird innerhalb seines
bestehenden Test-Callers erweitert. Er prueft zusaetzlich:

1. `3/4 + 1/4`, Staff 2 -> Staff 1: genau eine Cross-Staff-Gruppe, L1
   `begin/end`, kein erfundenes L2;
2. unmittelbarer zweiter Lauf: byteidentisch, keine weitere Gruppe geschrieben;
3. spiegelbildlich `1/4 + 3/4`, Staff 1 -> Staff 2: gleiche Regel;
4. derselbe `3/4 + 1/4`-Rhythmus auf demselben Staff: ausserhalb dieses Owners;
5. falsches Dauernverhaeltnis trotz Staff-Wechsel: ausserhalb der neuen Klasse;
6. der bereits vorhandene Vier-Sechzehntel-Cross-Staff-Fall bleibt unveraendert.

## Korpus-Evidenz vor dem Take

Eine read-only Auswertung der zehn uebergebenen r556-Render-MusicXML-Dateien
reproduziert 23 bestehende historische Cross-Staff-Voice/Beat-Gruppen. Die neue
Klasse qualifiziert genau einen zusaetzlichen Fall:

- Etuede 3, Takt 19, Beat 2: Start 96, Dauern 72 + 24, Staff 2 -> 1,
  Typen eighth + 16th, bisher ohne Beam-Tags.

Kein zweiter zusaetzlicher Kandidat erscheint im uebergebenen 10er-Korpus.

## Invarianten

Unveraendert bleiben:

- Pitch;
- Onset und Duration;
- Staff und Voice;
- RestExistence und RestVisibility;
- Clef- und Registerentscheidungen;
- Master-MusicXML;
- Pass-Reihenfolge;
- StructuralWriteAuthority und Writer-Root;
- bestehende Cross-Staff-Klasse;
- Schwellen, Gewichte, Layout und Renderer-Ownership;
- Security-, Build- und Take-Gates.

Erlaubtes Delta sind ausschliesslich die bereits vorhandenen Beam-L1-Tags auf den
zwei qualifizierten Render-Noten der neuen metrischen Klasse.

## Abnahme

Der reale Windows-Take muss fail-closed durch PRE-TAKE-DIFF-GATE, Build, Strict
Clippy und Tests laufen. Danach zuerst Etuede 3 pruefen:

- Takt 19, Zaehlzeit 2 besitzt eine gemeinsame Cross-Staff-Balkengeste;
- die benachbarten Zaehlzeiten bleiben unveraendert;
- kein Pitch-/Onset-/Duration-/Staff-/Voice-Delta.

Danach Vollkorpus. Besonders gegenlesen:

- Etuede 8, Takt 45 als bestehender Mixed-Duration-Cross-Staff-Comparator;
- Etuede 8, Takt 26 als bestehende Same-Staff-Clef-Grenzen-Ausnahme;
- keine neuen Beam-Tags auf Pausen oder versteckten Noten;
- keine neue L2-Autoritaet fuer die Zielgruppe.
