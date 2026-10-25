# r1509 - Planung einer lesbaren Oktavdarstellung

## Ausgangspunkt

Angenommener Parent: r1508, Version 0.10.1839, Take-SHA-256
`2b5b26a70bf1e4807a56ad25bea036fab31b2854d063de297712637df0450144`.
Der neue Export bindet den Source-Snapshot mit SHA-256
`c3659dc1de9ab672ed1f44c8d8e31f64e35c8c0545bb3228517e40b1901c0a9f`.
Native Evidenz: 2465 Tests bestanden, 0 fehlgeschlagen, 1 ignoriert;
Build und Clippy Exit 0, Korpus 10/10. Alle 20 XMLs sind gegen r1507 bytegleich.
Alle zehn PDFs / 50 Seiten sind bei 144 dpi mit derselben PDFium-Engine gegen
r1507 pixelgleich; die PDF-Dateibytes unterscheiden sich.

## Musikalisches Ziel

Eine lesbare Registerdarstellung soll zusammenhaengende Figuren beruecksichtigen.
Ein Oktavwechsel ist weder automatisch falsch noch allein deshalb gut, weil eine
einzelne hohe Note weniger Hilfslinien hat. Die Darstellung des ganzen lokalen
Zusammenhangs muss erklaerbar sein. 15ma bleibt eine zulaessige und teils sinnvolle
Darstellung. Es wird kein starres Drei-Hilfslinien-Limit eingefuehrt.

Die vorherige Sichtpruefung liefert einen Evaluationsfall: in Etuede 4, Takt 7-8,
oben liegen sieben reale Start-/Stop-Aenderungen des Oktavzustands. Bei Tick 144
von Takt 7 beginnt 15ma innerhalb einer Primaerbalkengruppe; H6 am Taktende hat
fuenf Hilfslinien. Eine rein rechnerische durchgehende 8va-Variante fuer die dort
sichtbaren 25 Koepfe ergibt maximal drei Hilfslinien. Diese Gegenrechnung ist kein
gerenderter Reparaturbeleg und prueft keine vollstaendige Plan-/Tie-/Custody-Kette.
Sie begruendet weder eine Taktnummernregel noch die pauschale Wahl von 8va.

## Nachgewiesener Ausgangsmechanismus und offene Frage

Die Fachplanung in `src/registernotation/mod.rs` liest vor K00 den geparsten
`model::Score`. Das Ereignismodell enthaelt Pitch, Einsatz, Dauer, Sichtbarkeit,
Akkordrolle und tie_stop, aber keine Primaerbalken-Mitgliedschaft.
`lokale_sichere_schnittindizes` verwendet zeitliche Nichtueberlappung fuer moegliche
Schnitte; die bestehende Optimierung beruecksichtigt lokale Lesbarkeitswerte und
die Strafe fuer Doppeloktavbezeichnungen. Ein Bezug zur vorhandenen sichtbaren
Primaerbalkengruppe ist in diesem Planungsmodell nicht enthalten.

Das ist noch kein Beweis, dass ein bestimmter frueher Plan dieselbe Beziehung
bereits haette konsumieren koennen. Zuerst muss ihre Verfuegbarkeit und Autoritaet
am tatsaechlichen Entscheidungseingang geklaert werden. Der finale Balken im PDF
ist kein Ersatz fuer diesen Nachweis. Der volle Eingang vor K00 wird vom normalen
Assistant-Export nicht als eigenstaendiger zeitgenauer Planer-Input mitgeliefert.

## Arbeitsschritte und Ausgaenge

1. **Dieser Take: Fehlerklasse einfrieren.** Der neue
   [Vertrag](selector_derivation_contract_register_gesture_context.md) trennt
   fehlenden Entscheidungskontext, eine gerechtfertigte interne Umschaltung und
   eine moegliche Reparatur. Es gibt keine Rust- oder Musikveraenderung.
   Der Parent-Pruefer verlangt einen bereits akzeptierten Klassenvertrag, bevor
   eine neue selector-sensitive Regel eingefuehrt wird. Siehe
   [bestehendes Template](selector_derivation_contract_template.md).
2. **Nach Annahme: echten Eingang und Gruppenbindung klaeren.** Fuer eine Gruppe
   den aktuellen musikalischen Owner, seine Phase und die Bindung an genau die
   beobachteten Ereignisse ermitteln. Positiv, negativ und UNKNOWN unterscheiden.
   Falls die Beziehung erst spaeter autoritativ entsteht, zuerst die benoetigte
   Voraussetzung definieren und pruefen; keine stille Phasenumstellung.
3. **Schreibfreie Gegenrechnung aufbauen.** Vollstaendige aktuelle Darstellung und
   vollstaendige Alternativen mit gleichem Material, gleichen echten Schluesseln
   und expliziten Aussengrenzen vergleichen. Hoechste Hilfslinienlast, aufsummierte
   sichtbare Last und Wechsel der Lesekonvention getrennt zeigen. Eine Gewichtung
   oder Regelwahl ist als eigene musikalische Entscheidung offenzulegen.
4. **Erst danach produktive Auswahl aendern.** Ein eigener Repair-Scope muss den
   unveraendert akzeptierten Klassenvertrag nennen. Bestehenden Register-Owner,
   Sichtauftrag und zentralen Writer beibehalten; tatsaechliche Materialisierung,
   Receipts, Adoption und Reentry pruefen. Keine nachtraegliche XML-Glattung.
5. **Nativer Vollkorpus und Sichtabnahme.** Ereignisse, klingende Signatur, Rhythmen,
   Sichtbarkeit, Ueberbindungen und Balken erhalten; alle veraenderten Notenseiten
   gegen den angenommenen Parent ansehen. Bessere lokale Optik darf keine neue
   unlesbare tiefe Passage, falsche Oktavgrenze oder verlorene Bindung erzeugen.

Unabhaengige synthetische Proben muessen unter anderem notwendige Registerwechsel,
sinnvolle 15ma-Spannen, ungeklaerte Gruppen, Cross-Staff und Duplikation abdecken.
Die Einzelheiten stehen im Klassenvertrag. Der Korpus liefert Breitenpruefung,
keine Auswahl-Whitelist. Es wird keine folgende Revision reserviert oder als
angenommen vorausgesetzt.

## Abnahme dieses Takes

ChangeClass RefactorOnly. Gegen r1508 aendern sich nur ZIP_KIND.txt, diese Planung,
der neue Klassenvertrag, der beratende Kurzstand 050 und das vorgeschriebene
Change-Manifest. Alle Rust-Dateien, Gates, Testdateien, Cargo-Dateien und
qualifizierten Komponentenbytes bleiben bytegleich. Root-Cargo.lock ist lokaler
Zustand und keine Take-Nutzlast. Die Lizenzbeigaben bleiben vollstaendig erhalten.

Lokale Pruefung: unveraenderten Parent-Incoming-Pruefer aus genau r1508 verwenden,
Deklaration gegen den tatsaechlichen Vollbaum vergleichen und Dead-Code-Integritaet
pruefen. Keine neuen Tests fuer reine Dokumentation. Kein neuer Build- oder
Korpusbeleg wird aus dem Parent uebernommen und als frisch ausgefuehrt bezeichnet.

Anwendung im angenommenen r1508-Projekt: `.\run.cmd --take --all`.
Erwartung: 20 XMLs bytegleich, unveraenderte PDF-Notenbilder, native Annahme von
r1509. Bis zu diesem Lauf bleibt r1509 ein Kandidat. E6/T22 und allgemeine
Spielbarkeitsreparaturen bleiben eigene Entscheidungen.
