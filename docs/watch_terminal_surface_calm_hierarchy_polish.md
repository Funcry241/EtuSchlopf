# Watch Terminal Surface Calm Hierarchy Polish

Version: 0.10.1735 r03

## Scope

Dieser Take veraendert ausschliesslich die sichtbare Terminaloberflaeche der bereits
akzeptierten EtuSchlopf Watch. MQTT-Broker, Client-Vertrag, SessionSnapshotV1, UDP,
Aggregator, Launcher, run.cmd und alle Musikpfade bleiben unveraendert.

## Ziel

Der erste echte `--watch`-Lauf soll nicht wie Diagnose-Rohbau wirken. Die Anzeige wird
ruhiger, klarer und hierarchischer, ohne eine zweite Runtime-Wahrheit einzufuehren.

## Aenderungen

- feste ruhige 78-Spalten-Komposition statt lose aneinandergereihter Diagnosefelder
- Connection-Zustand unterscheidet `CONNECTING`, `LIVE` und `RECONNECTING`, damit der
  erste Frame nicht faelschlich wie ein Fehlerzustand wirkt
- Sessionstatus steht als klare Kopfzeile getrennt von Source-Zeilen
- Corpus, Launcher und Projekte verwenden dieselbe kompakte Source-Zeile
- `last_seen` und `last_state_change` bleiben sichtbar, aber als ruhige rechte Spalte
- Corpus-Fortschritt bleibt erhalten und wird mit seiner echten Stage kombiniert
- entscheidend: eine Activity ersetzt nicht mehr die uebergeordnete Stage; dargestellt
  wird z. B. `Final Notation › Beam Planning`
- fehlende Sessions werden mit einer kurzen Warteansicht dargestellt, nicht mit einer
  technischen Erklaerung

## Nicht geaendert

Keine Farbe, keine Animation, keine neue Health-Semantik, kein History-Browser, keine
Kommandos, keine neuen Topics, keine Persistenz und keine neue Cargo-Abhaengigkeit.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 6"
```

Danach erstmals der reale Ein-Befehl-Lauf:

```powershell
.\run.cmd --watch --strict "Etüde Nr. 6"
```
