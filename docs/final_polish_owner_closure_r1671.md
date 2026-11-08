# r1671 – Abschluss beim Owner statt äußerer Reparaturschleife

ChangeClass: RefactorOnly. Parent ist der nativ angenommene r1669 / 0.10.1839,
Take-SHA256 `18c74ac39eb01d3cea6352a7658b704b4dfeb088ba74aa53c24d68202039d0d2`.
Der Export `assistant_export_latest(20261108-004728).zip` bindet den überprüften
Annahmebeleg und vollständigen Source-Snapshot. Alle verwalteten Dateien sind
bytegleich mit der zuvor vorbereiteten Parent-Basis. Root-Cargo.lock bleibt
lokaler Zustand und gehört nicht in das eingehende Take-Paket.

Dieser Take ersetzt r1670 mit neuer eindeutiger Revision. Produktivcode und
normative Tests der Schleifenentfernung bleiben gegenüber dem lokal qualifizierten
r1670 bytegleich. Geändert sind ausschließlich Take-Identität, Basisnachweis und
diese Dokumentation samt gebundenem Änderungsmanifest.

Der angenommene r1669-Korpus ist formal grün: 2.980 Tests bestanden, 4 ignoriert;
Build-, Clippy- und Launcher-Test-Exit jeweils 0; zehn gültige Projekte. Alle
20 MusicXML-Dateien sind gegenüber dem vorherigen r1667-Export bytegleich.
Die beobachtete Korpuszeit beträgt 13:25 Minuten.

## Gefundene Schleife und Rückführung

`apply_final_notation_polish_guard` rief nach dem einmaligen Sichtbarkeitspass
bis zu viermal dieselben drei bestehenden Owner auf:

1. `remove_orphan_slur_and_tie_endpoints`
2. `prune_rest_only_staff2_clef_noops`
3. `prune_redundant_lower_staff_clefs`

Die Schleife brach erst ab, wenn die summierten Änderungszähler null waren.
Sie wiederholte damit Entscheidungen über bereits vollständig bearbeitete
Eigentümerdomänen. Kein nachgelagerter Schritt dieser Folge erzeugt eine neue
Löschpflicht beim vorherigen Owner.

Die äußere Wiederholung und ihr Viererlimit entfallen. Die drei vorhandenen
Owner werden in derselben Reihenfolge genau einmal aufgerufen. Jeder liefert
weiterhin seine eigenen Originalbelege; der Wrapper transportiert sie weiterhin
in Ausführungsreihenfolge und erzeugt keine Ersatzquittung. Die vorausgehende
Sichtbarkeitsbehandlung bleibt erhalten. Es gibt keinen neuen Writer, keinen
Ersatz-Repair und keinen späteren Prüf-/Reparaturlauf.

## Abschlussargument für die bestehenden Owner

### Bogen-Endpunkte

Ein erhaltener Slur-Anfang besitzt einen späteren sichtbaren Schluss gleicher
Nummer. Dieser Schluss besitzt damit zugleich einen früheren Anfang. Die beiden
stützenden Endpunkte werden im selben Durchlauf erhalten. Bei sichtbaren Tied-
Endpunkten gilt die entsprechende Gegenseitigkeit für die unmittelbar benachbarten
passenden Noten desselben Notationsstroms. Die Löschung anderer Endpunkte ändert
weder Notenreihenfolge noch Sichtbarkeit, Tonhöhe, System oder Stimme.
Die anschließende Entfernung leerer Notationshüllen ist vollständig.
Klingende MusicXML-`tie`-Elemente werden weiterhin nicht bearbeitet.
Die beiden Schlüssel-Owner ändern keine dieser Voraussetzungen.

### Schlüssel zwischen sichtbaren unteren Tonereignissen

Der erste Schlüssel-Owner entfernt alle nicht initial geschützten Schlüssel,
zwischen denen und ihrem Nachfolger kein sichtbarer unterer Ton steht.
Jeder erhaltene nicht letzte und nicht initial geschützte Schlüssel besitzt
danach mindestens ein solches Tonereignis bis zum nächsten Schlüssel.
Der zweite Owner entfernt ausschließlich Schlüssel-/Attributblöcke. Das
Zusammenlegen benachbarter Intervalle durch deren Entfernung kann ein bereits
vorhandenes Tonereignis nicht verschwinden lassen. Es entsteht daher keine neue
leere Zwischenstrecke für den ersten Owner.

### Redundante Schlüssel

Der zweite Schlüssel-Owner führt den aktiven Schlüssel bereits im selben
Durchlauf ausschließlich über seine erhaltenen Einträge fort. Entfernte Einträge
gehen nicht in diesen Zustand ein. Ein weiterer Durchlauf erhält daher an jedem
verbleibenden Eintrag denselben vorherigen Schlüsselzustand. Entfernte
Nachbarschlüssel können betrachtete Tonintervalle nur vergrößern. Eine vorhandene
Tonbelegung kann nicht leer werden. Die vorhandene Vorschau auf initiale
Schlüssel des nächsten erkannten Takts kann durch Löschung von Attributblöcken
nur einen positiven Beleg verlieren, keinen neuen gewinnen.
Keine Löschbedingung wird dadurch neu wahr. Der kombinierte Abschluss ist nach
einem Durchlauf erreicht.

Das Argument benutzt exakt die vorhandenen Leser und deren Eingabeverträge.
Taktidentifikation, Schlüsselbewertung, Initialschutz, System-/Stimmenzuordnung,
Auswahlbedingungen und erlaubte Löschbereiche werden nicht erweitert oder geändert.
Die produktiven Owner- und Autorisierungsdateien bleiben bytegleich zum Parent.

## Synthetische Gegenproben

Die unabhängige begrenzte Kontrollkomposition im Test verwendet die ursprüngliche
Viererwiederholung mit denselben tatsächlichen Ownern. Die neue produktive
Komposition muss dieselben XML-Bytes und dieselbe Anzahl nichtidentischer
Originalbelege liefern. Zusätzlich wird jeder einzelne nachgelagerte Owner
nochmals abgefragt: alle Zähler sind null, alle Belege fehlen und alle Bytes
bleiben gleich. Auch die erneute Gesamtkomposition darf nichts ändern.

2.048 Kombinationen variieren beide Anfangsschlüssel, drei aufeinanderfolgende
Taktinhalte aus acht Mustern und zwei voneinander unabhängige Nummerierungen.
Enthalten sind sichtbare/unsichtbare obere und untere Töne, Pausen, gültige und
verwaiste Endpunkte, leere Hüllen, Mehrfachschlüssel sowie Backup/Forward.
Positive Zähler verlangen tatsächliche Arbeit in allen drei Cleanup-Familien.
Eine lange stumme Schlüsselkette und deren versetzte Duplikation über getrennte
Parts sowie die leere Eingabe bilden zwei weitere Fälle. Die Proben binden keine
reale Etüde, Taktadresse oder Korpusausgabe als normative Fixture.

Eine zweite Probe benutzt einen tatsächlich erzeugten, übernommenen Master mit
Custody und echten gebundenen Rhythmusfragmenten. Sie führt die produktive
Gesamtkomposition durch den Originalbelegtransport bis zur Master-Adoption.
Die Gegenprobe lässt dieselben Originalbelege weg und verlangt die Verweigerung
bei unverändertem vorherigem Master. Ein gültiger Custody-Fortgang ersetzt die
fehlende Herkunftsquittung weiterhin nicht.

## Native Nachweisgrenze

Die erwartete Ausgabe bleibt unverändert. Die lokale Qualifikation prüft die
Gleichheit der Komposition und den ursprünglichen Belegweg; ein neuer nativer
Korpuslauf mit r1671 bleibt ein separater Nachweis. Die vorhandenen akzeptierten
Korpusausgaben dienen keiner neuen musikalischen Selektionsregel.

## Lokale Qualifikation und Wiederverwendung

Die 2.202 Hauptprogrammtests (4 bewusst ignoriert), einschließlich aller neuen
Abschluss-/Adoptionsproben, sind mit diesem unveränderten Rust-Quellcode lokal
grün. Der zuvor erstellte Test-Binary wird erneut ausgeführt. Die unveränderten
Produktiv- und Testquellen sind per SHA256 gegen den eingefrorenen Prüfstand
gebunden; das strenge Produktions-Clippy-Ergebnis von r1670 bleibt dafür gültig.
Die statische Take-Prüfung wird mit dem aktuellen Annahme-Snapshot von r1669 und
dem neuen vollständigen Manifest erneut ausgeführt. Diese lokale Qualifikation
ersetzt keine native Annahme von r1671.
