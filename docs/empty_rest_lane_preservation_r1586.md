# r1586 – Vorhandene Pausen leerer technischer Stimmen erhalten

MusicMutation auf nativ angenommenem r1585. Dies ist ein musikalischer
Korrektur-Take mit unveränderter vorhandener Diagnose.

## Belegter Übergang

r1585 wurde nativ angenommen; 2895 Tests bestanden, drei regulär ignoriert,
Clippy ohne Warnung, alle zwanzig MusikXML-Dateien bytegleich zu r1583.
Im exportierten Readback sind alle 195/375 unterschiedlichen Takttexte der
Projekte 1/2 anhand ihrer ursprünglichen SHA256 und Bytezahl rekonstruierbar.
In 1T13 führt Beobachtung 27 den tatsächlichen Render-Eingang D10 nach D11;
in 2T29 führt dieselbe Schreibgrenze D9 nach D10. Beide entfernen die vier
sichtbaren unteren Viertelpausen 0..96, 96..192, 192..288, 288..384 und ersetzen
sie durch nackte Forwards. Die spätere Master-FinalChord-Projektion teilt die
entsprechenden ursprünglichen Restträger dagegen nach klingender Belegung.

Der tatsächliche Eingang besitzt in beiden Fällen sichtbare Tonereignisse im
unteren und oberen System unter derselben technischen Stimme, aber keine
sichtbaren Töne in der unteren Pausenstimme. Damit erfüllt er den alten
volltaktigen Leer-Stimmen-Zweig des ursprünglichen Suppressions-Owners.
Das belegt keine kanonische Stimmenrolle und ist keine Schreibfreigabe.

## Korrektur

Der bisherige Owner kann aus leerer technischer Stimme, Aktivität anderswo,
ganztaktiger Abdeckung oder Oktavzeichen keine entbehrliche musikalische
Pausenpflicht ableiten. Er bewahrt jetzt die sichtbaren Pausen solcher leeren
Stimmen. Das gilt für einzelne Ganztaktpausen, geteilte Ganztaktpausen sowie
Teilpausen in beiden Systemen. Auch verborgene Töne beweisen keine sichtbare
technische Pausenrolle. Die Reparatur benutzt keine Etüden-/Taktnummer oder
gezielte Kombination von Zeitwerten, Stimmen- und Systembezeichnungen.

Die zwei allein auf leeren Stimmen beruhenden Shortcuts werden entfernt.
Die verbleibenden cross-staff und oberen Regeln überspringen leere Stimmen;
für bereits sichtbar klingende Stimmen bleibt der ursprüngliche Pfad erhalten.
Diese Erhaltung behauptet keine neue Rollenkenntnis und bestätigt nicht
pauschal die Begründung aller verbleibenden Legacy-Elisionen.

Der ursprüngliche Writer, seine Quittungen und Adoptionsprüfungen bleiben.
Ein unveränderter Zustand erhält keine Quittung. Es werden keine verborgenen
Pausen sichtbar gemacht und keine Forwards in neue Pausen umgewandelt.
Alle kanonischen FinalChord-, Pausen-, Balken- und Zustandsprüfungen bleiben
aktiv. Keine Änderung an Pins, Kodex, Ressourcen oder Suchbudgets.

## Prüfung und musikalischer Umfang

Sechs synthetische Tests benutzen den tatsächlichen Owner: Staff-Tausch,
Stimmenumbenennung, Zeitskalierung, Einzel-/Mehrfach-/Teilpausen, verborgene
Töne, unabhängige Verdopplung, Idempotenz, exakte Erhaltung und fehlende
Quittungen. Ein positiver Kontrollfall erhält den ursprünglichen Pfad einer
bereits klingenden Stimme samt Quittung. Die reine Vorschlagsbildung wird zur Prüfung aus dem bestehenden Owner
ausgelagert; derselbe ursprüngliche kanonische Materialisierer prüft die
Vorschläge und Quittungen. Neue Legacy-Writer-Aufrufer werden nicht eingeführt.
Ein zusätzlicher Nullbudget-Fall prüft fehlende Vorschläge und Quittungen.

Korpusdateien sind ausschließlich Messmaterial und werden nicht als normative
Fixtures oder als Reparaturselektor eingebaut. Die native Abnahme muss 1T13,
2T29 und den ganzen Korpus einschließlich 6T6 visuell und anhand der XMLs prüfen.
Die erwartete Wirkung ist die Erhaltung der Pausen bis zur ursprünglichen
kanonischen Projektion. Die richtige Balkenbrücke hängt weiter von deren
positiver Original-/Stimmenfreigabe und dem Zeitpunkt der vorhandenen Regeln
ab; eine abgeschlossene Balkenkorrektur wird vor der nativen Prüfung nicht
behauptet. 6T6-Pausenzusammenfassung bleibt separat.

Normaler Ablauf: run.cmd --take --all, ohne -f und ohne Zusatzflags.
