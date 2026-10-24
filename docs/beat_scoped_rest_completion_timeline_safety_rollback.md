# BeatScopedRestCompletionTimelineSafetyRollback

## Anlass

v0.10.1755 versuchte den letzten historischen Kombipass
`enforce_beat_scoped_beaming_and_rest_completion_in_xml()` ebenfalls auf
`MetricRestNotation` umzustellen. Der Einzelprojektlauf war formal dunkelgrün,
der vollständige 1754->1755-Endvergleich zeigte jedoch eine unzulässige
gepitchte Timeline-Aenderung in einer allgemein vorkommenden Struktur mit
wechselnden Staffs, sichtbaren Pausen und versteckten Chord-Ankern derselben
Voice.

Der Befund ist keine Taktregel: Er beweist, dass dieser alte Kombipass seine
Rest-Semantik nicht isoliert genug besitzt, um nur durch Austausch der
Dauerzerlegung zentralisiert zu werden.

## Entscheidung

Der produktive 1755-Delta dieses Kombipasses wird vollständig auf die
nachweislich grüne v0.10.1754-Logik zurueckgenommen.

Damit gilt:

- die in v0.10.1747 bis v0.10.1754 bereits sicher zentralisierten partiellen
  Pausen-Owner bleiben unveraendert bei `MetricRestNotation`;
- nur der historische fruehe Kombipass behaelt vorerst seine alte
  beat-bounded Restlogik;
- keine Etueden-, Takt-, Staff- oder Onset-Sonderregel;
- keine Gate- oder Writer-Lockerung;
- keine neue Writerwurzel;
- keine neue musikalische Reparaturklasse.

## Architektururteil

Der verbleibende Kombipass ist kein geeigneter Ort fuer eine weitere
mechanische Ownership-Zentralisierung. Beaming, Rest-Completion und
MusicXML-Cursor-/Chord-Ankersemantik muessen dort zuerst als getrennte
fachliche Grenzen modelliert werden. Bis dahin ist der bekannte gruene
Legacy-Pfad sicherer als ein formal zentraler, aber semantisch falscher Write.

Das ist bewusst ein fail-closed Abschluss des Pausenblocks, kein Auftrag fuer
einen weiteren Refactor.

## Abnahme

Zuerst nur Etuede Nr. 1:

```powershell
.\run.cmd --take --strict "Etüde Nr. 1" --display
```

Erwartung:

- Takt 2 bleibt `Halbe + Viertel + Achtel`;
- Takt 17 bleibt mit seinen hidden pitched Chord-Ankern unveraendert;
- die vollstaendige Master- und Render-Noten-/Timeline-Struktur entspricht
  wieder dem sicheren v0.10.1754/1753-Verhalten, abgesehen von Versionsmetadaten.
