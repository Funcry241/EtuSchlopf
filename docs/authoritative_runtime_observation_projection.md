# AuthoritativeRuntimeObservationProjection

## Ziel

Die Watch ist Projektion bestehender Runtime-Wahrheiten und kein eigener Status- oder Timing-Owner.

## Runtime-Zeit

`SourceStateOwner` besitzt `started_at`, `started_unix_millis` und bei der ersten terminalen
Transition `terminal_duration_millis`. Laufende Anzeige wird serverseitig aus dem producer-
Startzeitpunkt projiziert; nach `Completed`/`Failed` wird nur noch die monotone producer-eigene
terminale Dauer verwendet. Der bisherige `first_seen -> terminal_at`-Beobachtungszeitraum im
`SessionAggregator` entfaellt.

## Protokoll

Der UDP-Source-Snapshot und der daraus abgeleitete MQTT-Session-Snapshot werden bewusst auf
Schema v2 gehoben. V2 transportiert `started_unix_millis`, `terminal_duration_millis` und die
optionale `outcome_color`. Es gibt keine stille Schemaerweiterung unter einer v1-Kennung.

## Aktuelle Session

Die Web-Watch besitzt serverseitig `current_session_id`. Ein expliziter Launcher-Pin hat Vorrang.
Ohne Pin bindet die Watch beim manuellen Start genau die erste gueltige Session und bleibt daran
gebunden. Andere Sessions koennen die Produktoberflaeche nicht durch einen juengeren
`last_seen`-Zeitstempel uebernehmen.

## GUI

- Browser waehlt keine Session per Freshness.
- Projektzeilen bleiben natuerlich nach Projektname sortiert.
- Alle parallel laufenden Projekte erscheinen im Aktivitaetsblock.
- `Laufzeit` und `Dauer` bleiben getrennt von `Phase seit`.
- `outcome_color` ist typisiert vorbereitet; Lifecycle und Ergebnisfarbe bleiben getrennte Achsen.

## Nicht enthalten

Keine Musikmutation, kein Result-Parsing im Browser, keine PDF-Aenderung, keine neue Recovery,
keine Gate-Lockerung.
