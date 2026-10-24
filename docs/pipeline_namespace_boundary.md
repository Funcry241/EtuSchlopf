# Pipeline-Namensraumgrenze v0.10.1521

## Ausgangslage

`src/main.rs` war zugleich Binary-Einstieg, Modulwurzel, Report-Hilfsfläche und lexikalischer Eigentümer aller 46 Pipeline-Shards. Durch `include!` wurden sämtliche Shard-Elemente in den Crate-Root eingefügt.

Das war kompilierbar, aber architektonisch teuer:

- mehr als zweitausend Pipeline-Definitionen und Methoden wurden im Root-Kontext kompiliert;
- der Binary-Einstieg war in einem fachfremden Shard versteckt;
- Pipeline-Implementierung und Crate-API waren nicht unterscheidbar;
- Supportmodule griffen über ihre Elternbeziehung unbemerkt auf private Root-Symbole zu.

## Erkenntnisse aus den verworfenen Kandidaten

v0.10.1518 verschob die 46 Shards erstmals in `src/pipeline.rs`. Der reale Strict-Clippy-Test stoppte korrekt mit 70 Compile-Fehlern. Dieser Befund zeigte drei bislang verdeckte Eigentumsbeziehungen:

- `notation` nutzt einen privaten Pipeline-Helfer;
- `render` und sein Silent-Coverage-Writer nutzen private Pipeline-Timeline- und Eventtypen;
- `wav_truth_articulation` nutzt private Pipeline-Audittypen und Analysehelfer.

Ein globales `pub use pipeline::*` oder eine massenhafte Umstellung auf `pub(crate)` hätte die neue Grenze wieder aufgehoben.

v0.10.1519 modellierte die Eigentumsbeziehungen korrekt, war im Change-Control jedoch fälschlich von der nie installierten v0.10.1518 abhängig.

v0.10.1520 war korrekt direkt von v0.10.1517 abgeleitet. Der reale Gate-Test stoppte mit genau einem Fehler, bevor der untergeordnete Render-Writer geladen wurde: Da `render.rs` selbst per `#[path = "render.rs"]` unter `pipeline` eingebunden wird, löste Rust dessen unannotiertes Untermodul relativ zum neuen Modulkontext auf und suchte am falschen Dateipfad.

## Direkte geschlossene Grenze v0.10.1521

v0.10.1521 ist vollständig und direkt von der real grünen v0.10.1517 abgeleitet. `src/pipeline.rs` ist Eigentümer der drei tatsächlich pipelinegebundenen Supportmodule und der unveränderten Shardfolge:

```text
pipeline
  ├─ notation
  ├─ render
  │   └─ silent_voice_visible_measure_coverage_render_writer
  ├─ wav_truth_articulation
  └─ pipeline_shards/*
```

Nachkommen dürfen in Rust private Elemente ihrer Vorfahren verwenden. Die bisher implizite Root-Kopplung wird dadurch als echte Eigentumsbeziehung modelliert, ohne interne Pipeline-Typen zu exportieren.

Der Render-Writer wird an seinem unveränderten Dateispeicherort explizit gebunden:

```rust
#[path = "render/silent_voice_visible_measure_coverage_render_writer.rs"]
mod silent_voice_visible_measure_coverage_render_writer;
```

Die verbleibende Sibling-Abhängigkeit der `structural_write_authority` auf das funktionale Master-Respelling wird durch exakt ein `pub(super)`-Symbol geschlossen. Seine Sichtbarkeit endet beim Elternmodul; es entsteht keine öffentliche oder crate-weite Pipeline-API.

## Warum noch keine 46 Einzelmodule?

Die Shards teilen heute zahlreiche private Typen und Funktionen. Eine sofortige Umwandlung in 46 Rust-Module würde gleichzeitig Sichtbarkeit, Imports, Eigentum und fachliche Reihenfolge verändern. Das wäre ein unkontrollierter Mischumbau.

Der gemeinsame Pipeline-Namensraum schafft zuerst eine stabile Außengrenze. Künftige Schritte können innerhalb dieses Eigentümers jeweils einen zusammenhängenden Fachblock mit expliziten Eingangs- und Ausgangstypen herauslösen.

## Verbindliche Invarianten

- direkte Ableitung von v0.10.1517;
- unveränderte Include-Reihenfolge;
- unverändertes `real_main()`;
- keine neue Writer-Erreichbarkeit;
- keine Musikmutation durch die Grenzbildung;
- kein Sicherheits-, Launcher- oder Bootstrap-Delta;
- kein pauschaler Pipeline-Re-Export;
- genau ein namentlich begrenztes `pub(super)`-Brückentor zur bestehenden Structural Write Authority;
- expliziter Quellpfad für das verschobene Render-Untermodul;
- Prozessausgang über `std::process::ExitCode`, nicht durch tiefes `process::exit`;
- Reportformatierung getrennt vom Binary-Einstieg.
