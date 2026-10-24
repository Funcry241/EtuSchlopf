# r1289 – SSC-Vertrag v4, Aufräum-Take: keine zweite Klassifikation verworfener Kandidaten

Parent: accepted r1288 (SSC abgeschlossen). Klasse `GovernedChange`, `music_effect=none`, genau eine `proof_rule` auf `src/ssc_evidence_proof.rs`.

## Anlass: Laufzeit

| Revision | Vollkorpus |
|---|---:|
| r1277–r1284 | 6:54–7:07 |
| r1285 (Messung aller Commits im Bericht) | 10:52 |
| r1286 | 11:17 |
| r1287 (Enforcement beim Commit) | 12:29 |
| r1288 | 12:28 |

r1285 klassifiziert im Bericht alle rund 12 600 verworfenen Kandidaten-Records komplett: Captures entpacken, parsen, Invariante und Containment prüfen. Das geschieht unter dem globalen Record-Mutex. Seit r1287 prüft `ssc_adoption_verdict` jeden dieser Records bereits **vor** seinem Commit. Die Bericht-Klassifikation ist damit dieselbe Arbeit ein zweites Mal.

## Änderung

- `count_unreleased` klassifiziert nicht mehr. Records entstehen ausschließlich über den Receipt-`commit`. Den rufen nur die beiden verdrahteten A7-Grenzen auf, und zwar erst nach Permit von `ssc_adoption_verdict`; der Abschlussnachweis r1288 erzwingt diese Verdrahtung. Ein unreleased Record zählt deshalb als zugelassen.
- Duplikate und Domain-Konflikte bleiben Integritätsfehler wie bisher.
- Neue Berichtszeile `V4UnreleasedBasis: adoption_verdict_at_commit`. `V4UnreleasedCommits` und `V4AllCommitsPermitted` bleiben im Format gleich.
- Die Zeilen `V4UnreleasedDeniedDescriptor…` und `V4UnreleasedDeniedWitness` entfallen. Ein verweigerter Kandidat wird nie committet; sein Witness steht in der Abbruchmeldung des Enforcements.
- Die released Events (1438) werden weiterhin vollständig im Bericht klassifiziert, als V4-Messung.

## Erwartung

- Laufzeit: etwa 8 bis 9 Minuten statt 12:30. Übrig bleibt der Preis des Enforcements selbst, rund 1:20. Das ist eine Schätzung; gemessen wird es erst auf Windows.
- 20/20 XML bytegleich, `V4UnreleasedCommits` mit denselben Zahlen wie in r1288, `V4AllCommitsPermitted: yes`, `PermitEnforcement: active`. Der Abschlussnachweis bleibt gültig.
