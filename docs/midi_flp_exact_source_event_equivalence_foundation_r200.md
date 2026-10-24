# r200 – MIDI/FLP Exact Source Event Equivalence Foundation

## Zweck

r200 ist der nicht-destruktive Sicherheitsunterbau fuer die Ablösung der FLP-Dateien durch echte Standard-MIDI-Dateien. Der Take schaltet die produktive Quelle noch nicht um. FLP bleibt alleiniger Lieferant der Solver-Noten; MIDI ist zusaetzliche, verpflichtende Uebergangsevidenz.

## Kanonischer Gleichheitsvertrag

FLP und MIDI werden unabhaengig dekodiert und anschliessend in denselben bereits bestehenden Ereignisvertrag ueberfuehrt. Jedes Ereignis belegt exakt 28 Bytes:

1. `source_ordinal` als `u64` little-endian,
2. Einsatzposition als `i64` little-endian,
3. Dauer als `i64` little-endian,
4. MIDI-Tonhoehe als `i32` little-endian.

Vor dem SHA-256-Fingerprint werden die Ereignisse nach Einsatzposition, MIDI-Tonhoehe und Quellordinal sortiert. Die bestehende FLP-Definition wurde hierzu unveraendert in einen gemeinsamen audit-only Owner verschoben. Damit gibt es keine zweite Definition von Quellidentitaet.

Der Projektlauf wird fail-closed abgewiesen, sobald einer dieser Punkte abweicht:

- Qualifikation des FLP- oder MIDI-Lesers,
- kanonische Divisions,
- Ereigniszahl,
- Zahl mehrfacher gleicher Tonhoehen am gleichen Einsatz,
- terminale Endposition,
- ordinal-sensitiver SHA-256-Ereignisfingerabdruck.

## MIDI-Grenze

Der Decoder akzeptiert Standard-MIDI-Formate 0 und 1 mit PPQ-Zeitteilung. Jede Einsatzposition und Dauer muss verlustfrei auf die kanonischen 96 Divisions abbildbar sein; rundende oder mehrdeutige Normalisierung wird abgewiesen. Er verarbeitet Running Status, Note-On, Note-Off und Note-On mit Velocity 0 als Note-Off. Er weist unter anderem SMPTE-Zeitteilung, Format 2, abgeschnittene Chunks, fehlendes End-of-Track, ungueltige Datenbytes, Note-Off ohne offene Note, Nulldauern, nicht geschlossene Noten und nicht deklarierte Restbytes ab.

Tempo- und Taktart-Metadaten werden im Bericht sichtbar gemacht. Sie sind in r200 rein beobachtend und besitzen keinerlei Source-, Metric-, Solver-, Hand-/Staff- oder Writer-Autoritaet.

## Vorabbeweis fuer die zehn gelieferten MIDI-Dateien

Die gelieferten Dateien wurden ausserhalb des produktiven Programmlaufs mit demselben kanonischen Ereignisvertrag gegen die FrozenLegacySourceOracle-Werte aus den r199-Ergebnisberichten verglichen:

| Etuede | Ereignisse | Mehrfachereignisse | Endposition | SHA-256-Ereignisfingerabdruck |
|---:|---:|---:|---:|---|
| 1 | 503 | 0 | 6912 | `d68fe6b9a5ed6bec89dd6f0938d47510a95eb0ceff659280502fc07a400e730c` |
| 2 | 1013 | 3 | 11712 | `3081b4375dba6bf37e75f8f26902ffb5df5be0452342a93edf663e0c691852bc` |
| 3 | 829 | 0 | 11088 | `e2e11e5c10df0e4002a711baed16ecccb83182e0bb339108e4a0abd22cd36b3b` |
| 4 | 851 | 0 | 11832 | `c43f148455a2e275768632aad42345ce4a90d4c133d88b01aafc4284b5ecb771` |
| 5 | 1457 | 1 | 19536 | `3aa312a5bb2a4cbd2d402fa78d441f8f1df9367c0a8b52581481942152a0b922` |
| 6 | 1340 | 0 | 16896 | `e90f912486909d78d709fd333d37e058b974252388ac13e616297ad983a5d59e` |
| 7 | 1044 | 1 | 18384 | `3b8180cb3e6d3fa298a6f9f03bbe4b07a1a9214d6c4ce30538e0f1d6dc283489` |
| 8 | 981 | 0 | 17808 | `54f16234e6a1051a29c0efa88a4e8f61bf2dff83b77e610cf310171b9eb14595` |
| 9 | 1073 | 1 | 14520 | `a1056163d1fe37a78a74123ffe8447a5f89094b7f324ab94cc68adf96109729f` |
| 10 | 1075 | 0 | 17304 | `06fa7724c67fc84e4ad84b1ef655a534c8eae32485ff89e14c382b9adf797ab1` |

Alle zehn MIDI-Dateien sind Format 1, verwenden PPQ 96, enthalten 4/4 und 461538 Mikrosekunden pro Viertelnote und stimmen im Ereignisfingerabdruck exakt mit r199 ueberein.

## Erwartete Abnahme

1. Alle zehn `.mid`-Dateien neben die gleichnamigen `.wav`- und `.flp`-Dateien legen.
2. Take mit der bestehenden Ein-Befehl-Regel installieren und Etuede 6 ausfuehren.
3. Im Bericht muss `MIDI TRANSITION EQUIVALENCE PROVEN` erscheinen.
4. Master- und Render-MusicXML muessen gegen r199 byte-identisch bleiben.
5. Danach Vollkorpus 10/10 ausfuehren; erst dessen gruener Beweis autorisiert den produktiven MIDI-Cutover in r203.
