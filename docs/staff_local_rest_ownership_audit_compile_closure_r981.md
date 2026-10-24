# StaffLocalRestOwnershipAuditCompileClosure r981

## Parent

`r981` baut erneut direkt auf dem real akzeptierten `v0.10.1839 r979 PostRejectedTakeCorpusRecovery` auf. Der verworfene `r980 StaffLocalRestOwnershipAudit` bleibt Failure Evidence und ist niemals Parent.

## r980 Failure Evidence

Der reale Windows-Take-Lauf akzeptierte den r980-Diff zunaechst durch den `PRE-TAKE-DIFF-GATE`, Build und Clippy des normalen Binaerpfads waren gruen. Erst das integrierte `cargo test --all-targets` kompilierte den neuen `#[cfg(test)]`-Pfad und stoppte korrekt mit Exit 101.

Ursache war ausschliesslich der neue Testhelper `score(body)`: Er verwendete `format!(concat!(..., "{body}..."))`. Rust erlaubt implizites Format-Argument-Capturing wie `{body}` nur in einem direkten Formatstring-Literal, nicht wenn der Formatstring aus einem Makro wie `concat!` expandiert wurde. Dadurch entstand exakt der Compilerfehler `there is no argument named body`.

Der automatische Rollback stellte danach den vorherigen r979-Quellstand wieder her; r980 wurde rejected.

## Closure

r981 reappliziert die fachliche r980-Aenderung unveraendert direkt auf r979 und ersetzt ausschliesslich den fehlerhaften Testhelper durch einen direkten Formatstring-Literal mit `{body}`. Auditlogik, Reportintegration, Messklassen, erwarteter Korpusbefund und Authority-Grenzen bleiben identisch.

Die Projektion `STAFF-LOCAL-REST-OWNERSHIP-AUDIT` misst weiterhin getrennt:

1. `DUPLICATE_VISIBLE_REST_OWNERSHIP`: gleiche Part/Measure/Staff/Onset/Duration, sichtbar, Rest, aber in mindestens zwei Voices;
2. `SAME_LANE_REST_PITCH_OVERLAP`: sichtbare Pause und sichtbares klingendes Ereignis ueberlappen sich in derselben Part/Measure/Staff/Voice-Lane;
3. `MASTER_HIDDEN_TO_RENDER_VISIBLE_EXACT`: eine im finalen Master explizit unsichtbare Pause erscheint im finalen Render mit exakt derselben Staff/Voice/Onset/Duration-Identitaet sichtbar.

Der menschliche Leitfall bleibt Etuede 1, Takt 2, oberes System. Der vor dem Take ausgefuehrte read-only r979-Gegencheck ergab dort zwei exakte Cross-Voice-Doppelgruppen, eine Same-Lane-Rest/Pitch-Ueberlappung und einen Hidden->Visible-Exact-Fall. Die allgemeinere Same-Lane-Klasse kommt auch in anderen Etueden vor und erhaelt deshalb weiterhin kein Schreibrecht.

## Grenzen

- `AuditOnly`;
- keine Musikmutation;
- keine Regel- oder Gewichtsaenderung;
- keine Writer-, Repair-, Release- oder Selection-Authority;
- keine automatische Restunterdrueckung;
- keine Aenderung an PDF, MusicXML, Renderer, Layout oder Sounding-Material;
- Root-`Cargo.lock` bleibt ausserhalb des Take-Payloads.

## Tests

Die drei r980-Unit-Tests bleiben unveraendert erhalten; nur ihr XML-Stringhelper ist compile-korrekt. Ausgehend von 1.268 Strict-Tests in r979 werden fuer r981 weiterhin **1.271 Tests** erwartet.

## Acceptance

Ein Befehl:

```powershell
.\run.cmd --take --strict --all
```

Erwartung:

- Build und Strict Clippy gruen;
- 1.271/1.271 Tests gruen;
- 10/10 Korpusprojekte weiterhin OK und final-clean;
- `.musicxml`, `.render.musicxml` und `.pdf` semantisch unveraendert zu r979;
- neue Auditsektion in den `.result.txt`-Dateien;
- Etuede 1 meldet die zwei exakten Cross-Voice-Doppelgruppen in Takt 2 oben.

Erst der reale Windows-Lauf entscheidet ueber Acceptance und einen spaeteren engen Rest-Ownership-`MusicMutation`-Take.
