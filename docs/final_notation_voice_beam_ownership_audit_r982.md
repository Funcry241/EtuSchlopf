# r982 FinalNotationVoiceBeamOwnershipAudit

## Parent und Ziel

r982 baut direkt auf dem real akzeptierten r981 `StaffLocalRestOwnershipAuditCompileClosure` auf. Der r981-Windowslauf war vollstaendig: Build/Clippy gruen, 1.271 Tests gruen und 10/10 Korpusprojekte abgeschlossen.

Der Take ist `AuditOnly`. Er schafft vor einem produktiven Retirement alter Balkenlogik eine explizite Beweisflaeche fuer die finale Beziehung zwischen kanonischer Voice-Struktur und sichtbarer Balkenstruktur.

## Konkreter Ausloeser: Etuede 6, Takt 1

Im akzeptierten r981-Master und -Render liegen auf Staff 1 drei primaere Balkenketten, deren Noten innerhalb derselben metrischen Linie zwischen Voice 1 und Voice 3 wechseln. Die Balkentags sind bereits im Master vorhanden; der Render erzeugt den Widerspruch also nicht erst spaet. Der heutige spaete strukturelle Beam-Closure partitioniert dagegen nach `(staff, voice, beat)` und kann diese Linie dadurch nicht als eine gemeinsame visuelle Balkengruppe modellieren.

## Historische Spezifitaet

Die Repository-Historie liefert einen zweiten, unabhaengigen Grund fuer den Audit:

- `docs/historical_measure_identity_debt_retirement.md` dokumentiert, dass die heutige `ScorewideMetricBeamContinuity*`-Familie zuvor `M10` in lokalen Namen trug. Beim Retirement der historischen Measure-Identitaet blieb die Candidate Detection byte-aequivalent; Bedingungen, Schwellen und Reihenfolge wurden ausdruecklich nicht geaendert.
- `docs/scorewide_metric_beam_plan_adoption.md` dokumentiert spaeter ebenfalls, dass die Auswahl der Staff-2-/Voice-Kandidaten sowie `erste Dreiergruppe und letzte Zweiergruppe` bewusst unveraendert blieb.

Damit ist die heutige Namensgebung scoreweit, die Auswahlform aber weiterhin historisch spezifisch. Vor einem Ausbau muss deshalb bewiesen werden, welche reale finale Struktur dieser Pfad noch besitzt und welche Aufgabe inzwischen von allgemeineren Operatoren uebernommen werden kann.

## Neue Projektion

`FINAL-NOTATION-VOICE-BEAM-OWNERSHIP-AUDIT` liest Master- und finalen Render-Snapshot ohne Mutation und berichtet:

- Anzahl primaerer Balkenketten,
- Anzahl sichtbarer same-staff Balkenketten mit mehr als einer Voice,
- Anzahl zusammenhaengender beambarer Cross-Voice-Runs innerhalb eines Beats,
- davon Runs mit bereits exakt vorhandener primaerer Balkenkette,
- davon Runs ohne entsprechende primaere Balkenkette,
- diagnostische Locator-Zeilen mit Part, Measure, Staff, Beat/Intervall und beteiligten Voices.

Die Projektion besitzt keinerlei Writer-, Repair-, Selection-, Rule- oder Release-Authority.

## r981-Vorabscan

Ein read-only Scan des akzeptierten r981-Vollkorpus ergab als Groessenordnung 1.695 primaere Balkenketten, davon 365 same-staff Cross-Voice-Ketten. Aus der metrischen Sicht wurden 393 zusammenhaengende Cross-Voice-Runs gefunden; 338 besitzen bereits eine exakt passende primaere Balkenkette, 55 nicht. Etuede 6, Takt 1 enthaelt allein drei solcher Cross-Voice-Balkenketten. Diese Zahlen sind Vorab-Evidenz und werden vom realen r982-Windowslauf erneut durch die produktnahe Rust-Projektion gemessen.

## Invarianten

- Master-MusicXML unveraendert.
- Render-MusicXML unveraendert.
- PDF unveraendert.
- Keine Voice-, Staff-, Pitch-, Onset-, Dauer-, Rest- oder Beam-Mutation.
- Keine Aenderung an `ScorewideMetricBeamContinuityWrite` in diesem Take.
- Keine neue Takt-, Projekt- oder Etuedenidentitaet als Selektor.
- Erwarteter Testbestand: 1.274.

## Naechste Entscheidung nach dem Korpuslauf

Der r982-Korpus soll die Retirement-Grenze festlegen. Erst danach darf der historische Spezialpfad entfernt oder durch eine finale strukturelle Visual-Voice/Beam-Ownership ersetzt werden. Ein Writer-Take muss dabei nachweisen, dass die bisherige Sounding-/Timing-/Voice-Semantik erhalten bleibt und nur die visuelle Notationsownership neu gebunden wird.
