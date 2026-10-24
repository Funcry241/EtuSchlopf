# FinalBeamLostBeginRecoveryAuthorityRetirement

## Version

`v0.10.1780 r01`

## Ausgangspunkt

Autoritative Basis ist `v0.10.1779 r02 BeatScopedBeamDecisionScopeMaterializationClosure`.
Der 1779-Vollkorpus ist 10/10 dark_green; gegen die letzte autoritative 1776-Referenz
sind 20/20 Master-/Render-MusicXML semantisch identisch und 57/57 PDF-Seiten
pixelidentisch. `FinalBeamLostBegin` meldet im 1779-Korpus 0 Kandidaten und 0 Writes.

Runtime-Null allein ist kein Retirement-Beweis. Deshalb wurde die exakte historische
Damage-Domain statisch gegen alle verbleibenden Beam-Writer geprueft.

## Exakte historische Damage-Domain L

Der Selektor akzeptiert nur einen mastergebundenen Fall:

1. Die erste sichtbare Pitch-Note besitzt im Master mindestens einen Beam-Level und
   alle vorhandenen Levels stehen auf `begin`.
2. Dieselbe eindeutige Render-Note besitzt keinerlei Beam-Tags.
3. Staff/Voice/Onset/Duration/Pitch/Visibility und Timeline sind exakt gebunden.
4. Vor der Zielnote ist auf keinem betroffenen Level bereits eine Beam-Kette offen.
5. Die folgenden Master-Noten sind streng onset-kontiguierlich und schliessen alle
   offenen Levels ausschliesslich ueber `continue`/`end`.
6. Jede Folgenote ist im Render beam-semantisch exakt masteridentisch.
7. Innerhalb der Gruppe existiert kein bebeamter Rest.

Damit ist L nicht "Beam kaputt", sondern exakt:

```text
Master:  begin -> continue* -> end
Render:  none  -> continue* -> end
         ^ nur der Gruppenanfang ging verloren
```

## Statischer Unerreichbarkeitsbeweis nach v0.10.1779

Fuer Retirement muss gelten:

`Image(P_remaining) ∩ L = ∅`.

Die verbleibenden Writer wurden nach ihrer Materialisierungsform getrennt.

### Ganze Gruppen materialisierende Writer

`PartialBeatPostRest`, Variant-A nach 1775, `ScorewideMetricBeamContinuity`, Gould,
ExternalFeedback, CrossStaffVoiceLedger und der PartialBeat-3er-Finalizer schreiben
positive Beam-Gruppen als vollstaendige Plaene. Sie koennen nicht isoliert nur den
Gruppenanfang entfernen und alle Folgenoten masteridentisch stehen lassen.

### Spaete Sonderpfade

`LowerDualEdge` schreibt nur die zweite Note einer speziellen Rest-Gruppe; diese
Domain ist mit L inkompatibel, weil L bebeamte Reste innerhalb der Gruppe verbietet.
`FinalCompleteBeatPrimaryBeamContinuity` schreibt Level 1 ueber die komplette
kontiguierliche Beat-Gruppe. Der cursorneutrale Structural Writer besitzt einen
expliziten Note/Beam-Fingerprint-Erhaltungsbeweis. SilentCoverage und OrphanChordHost
veraendern Rest-/Forward-/Host-Struktur, nicht Beam-Tags sichtbarer Pitch-Noten.

### NotationPolish und BeatScopedBeaming

Nach der 1778-Trichotomie erhaelt NotationPolish eine vollstaendig gebundene fremde
Beam-Kette. Nach der 1779-Closure materialisiert BeatScopedBeaming seinen kompletten
DecisionScope: Wird ein Teilrun akzeptiert, werden nicht ausgewaehlte Kandidaten der
entschiedenen Gruppe explizit negativ; das erste Mitglied jedes akzeptierten Runs
wird jedoch zwingend `begin`. Daher kann BeatScoped zwar einen alten Prefix entfernen,
aber nicht den L-Zustand erzeugen, in dem der erste Render-Event beamlos ist und alle
Master-Folger unveraendert `continue/end` bleiben. Gibt es keinen positiven Run,
schreibt BeatScoped die Gruppe ueberhaupt nicht.

Damit ist L nach 1779 statisch unerreichbar.

## Architekturwechsel in v0.10.1780

Die produktive Recovery wird vollstaendig entfernt:

```text
vorher:
pre-commit candidate
-> FinalBeamLostBegin selector
-> BeamPlan fuer erste Note
-> autorisierte Beam-Mutation
-> Readback
-> Commit

nachher:
pre-commit candidate
-> keine LostBegin-Mutation
-> Commit
-> persistiertes Render-MusicXML neu lesen
-> derselbe mastergebundene LostBegin-Selektor read-only
-> Kandidaten == 0 oder fail closed
```

Der historische Report-Prefix `FinalBeamLostBeginRenderWriterStage` bleibt aus
Schema-/Security-Control-Kompatibilitaet erhalten, traegt aber jetzt explizit
`RecoveryEnabled nein` und einen committed-state Gate-Verdict. Die bestehenden Corpus-Manifest-Felder
`beam_lost_begin_candidates`, `beam_lost_begin_notes_written` und
`beam_lost_begin_candidate_rows_after` bleiben schema-kompatibel; `notes_written`
bleibt per Architektur 0. Kandidaten stammen jetzt vom committed Artefakt.

`CommittedStateReadbackGreen` verlangt zusaetzlich:

`CommittedBeamLostBeginCandidates == 0`.

## ChangeClass

`RefactorOnly`.

Der 1779-Korpus besitzt 0 produktive LostBegin-Writes, die Damage-Domain ist statisch
unerreichbar, und der Wurf entfernt Schreibrecht statt Musiklogik hinzuzufuegen.
Erwartet wird vollstaendige Output-Invarianz gegen 1779.

## Abnahme

Erste Stressprobe:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Erwartung:

- `FinalBeamLostBeginRenderWriterStage Candidates 0`
- `NotesWritten 0`
- `CommittedBeamLostBeginCandidates 0`
- `CommittedStateReadbackGreen ja`
- alle bisherigen committed Gates gruen
- MusicXML gegen 1779 invariant bis auf Version/Whitespace
- PDF pixelidentisch

Danach Vollkorpus 10/10 und direkter 1779↔1780-Outputvergleich.
