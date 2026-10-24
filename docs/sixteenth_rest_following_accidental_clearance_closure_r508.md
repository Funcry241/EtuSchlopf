# SixteenthRestFollowingAccidentalClearanceClosure

## Version

`v0.10.1839 r508`

## Frische Evidenz

Der bestaetigte r507-Stand ist 10/10 dunkelgruen. Eine anschliessende menschliche Sichtpruefung zeigt in Etuede 1, Takt 3, unteres System eine sichtbare Kollision: Die erste 16tel-Pause ueberdeckt das Vorzeichen der unmittelbar folgenden Note.

Die Master->Render->PDF-Rekonstruktion lokalisiert den Befund im bestehenden Pausenhoehen-Owner:

- Im Master besitzt die Pause keine explizite `display-step`/`display-octave`-Koordinate.
- Im finalen Render-MusicXML materialisiert `FinalPianoRestGlyphCanonicalHeight` fuer die 16tel-Pause im Bassschluessel die kanonische Mittelposition `D3`.
- Direkt links der Pause steht `B3`, direkt rechts `Bb3`; die rechte Note benoetigt damit sichtbar ein neues Alterationszeichen.
- Der hohe obere Glyph-Anteil der 16tel-Pause in Mittelposition kollidiert im PDF mit diesem unmittelbar rechts stehenden Vorzeichen.

Ein scoreweiter r507-Readback findet exakt drei Rest-Zeilen mit derselben lokalen Struktur:

1. Etuede 1, Takt 3, Staff 2, Voice 2, Onset 24: `B3 -> 16tel-Pause -> Bb3`.
2. Etuede 3, Takt 7, Staff 2, Voice 2, Onset 312: `Bb3 -> 16tel-Pause -> B3`.
3. Etuede 5, Takt 14, Staff 2, Voice 2, Onset 216: `F#3 -> 16tel-Pause -> F3`.

Die beiden unabhaengigen Vergleichsstellen zeigen denselben gedrungenen horizontalen Raum zwischen Pause und neuem Vorzeichen. Eine globale Regel "jede Pause vor einem Vorzeichen verschieben" waere dagegen zu breit und wuerde viele nicht belegte Korpusstellen veraendern.

## Ursache

Die typed Rest-Placement-Authority kennt bisher nur Glyphklasse und Schluessel. Fuer eine 16tel-Pause im Bassschluessel liefert sie deshalb immer den kanonischen Mittelanker `D3`.

Der finale Writer kennt zwar die reale lokale Voice-Timeline, uebergibt diese Information aber nicht an die Rest-Placement-Entscheidung. Dadurch bleibt eine nachweislich kollisionsrelevante Nachbarschaft unbeachtet:

- unmittelbar vorher klingendes Event derselben Staff/Voice,
- unmittelbar folgende klingende Note derselben Staff/Voice,
- vorherige und folgende Note besitzen dieselbe diatonische Staffposition,
- die Alteration wechselt,
- dazwischen liegt genau die sichtbare 16tel-Pause.

## Closure

r508 erweitert ausschliesslich die bestehende typed Rest-Placement-Authority und ihre vorhandenen produktiven Consumer:

1. Die kanonische Restposition bleibt der Default.
2. Nur fuer `16th` wird ein zusaetzlicher lokaler Clearance-Hinweis ausgewertet.
3. Der Hinweis entsteht nur, wenn die Pause links und rechts unmittelbar durch dieselbe diatonische Staffposition derselben Staff/Voice begrenzt wird und rechts ein neues Alterationszeichen erforderlich ist (Alterationswechsel oder explizites `<accidental>`).
4. Die 16tel-Pause wird genau eine Staff-Linie **vom folgenden Vorzeichen weg** verschoben:
   - Vorzeichen an/oberhalb des kanonischen Restankers -> Rest eine Linie tiefer,
   - Vorzeichen unterhalb -> Rest eine Linie hoeher.
5. Alle anderen Glyphklassen und alle 16tel-Pausen ohne diese lokale Sandwich-Struktur behalten exakt die bisherige kanonische Position.
6. Es entsteht kein neuer Writer. Die vorhandene `RestDisplayPitchCapability` materialisiert weiterhin nur `display-step`/`display-octave` im Render-MusicXML.

Damit wird weder horizontales Spacing simuliert noch eine renderer-spezifische Sonderregel eingefuehrt. Die fachliche Entscheidung bleibt eine lokale, typed Pausenhoehenentscheidung im vorhandenen Owner.

## Invarianten

- Pitch der klingenden Events unveraendert,
- Onset unveraendert,
- Duration unveraendert,
- Staff/Voice unveraendert,
- Beam/Tie unveraendert,
- Restdauer und Restanzahl unveraendert,
- Master-MusicXML unveraendert,
- nur finale Render-Pausenkoordinate darf sich aendern,
- keine neue Writer-/Repair-/Recovery-Authority,
- keine Projekt-, Takt-, Pitch- oder Etuedenidentitaet in produktiver Logik.

## Regression

Typed Policy:

- Bassschluessel + 16tel + folgendes Vorzeichen oberhalb -> eine Linie tiefer (`B2` statt `D3`).
- Bassschluessel + 16tel + folgendes Vorzeichen unterhalb -> eine Linie hoeher (`F3` statt `D3`).
- andere Restklasse bleibt unveraendert.

Integration:

- `B3 -> 16tel-Pause -> Bb3` muss Clearance aktivieren.
- `F#3 -> 16tel-Pause -> F3` muss Clearance aktivieren.
- ohne Alterationswechsel bleibt `D3` erhalten.
- ein frueherer gleichnamiger Ton reicht nicht; die gleiche diatonische Staffposition muss die Pause **unmittelbar links** begrenzen. Damit bleibt die Regel fail-closed und lokal.

## ChangeClass

`MusicMutation`.

Die Klasse ist erforderlich, weil sich die sichtbare finale Render-Notation durch eine andere `display-step`/`display-octave`-Koordinate aendern kann. Es gibt keine SoundingEventMutation und keinen MasterApply.

## Erste Abnahme

```powershell
./run.cmd --take "Etüde Nr. 1"
```

Danach visuell Etuede 1 Takt 3 unten pruefen. Erwartung: Die erste 16tel-Pause steht frei vom Vorzeichen der folgenden Note. Anschliessend Etuede 3 Takt 7 und Etuede 5 Takt 14 als unabhaengige Comparatoren pruefen. Erst danach Vollkorpus als Breitenbeweis.
