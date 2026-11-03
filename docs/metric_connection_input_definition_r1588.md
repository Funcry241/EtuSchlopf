# r1588 – Eingangsabweichung des frühen Balkenowners definieren

KodexOnly auf nativ angenommenem r1587. Keine produktive Rust-Datei wird
geändert. Diese Stufe definiert eine bisher nicht eingefrorene Fehlerklasse;
sie ist noch keine Balkenkorrektur und fügt keine Messstellen hinzu.

## Aktueller Beleg, ausschließlich Evaluation

Der native r1587-Export bindet den angenommenen Kandidaten
983ddea2e43fdfc4801cf3db2f59c6058e06d6eac71df564d18d220b79379694.
Alle zehn Projekte und die XSD-Prüfung sind grün; 2.903 Tests bestanden,
0 fehlgeschlagen, 3 ignoriert, Clippy ohne Warnung.

Im hashgeprüften Readback der realen Stelle 1T13 trifft der bestehende E050
am Eingang D15 auf untere Toncontainer bei 0, 48 und 96 in XML-Stimme 1,
aber auf die erhaltenen Viertelpausen in XML-Stimme 2. Der spätere Zustand D18
weist die unteren Toncontainer der finalen XML-Stimme 2 zu und enthält unter
anderem die sichtbaren Pausen 24..48 und 72..96. Der tatsächliche FinalChord-
Übergang D17 nach D18 ist im bestehenden Bericht als committed projection
operands gebunden. Die ursprünglichen Taktbytes und SHA-256 wurden geprüft.

`choose_structural_rest_bridge_candidate_groups` gruppiert nach System, XML-
Stimme und Schlag. `visible_rests_cover_gap` verlangt in genau dieser Gruppe
sichtbare, exakt aneinander anschließende Pausen. D15 erfüllt dies nicht;
D18 besitzt die strukturelle Lückendeckung zwischen den Noten bei 0 und 48.
Das ist keine bewiesene Schreibzulassung. Insbesondere sind aus dem XML noch
keine unabhängige musikalische Pausenrolle und keine vollständige E050-
Quittungs-/Adoptionskette für eine neue Verbindung bewiesen.

Die native E050-Zeile für Projekt 1 meldet weder gesperrte Split-Gruppen noch
ungebundene Gruppen. Der fehlende Balken scheitert bereits vor dieser
Zulassung an der Kandidatenbildung. Pausenerhaltung allein konnte diese
Reihenfolgeabhängigkeit deshalb nicht schließen.

## Warum Definition vor Implementation

Der bestehende Zulassungsvertrag in `src/take_change_control.rs` verlangt eine
im angenommenen Parent eingefrorene kausale Fehlerklasse, bevor neue
selector-sensitive Klassifikations- oder Repair-Logik zugelassen wird.
Die bestehende Klasse beam_connection_voice_separation betrifft die Trennung
einer Verbindung durch spätere Stimmenzuweisung. Hier entsteht noch gar
keine Verbindung. Die Pausenverlustklasse betrifft gelöschte Träger; diese sind
in r1587 inzwischen erhalten. Beide Definitionen werden nicht umgedeutet.

r1588 friert deshalb die allgemeine Eingangsabweichung getrennt ein. Die
Definition enthält keine Korpusorte, Zielzahlen oder technischen Stimmennummern
als Entscheidungsmerkmale. Sie verleiht keine neue Schreiberlaubnis.

## Vorarbeit für die tatsächliche Korrektur

Der bestehende E050 bleibt fachlicher Beam-Owner. Zu entscheiden und zu prüfen
ist sein Zugriff auf einen vollständig autorisierten Kontext: entweder eine
gebundene frühe Sicht auf die relevante kanonische Beziehung und vorhandenen
Pausenverpflichtungen oder eine zulässige spätere Entscheidung desselben Owners.
Keine dieser Möglichkeiten wird durch diesen Take als bereits zugelassen oder
implementiert erklärt. Eine reine Umbenennung der XML-Stimmen, ein Clippen
überlappender Pausen oder ein zweiter pauschaler Balkenpass reicht nicht.

Der bestehende Vertrag `beam_extension_evidence_contract_r1474.md` verbietet,
FinalChord allein für die Brücke vorzuziehen. `canonical_rest_context_v1`
verbietet, Pausenverantwortung aus späteren Kanalbezeichnungen zu erfinden.
Die spätere Implementation muss die tatsächlich vorhandene Autorität und den
Origin-/Occurrence-Transport nachweisen; bei fehlender Grundlage bleibt die
Verbindung ungeschrieben. Der sealed Voice-Guard, E050-Receipt, Main-Adoption,
Extension-Custody und späte Rücknahme dürfen nicht abgeschwächt werden.

## Abnahme

`run.cmd --take --all` ohne `-f`. Erwartet werden bytegleiche 20 Master-/Render-
MusicXML-Dateien gegenüber r1587. 1T13 bleibt in diesem Voraussetzungstake noch
ohne die offene untere Balkenbrücke. Erst der hashgebundene Annahmebeleg macht
die neue Definition zu einer verwendbaren Parent-Voraussetzung.
