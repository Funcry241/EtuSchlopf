# MetricFinalNotationLargeXmlMovePerformanceClosure r196

## Ausgangspunkt

r195 ist der autoritativ gruene Stand. Der normale Clippy-Sockel liegt bei 98 Meldungen und `architectural_budget=0`.

## Befund

In `pipeline_orchestrator_metric_final_notation_stage.rs` existierten zwei von Deep-Clippy als redundant erkannte `String::clone()`-Operationen auf grossen MusicXML-Payloads:

1. `scorewide_metric_beam_prewrite_shadow_xml` wurde unmittelbar in `structural_lower_beam_source_xml` geklont, obwohl der Ursprungswert danach keinen weiteren Leser besitzt.
2. `render_xml_candidate: Option<String>` wurde nach seinen read-only Vorpruefungen fuer `committed_source_xml` geklont, obwohl die Option danach keinen weiteren Leser besitzt.

## Aenderung

Beide Ownership-Uebergaenge konsumieren nun den bereits besessenen Wert. Die Bytes, Kontrollflussbedingungen und nachgelagerten Leser bleiben identisch.

Bewusst nicht veraendert wird `committed_master_xml.clone()`, weil `committed_master_xml` im Render-Quellpfad spaeter weiterhin fuer den Shadow-Master benoetigt wird.

## Sicherheits- und Musikgrenze

Keine architektur-gepinnte Datei wird veraendert. Es gibt keine Signatur-, Writer-, Renderer-, Security-, Threshold-, Scoring- oder Musiklogikaenderung.

## Abnahme

- normaler Clippy `98 = 0 + 98`
- `RunSeverity dark_green`
- Master-MusicXML byte-identisch zu r195
- Render-MusicXML byte-identisch zu r195
