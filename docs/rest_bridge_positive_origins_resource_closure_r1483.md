# r1483: positive Balkenurspruenge und vollstaendige Pruefressourcen

Parent: nativ angenommener r1482, SHA-256
43295871f3ed161a244dee3eb4774cf0813b748103323c528db0f25a848f3d73.
Version 0.10.1839; GovernedChange mit erklaerter Balkenwirkung.

## Anlass und Ursache

Der native r1482-Lauf ist accepted, 10/10 dunkelgruen, 2442 Tests bestanden,
einer uebersprungen, Clippy sauber. Alle 20 Master-/Render-XMLs sind bytegleich
zu r1478. Das Reparaturziel wurde dadurch nicht erreicht. Export:
assistant_export_latest(20261023-095019).zip; eingebetteter Source-Snapshot:
b7359f7d2915722ba28ec13e32742438255da58bff66414845d7ce7513bfe33b.

Die Aufnahme verlangte eine einzelne urspruengliche Primaergruppe, erhielt aber
bereits die ueber gemeinsame Quellen verbundenen Komponenten. Das Paar der
bekannten T32-Stelle gehoert zu einer solchen Komponente aus zwei Gruppen und
wurde nie aufgenommen. Zusaetzlich schloss die alte Aufnahme alle ausserhalb
der geschriebenen Verbindung liegenden Fragmente derselben Quelle/Attacke aus.
Die Originalgruppen-Zusammenlegung fuer die Stimmenausrichtung bleibt richtig.
Die fruehere Vermutung eines verlorenen Projektions-Receipts wurde widerlegt.

## Aenderung

Jede positive Gruppe bleibt beim echten Original-Owner erhalten, bevor sie mit
anderen Gruppen zur Stimmenkomponente zusammengelegt wird. Ihre Zuordnung zur
Komponente entsteht direkt aus dieser Zusammenlegung; Diagnose-IDs werden
nicht zerlegt. Die Aufnahme verwendet genau eine vollstaendige positive Gruppe
innerhalb der vom echten E050-Writer ausgewaehlten Koepfe als Balkenursprung.

Die ganze zugehoerige Originalkomponente sowie alle authentisch gebundenen
Fragmente derselben Quelle/Attacke und deren Akkordkohorten werden bis zum
Fixpunkt als Ressourcen mitgefuehrt. Fehlende Fragmente verhindern die Aufnahme.
Ressourcen haben keine Stimme bei der Trennungsentscheidung und werden nicht
zu Ruecknahme-Schreibzeilen. Die vorhandene Stimmenentscheidung muss die ganze
urspruengliche Komponente zusammenhalten.

Geteilte Pruefressourcen sind erlaubt. Jede Verbindung prueft ihre Ressourcen;
Balkendrift pensioniert alle davon abhaengigen Verbindungen und laesst andere
Verbindungen bestehen. Schreibziele muessen disjunkt sein; Ueberlappung bleibt
Fehler. Eine pensionierte Verbindung entsteht nicht aus einem spaeter wieder
passenden Bild neu.

FinalChord transportiert Mitglieder und Ressourcen ueber seine wirklichen
Output-Occurrences. Bei diesem autorisierten Projektionsschritt muss jede
Ressource ihre belegte sealed_voice erhalten; normale Erhaltungsschritte
verlangen ihre unveraenderte Stimme und ihren unveraenderten Zustand.
Die globale pitched-sequence-Pruefung und das Stimmenkriterium in through
bleiben unveraendert. Der durchgehende Test liefert bereits ohne deren Aenderung
Ruecknahmezeilen. Die spaete OriginalBeamVoiceAlignment-Entscheidung und die
fruehe E050-Sperre werden nicht veraendert.

Die Diagnose nennt Projektpfad, Main-/Shadow-Zweig, Grund, positive
Ursprungszuordnung sowie Mitglieder-/Ressourcen-Identitaeten und Positionen.
Der Shadow-Zweig wird an der echten Planungs-Kopie markiert. Das aendert keine
Identitaet, Stimme, Auswahl, Dateischreibbefugnis oder Main-Adoption.

## Pruefung und Abnahme

Der synthetische Durchlauftest benutzt den echten Original-Balken-Owner, zwei
Gruppen mit gemeinsamer Quelle, angebundene Akkordnoten und eine angebundene
Erweiterungsfortsetzung. Er durchlaeuft echte E050-Ausgabe und Main-Adoption,
Output-Label-Alpha-Pruefung, FinalChord-Projektion mit fremdem Stimmenwechsel,
Cursor-Fortsetzung, echte spaete Komponentenentscheidung, E050-Ruecknahme und
die vorhandene skalare Stimmenmaterialisierung samt Occurrence-Transport.
Er verlangt identische Nicht-Balken-Inhalte gegen die unveraenderte Zuteilung
und wiederhergestellte Original-Balkenanzahl. Der Verschiebungs-Pruefstand
prueft u. a. Umbenennung, Relokation, Attributvererbung und Duplikation.
Gegenproben: fehlende Ressourcenfragmente, mehrdeutiger Ursprung, ueberlappende
Schreibziele, gemeinsamer Ressourcen-Balkendrift, keine Wiederauferstehung.

Nativ ausfuehren:

    .\run.cmd --take --all --writer-trace

Evaluation, keine Selektoren: E6/T32 soll 192/216 als Paar behalten und 264
ohne Balken zeigen. Stimmen und alle Master-XMLs muessen gegen r1482 gleich
bleiben. Weitere Render-Differenzen sind vollstaendig zu erklaeren; die drei
Feinschliff-Bruecken bleiben gesperrt und T34 bleibt geschuetzt. PDF-Sichtpruefung
und native Musikabnahme bleiben separat. MuseScore-Varianten bleiben unveraendert.
