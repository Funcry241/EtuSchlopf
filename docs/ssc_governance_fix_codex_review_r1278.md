# r1278: Fix der Codex-Befunde zu r1277 (Vertrag v4, Teil B)

Benannte `KodexOnly`-Revision `SscK2GovernanceRevisionKodex` auf accepted r1277. Laufzeit, Musik und SSC-Bilanz bleiben unverändert. Die Befunde stammen aus `Codex_Befund_r1277.md`.

| Befund | Umsetzung | Negativtest (Gegenbeispiel aus dem Befund) |
|---|---|---|
| F1 Alias/Re-Export | Geschlossene Syntax für die vier Beweisdateien: std/core/alloc nur als ausgeschriebene Pfade aus fester Liste (`std::io` nur `Read`), kein `use … as`, kein `pub use`, nur gelistete Makros, kein `extern`, kein `mod x;`, kein `#[path]`. Identity-Funktionen nur aus exakter Liste (keine `ssc_*`-Präfixzulassung); jede genutzte muss ein echtes `fn` im Identity-Modul sein. Das Identity-Modul selbst liest keine Umgebung, Dateien, Zeit oder Prozesse und re-exportiert nur eigene Untermodule. | `use std as ambient`, `pub(crate) use std::env as ssc_external`, fremde Makros, `extern "C"`, `mod hidden;` |
| F2 Observer | Signatur exakt V2 oder exakt V4 (Containment); V4 nur zusammen mit der eingefrorenen V4-Oberfläche. | `SscAmbientInput { pub accept: bool }` als Parameter, generischer Parameter |
| F3 Owner-Zweck | Für jeden Descriptor (75, ohne Genesis) friert der Parent Receipt-Art und Zweckklassen ein. Zeilen müssen innerhalb der Klassen liegen, die Invariante des Envelope-Typs benennen, Lane nur für F5/F7/F9 nutzen und Content nur deklarieren, wo die Klasse ihn erlaubt. Der Descriptor wird gegen die Parent-Tabelle geprüft, nicht nur gegen die Identity-Datei des Kindes. | Beam-Owner mit `pitch.step` und `content:`-Text; Layout-Owner mit Clef/Beam; Barline mit `remove`; Lane für Nicht-Lane-Owner |
| F4 Wiring | `wiring=<pfad>::<funktion>`, nur `commit`/`commit_owner_receipts` (Master) und `commit` (Render). Verdikt-Tokens sind nur innerhalb dieser Funktionen erlaubt. Eine spätere Änderung einer verdrahteten Funktion braucht `govern=verdict_wiring`. | Verdikt in einem Helper derselben Datei |
| F5 Surface | Nur eine nackte Einheitsvariante an `SscProofFamily`, `SscEvidenceIntegrityFailure` oder `SscInvariantFailureKind`. | `variant=SscEventBinding|pub ambient: &'event str`, Varianten mit Nutzlast, fremde Enums |
| F6 #57 | Der ganze `impl BlockTransformationReceipt` ist gebunden (Mint, Post-Split, Block-Übergang). | Entfernter Fingerprint-Filter in `split_realized_post_document` |
| H1 Musik | Unverändert: Neutralität wird bei jeder Abnahme am Export geprüft (20 XML bytegleich, PDF gerendert, Lineage). `declared` ersetzt keine ausdrückliche Nutzerfreigabe. | – |
| H2 Bericht | Die `govern=`- und `music_effect=`-Zeilen stehen wörtlich in `take_acceptance_candidate_intent.txt` jedes Assistant-Exports (dauerhafte Abnahmeprojektion). | – |

**Zusätzlich:**

- Die V4-Containment-Oberfläche (A2) ist vorab eingefroren (`ssc_future_type_surface_v4.txt`). Damit braucht Take 2 keine Surface-Erweiterung.
- Die Content-Regel ist präzisiert: `note` zählt nur dann als Inhaltsänderung, wenn es als ganzes Element eingefügt oder entfernt wird. Gezählt werden außerdem `divisions`, `time`, `chord`, `tie` und `rest`.

**Grenze von F3:** Die Zweckklassen sind grob. Sie wurden aus Population-/Exclusion-Notizen, Owner-Signatur und Owner-Namen abgeleitet, nicht aus einer Zeile-für-Zeile-Lektüre jedes Owners. Sie sind eine Obergrenze. Die Envelope-Zeilen der Verdikt-Takes bleiben enger und werden pro Take adversarial geprüft. Dokumentweite Owner (#52, #57, #58) tragen alle Klassen. Das ist ehrlich als breit deklariert, nicht verengt.
