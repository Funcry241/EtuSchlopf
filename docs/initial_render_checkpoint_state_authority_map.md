# Shared Render-XML Checkpoint Snapshot – v0.10.1585 Ergänzung

Stand: v0.10.1585 r01  
Ausgang: v0.10.1584 r01 / Export 524

## Reale Ausgangsbestätigung

- Strict-Build und Strict-Clippy grün,
- `FinalExit 0`, Etüde 4 `OK`, Blocker 0, Warnungen 0,
- gültiges PDF trotz bekanntem externen MuseScore-Prozessabgang,
- Gesamtzeit 294.897 ms, Build 100.763 ms, Programmlauf 161.028 ms.

## Gefundene Kostenvermischung

Der Owner aus v0.10.1584 war semantisch korrekt, besaß aber zwei unnötige
physische Kopierpfade:

1. `advance_after_write(false)` rekonstruierte denselben Zustand durch
   `fallback_xml.to_string()` und ersetzte den bisherigen String durch seine
   Vollkopie.
2. Jeder der sechs `capture()`-Aufrufe kopierte das gesamte Render-MusicXML,
   obwohl die Projektion unveränderlich und nur lesend ist.

Diese Kopien erzeugten keine neue fachliche Wahrheit. Sie waren reine
Transportkosten und zusätzliche Speicherbesitzer.

## Zielzustand

```text
RenderXmlCheckpointState
    current_xml: Arc<String>

NoWrite
    -> current_xml unverändert

ConfirmedWrite
    -> Checkpoint genau einmal lesen
    -> neuer Arc<String>-Snapshot

TraceCapture
    -> Arc::clone
    -> gleicher unveränderlicher Snapshot
```

## Rechte

Unverändert:

- Reader darf nur den materialisierten Checkpoint lesen.
- State darf nur den aktuellen unveränderlichen Snapshot auswählen.
- Trace darf nur Snapshot und Label halten.
- Kein Bestandteil darf schreiben, reparieren, musikalisch entscheiden,
  Passreihenfolge ändern oder die PDF-Quelle auswählen.

## Beauty Ledger vorher / nachher

| Merkmal | v0.10.1584 | v0.10.1585 |
|---|---:|---:|
| State-Owner | 1 | 1 |
| produktive Writer | unverändert | unverändert |
| `advance_after_write`-Aufrufe | 10 | 10 |
| Vollkopie bei No-Write | ja | nein |
| Trace-Captures | 6 | 6 |
| Vollkopie je Trace-Capture | ja | nein |
| neue Module/Adapter | 0 | 0 |
| unterschiedliche fachliche Zustände | unverändert | unverändert |
| unabhängige Besitzer desselben Snapshots | bis zu 7 | 1 geteilter Owner |

## Beweis

Der lokale Test bindet die Speicheradresse des aktuellen Strings:

- vor und nach No-Write identisch,
- aktueller Zustand und Capture identisch,
- nach materialisiertem Write neuer Zustand,
- nach anschließendem No-Write wieder identisch,
- Trace-Inhalte exakt `retained=initial`, `first=first`.

Damit wird nicht nur Wertgleichheit, sondern das Ausbleiben der tiefen Kopie
bewiesen. Output- und Writerverhalten bleiben unverändert.

---

# Initial Render-XML Checkpoint State - Authority Map

Stand: v0.10.1584 r01
Ausgang: v0.10.1583 r01 / Export 523

## Reale Ausgangsbestätigung

Der v0.10.1583-Take ist auf dem Zielsystem installiert und grün bestätigt:

- Update `0.10.1582 -> 0.10.1583`,
- Strict-Build und Strict-Clippy erfolgreich, Clippy-Warnungen 0,
- `FinalExit 0`, Ergebnis Etüde 4 `OK`, Blocker 0, Warnungen 0,
- der bekannte externe MuseScore-Prozessabgang trat erst nach gültiger PDF auf
  und blieb korrekt hellgrün,
- Master- und Render-MusicXML unterscheiden sich von v0.10.1582 ausschließlich
  in der `<software>`-Versionszeile,
- alle sechs PDF-Seiten sind bei 200 dpi pixelidentisch.

Damit ist der schreibfreie `RenderXmlSnapshotReader` als Ausgangsgrenze real
bewiesen.

## Problemgrenze vor diesem Cut

Der Reader besaß bereits Pfad- und Fallback-Autorität. Die erste
zusammenhängende Renderkette in `real_main()` führte den aktuellen XML-Zustand
aber weiterhin über zehn nacheinander benannte `String`-Zwischenstände fort.

Sechs dieser Zwischenstände wurden später ein zweites Mal als
Post-Candidate-Beweissnapshots zusammengesetzt. Damit existierten für dieselbe
Wahrheit zwei Darstellungen:

1. Ablaufvariable für den nächsten Operator,
2. später erneut montierte Trace-Projektion.

Die produktiven Writer waren eindeutig, die Zustandsfortschreibung und ihre
Beweisprojektion jedoch noch im Orchestrator verteilt.

## Zielgrenze

Der private Owner `render_xml_checkpoint` enthält nun gemeinsam:

- `RenderXmlSnapshotReader` für den unveränderten Datei-/Fallback-Vertrag,
- `RenderXmlCheckpointState` für genau einen sequenziellen In-Memory-Zustand,
- `RenderXmlCheckpointTrace` als read-only Projektion der sechs bereits
  vorhandenen Beweispunkte.

Der begrenzte Datenfluss lautet:

```text
Initial Master XML
  -> writer reports written/not-written
  -> RenderXmlCheckpointState advances or retains
  -> next existing operator reads current_xml()
  -> six existing evidence points are captured read-only
  -> final current XML + trace leave the bounded chain
```

Die Grenze beginnt nach `RenderXmlPrunedPdfTrial` und endet nach
`LowerStaffVisibleRhythmRestLaneTrial`, unmittelbar vor
`ProfessionalNotationVisualWeakspotAudit`.

## Rechte

Der Checkpoint-Owner darf:

- den bestehenden Checkpoint lesen,
- bei nicht gemeldetem Write den bisherigen In-Memory-Zustand behalten,
- den aktuellen Zustand sequenziell fortführen,
- vom Aufrufer benannte read-only Beweissnapshots speichern.

Er darf nicht:

- einen Write autorisieren oder ausführen,
- musikalische Fälle klassifizieren,
- eine Policy oder Reparatur auswählen,
- die Passreihenfolge verändern,
- Reportzeilen oder Releaseentscheidungen auswählen,
- die PDF-Quelle bestimmen.

Die sechs Trace-Bezeichnungen und ihre Reihenfolge bleiben exakt erhalten:

1. `RenderAfterHiddenMaterialPrune`
2. `RenderAfterHiddenRestForwardCollapse`
3. `RenderAfterPianisticRestRepair`
4. `RenderAfterPrintCleanup`
5. `RenderAfterMissingRestInsert`
6. `RenderAfterBehindBarsAndCrossStaffRole`

## Beauty Ledger

| Merkmal | v0.10.1583 | v0.10.1584 |
|---|---:|---:|
| Direkte `select_after_write`-Aufrufe im Orchestrator | 19 | 9 |
| Sequenzielle XML-Ablaufvariablen in der ersten Renderkette | 10 | 0 |
| Getrennt montierte frühe Trace-XML-Variablen | 6 | 0 |
| Sequenzielle Checkpoint-State-Owner | 0 | 1 |
| Read-only Checkpoint-Trace-Owner | 0 | 1 |
| Produktive Writer | unverändert | unverändert |
| Musikalische Policies | unverändert | unverändert |
| Orchestrator-Datei | 5.497 Zeilen | 5.467 Zeilen |
| Physische Rust-Zeilen unter `src` | 128.204 | 128.262 |
| Netto physische Rust-Zeilen | - | +58 |

Die bisherige Reader-Datei wird im selben Take durch den umfassenderen
Checkpoint-Owner ersetzt. Es existiert keine parallele Reader-, Shadow- oder
Legacy-Schicht.

## Beweisumfang

1. Unit-Test des unveränderten Reader-Fallback-Vertrags.
2. Unit-Test der sequenziellen Fortschreibung einschließlich false/true/false/
   true-Folge und unveränderter Trace-Snapshots.
3. Strict-Build und Strict-Clippy.
4. Etüde 4 als betroffenes Projekt.
5. Etüde 6 als unabhängiges Kontrastprojekt.
6. Vergleich von Master-MusicXML, Render-MusicXML und PDF gegen v0.10.1583;
   Versionszeile normalisieren.

Ein Korpuslauf ist nur nötig, wenn einer dieser lokalen Beweise eine
inhaltliche oder visuelle Abweichung zeigt.

## Noch offene Schuld

Dieser Cut umfasst bewusst nur die erste lineare Renderkette. Offen bleiben:

- spätere wiederholte `read_or_fallback`-Aufrufe,
- die große PDF-Quellenevidenzliste,
- Writerfamilien und historische Fallbacks,
- direkte Reportmontage im Orchestrator.

Der nächste Schritt darf den State-Owner nur dort erweitern, wo erneut eine
wirklich lineare Checkpointfolge vorliegt. Verzweigte Writer- und
PDF-Quellenautorität darf nicht in einen Universalzustand gepresst werden.
