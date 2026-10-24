# Opus: Stimmen-Kollisionen – Stand nach r1313 (accepted r1310)

Advisory only. Keine neue Musik-, Writer-, Selector-, Proof- oder Release-Authority.
Dieser historische Befund ersetzt nicht die frischen Artefakte des aktuellen Exports.

## Gültiger Stand
- **Accepted: r1310 `NotatedChannelOverlap`** (AuditOnly), SHA-256 `42e2d870b89be2ce22094dece5808fa165479f671fdc58f5e49d5f6e3c6df981`, Version 0.10.1839.
- Musikalisch identisch zu r1308: alle 20 Master-/Render-XMLs bytegleich, FinalExit 0, 10/10.
- r1311, r1312 und r1313 wurden abgelehnt und transaktional zurückgerollt. Der wiederhergestellte Quellbaum im r1313-Export ist dateigleich zum r1310-Take (einzige Ausnahme: das Root-`Cargo.lock`).
- Hinweis: Die Fehler-Exporte nennen als Parent-Acceptance weiterhin r1308 (`take_revision=1308`, `prior_rejections=496` unverändert). Der Baum ist aber r1310. Das sieht nach einer veralteten Projektion im Fehlerexport aus, nicht nach einem falschen Baum.

## Offener Befund: neun Stimmen-Kollisionen (Etüden 1, 2, 3, 6, 7)
- **Ursache (durch r1310 gemessen):** Die Stimmenkanäle werden aus dem eingefrorenen Klangmaterial geplant. Dort klingen Noten oft kürzer als notiert, deshalb sieht der Plan die notierte Überlappung nicht. In 11 von 12 Paaren lautet die Einstufung `sounding_intervals_disjoint`. Ein Paar (Etüde 1, D4/A3) überlappt auch im Klang und wird nur wegen der Wiederverwendungsregel geteilt, die je ein Kanalmitglied prüft.
- In den Etüden 4, 5, 8, 9 und 10 gibt es keine solchen Paare.

## Versuche
| Take | Ansatz | Ergebnis |
|---|---|---|
| r1309 | Wiederverwendung prüft alle Kanalmitglieder (Klangsicht) | Großflächige Drift, Etüde 3 mit HAN03-WARNUNG, abgelehnt |
| r1311 | Notierte Überlappung aus dem Pre-FinalChord-XML, neuer Kanal | 9/10 kollisionsfrei, aber Stimmen im ganzen Stück umnummeriert; Etüde 3 scheitert an der Schlüsselplanung |
| r1312 | Wie r1311, aber Wechsel in den Hauptkanal der anderen Hand statt neuem Kanal | Keine Umnummerierung, aber Fehlauslösungen in 14 Takten von Etüde 3 (Pre-FinalChord-Dauern ≠ geschriebene Dauern); Schlüsselplanung scheitert |
| r1313 | Notierte Überlappung aus den geschriebenen Noten **nach** der FinalChord-Projektion, plus Wechsel in die andere Hand | Etüden 4, 5, 8, 9, 10 bytegleich; in 1, 2 und 7 nur die Kollisionstakte geändert, keine Ton- oder Schreibweisenänderung. Abgelehnt wegen dreier Abstürze (siehe unten) |

## r1313 im Detail
- Kandidat: `EtuSchlopf_v0_10_1839_r1313_WrittenOccurrenceVoice.zip`, SHA-256 `a0e678954aa213747055f5bdd990a1ddb918b1b95aae424a0659f01126d3be46`.
- Lokal: Admission (r1310-Validator) OK, Clippy `--all-targets` sauber, 1907/1907 Tests grün.
- Windows-Ergebnis:
  - Etüde 2, Takt 13: F#4 im unteren System 2→1.
  - Etüde 7, Takt 25: Eb4 2→1.
  - Etüde 1, Takt 17: G4 2→1.
  - Etüde 1, Takt 12: spiegelverkehrt gelöst. Statt des D4 wechselt das A3 der linken Hand (48/48 und 96/24) in Stimme 1. Kollisionsfrei, musikalisch aber nicht ideal.
- Abstürze:
  - Etüde 6: Panik in der GuidedSearch (`MASTER SOVEREIGNTY VIOLATION: SafeAtomicCandidateProducer/Patch`), also **bevor** der geänderte Code läuft.
  - Etüde 3: Exit `0xC0000005` in `InitialRenderTransaction`.
  - Etüde 1: Exit `0xC0000005` am Berichtsende, nachdem der geänderte Code fehlerfrei durchgelaufen ist.
  - Ein ursächlicher Zusammenhang mit der Änderung ist nicht belegt, für Etüde 3 aber nicht ausgeschlossen.
- Abweichung vom Auftrag: `freeze_final_sounding_material` liest fünf Felder aus den bereits übergebenen `audit_only`-Zeilen (`PreSerializationOccurrenceRow`). Der Orchestrator ist per Hash festgeschrieben (SSC #57) und darf in einer MusicMutation nicht geändert werden.

## Nächste Schritte
1. r1313 unverändert noch einmal laufen lassen, um die Abstürze auf Reproduzierbarkeit zu prüfen.
2. Wenn sie reproduzierbar sind: Etüde 3 und 6 einzeln untersuchen.
3. Wenn sie nicht reproduzierbar sind: Abnahme prüfen und danach die Richtung in Etüde 1, Takt 12 verfeinern.
4. Optional, für einen sauberen Weg ohne die Audit-Typ-Lesung: einen Kodex-Take vorschalten, der den Orchestrator so ändert, dass er die geschriebenen Noten in einem eigenen Produktionstyp liefert.
