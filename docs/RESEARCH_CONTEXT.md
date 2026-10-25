# Forschungsbezüge und ihre tatsächliche Rolle

Bibliografische Prüfung: 25.10.2026. Hier stehen Bezüge, deren Rolle am aktuellen
Quellbaum nachvollziehbar ist. Eine Referenz ist weder eine pauschale Bestätigung
von EtuSchlopf noch eine empirische Kalibrierung seiner Heuristiken.

## 1. Integrierte Modellgrundlage: Basic Pitch

Rachel M. Bittner et al. (2022):
*A Lightweight Instrument-Agnostic Model for Polyphonic Note Transcription and Multipitch Estimation*.
ICASSP 2022; [Autorenversion auf arXiv](https://arxiv.org/abs/2203.09893).

Die Arbeit beschreibt ein kompaktes Modell für mehrstimmige Notentranskription
mit gemeinsam geschätzten Onsets, Tonhöhen und Notenaktivierungen.

**Nachweisbare Rolle:** Der separate Rust-/ONNX-Sensor unter
[tools/wav_pitch_external_sensor_audit](../tools/wav_pitch_external_sensor_audit/)
verwendet das Basic-Pitch-Modell als unabhängige WAV-Beobachtung. Modellbindung,
Dekodierung, Freeze und Auswertung werden im
[Integrationsvertrag](wav_pitch_external_sensor_rust_basic_pitch_managed_tree_residue_recovery_r166.md)
beschrieben; die Komponente besitzt eigene Voraussetzungen.

**Grenze:** Dies ist Sensor-/Benchmark-Evidenz, keine übertragene Befugnis zur
Notenwahl oder zum Schreiben. EtuSchlopf ist damit nicht als allgemeiner Ersatz
für automatische Musiktranskription validiert. Das Projekt übernimmt nicht die
Benchmark-Ergebnisse des Papers als eigene Messwerte.

## 2. Aktueller Forschungskontext: Handspanne und Klaviatur

Craig Turner et al. (2026): Studie zu einer 6-Zoll-Oktavklaviatur, Muskelaktivität
und Fingerhaltung bei Expertinnen und Experten mit kleineren Handspannen.
Applied Ergonomics 133, 104709.
[DOI 10.1016/j.apergo.2025.104709](https://doi.org/10.1016/j.apergo.2025.104709),
[Abstract und Publikationsdaten](https://pubmed.ncbi.nlm.nih.gov/41385802/).
Online veröffentlicht am 12.12.2025, Heftveröffentlichung Mai 2026.

Die Studie untersucht unterschiedliche Belastungs- und Haltungsmerkmale in
Abhängigkeit von Hand- und Klaviaturgröße. Sie stützt die Notwendigkeit,
Spielanforderung, Instrumentengeometrie und individuelle Fähigkeit zu trennen.

**Nachweisbare Rolle:** Die DOI ist bereits im
[Referenz-Annahmenmodell](../src/audit_only/pianistic_reference_assumption_model.rs)
und im [Kalibrierungsprofil-Vertrag](../src/audit_only/pianistic_empirical_calibration_profile_contract.rs)
als `PublishedContextOnly` hinterlegt. Die Literaturzuordnung wird hier anhand
der vorhandenen Code-Referenzen und der Primärpublikation überprüft.

**Grenze:** Aus der Studie stammen keine automatisch gültigen Schwellenwerte
für EtuSchlopf. Sie belegt keine Spielbarkeit seiner Ausgaben und ersetzt kein
konkretes empirisches Pianisten-/Instrumentenprofil.

## 3. Weiterer ausdrücklich referenzierter Kontext

Ju-Yang Chi et al. (2021): Beobachtungsstudie über Handspanne, verschiedene
Klaviaturgrößen und EMG-Aktivität beim Klavierspiel. Applied Ergonomics 97, 103518.
[DOI 10.1016/j.apergo.2021.103518](https://doi.org/10.1016/j.apergo.2021.103518),
[Publikation und Abstract](https://pubmed.ncbi.nlm.nih.gov/34246074/).

Auch diese Arbeit ist in den genannten Modulen als Kontext hinterlegt. Ihre
Ergebnisse betreffen das Verhältnis individueller Handgeometrie und Instrument,
nicht einen universellen Grenzwert für jeden Akkord oder jede Passage. Die
projektinternen Referenzannahmen bleiben als Annahmen ausgewiesen.

## Was daraus nicht folgt

Diese Bezüge zeigen reale Schnittstellen zu offenen Forschungsfragen. Sie
begründen weder einen allgemeinen Stand-der-Technik-Anspruch noch eine
wissenschaftliche Validierung der gesamten Pipeline. Forschung zu Transkription,
Motorik und Spielbarkeit liefert unterschiedliche Arten von Evidenz; sie darf
nicht miteinander oder mit einem grünen Softwarelauf gleichgesetzt werden.

Die [implementierten Grundlagen](IMPLEMENTED_FOUNDATIONS.md) zeigen getrennt,
welche Optimierungen, symbolischen Bedingungen und Herkunftsprüfungen im Code
stehen. Aktuelle Forschung ist hier eine überprüfte Einordnung mit expliziter
Rollenangabe, keine rückwirkend erfundene Entstehungsgeschichte.
