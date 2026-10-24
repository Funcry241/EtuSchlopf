# r184 – WavObservationSignalLegacyBoundarySplitPackagingRecovery

## Problem

`src/audit_only/wav_observation_legacy_oracle.rs` vereinte zwei heute getrennte
Verantwortungen:

- den bewusst eingefrorenen FLP-Legacy-Oracle und seine Qualifikation;
- den neutralen RIFF/WAVE-Signaldecoder, den aktuelle WAV-Evidenzmodule benutzen.

Dadurch mussten aktuelle Audio-Proben ihren Decoder und den Typ
`DecodedWavSignalAudit` aus einer als `legacy_oracle` bezeichneten Grenze importieren,
obwohl die Decodierung selbst keine Legacy-Wahrheit besitzt.

## Entscheidung

Der neutrale Signaldecoder wird unveraendert nach
`src/audit_only/wav_signal_decode.rs` verschoben.

Verschoben werden ausschliesslich:

- `DecodedWavSignalAudit`;
- `load_decoded_wav_signal_for_score_title`;
- der interne WAV-Dateidecoder;
- die formatneutrale Einzelwert-Decodierung, die auch die WAV-Qualifikation nutzt.

Die drei bisherigen Decoderfunktionen behalten ihren r182-Funktionskoerper. Die
Sichtbarkeit wird auf die gemeinsame `audit_only`-Grenze verengt.

Der eingefrorene FLP-Oracle, seine Eventprojektion, PPQ-Skalierung, Fingerprints,
Qualifikationsgruende und Berichtsschnittstelle bleiben im Legacy-Modul. Der
Qualifikationsbericht behaelt seinen bisherigen Namen und seine bisherigen Felder.

## Verbraucher

Aktuelle WAV-Evidenz bezieht das Signal nun aus der neutralen Grenze:

- `wav_attack_pulse_metric.rs`;
- `wav_pitch_harmonic_evidence.rs`;
- `wav_pitch_external_sensor_benchmark.rs`.

Legacy-FLP-Auswertung wird weiterhin explizit aus
`wav_observation_legacy_oracle.rs` bezogen. Damit ist im Importgraph sichtbar, wann
tatsaechlich historische Oracle-Evidenz verwendet wird.

## Regression

Die bestehende Abschlusswache
`tests/wav_pitch_rescue_selector_retirement_closure.rs` wird erweitert. Sie bindet:

- die Existenz des neutralen WAV-Decoders;
- die Abwesenheit des Signaldecoder-Owners aus dem Legacy-Modul;
- die Abwesenheit von FLP-Legacy-Pitch-APIs aus dem neutralen Decoder.

Der neue Decoder besitzt zusaetzlich einen kleinen PCM16-Stereo-Test fuer SampleRate,
Kanaele, Frame-Domaene und decodierte Samplezahl.

## Nicht geaendert

Keine Schwelle, Gewichtung, WAV-Metrik, Pitch-Hypothese, FLP-Evaluation,
Source-Event-Autoritaet, MusicXML-/Render-/PDF-Ausgabe, Writer-Reichweite,
Musikmutation, Release- oder Exit-Semantik wird geaendert.

## ChangeClass

`RefactorOnly`: technische Trennung einer neutralen Audio-Decodierung von einem
historischen Audit-Oracle bei unveraenderter fachlicher Berechnung.


## Packaging-Recovery

Der zuvor gelieferte r183 wurde **vor Installation** vom bestehenden
Take-Tree-Integrity-Gate korrekt fail-closed verworfen, weil das ZIP versehentlich
einen nicht verwalteten Root-Pfad `Cargo.lock` enthielt. r184 verwendet denselben
fachlichen Decoder-Split, entfernt ausschließlich diesen unzulässigen Paketpfad und
trägt wegen möglicher Same-Revision-Ambiguität eine neue eindeutige Revision.

Zusätzlich wird das bereits vorhandene Assistenten-Gedächtnis um genau dieses
bekannte Verpackungsfehlerbild ergänzt. Das verändert keine Laufzeit-, Musik-,
Writer-, Gate- oder Capability-Semantik; es macht die bestehende Paketregel für
spätere Threads unmittelbar sichtbar.
