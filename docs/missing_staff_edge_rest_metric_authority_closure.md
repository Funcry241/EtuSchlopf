# MissingStaffEdgeRestMetricAuthorityClosure

Version: 0.10.1753 r01

## Anlass

Nach v0.10.1752 entscheiden die aktiven Rest-Konsolidierer ihre sichtbare
partielle Pausendauer ueber den writer-freien `MetricRestNotation`-Owner.
Der verbleibende beat-lokale Missing-Staff-Edge-Pfad erzeugt jedoch noch eine
neue sichtbare Randpause direkt aus `gap_duration` und einer lokalen
Resttyp-Pruefung.

Damit existiert fuer neu materialisierte partielle Stille noch eine zweite
Dauerfreigabe, obwohl Staff-/Voice-/Cross-Staff- und Gap-Beweis bereits beim
bestehenden Candidate-Owner liegen.

## Aenderung

`apply_beat_local_missing_staff_edge_rest_rewrite()` behaelt seine komplette
Candidate- und Writer-Grenze. Bevor die bereits bewiesene Randluecke als genau
eine sichtbare Pause materialisiert wird, verlangt der Pfad jetzt vom zentralen
`MetricRestNotation`-Planner einen exakten Ein-Pausen-Plan fuer
`beat_start .. first_target_onset`.

Die bestehende Grenze `gap_duration <= 48`, der Cross-Staff-Beweis, die freie
Hilfsvoice, die Backup-/Forward-Materialisierung und der autorisierte Writer
bleiben unveraendert.

## Invarianten

- Kein Writername und keine WriterReachability werden geaendert.
- Kein neues Schreibrecht und kein neuer Candidate-Typ.
- Keine Etueden-, Takt-, Staff- oder Onset-Sonderregel.
- Keine Ganztaktpausen-Aenderung.
- Keine Aenderung an Hidden-Rest-Visibility oder Backup-Anchor-Rewrite.
- `MetricRestNotation` entscheidet nur die sichtbare Dauerstruktur einer
  bereits bewiesenen partiellen Stille.
- Bei nicht kanonischer Zeitbasis entscheidet der Pfad fail-closed.
- Fuer die produktive 96er-Zeitbasis bleibt die bisherige kurze
  beat-lokale Ein-Pausen-Semantik unveraendert.
- Pitch, Onset, Dauer, Staff und Voice klingender Ereignisse werden nicht
  absichtlich veraendert.

## Abnahme

Zuerst ein Einzelprojektlauf mit Etuede Nr. 1. Erwartet werden:

- Takt 2 bleibt `192+96+48` Pause vor den zwei Sechzehnteln.
- Takt 17 bleibt chord-anker-sicher.
- Master- und Render-Notenereignisse bleiben gegen v0.10.1752 identisch.

Ein Vollkorpuslauf ist fuer diesen Ownership-Refactor nicht erforderlich.
