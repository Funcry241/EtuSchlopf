# r630 – SourceSelectionCausalLedgerFoundation

## Ausgangspunkt

r630 basiert direkt auf der autoritativen GreenBase r629
`ExternalNotationVendorNeutralVocabularyCleanupRefactorRecovery`:

- Vollkorpus 10/10 GREEN,
- 1235/1235 Tests GREEN,
- zehn produktive MIDI-Quellen verifiziert,
- 10.166 CanonicalSourceEvents,
- keine aktive Reparatur und keine offene Musikmutation.

Der historische CanonicalSourceEvent-Vertrag aus r200 bleibt unveraendert: Ein Ereignis belegt
28 Bytes (`source_ordinal` u64 LE, Position i64 LE, Dauer i64 LE, MIDI-Pitch i32 LE), wird fuer
den historischen Fingerprint nach Position, Pitch und Ordinalitaet sortiert und bleibt dadurch
ordinal-sensitiv.

## Geschlossene Kausalkette

Vor r630 war die produktive MIDI-Paarung deterministisch, aber ihre konkrete Rohereignis-
Provenienz wurde nach der Erzeugung des CanonicalSourceEvent verworfen. r630 fuehrt deshalb ein
typisiertes, read-only `SourceEventCausalLedger` ein. Jeder Eintrag bindet exakt:

1. die Source-Ordinalitaet,
2. den Roh-Locator des Note-On (`tick`, `track_index`, `event_index`, `channel`, `key`),
3. den Roh-Locator des zugeordneten Note-Off,
4. das daraus erzeugte CanonicalSourceEvent.

Die produktive Paarungsregel bleibt byte- und verhaltenslogisch dieselbe:

`globale Ordnung (tick, track_index, event_index) -> FIFO je (channel, key)`.

Das Ledger wird nicht als Quelle fuer eine musikalische Entscheidung verwendet. Es beobachtet und
beweist nur die bereits erfolgte Dekodierung.

## Unabhaengiger Replay

`project_source_event_causality()` rekonstruiert die Kausalitaet fail-closed und prueft:

- exakt zwei gezaehlte Roh-Notenereignisse je Ledger-Eintrag,
- gleiche Zahl von Ledger-Eintraegen und CanonicalSourceEvents,
- eindeutige und lueckenlose Ordinalitaeten `0..n-1`,
- Source-Ordinalitaet entspricht exakt der globalen Note-On-Reihenfolge,
- Note-On und Note-Off stimmen in Kanal und Pitch ueberein,
- positive Rohdauer,
- jeder Roh-Locator wird exakt einmal verwendet,
- FIFO-Reihenfolge je Kanal/Pitch ist reproduzierbar,
- Rohstart und Rohdauer sind verlustfrei auf 96 Divisions abbildbar,
- die exakte Normalisierung reproduziert Position, Dauer und Pitch des produktiven
  CanonicalSourceEvent,
- das im Ledger eingebettete CanonicalSourceEvent stimmt exakt mit dem produktiven Ereignis
  derselben Ordinalitaet ueberein.

Jede Abweichung beendet den Projektlauf vor Arrangement-, Hand-, Staff-, Voice-, Writer- oder
MusicXML-Entscheidungen.

## Fingerprints

r630 erzeugt zusaetzlich zwei domain-separierte SHA-256-Fingerprints:

- `RawPairLedgerFingerprintSHA256`: PPQ, Rohereigniszahl, Eintragzahl, Ordinalitaet und beide Roh-Locators,
- `CausalLedgerFingerprintSHA256`: dieselbe Rohpaarung plus Canonical Divisions und das gebundene
  28-Byte-CanonicalSourceEvent.

Der historische `EventFingerprintSha256` wird nicht ersetzt oder veraendert. Die neuen
Fingerprints sind zusaetzliche Provenienzbeweise.

## Vorabbeweis am gelieferten Zehnerkorpus

Eine unabhaengige Vorabprojektion der zehn gelieferten MIDI-Dateien reproduziert alle historischen
r200-Fingerprints exakt. Sie ergibt 20.332 Roh-Notenereignisse, 10.166 Ledger-Eintraege, sechs erhaltene
Pitch-Onset-Mehrfachereignisse und null Track-uebergreifende Note-On/Note-Off-Paare.

| Etuede | Events | Mehrfach | RawPairLedger SHA-256 | CausalLedger SHA-256 |
|---:|---:|---:|---|---|
| 1 | 503 | 0 | `0873fb6fdcdc35235a520500c520251ce7d3f33198068a654654e07e432780a9` | `c0bf148480b1dfa8ffa3cefb729ed2f40191d1e1dfcbe44d78055ebca0bf9533` |
| 2 | 1013 | 3 | `f6ef3099782d754bcc2b1749abd56f87caa4217968adec6c29e8fb45f7e6a454` | `5ca068626d86832a29e3ee78d49b5afab9bc26fef90a114ad2a47b79075e9978` |
| 3 | 829 | 0 | `3d1b0476081f61d5eaa7d4aafbef8d143d14265ff7d1b2fb446cc9e3a795cc94` | `059be3b1209ff04236bf3245beab00224978e6f68c3efb0c0562045fef9c3342` |
| 4 | 851 | 0 | `77edefd0d9ef5962080fb96b1edefd28e28eaed3076136c3b6dfd8911defcdb5` | `343e92b278e33797926676eb826a212d2760dae4af7026aa17cb2e5ed4e80b7b` |
| 5 | 1457 | 1 | `ad23a6ab982b8b530fb5164a2eb5b6c177063150c1dbce1a85c67723c454566a` | `1b7dbb3025b36caf74210db6db930aa1170e69fbe14156c5e4f296ddaa0cb711` |
| 6 | 1340 | 0 | `ee03290d9034f904c702ce6a38a769a3075a91a7d7e37f1f2b4ffd743e4e7ccb` | `4e9a9ae2c12e2dae9b2ce6ab7aeca8ae9504dd46d5d66d35af501ba16c74e97a` |
| 7 | 1044 | 1 | `6ecd3b5fe718ed6c96e320a79c56423dc62170a036055cfbda64ce31c158545f` | `19ff5ce40b3937110eca4d42198f9efcec7bcd4ffbc7d945d483bd808ae3a6f7` |
| 8 | 981 | 0 | `c558358a7f89a83984002c4feb383d1a1fe26b6957b5674ba0aeec2a280b7c98` | `e55c3124c7da38841bc5e196c239fdda46996b5a33535b826f24b0363372ad54` |
| 9 | 1073 | 1 | `f5cedeee10508ea382d370bf3af739e31ce11e3d75c29997c2fa5c94da00d901` | `6d0a52842c2fd1bfdc73016d3f5fd49a3e18e1a3da97da05b4b1bd3c0f660ed0` |
| 10 | 1075 | 0 | `c5055cb10b2ad4975b6b32c09efa069b436ff99f2b94fc45d94fac07861853d8` | `642c6baddb6f08dfc691fbb6189ef07ae95ee6175a55a56b24d692ee08199bb3` |

Die sechs Mehrfachereignisse liegen in vier Etueden: drei in Etuede 2 und je eines in Etuede 5,
7 und 9. Sie bleiben durch ihre Source-Ordinalitaet getrennt; r630 fuehrt keine Verschmelzung ein.

## Sichtbarkeit

Der Projektbericht erhaelt genau eine neue Sektion:

`SOURCE-EVENT-CAUSAL-LEDGER-FOUNDATION`

Sie zeigt Replay-Status, Ereignis- und Mehrfachzahlen, Track-uebergreifende Paare, historischen
Canonical-Fingerprint sowie beide neuen Ledger-Fingerprints. Die Sektion deklariert ausdruecklich:

- `SelectionSemantics: none`,
- `Ranking: none`,
- `Weighting: none`,
- `HandStaffVoiceAuthority: none`,
- `WriterAuthority: none`,
- `MusicMutation: nein`.

## Nicht-Scope

r630 entwickelt ausdruecklich noch keine neue Selection-Semantik. Insbesondere unveraendert:

- MIDI-Seed und Solver-Input,
- Candidate-Erzeugung und Candidate-Reihenfolge,
- Scores, Schwellen und Gewichte,
- Hand-, Staff- und Voice-Zuordnung,
- Rhythmus-, Register-, Layout- und Notationsregeln,
- MusicXML-, Render- und PDF-Writer,
- Pass-Reihenfolge,
- vorhandene Reparatur- und Recovery-Autoritaeten.

`ChangeClass=RefactorOnly`, weil der beweisfaehige Transport durch bestehende produktive
Source-Dateien gefuehrt wird; fachlich bleibt die Erweiterung read-only. Keine MusicXML-, Render-,
PDF- oder Musikmutation. Zwei neue Regressionen erhoehen den erwarteten Testbestand von 1235 auf
1237.
