# Unified Watch Runtime Experience

Version: 0.10.1742 r01

## Ziel

Die lokale Watch wird von einer technischen Beobachtungsseite zu einer ruhigen Laufoberflaeche, ohne eine neue Runtime-Wahrheit einzufuehren. Alle sichtbaren Zustaende, Stages, Activities, Korpuszaehler und PDF-Locators stammen weiterhin aus den bestehenden Observation-/Snapshot-Ownern.

## Laufzeit und Aktivitaet

Die bereits vorhandenen `last_state_change_age_millis` werden als sichtbare Zustands-/Phasenzeit projiziert. Es wird keine historische Phasendauer erfunden: bei laufenden Sources steht `seit`, bei terminalen Sources `vor`. Einzelprojekte zeigen die sechs bestehenden Project-Stages als Arbeitsfluss; vorherige Stages werden nur aus der monotonen aktuellen Stage projiziert und erhalten keine Prozentsemantik.

Aktive Projekte erhalten ausschliesslich dezentes Eye-Candy: pulsierender Punkt und ruhiger Sweep. `prefers-reduced-motion` schaltet diese Animationen ab.

## Korpuslayout

Die Korpusliste besitzt ein festes Raster mit `Projekt`, `Aktueller Stand` und `Zeit`. Laufende Stage-Pfade und terminale Statuswerte beginnen dadurch an derselben linken Kante. `RUNNING` wird nicht noch einmal als Session-Badge wiederholt; der globale Lifecycle-Status bleibt oben Owner dieser Aussage.

Der Fortschrittsbalken verwendet weiterhin ausschliesslich `SnapshotProgressV1.completed / total`. Die Stage-Leiste des Einzelprojekts ist keine Fortschrittsprozentzahl.

## Stabiles DOM

Der Browser baut die Session-Oberflaeche nicht mehr sekundenweise neu. Ein struktureller Fingerprint verwendet nur die vorhandenen Source-Payloads und ignoriert deren Alterswerte. Heartbeats aktualisieren deshalb nur die sichtbaren Uhren. Ein echter Source-/Revision-/Status-/Stage-/Progress-Delta baut die Struktur neu.

PDF-Bereitschaft ersetzt ebenfalls nicht mehr die komplette Session: die vorhandenen `/pdf-data/<ProjectLabel>`-Links werden in-place aktiviert. Dadurch bleibt ein Link unter dem Mauszeiger stabil.

## Progressive Disclosure

Ein Einzelprojekt zeigt Projekt, Arbeitsfluss, aktuellen Stand und Phasenzeit zuerst. Launcher-/Corpus-/Detailkarten liegen unter `Technische Details`. Korpuslaeufe bleiben kompakt und priorisieren aktive/fehlgeschlagene Projekte.

## Vorbereitung der Headless-Closure

`ETUSCHLOPF_WATCH_WEB_ONLY` ist ein opt-in Startvertrag: nur wenn der Launcher diesen Marker setzt, beendet sich die Watch bei nicht startbarer Weboberflaeche mit Fehler statt in eine unsichtbare Terminal-UI zu fallen. Ohne Marker bleibt der bisherige manuelle Terminal-Fallback byte-semantisch erhalten. Der Marker wird in r01 noch nicht gesetzt.

## Nicht geaendert

- kein SourceSnapshot-/SessionSnapshot-Protokoll
- keine neue Status-, Ergebnis- oder Farbsemantik
- kein Log-Scraping
- keine PDF-Erzeugung oder PDF-Source-Auswahl
- keine Musik-, Writer-, Render-, Recovery- oder Manifest-Mutation
- keine neue Cargo-/Web-Abhaengigkeit
- kein Launcher-/Take-/Change-Control-Diff in r01

## Abnahme

```powershell
.\run.cmd --take --strict --all --display --visual-preflight
```
