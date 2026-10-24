# Watch MQTT Subscription Lifecycle Closure

## Ziel

Dieser Take schliesst ausschliesslich den Lebenszyklus lokaler MQTT-Client-Registrierungen
der `EtuSchlopf Watch`. Eine Subscription lebt genau so lange wie die zugehoerige
TCP/MQTT-Client-Verbindung. Alte Subscriptions duerfen nach dem Ende dieser Verbindung
nicht im eingebetteten Broker verbleiben.

## Lifecycle-Authority

Nach erfolgreichem MQTT-3.1.1-CONNECT registriert der Broker den Client mit einer
eindeutigen internen Registration-ID. Direkt danach besitzt `serve_client()` einen
RAII-Guard fuer genau diese Registrierung. Jeder Exit nach der Registrierung fuehrt
damit durch denselben Cleanup-Pfad.

Das gilt insbesondere fuer:

- MQTT `DISCONNECT`
- TCP EOF / `UnexpectedEof`
- `ConnectionReset`
- `ConnectionAborted`
- `BrokenPipe`
- Protokoll-/I/O-Fehler nach der Registrierung

Der Cleanup entfernt die komplette `ClientRegistration` und damit alle zu diesem Client
gehoerenden Topic-Filter unmittelbar aus `BrokerState.clients`. Ein spaeterer Publish ist
nicht mehr Voraussetzung fuer die Bereinigung.

## Ephemere Session-Semantik

Der lokale Watch-Broker besitzt weiterhin keine persistenten MQTT-Sessions. Deshalb wird
nur MQTT 3.1.1 CONNECT mit gesetztem `Clean Session` akzeptiert. Ein Client kann damit
nicht versehentlich Session-Persistenz anfordern, die der kleine lokale Broker bewusst
nicht implementiert.

Retained Session-State ist davon getrennt: Die retained Payload unter
`etuschlopf/v1/session/<ObservationSessionId>/state` bleibt der letzte veroeffentlichte
Watch-Zustand waehrend der Broker-Lebensdauer. Subscription-Lifecycle und retained
Publication-State sind zwei verschiedene Verantwortlichkeiten.

## Beweisgrenze

Die Broker-Tests decken ab:

- `Clean Session` wird verlangt.
- Ein sauberer MQTT `DISCONNECT` entfernt Client und Subscription.
- Ein abrupter TCP-Verbindungsverlust entfernt Client und Subscription.
- Der bereits bestehende retained Wildcard-Consumer-Vertrag bleibt bestehen.

Keine Aenderung an Musik, Writer, Render, Release, UDP, SessionAggregator, Watch-UI,
Launcher oder `run.cmd`. Keine neue Cargo-Abhaengigkeit.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 6"
```

Nach erfolgreicher Installation folgt weiterhin der reale Ein-Befehl-Watch-Test:

```powershell
.\run.cmd --watch --strict "Etüde Nr. 6"
```
