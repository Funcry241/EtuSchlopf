# r304 – CrossStaffClefIslandBeamMembershipPreservation

## Parent

- Version: `0.10.1839`
- Revision: `r303`
- Status: real GREEN
- FinalExit: `0`
- RunSeverity: `dark_green`
- ReleaseDecision: `released`
- Build/Clippy/Renderer: passed
- Tests: `1063/1063`
- reale Strict-Clippy-Diagnosen: `146` (r300: `184`, r303 Delta: `-38`)

## Reeller Befund

Im autoritativen r303-Assistant-Export von Etuede Nr. 6 gibt es ueber den gesamten Score genau einen Note-Event, dessen sichtbarer Staff zwischen Master- und Render-MusicXML wechselt:

- Takt 29
- Position `216/384`
- Pitch `F#5`
- Voice `2`
- Master: Staff `2`
- Render: Staff `1`
- Dauer: `24`
- Master Beam 1: `continue`
- Master Beam 2: `continue`
- Render Beam 1/2 vor r304: entfernt

Pitch, Onset, Dauer und Voice bleiben unveraendert. Es ist keine zusaetzliche klingende Note.

## Owner

Der Owner ist `notation::decompose_lower_staff_clef_islands()`, konkret `project_short_lower_staff_clef_island()`.

Die bestehende Regel entfernt fuer kurze untere G-Schluesselinseln die temporaeren Staff-2-Clefs und projiziert die Inselnoten nach Staff 1. Beim Staffwechsel wurde bisher zusaetzlich `strip_beam_tags(...)` angewendet.

Damit veraenderte eine reine sichtbare Staff-Projektion gleichzeitig die rhythmische Beam-Zugehoerigkeit. Im bekannten Fall entsteht aus einem `continue/continue`-Mitglied einer vierteiligen 16tel-Gruppe eine optisch isolierte Note.

## Systemische Invariante

Eine Staff-Projektion darf vorhandene strukturelle Rhythmuszugehoerigkeit nicht nebenbei vernichten.

Daher gilt ab r304:

> Wenn `LowerStaffClefIslandDecompositionRule` eine vorhandene Note von Staff 2 nach Staff 1 projiziert, bleiben vorhandene Beam-Tags unveraendert erhalten.

Die Regel entscheidet weiterhin generisch anhand ihrer vorhandenen Clef-Island-Bedingungen. Es gibt keinen Takt-29-, Etueden-, Pitch-, F#5- oder Locator-Sonderfall.

## Aenderung

Nur die bestehende Projektion wird enger:

- vorher: `strip_beam_tags(note).replace(staff2, staff1)`
- nachher: `note.replace(staff2, staff1)`

Unveraendert bleiben:

- Kandidatenerkennung der kurzen Clef-Insel;
- Staff-2 -> Staff-1-Projektionsentscheidung;
- Pitch;
- Onset;
- Dauer;
- Voice;
- Stem;
- Tie/Notations;
- Handrolle;
- temporaere Clef-Entfernung.

Neu explizit invariant bleibt:

- vorhandene Beam-Zugehoerigkeit.

## Regression

Eine neue allgemeine Unit-Regression konstruiert eine kurze hohe Staff-2-G-Schluesselinsel mit einer 16tel-Note, die auf Beam-Level 1 und 2 jeweils `continue` traegt.

Der Test fordert:

1. Staff 2 -> Staff 1 findet weiterhin statt.
2. Die temporaere G-Clef-Insel wird zerlegt.
3. Beam-Level 1 `continue` bleibt genau einmal erhalten.
4. Beam-Level 2 `continue` bleibt genau einmal erhalten.
5. Kein Score-, Takt- oder Pitch-Sonderfall ist Teil des Tests oder der produktiven Regel.

## ChangeClass

`MusicMutation`.

Grund: Die sichtbare Render-MusicXML darf sich bewusst aendern, und damit darf auch das PDF sichtbar anders werden. Es wird keine neue Writer-/Repair-Authority eingefuehrt; die Korrektur erfolgt im bestehenden fachlichen Owner.

## Erwartete Verifikation

- Take-Change-Control GREEN.
- Build GREEN.
- Strict Clippy GREEN.
- bestehende 1063 Tests GREEN plus 1 neue Regression = erwartete 1064/1064.
- Renderer GREEN.
- Master-MusicXML gegen r303 invariant.
- Render-MusicXML: der bekannte F#5-Event bleibt Staff 1, besitzt aber wieder Beam 1/2 `continue`.
- keine Pitch-/Onset-/Duration-/Voice-Deltas.
- PDF visuell pruefen: die zuvor isoliert wirkende Note muss als Mitglied ihrer rhythmischen Gruppe lesbar werden.
