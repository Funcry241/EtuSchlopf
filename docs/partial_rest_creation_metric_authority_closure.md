# PartialRestCreationMetricAuthorityClosure

## Ziel

Nach v0.10.1753 entscheidet `MetricRestNotation` bereits die sichtbare Dauerstruktur
der aktiven Rest-Konsolidierer, des Missing-Staff-Edge-Writers und der Rest-Lane-
Planung. Zwei produktive Erzeugungspfade besaßen aber noch lokale Dauerhoheit:

1. `StrictStaffLocalRelease` zerlegte bewiesene Staff-Gaps selbst an jeder
   Zählzeitgrenze und materialisierte diese lokalen Teile als sichtbare Pausen.
2. `UpperInitialRestCoalescing` prüfte die neu einzufügende Anfangspause noch
   über die lokale Resttyp-Tabelle, obwohl seine spätere Coalescing-Entscheidung
   bereits beim zentralen Metric-Owner lag.

Dieser Take entfernt diese beiden lokalen Entscheidungen. Der bestehende
`StrictStaffLocalRelease` übergibt jeden bereits bewiesenen Gap-Span vollständig an
`MetricRestNotation` und materialisiert nur dessen Plan. Der Upper-Initial-Pfad
erzeugt eine Anfangspause nur noch, wenn derselbe Owner den Span `0..first_onset`
als genau eine sichtbare Pause bestätigt.

## Nicht geändert

- keine Writerwurzel und kein Writername;
- keine Etüden-, Takt-, Staff- oder Onset-Sonderregel;
- keine neue Writerwurzel oder Kandidatenreichweite; metrisch erlaubte beat-übergreifende Pausenwerte kommen ausschließlich vom bestehenden Metric-Owner;
- keine Voice-/Staff-/Cross-Staff-Klassifikation;
- keine Hidden-Rest-Visibility- oder Backup-/Forward-Policy;
- keine Ganztaktpausen-Policy;
- keine klingende Pitch-/Onset-/Dauer-Mutation.

Die generischen MusicXML-Materialisierer bleiben reine Repräsentationsprüfung:
Wenn ein vom Metric-Owner geplanter Wert nicht durch die vorhandene
`duration_to_rest_type_and_dots`-Tabelle darstellbar ist, wird fail-closed nicht
geschrieben.

## Metrischer Vertrag

Auf der kanonischen 96er-Zeitbasis darf ein bewiesener Gap jetzt dieselbe
Pausengestalt erhalten wie jeder andere partielle Stillespann. Beispiel 4/4:

- `0..288` -> `192 + 96`
- `96..288` -> `96 + 96`

Damit entscheidet nicht mehr `StrictStaffLocalRelease`, dass jeder Beat zwingend
eine eigene Pause sein muss. Vollständige Taktruhe bleibt weiterhin beim separaten
Ganztaktpausen-Owner.

Nichtkanonische Zeitbasen werden vom zentralen Planner weiterhin fail-closed
abgelehnt.

## Regression

Neue reine Planungstests belegen:

- ausgerichtete Zwei-Beat-Stille wird als Halbe geplant;
- eine auf schwachem Beat beginnende Zwei-Beat-Stille überschreitet die
  Taktmitte nicht;
- die Upper-Initial-Erzeugung wird auf kanonischer Basis weiterhin zugelassen;
- dieselbe Upper-Initial-Erzeugung bleibt auf einer 48er Zeitbasis fail-closed;
- die Strict-Staff-Gap-Planung bleibt auf einer 48er Zeitbasis ebenfalls
  fail-closed.

Erste Abnahme weiterhin nur an Etüde Nr. 1:

```powershell
.\run.cmd --take --strict "Etüde Nr. 1" --display
```

Erwartung: Takt 2 bleibt `Halbe + Viertel + Achtel`, Takt 17 bleibt wegen der
hidden pitched Chord-Anker unverändert sicher, und die gepitchte Timeline bleibt
vollständig stabil.
