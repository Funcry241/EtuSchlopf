# r965 ArchitectureInventoryAssistantFastPathV1

## Ziel

r965 konserviert ausschliesslich den beim ersten realen AIC-1-Cold-Start gewonnenen Effizienzbefund fuer kuenftige Assistenten. AIC-1 selbst bleibt byteinhaltlich unveraendert; EtuSchlopf-Runtime, Musiklogik, Metrics, Export-Bindung und Excel-Erzeugung bleiben unveraendert.

Der neue advisory Assistant-Fast-Path liegt in:

```text
docs/assistant_memory/architecture_inventory_assistant_fast_path_v1.json
```

Profilversion: `AIF-1`.

## Konkreter Befund

Der erste praktische Cold-Start auf Physical Source r964 ergab 253 Rust-Dateien und 5.770 eindeutige qualifizierte Symbole. Der funktional korrekte erste Assistant-Weg las den Source fuer Symboluniversum und Call-/Namenssignale mehrfach. Der zusammengefuehrte Ein-Pass-Index reproduzierte dieselben 5.770 Inventurzeilen und dieselbe A/B/C/N/OFFEN-Verteilung, reduzierte aber die beobachtete Source-/Interpretationsarbeit deutlich.

Die Laufzeiten sind nur Beobachtung, kein Performance-Gate.

## AIF-1

AIF-1 legt nur die assistant-seitige Ausfuehrungsreihenfolge fest:

1. `assistant_context.json` einmal lesen.
2. Gebundenen physischen Source-Snapshot einmal aufloesen und seinen SHA-256 als Cache-Identitaet verwenden.
3. AIC-1-Scope einmal bestimmen.
4. In einem Rust-Syntax-Pass gleichzeitig Symboluniversum und repositoryweiten Token-/Call-Index bilden.
5. Nur mechanische Repository-Fakten bei identischem Source-Snapshot-SHA wiederverwenden.
6. KI-Interpretation niemals als Repository-Wahrheit cachen; sie wird ueber der vollstaendigen aktuellen Symbolpopulation ausgefuehrt.
7. Native Kennzahlen nur aus einem gebundenen CMS-Snapshot lesen; bei fehlendem Snapshot nichts aus Logs rekonstruieren.
8. Nach finaler Klassifikation genau eine AIC-1-Arbeitsmappe rendern und nur deren Struktur pruefen.

## Harte Grenzen

AIF-1 definiert nicht, welche Symbole existieren und wie A/B/C/N/OFFEN zu vergeben ist. Diese Semantik bleibt ausschliesslich AIC-1 plus aktueller KI-Interpretation.

AIF-1 fuehrt insbesondere nicht ein:

- keinen Rust-Symbolscanner in EtuSchlopf,
- keine Runtime-Metric,
- keine neue Export-Bindung,
- keine Inventurdatenbank,
- keine historische Vergleichsschicht,
- keinen Excel-Writer im Produkt,
- keine MusicMutation-, Writer-, Repair-, Proof-, Recovery- oder Release-Authority.

Ein Rendererfehler darf nur den Renderer erneut starten. Er darf keinen neuen Source-Scan, keine neue Klassifikation und keine neue Wahrheit erzwingen.

ChangeClass: `AuditOnly`.
