# Gould Legacy Shadow Retirement + Canonical Readback SuperCut

Version: v0.10.1764 r01

## Ziel

Der produktive Gould-Metric-Beam-Pfad soll nicht laenger von einer zweiten historischen Implementierung bestaetigt werden muessen.

Vorher:

```text
Source Render-XML
   |\
   | +--> Legacy 16tel-8tel-16tel Writer --> isolierte Referenzkopie
   |                                      |
   +--> zentraler Detector/Token/Writer --+--> Byte-Paritaetsveto --> Commit
```

Nachher:

```text
Source Render-XML
   |
   +--> zentraler Detector
          |
          v
      AuthorizationInput
          |
          v
      move-only Token
          |
          v
      Authorized Writer
          |
          v
      Canonical Readback
          |
          v
      move-only Verified Candidate
          |
          v
      Render Commit
```

## Entfernt

- `gould_metric_beam_shadow_audit.rs`
- `GouldWriteIntentShadowParityAudit`
- isolierte Legacy-Referenzkopie im Metric-Final-Notation-Orchestrator
- `MetricTerminalSixteenthEighthSixteenthBeamAudit`
- `apply_metric_terminal_sixteenth_eighth_sixteenth_beam_write()`
- exakter Candidate-vs-Legacy-Bytevergleich
- alle Gould-spezifischen Runtime-Shadow-Symbole

## Kanonischer Readback

Der zentrale Candidate wird nicht einfach nach `beam_only_delta` freigegeben. Der neue direkte Readback bindet die bereits autorisierten Source-Zielbereiche an Dokument-Notenordinale und verlangt:

1. Source und Candidate sind parsebar.
2. Der beamlose Gesamtdokument-Payload ist identisch.
3. Die Anzahl der `<note>`-Elemente ist identisch.
4. Jeder gebundene Zielbereich muss exakt einer Source-Note entsprechen.
5. Kein Zielordinal darf doppelt autorisiert sein.
6. Jede Nicht-Zielnote bleibt bytegleich.
7. Jede Zielnote bleibt nach Entfernen aller Beam-Tags semantisch/textuell gleich.
8. Jede Zielnote besitzt danach exakt einen Beam-Tag.
9. Der primaere Beam-Zustand entspricht exakt dem autorisierten Gould-Plan (`begin`, `continue`, `end`).
10. Nur bei vollstaendig gruenem Readback wird der move-only `VerifiedGouldMetricBeamCandidate` erzeugt.

## Unveraendert

- `detect_gould_metric_beam_plan()` bleibt einziger Gould-Detector.
- Der Detector bleibt identity-free.
- Der Token bleibt move-only und dokument-/source-/locator-/state-/invariant-gebunden.
- `apply_authorized_gould_metric_beam_write()` bleibt einziger Gould-Mutationsentrypoint.
- `commit_verified_gould_metric_beam_candidate()` akzeptiert weiterhin nur einen typisierten Verified Candidate.
- Pitch, Onset, Dauer, Staff und Voice duerfen durch den Gould-Pfad nicht veraendert werden.
- Keine Etueden-, Takt-, Pitch-, Staff- oder Projektselektoren.
- Keine Gate-Lockerung.

## Erwartete Abnahme

Gegen v0.10.1763/r02:

- Build/Strict/Launcher gruen.
- Gould: fuer Etuede 1 weiterhin 2 Candidate-Gruppen, 2 Tokens, 6 geschriebene Beam-Zeilen.
- `CanonicalCandidateVerified ja`.
- Master-/Render-MusicXML bis auf Software-Versionszeile identisch.
- PDF 3/3 Seiten pixelgleich.
- Kein aktiver Rust-Treffer auf die entfernten Legacy-/Shadow-Symbole.
