# r288 – Registernotation: MusicXML-Dokumentordnungs-Identitätsabschluss

## Korpusfalsifikation

Der erste Vollkorpuslauf nach der musikalischen Freigabe von r287 lief für neun Projekte vollständig. Etüde 3 beendete die Einzelpipeline mit Exit 1 und erzeugte keine PDF.

Die dennoch erhaltene Haupt- und Render-MusicXML zeigt genau eine Oktav-Identitätskollision:

- eine obere 8va-Linie auf Staff 1 beginnt in Takt 17 und endet in Takt 19;
- eine kurze obere 8va-Linie auf Staff 2 liegt musikalisch früher innerhalb von Takt 17;
- beide tragen `number="1"`;
- durch die polyphone MusicXML-Reihenfolge steht der Staff-1-Start bereits im Dokument, bevor nach einem `backup` die Staff-2-Linie geschrieben wird;
- damit überlappen die beiden Spanner in MusicXML-Dokumentordnung, obwohl ihre musikalischen Zeitintervalle getrennt sind.

Die übrigen neun Korpus-MusicXML-Dateien zeigen keine doppelt aktive Oktav-`number`-Identität dieser Klasse.

## Fachliche Korrektur

MusicXML `number-level` unterscheidet gleichzeitig offene gleichartige Kontrollelemente nach **MusicXML-Dokumentordnung**. Bei polyphonen Parts kann diese Reihenfolge von der musikalischen Zeit abweichen.

r287 hatte die Liniennummer ausschließlich aus musikalischer Zeitüberlappung abgeleitet. Das war als Sichtmodell plausibel, aber als technische MusicXML-Projektion unvollständig.

r288 verschiebt deshalb ausschließlich diese technische Verantwortung:

```text
Sichtzustandspfad
→ reale Start-/Stop-Tonanker
→ Dokumentintervall je Sichtlinie
→ number-level-Färbung 1..=16 nach Dokumentüberlappung
→ MusicXML-Projektion
```

Die musikalischen Domänen bleiben unverändert.

## Fail-closed-Invarianten

- Jede Sichtlinie muss weiterhin einen sichtbaren Start- und Stop-Tonanker besitzen.
- Die Dokumentpositionen werden aus exakt denselben Ankern bestimmt, an denen anschließend geschrieben wird.
- Innerhalb eines Parts dürfen zwei in Dokumentordnung überlappende Oktavlinien niemals dieselbe Liniennummer besitzen.
- Mehr als 16 gleichzeitig in Dokumentordnung überlappende Linien führen fail-closed zum Abbruch.
- Kontakt ohne Dokumentüberlappung darf eine Liniennummer wiederverwenden.
- Kein `<offset>`.
- Kein `default-y`.
- Keine Etüden-, Takt-, Datei- oder Positionssonderlogik.
- Keine neue musikalische Entscheidung.

## Regression

Der neue Test erzeugt einen polyphonen Takt, in dem:

1. eine Staff-1-Oktavlinie musikalisch bei 120 beginnt und erst in einem späteren Takt endet;
2. eine Staff-2-Kurzlinie musikalisch bereits 72–96 gilt;
3. wegen `backup`/Staff-Reihenfolge der Staff-1-Start im XML vor der Staff-2-Linie steht.

Die Linien sind musikalisch getrennt, ihre Dokumentintervalle überlappen aber. Erwartung: verschiedene `number`-Werte.

Gegenüber r287 kommt genau eine neue Rust-Prüfung hinzu. Erwartung: **1005/1005**.
