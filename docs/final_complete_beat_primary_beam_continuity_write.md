# FinalCompleteBeatPrimaryBeamContinuityWrite

Version: **v0.10.1614 r02**

## v0.10.1614 – pensionierter Kontext entfernt und Preservation typisiert

Die mastergebundene v0.10.1611-Closure benoetigt weder die alte
Complete-Beat-Baseline noch den normalisierten Zwischenstand. v0.10.1614
entfernt deshalb den pfadgebundenen Mutexstore und die beiden pro Projekt
gespeicherten XML-Vollkopien vollstaendig. Ein vergifteter, fachlich
wirkungsloser Kontext kann den finalen Writer nicht mehr vetoieren.

Der Preservation-Delta wird ausserdem als gemeinsamer `BeamPlan` mit
`BeamState` gebunden und erst innerhalb der vorhandenen zentralen
Final-Notation-Senke in MusicXML-Text serialisiert. Zielauswahl,
Beam-Level-1-Schutz, Beam-only-Readback und Writerautoritaet bleiben
unveraendert.

## Reales Ergebnis v0.10.1610

Der Zielsystemlauf ist technisch sauber, aber die bisherige Preservation bleibt
leer:

- Strict Clippy 0,
- Build 0,
- FinalExit 0,
- `SourceHigherBeamNotesRestored 0`,
- Render-MusicXML und PDF gegen v0.10.1609 unveraendert.

Der prozessweite Pending-Store hat die drei fehlenden Beam-Level-2-`begin`
somit nicht wiederhergestellt. Die Baseline-/Normalized-Uebergabe wird fuer
die musikalische Entscheidung nicht mehr benoetigt.

## v0.10.1611 – mastergebundener Sekundaerbalken-Abschluss

Der finale PDF-Quellenplan vergleicht Master und finalen Renderkandidaten nur
fuer eine exakt definierte Struktur:

1. vier sichtbare, lueckenlose 16tel derselben Staff und Voice,
2. vollstaendige Viertelschlagabdeckung,
3. identische primaere Folge `begin/continue/continue/end` in Master und Render,
4. sekundare Masterfolge `begin/continue/continue/end`,
5. sekundare Renderfolge `fehlend/continue/continue/end`,
6. eindeutige Ereignisbindung ueber Part, Takt, Staff, Voice, Onset, Dauer,
   Pitch und Reststatus.

Nur die erste Note erhaelt Beam-Level 2 `begin`. Beam-Level 1 und alle weiteren
Zustaende bleiben unveraendert. Gemischte Notenwerte, Hooks, geteilte
Sekundaerbalken, unvollstaendige Schlaege und mehrdeutige Ereignisse werden
fail-closed abgewiesen.

Auf dem realen v0.10.1610-Endstand liefert diese Regel exakt drei Ziele in den
Takten 13, 15 und 18. Die schon vorher vorhandenen Sonderformen in Takt 8, 11
und 12 passen nicht auf die enge Zustandsfolge und bleiben unberuehrt.

Die Materialisierung verbleibt im bestehenden zentralen FinalNotation-
Checkpoint-Writer. Der Renderpfad plant nur; es entsteht keine neue externe
Schreibflaeche und keine neue Writerwurzel.

## Musikalische Primaerregel

Sichtbare balkenfaehige Noten derselben Staff und Voice werden auf Beam-Level 1
als vollstaendige Viertelschlaggruppe normalisiert, wenn sie den Schlag ohne
Pause, Luecke, Ueberlappung oder Cross-Staff-Bindung vom Anfang bis zum Ende
ausfuellen. Die Regel ist scoreweit und ohne Projekt-, Etueden-, Takt-, Pitch-
oder Locatorselektor.

## Erwartete Abnahme Etuede Nr. 4

Gegen v0.10.1606:

- Master-MusicXML nur mit neuer Werkzeugversion,
- 12 freigegebene Primaerbalkengruppen,
- 14 geaenderte Beam-Level-1-Zustaende,
- 3 wiederhergestellte Beam-Level-2-`begin` in Takt 13, 15 und 18,
- keine weitere Aenderung auf hoeheren Balkenebenen,
- Pitch, Onset, Dauer, Staff, Voice, Pausen, Notenzahl und Cursor stabil,
- `SourceHigherBeamNotesRestored 3`,
- Strict Clippy ohne Warnung.

---

# FinalCompleteBeatPrimaryBeamContinuityWrite

Version: **v0.10.1610 r01**


## Reales Ergebnis v0.10.1609

Der Zielsystemlauf ist technisch sauber, die Preservation bleibt aber leer:

- Strict Clippy 0,
- Build 0,
- FinalExit 0,
- `SourceHigherBeamNotesRestored 0`,
- Render-MusicXML und PDF gegen v0.10.1608 unveraendert.

Die drei verlorenen Beam-Level-2-`begin` sind daher weiterhin nicht
wiederhergestellt. Die Pending-Baseline war als `thread_local` gebunden und
stand dem spaeteren PDF-Quellenabschluss nicht threaduebergreifend zur
Verfuegung.

## v0.10.1610 – pfadgebundener prozessweiter Kontext

Baseline-/Zwischenstand und der daraus erzeugte finale Preservation-Plan werden
nun ueber mutexgeschuetzte, prozessweite `BTreeMap<PathBuf, ...>`-Stores
transportiert. Mehrere Projekte bleiben nach ihrem konkreten Pfad getrennt;
jeder Eintrag wird genau einmal verbraucht. Ein leerer oder nicht passender
Kontext wird nicht geraten, und ein vergifteter Store fuehrt fail-closed zum
Abbruch.

Die musikalische Auswahl und die zentrale Materialisierung bleiben
unveraendert. Der Take erweitert weder die Kandidatenmenge noch eine
Writer-Capability; er schliesst ausschliesslich die reale Thread-Grenze zwischen
Complete-Beat-Notationsabschluss und finaler PDF-Quellenbindung.

## Musikalische Regel

Sichtbare balkenfaehige Noten derselben Staff und Voice werden auf Beam-Level 1
als vollstaendige Viertelschlaggruppe normalisiert, wenn sie den Schlag ohne
Pause, Luecke, Ueberlappung oder Cross-Staff-Bindung vom Anfang bis zum Ende
ausfuellen. Die Regel ist scoreweit und ohne Projekt-, Etueden-, Takt-, Pitch-
oder Locatorselektor.

## Reales Ergebnis v0.10.1608

Der Primaerbalkenoperator wirkt musikalisch wie beabsichtigt, aber die Abnahme
bleibt geschlossen:

- Strict Clippy: eine `type_complexity`-Warnung in `src/pipeline.rs`.
- `SourceHigherBeamNotesRestored 0`.
- Render-MusicXML und PDF gegen v0.10.1607 sichtbar unveraendert.
- Beam-Level-2-`begin` fehlen weiterhin in Takt 13, 15 und 18.

Die v0.10.1608-Preservation verwendete als Referenz einen Puffer, der bereits
hinter der Complete-Beat-Normalisierung lag. Daher waren die verlorenen
Sekundaerbalken in Referenz und Kandidat gleichermassen nicht mehr vorhanden.

## v0.10.1609 – Baseline- und Zwischenstandbindung

Die vorhandene Complete-Beat-Writergrenze bindet rein lesend:

1. das XML unmittelbar vor der Primaerbalkennormalisierung,
2. das XML unmittelbar nach der Primaerbalkennormalisierung.

Die finale Preservation bestimmt daraus exakt die vollstaendigen
Viertelschlaggruppen, in denen Beam-Level 1 durch den Operator geaendert wurde.
Nur fuer Noten dieser Gruppen werden die im Baseline-Puffer vorhandenen
Balkenebenen ab Level 2 in den finalen Kandidaten vereinigt. Beam-Level 1 bleibt
vollstaendig ausserhalb der Closure. Spaeter autorisierte zusaetzliche Ebenen
bleiben bestehen.

Damit werden keine bereits vorher fehlenden Sekundaerbalken aus Master oder
einer allgemeinen Zielnorm ergaenzt. Der Plan bewahrt nur die hoeheren
Balkenzustaende, die durch den neuen Primaeroperator beziehungsweise seine
spaeteren Folgestufen verloren gingen.

Die Materialisierung verbleibt im bereits bestehenden zentralen
`write_render_checkpoint`-Writer. Bindungs-, Vergleichs- und Gruppierungslogik
schreibt nicht. Ueberlappende Ranges, Mehrdeutigkeit, Level-1-Aenderung,
Nicht-Beam-Delta und nicht parsebares MusicXML werden fail-closed abgewiesen.

## Erwartete Abnahme Etuede Nr. 4

Gegen v0.10.1606:

- Master-MusicXML nur mit neuer Werkzeugversion,
- 12 freigegebene Primaerbalkengruppen,
- 14 geaenderte Beam-Level-1-Zustaende,
- 3 wiederhergestellte Beam-Level-2-`begin` in Takt 13, 15 und 18,
- keine weitere Aenderung auf hoeheren Balkenebenen,
- Pitch, Onset, Dauer, Staff, Voice, Pausen, Notenzahl und Cursor stabil,
- `SourceHigherBeamNotesRestored 3`,
- Strict Clippy ohne Warnung.
