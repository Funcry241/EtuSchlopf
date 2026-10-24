# r964 ArchitectureInventoryContractV1

## Ziel

r964 schliesst ausschliesslich den repository-residenten Cold-Start-Vertrag fuer die spaetere assistant-seitige Architektur-Inventur. Der Take fuehrt keine Runtime-Inventur, keinen Symbolscanner im Produkt, keine neue Metric und keine Excel-Erzeugung in EtuSchlopf ein.

Die stabile Semantik liegt in:

```text
docs/assistant_memory/architecture_inventory_contract_v1.json
```

Vertragsversion: `AIC-1`.

## Deterministische Grundgesamtheit vor KI-Interpretation

AIC-1 trennt Existenz und Bedeutung hart:

- die Symbolgrundgesamtheit wird assistant-seitig aus dem gebundenen physischen Source-Snapshot bestimmt,
- Extraktion basiert auf Rust-Syntax/AST und nicht auf Regex-Heuristik,
- der Source-Scope ist explizit eingeschlossen bzw. ausgeschlossen,
- anonyme Closures, generierte Makroexpansionen, externe/qualified Components und Build-Artefakte sind keine eigenen Inventurzeilen,
- jedes qualifizierte Symbol erscheint exakt einmal,
- `relative_source_path + qualified_symbol` ist die semantische Zeilenidentitaet,
- `start_line` bleibt nur Navigationshilfe,
- ein Symbol mit mehreren Architekturbezuegen erhaelt genau ein Primaerthema; weitere Beziehungen stehen im Relevanz-/Gegenbelegtext,
- unbewertete Symbole beginnen als `OFFEN`.

Damit ist die Grundbilanz reproduzierbar:

```text
Gesamt = A + B + C + N + OFFEN
```

Maus entscheidet nicht, was existiert. Maus entscheidet erst nach vollstaendiger Extraktion, was das Existierende bedeutet.

## AIC-1 Inhalt

Der Vertrag friert ein:

- Source-Scope und Ausschluesse,
- AST-basierte Symboldefinition,
- Zeilenkorn und Identitaet,
- A/B/C/N/OFFEN-Semantik,
- die 22 Inventurspalten und ihre Reihenfolge,
- kontrollierte Kategorien,
- Native-/Repository-/KI-/mechanisch-abgeleitete Provenienz,
- Ableitungsregeln fuer die Kopfkennzahlen,
- das aktuelle Excel-Renderprofil als untergeordneten Consumer.

Das Renderprofil verlangt aktuell genau eine Arbeitsmappe, genau ein Blatt `Inventur`, Kennzahlen oben und die strukturierte Tabelle `EtuSchlopfInventur` ab Zeile 8. Excel bleibt Darstellung; der semantische Vertrag heisst AIC-1.

## Bootstrap und Readback

Der bestehende Architecture-Essence-Bootstrap verweist auf AIC-1. `000_start_here.md` nimmt AIC-1 fuer Architektur-Inventur-Aufgaben in die Cold-Start-Reihenfolge auf.

Der bestehende read-only Regressionstest prueft nur:

- AIC-Pfad ist repository-resident und aufloesbar,
- `contract_version = AIC-1`,
- `advisory_only`, `normative = false`, `productive_authority = []`,
- keine Runtime-Implementierung,
- AST-Extraktion / kein Regex-only,
- `one_symbol_one_row = true`,
- genau 22 Spalten,
- eingefrorenes Renderprofil.

Der Test bewertet **nicht** die spaetere A/B/C/N/OFFEN-Klassifikation einzelner Funktionen und erteilt keinerlei produktive Autoritaet.

## Grenzen

Nicht Bestandteil von r964:

- keine Runtime-Metric oder Measurement-Aenderung,
- keine Assistant-Export-Bindungsaenderung,
- keine Corpus-Aggregation,
- kein Rust-Symbolscanner im EtuSchlopf-Produkt,
- keine Inventurdatenbank,
- keine Historie/Comparator-Logik,
- keine Excel-Erzeugung in EtuSchlopf,
- keine MusicMutation,
- keine Writer-/Repair-/Recovery-/Release-Authority,
- keine neue Essence-Regel E32.

ChangeClass: `AuditOnly`.
