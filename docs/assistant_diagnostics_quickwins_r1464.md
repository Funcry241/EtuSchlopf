# r1464 – Diagnose-Vorabcheck und Exportvergleich

ChangeClass: RefactorOnly. Version 0.10.1839. Angenommener Parent: r1463,
SHA-256 `2a79cff78bc4629596cb37bf12e72452981e863ea564c20dfec8a5dd5ffea383`.

Der Nutzer hat die Installation der zuvor separat gelieferten beiden
Hilfspakete als regulären Take beauftragt. Dieser Take legt die Werkzeuge
unter `tools/assistant_diagnostics` ab und übernimmt die gekürzte 050.
Die vorherige 050 bleibt durch den Parent-Snapshot historisch rekonstruierbar;
eine weitere alte Vollkopie wird nicht wieder in den aktiven Handoff eingebaut.

Keine Rust-Produktivdatei, kein Cargo- oder Watch-Lock, keine Gate-Logik, kein
Root-Starter und keine musikalische Entscheidung werden geändert. Die
Python-Werkzeuge sind optionale lesende Diagnosen und erzeugen lediglich
neue Berichte außerhalb der untersuchten Projekte. Der normale Gesamtlauf
führt sie nicht automatisch aus und benötigt dafür keine Python-Abhängigkeit.

Der Vorabcheck kontrolliert deklarierte Dateihashes und undeclared Änderungen
gegen eine ausdrücklich gewählte Referenz. Standard ist die eingefrorene
r1463-Referenz; spätere Benutzer können mit `--reference-tree` ihren separat
bestätigten tatsächlichen Elternbaum verwenden. Diese Referenz ist kein neuer
Trust-Anchor. Die kanonische Take-Prüfung bleibt unverändert und vollständig.
Versionen und PDF-/SVG-Smoke müssen tatsächlich erfolgreich sein.

Der Exportvergleich unterscheidet verwaltete Dateien von lokalem Snapshot-
Zustand, XML-Bytegleichheit von PDF-Pixelgleichheit und neue Testlogs von
gemeldeter Receipt-Wiederverwendung. Er erteilt keine musikalische oder native
Abnahme. Die Grenzen und Aufrufe stehen in der installierten Werkzeug-README.

Lokale Qualifikation umfasst synthetische Python-Tests, echten MuseScore-
PDF-/SVG-Smoke über den vorhandenen Linux-Wrapper, den Vergleich der beiden
nativen r1463-Exporte, unveränderte Eltern-Incoming-Prüfung und isolierte
installierte Reexec-Prüfung. Die tatsächliche native Annahme steht bis zum
frischen Benutzerlauf aus. Für diesen RefactorOnly-Take müssen alle 20 XMLs
gegen den letzten angenommenen r1463-Korpus bytegleich und die PDF-Notenbilder
unverändert bleiben. Der bekannte Bogenverlust und Takt 40 werden nicht behoben.

Die Musikarbeit wartet weiterhin auf Dachs' Vorprüfung. Dieser zusätzliche
Hilfsmittel-Auftrag erweitert keine musikalische Reparaturklasse und hebt die
vereinbarte Obergrenze für die danach zu entscheidende Musikarbeit nicht auf.
