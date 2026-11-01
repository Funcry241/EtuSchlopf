# r1549 - JointRegisterDpC3Policy

Parent: accepted r1548 / 0.10.1839.
Accepted take SHA-256:
`1f175dda184b449120ed85a9a5ab748c0f0b632e00b888bd159f8ca5a996a43a`.
Bound source snapshot SHA-256:
`bef7491cc2f59ca40e4d95c72ff2ea3ee33e80f5405dd23d872ad63b915fbc15`.
ChangeClass: KodexOnly. Candidate; native acceptance pending.

## Konkreter Inhalt dieses Takes

Dieser Take legt die Vergleichspolitik
[Gemeinsame Registerplanung C3 v1](joint_register_dp_policy_v1.md) fuer die
anschliessende produktive Umsetzung fest. Die Reihenfolge lautet:

    h8,h7,h6,h5,h4,ko,uk,ks,gb,h3,ch,h2,h1

Konventionelle Schluessel-/Oktavierungskombinationen stehen vor kurzen
Registerabschnitten. Der Kopfschutz verhindert neue oder verschlechterte
Belastungen ab vier Hilfslinien je sichtbarem Kopf. Gruppenbrueche sind gezaehlt;
musikalische und materielle harte Pflichten gelten weiter.

Die Annahme dieses Kandidaten ratifiziert genau diese vorgeschlagene
Projektkonvention. Das ist keine Behauptung universeller Lesbarkeit. Die
unveraenderte bestehende kausale Klasse bleibt von der Praeferenz getrennt.

Alle Rust-Dateien, Tests, Cargo-Dateien im verwalteten Baum, Gates, Pins,
Launcher und produktiven musikalischen Entscheidungen bleiben bytegleich zu
r1548. Aenderungen: ZIP_KIND.txt, diese Notiz, neue Policy und das erklaerende
Take-Change-Manifest. Keine Python- oder XML-Diagnosewerkzeuge werden in den
produktiven Baum aufgenommen. Root Cargo.lock bleibt lokaler Zustand und ist
nicht Bestandteil des eingehenden Takes.

Dieser Vertrags-Take verbessert selbst noch kein PDF. Der nachfolgende
Umsetzungstake muss die Policy beim bestehenden fachlichen Owner integrieren,
die echte Ausgangsfassung im Graphen erhalten und alle Nachweise liefern.
Seine Revisionsnummer und Annahme werden hier nicht vorweggenommen.

## Evaluationsbeleg, strikt getrennt von der Policy

Dachs' Register-DP v3 mit korrigiertem Einzelabschnitt-Randfall wurde unter der
C3-Reihenfolge auf den zehn r1548-render.musicxml-Dateien erneut ausgefuehrt.
Der Dateiparser und die Gruppenbildung sind Prototypannahmen; ihre Ergebnisse
sind keine Faelligkeits- oder Reparaturbedingungen. Der Prototyp optimiert nur
Registerdarstellung bei fester Systemzuordnung.

| Merkmal, gleiche Kopfsegmentprojektion | Ausgang | C3 |
|---|---:|---:|
| Kurze Registerabschnitte | 183 | 6 |
| Unkonventionelle Koepfe | 710 | 0 |
| Koinzidenzen | 12 | 0 |
| Registerwechsel | 390 | 221 |
| Gruppenbrueche | 134 | 13 |
| Koepfe mit mindestens vier Hilfslinien | 7 | 0 |
| Koepfe mit drei Hilfslinien | 59 | 108 |
| Hilfsliniensumme | 1692 | 2245 |

Die echte r1548-XML hat 409 Registeroperationen und 9 Koinzidenzen im
Prototypzaehler. Die Werte 390/12 entstehen durch Verschieben auf
Kopfsegmentgrenzen. Der produktive Vertrag fordert deshalb ausdruecklich die
echte Ausgangsfassung und keine solche Projektion als Ersatz. Die obigen
Zahlen sind keine zugesicherten Resultate des spaeteren nativen Planers.

Gegenueber B+uk in derselben Diagnose sinken kurze Abschnitte von 84 auf 6 und
Wechsel von 282 auf 221; Dreihilfslinienkoepfe steigen von 46 auf 108 und die
Hilfsliniensumme von 1878 auf 2245. Dieser Preis ist offengelegt.
Die 84 kurzen Abschnitte bei B+uk sind mit ks als letztem Tie-Break nach allen
primaeren B+uk-Merkmalen reproduziert. Der alte B+uk-Pfad mit seinem bisherigen
Aenderungs-Tie-Break hat dieselben primaeren Werte, aber 88 kurze Abschnitte.
Diese Mehrdeutigkeit diagnostischer Groessen ist kein Widerspruch im C3-Optimum;
der neue Vertrag legt seine sekundaere deterministische Auswahl separat fest.

Die Vollaufzaehlung wurde fuer die tatsaechliche C3-Reihenfolge erneut
ueber 24 Fenster aus Etueden 3 und 6 ausgefuehrt, je fuenf Segmente und zehn
Zustaende: DP und Vollaufzaehlung stimmen ueberein. Die Fenster verwenden eine
Viertelnote als Mindestdauer, um Abschnittszaehlung im kurzen Fenster zu
aktivieren; dies ersetzt keine Pruefung aller Zeit-/Abschnittsrandfaelle.
Die beiden Auswertungen nutzen dieselben lokalen Kosten und Gruppen.

Die Bildprobe Etuede 6, Takte 25-44, wurde mit MuseScore 4.7.4 aus einer
separaten Diagnose-MusicXML gerendert. Sie zeigt insbesondere die verbliebenen
kurzen Abschnitte um Takt 32-34 und die Beseitigung der langen G/15mb-
Darstellung aus C2. Das ist Sichtpruefung, kein nativer Umsetzungslauf.
Alle Orte und Zahlen in dieser Notiz sind Evaluationsbelege, keine
normativen Test-Fixtures oder Selektoren.

## Pruefung und Anwendung

Lokal geprueft: verwalteter Paketbaum gegen den gebundenen Parent-Snapshot,
vollstaendiger deklarierter Diff, FNV je Aenderung, unbekannte Top-Level-Pfade,
Windows-Pfadkollisionen, bytegleiche Rust-/Test-/Cargo-/Gate-Bestaende sowie
ZIP-Integritaet. Das ist ein mechanischer Vorabcheck, kein Ersatz fuer die
nativen Take-Gates.

Rust/Cargo sind in dieser Arbeitsumgebung nicht verfuegbar. Build, Clippy,
native Rust-Tests, Runtime-Gates und native Annahme dieses Takes wurden hier
nicht ausgefuehrt. Die 2656 bestandenen Tests und 10/10 erfolgreichen Projekte
des Exports gehoeren zu r1548, nicht zu diesem Kandidaten.

Anwenden auf akzeptiertem r1548:

    .\run.cmd --take --all

Vor einer produktiven Implementierung muss der frische Export die native
Annahme dieses exakten Vertrags-Takes belegen. Die Registerbilder bleiben in
diesem Take produktiv unveraendert.
