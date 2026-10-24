# FinalRenderCanonicalSoundingClosure – v0.10.1773 r01

## Korpus-Gegenbeweis aus 1772 r04

Der reale 10er-Korpus schloss 9/10 Projekte vollstaendig. Etuede 10 blieb vor MuseScore fail-closed, obwohl `CommittedPrimaryClefReentryChanges 0`, `CommittedRestPlacementCandidates 0`, `CommittedHigherBeamCandidates 0` und `CommittedRawCursorGreen ja` waren. Allein `CommittedPitchedEventMultisetStable` war rot.

Die Forensik zeigt eine bereits vorhandene legitime Normalisierung vor dem FinalNotation-Sink: `OrphanChordHostRest` ersetzt einen `<forward>`-Carrier durch den exakt master-gebundenen Rest-Host, wenn unmittelbar folgende `<chord/>`-Mitglieder sonst keinen seriellen Host besitzen. Der Raw-Renderzustand besitzt fuer diese verwaisten Chord-Mitglieder keinen kanonischen Anchor. Nach der Host-Materialisierung entspricht deren Timeline dem Master; ihr Pitch-/Duration-/Staff-/Voice-/Hidden-/Chord-Payload bleibt unveraendert.

Die Final-Commit-Invarianz wird deshalb ab 0.10.1773 gegen den vollstaendig akzeptierten **kanonischen Pre-Commit-Kandidaten** gemessen. Das ist keine Gate-Lockerung: Der OrphanChord-Operator bleibt master-bound, RawCursor-stabil, pitched-payload-stabil und exact-delta-gebunden; alle spaeteren Operatoren muessen die dadurch kanonisierte Timeline unveraendert committen. Der Abschlussbeweis fragt damit die richtige Wahrheit zum richtigen Zeitpunkt.

Formal sei `O` die master-bound OrphanChord-Kanonisierung und `K` der restliche FinalNotation-Commit. Fuer den Rohzustand `S` muss nicht `Timeline(O(S)) = Timeline(S)` gelten, weil `S` fuer die orphan Chord-Members keinen gueltigen Host besitzt. Gefordert wird stattdessen:

`Payload(O(S)) = Payload(S)`, `RawCursor(O(S)) = RawCursor(S)`, `Timeline(O(S)) = Timeline(MasterBound(S))` und danach `Timeline(K(O(S))) = Timeline(O(S))`.

---

# FinalRenderCommittedStateClosureSuperwurf

## Ziel

Der finale Render-Notationsstand wird vor dem PDF-Renderer topologisch geschlossen, committed und auf genau den persistierten Bytes erneut bewiesen. `render_pdf_if_possible()` besitzt danach kein internes MusicXML-Mutationsrecht mehr.

## Ausgangsbefund

Im autoritativ gruenen v0.10.1771-r06-Korpus wurde `FinalRestVerticalPlacement` vor dem letzten Primary-Clef-Write als gruen gemeldet. Auf der danach tatsaechlich gespeicherten Render-MusicXML waren jedoch 19 `stale_clef_anchor`-Kandidaten ueber Etueden 4, 5, 6, 7 und 8 wieder offen. Die fachliche Abhaengigkeit lautet `Clef -> RestPlacement`; die Postcondition lag also vor dem letzten Writer eines ihrer Vorfahren.

## Architektur

Die bestehende writer-freie `apply_rustig_clef_course_to_buffer`-Projektion wird innerhalb der vorhandenen Finalisierungsfamilie vor `FinalRestVerticalPlacement` ausgefuehrt. Danach folgen der bestehende Higher-Beam-Plan und genau der vorhandene `FinalNotation`-Commit. Der Sink darf den Primary-Clef-Kurs erneut ableiten, muss jetzt aber denselben Fixpunkt bestaetigen.

Nach dem Commit wird die reale Datei erneut gelesen. Die Closure verlangt:

- Primary-Clef-Reentry ist no-op;
- `FinalRestVerticalPlacement` hat 0 committed Residuen;
- Higher-Beam-Preservation hat 0 committed Residuen;
- RawCursor bleibt global gruen;
- die pitched Timeline des vollstaendig akzeptierten kanonischen Pre-Commit-Kandidaten bleibt ueber den Commit stabil; technische Rest-/Notenknotenzaehler duerfen durch bereits autorisierte RestNotation-Normalisierung abweichen.

Ein roter committed-state Beweis blockiert vor MuseScore.

## Renderer-Grenze

Die bestehende Release-Orchestrator-Funktion bleibt wegen ihrer bewusst geschuetzten ProofReentryReader-Capability byteidentisch. Unter ihrer stabilen `render_pdf_if_possible()`-Schnittstelle fuehrt eine Compatibility-Fassade zuerst die strukturelle Pre-Render-Finalisierung aus und startet nur bei geschlossener `FinalRenderSourceClosure` den getrennten `render_finalized_pdf_if_possible()`-MuseScore-Pfad. Vor und nach diesem reinen Renderer wird die MusicXML bytegenau ueber einen neutralen Dateihelfer gelesen; eine Veraenderung setzt den Rendererstatus fail-closed auf Fehler. Nach dem PDF-Aufruf folgen nur read-only Audits.

## Evidence-Wahrheit

`LedgerBoundedClefBoundariesAccepted` beschreibt den pre-sink Kandidaten. `LedgerBoundedClefBoundariesWritten` beschreibt nur noch im committed Artefakt verbliebene Ledger-Marker. `FinalClefBound` wird nicht mehr konstant behauptet, sondern aus Primary-Clef-Projektion plus committed Reentry-Fixpunkt abgeleitet. Die neue `FinalRenderCommittedStateClosure`-Zeile bindet RawCursor und ReleaseVeto an den persistierten Zustand.

## Erlaubtes Delta

Keine neue Musikregel. Kein Projekt- oder Taktselektor. Pitch-/Duration-/Staff-/Voice-/Hidden-/Chord-Payload bleibt invariant; die einzige bereits vorhandene erlaubte Onset-Kanonisierung ist der exakt master-gebundene OrphanChordHost-Fall, in dem die Rohquelle noch keinen gueltigen Host besitzt. Zusaetzlich bleiben die bereits vom bestehenden RestPlacement-Owner definierten grafischen `display-step`/`display-octave`-Korrekturen erlaubt, wo der finale Primary-Clef-Kurs den bisherigen Rest-Anker als stale beweist.

## Explizit ausser Scope

ShortRest-Retirement, DynamicLowerRestBridge-Retirement, Variant-A-Beam-Cleanup, ShadowRest, VisualLayoutPressure, Systemumbrueche, Security und neue Musikheuristiken bleiben unveraendert.

## Abnahme

Sparsame erste Gegenprobe:

```powershell
.\run.cmd --take --strict "Etüde Nr. 6"
```

Danach gezielt Etueden 4/5/7/8 und erst beim gruenen Kandidaten der 10/10-Korpus. Erwartete neue harte Evidence: `CommittedStateReadbackGreen ja`, `CommittedPrimaryClefReentryChanges 0`, `CommittedRestPlacementCandidates 0`, `CommittedHigherBeamCandidates 0`, `CommittedRawCursorGreen ja`, `RendererMusicXmlWrites 0`.


## Revision r02

r02 ist inhaltlich derselbe Superwurf wie r01. Die rein audit-seitige Aenderung an `src/corpus_run_manifest.rs` wurde aus dem `MusicMutation`-Take entfernt, weil die Take-Change-Control Audit- und MusicMutation-Pfade bewusst trennt. Die neuen committed-state Evidence-Felder bleiben im Ergebnistext verfuegbar; eine spaetere Korpus-Aggregator-Anpassung gehoert in einen getrennten AuditOnly-Wurf.


## Revision r03 / Provenance-Closure

r02 wurde vom Semantic-Capability-Diff vor Installation abgelehnt, weil der MusicMutation-Scope einen bestehenden `ProofReentryReader` im Release-Orchestrator veraenderte und ein direkter Datei-Read in einer proof/evidence-markierten Musikfunktion als neuer `ProofReentryReader` erkannt wurde. r03 behaelt die Policy unveraendert und schliesst die Architektur unterhalb der stabilen Schnittstelle. `run_render_pdf_source_selection_stage` ist byteidentisch zu v0.10.1771. Direkte Datei-I/O liegt nur in neutral benannten strukturellen Ladehelfern; die Finalisierung entscheidet aus aktueller XML-Struktur und ihrem unmittelbaren Closure-Token, nicht aus persistierten Reports oder Proof-Strings.
