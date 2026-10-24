# r1484: vorläufige Pausenbrücken mit Beleg; MuseScore ohne `-f`

Parent: r1483. Class: GovernedChange, music_effect declared. Autor: Dachs.
Freigabe: Wolfgang am 23.10.2026 für E3/T6, E5/T36 und E5/T37.

## Teil 1: keine erzwungenen MuseScore-Aufrufe

Die vier Renderer-Varianten mit `-f` entfallen (`src/render.rs`). Ein Test verbietet
`-f` in der Variantenliste. Bisher reichte in jedem Lauf die erste Variante; lehnt
MuseScore eine Datei ab, schlägt der Lauf künftig sichtbar fehl, statt sie zu
erzwingen. Erwartet: `VariantsAttempted 1/7`, sonst keine Ausgabeänderung.

## Teil 2: Brücken, die der eingefrorene Plan trennt, nur vorläufig

Ursache (r1478): Die frühe Sperre aus r1476 vergleicht Einzellabels im eingefrorenen
Stimmenplan. In E3/T6, E5/T36, E5/T37 steht die erste Paarnote dort in einer anderen
Stimme, die späte Ausrichtung legt das Paar aber in eine Stimme. Die Brücke wäre
gültig, wurde aber gesperrt (Fähnchen statt Brücke).

Änderung:
- E050 schreibt eine solche Gruppe nur **vorläufig** und nur, wenn im selben Aufruf
  der vollständige Erweiterungsbeleg aus r1483 gebunden wird. Gelingt das nicht,
  wird der Durchlauf verworfen und ohne diese Gruppe wiederholt; sie bleibt gesperrt
  wie bisher.
- Bei der Rücknahme gilt für vorläufige Verbindungen: bleiben, wenn die gebundene
  endgültige Zuteilung alle Mitglieder in eine Stimme legt; sonst (getrennt oder
  unbekannt) zurücknehmen.
- Ohne gebundenen späten Plan entscheidet die aktuelle Stimme nach der Projektion
  (getrennt → zurücknehmen).
- **Sicherheitsnetz:** Geht eine vorläufige Verbindung vor der Entscheidung verloren
  oder erreicht sie keine Entscheidung, bricht der Lauf mit klarer Meldung ab
  (`provisional_custody_lost`, „Provisional rest bridge reached no withdrawal decision“).
- Unverändert: gewöhnliche Zulassung, Rücknahmeregel für gewöhnliche Verbindungen,
  Stimmenentscheidung, globale Prüfungen, `through`.

Tests (`tests/support/bridge_frozen_voice_compatibility.rs`): Brücke bleibt bei
gemeinsamer Endstimme; wird bei getrennter oder unbekannter Endstimme zurückgenommen;
ohne Beleg nie geschrieben; Belegverlust vor der Entscheidung schlägt fehl; Rücknahme
ohne späten Plan nach echter Projektionslogik; alles durch den Verschiebungs-Prüfstand.

## Erwartete Wirkung (Prognose)

- E3/T6, E5/T36, E5/T37: Pausenbrücke zurück, Balken wie in r1477.
- E6/T34: Brücke wird jetzt vorläufig geschrieben und spät zurückgenommen; Ergebnis
  wie r1478/r1483 (Paar, 168 ohne Balken).
- Alles andere bytegleich zu r1483 (bis auf Leerzeilen, falls E050 zusätzliche
  Gruppen neu schreibt), Prüfer für Gruppen über Stimmen 0 Takte, kein `-f`.
- Weicht das ab: Befund, kein Nachschieben.
