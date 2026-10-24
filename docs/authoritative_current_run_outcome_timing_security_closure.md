# AuthoritativeCurrentRunOutcomeTimingSecurityClosure

## Ziel

Die in r01 vorbereitete Runtime-Projektion wird an die bereits autoritativen Launcher-Wahrheiten
gebunden. Die Watch erhaelt die aktive Observation-Session explizit vom Launcher und die
terminale Ergebnisfarbe direkt aus der typisierten `TruthfulRunOutcomeProjection`.

## Aktive Session

Der Launcher uebergibt `ETUSCHLOPF_WATCH_ACTIVE_SESSION` mit seiner eigenen
`ObservationSessionContext::session_id`. Web- und Terminal-Watch filtern damit auf genau diesen
Lauf. Eine Browser-Freshness-Heuristik existiert nicht mehr.

## Ergebnisfarbe

`print_summary` liefert die bereits bestehende vierfarbige `OutcomeColor`-Projektion zurueck.
Nur der Launcher-Source-Owner schreibt diese Farbe beim terminalen Observation-Uebergang.
`Completed` wird niemals in der Watch zu Gruen umgedeutet.

## Strict-Test-Evidence

Das rohe integrierte Launcher-Testlog liegt unter `target/etuschlopf_logs/` statt als temporaere
Root-Datei. Dadurch kann die bestehende Root-Hygiene weiter strikt aufraeumen, waehrend der
Assistant-Export die aktuelle Test-Evidence vollstaendig mitnimmt.

## Nicht enthalten

Keine Musikmutation, keine PDF-Aenderung, keine neue Recovery, kein Log-Scraping und keine
Lockerung eines Change-Control- oder Identity-Gates.
