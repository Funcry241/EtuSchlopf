# Current Run Duration Watch Projection

Version: v0.10.1745 r04  
Change class: RefactorOnly

## Ziel

Die Web-Watch projiziert genau einen aktuellen Produktlauf statt aller jemals vom langlebigen
Watch-Prozess beobachteten Sessions gleichrangig anzuzeigen. Dadurch verschwinden verwaiste
Bootstrap-/Launcher-Sessions aus der Primaeransicht, ohne sie aus dem Observation-Datenmodell
zu loeschen.

## Zeitautoritaet

`SessionAggregator` ist bereits Owner der beobachteten Zeitbeziehungen `last_seen` und
`last_state_change`. Derselbe Owner fuehrt nun `first_seen` und `terminal_at`. Daraus entsteht
`run_elapsed_millis`:

- running: `now - first_seen`
- terminal: `first_terminal_transition - first_seen`

Die terminale Dauer bleibt stabil. Der Browser zeigt diese strukturierte Projektion als
`Laufzeit` beziehungsweise `Dauer`. `Phase seit` bleibt eine davon getrennte Information.
Kein Log-Scraping und keine browserlokale Startzeit werden zur fachlichen Laufzeitwahrheit.

## Aktueller Lauf

Die Web-Projektion waehlt die Session mit der kleinsten strukturierten
`last_seen_age_millis` als aktuelle Session. Aktive Publisher reannouncen im bestehenden
Observation-Vertrag jede Sekunde; beendete oder durch Bootstrap-Reexec verwaiste Sessions
altern dadurch deterministisch aus der Primaeransicht.

## Nicht enthalten

- keine Launcher-/CLI-Aenderung
- keine Visual-Preflight-Policy-Aenderung
- keine Musikmutation
- keine PDF-Erzeugungsaenderung
- keine neue RunOutcome-Farbwahrheit
