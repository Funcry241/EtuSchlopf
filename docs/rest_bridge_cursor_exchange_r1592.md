# r1592: produktiver E057-Tausch vor dem endgültigen PDF-Commit

MusicMutation auf dem nativ angenommenen r1591. Die angenommene Fehlerklasse
und sämtliche Gate-/Signaturzulassungen bleiben unverändert.

Der bestehende E057-Owner bekommt die zugelassene optionale kanonische Eingabe.
Ein expliziter typisierter Adapter im bestehenden Receipt-Wrapper wandelt die
bisherigen XML-only-Aufrufe intern in None um; ihre Aufrufstellen und der
geschützte Leser bleiben unverändert. Der gebundene Aufruf übergibt ausdrücklich
XML und den unveränderlichen kanonischen Guard als Paar. Es gibt keinen Callback,
keinen globalen Modus und keinen weiteren Receipt-Issuer.
Some verwendet ausschließlich den tatsächlich gebundenen Plan; ein ungültiger
Guard löst keinen Rückfall auf den alten Selektor aus.

Im bestehenden finalen PDF-Quellabschluss wird der Plan nach endgültiger
Stimmenzuordnung und Schlüssel-/Pausenplatzierung gebunden. Nur echte lebende
Verbindungskurse mit vollständigen aktuellen Occurrence-Bindungen liefern die
Toncontainer. Eine eindeutig deckende vorhandene Pausenspur und passende
vorhandene Forward-Platzhalter erlauben den Tausch vollständiger Ton-/Akkordblöcke.
Disjunkte Operationen bleiben unabhängig, auch im selben Takt. Geteilte
Ressourcen bilden Konfliktgruppen und werden ohne globalen Sieger abgelehnt.

Der echte E057-Owner erzeugt den Kandidaten; sein bestehender Wrapper mintet
die ursprüngliche einzelne PRESERVING-Quittung. Der unabhängige Transportprüfer
prüft Quittung, vollständigen Byte-Fußabdruck und die tatsächliche Geometrie.
Die vorhandenen Source-/Attack-/Fragment-Zeugen werden an die neuen Positionen
getragen. Gleiche Tonhöhen oder Einsatzzeiten rekonstruieren keine Identität.
Auch unverrückte Töne mit verschobenem Index, Akkordanker, bestehende Balkenbilder
und sämtliche mitgeführten Verbindungen werden vollständig abgebildet.

Der Guard hält die ursprüngliche Projektions-Epoche und den kanonischen
Quell-Arc fest. Gleiches XML aus einer fremden Epoche genügt nicht. Der
gewöhnliche reihenfolgeerhaltende Transport bleibt unverändert streng.
Die ursprüngliche Quittung läuft in der vorhandenen finalen Render-Transaktion;
erst deren erfolgreiche Übernahme und exakter Readback aktivieren das Ergebnis.

Die bestehenden Zeitlinien-, Cursor-, Schlüssel-Fixpunkt-, Pausen-, Balken-
und finalen Occurrence-Gates bleiben aktiv. Es gibt keinen zusätzlichen
MuseScore-Aufruf pro Brücke und keine neue Mess- oder Berichtssektion.

Die erweiterten vorhandenen E057-Adoptionstests verwenden ausdrücklich
synthetisch deklarierte bereits übernommene Eingänge. Sie prüfen den echten
E057-Writer, seine Quittung, Transport, vollständige Akkorde und identische
Unisoni sowie erfolgreiche, fehlgeschlagene und falsche Sink-Readbacks.
Neue kanonische Tests prüfen lokale Unabhängigkeit, geteilte Konflikte,
fehlende ursprüngliche Verbindungskurse und fremde Projektions-Epochen.
Die bereits zugelassenen E050-Owner-Proben prüfen zusätzlich: Aus den dort
wirklich geschriebenen und in Main übernommenen frischen Verbindungen entsteht
der erwartete kanonische Tauschplan. Ein vollständiger E050→E057→PDF-Lauf wird
damit nicht vorgetäuscht.

Frische postprojektierte E050-Verbindungen bekommen einen getrennten Beleg.
Er entsteht ausschließlich aus den tatsächlichen ausgewählten Eingangsnoten,
ihren vorhandenen kanonischen Bindungen und dem ursprünglichen E050-Ergebnis.
Die bestehende Quittungsprüfung und der begrenzte Balkenfußabdruck bleiben
unverändert. Weder ein erfundener alter Balkenursprung noch spätere Endtags
ersetzen die ursprüngliche Ausführung. Erst die erfolgreiche Main-Übernahme
aktiviert diesen Beleg. Seine vollständigen Akkord-/Fragmentressourcen bleiben
bei späteren Schritten gebunden; relevante Abweichungen ziehen den Beleg
endgültig zurück. Die bisherigen Erweiterungs- und Rücknahmebelege behalten
ihre eigene Zuständigkeit.

Die realen Stellen 1T13, 5T6 und 7T5 sind Evaluation. Ein erfolgreiches endgültiges
Notenbild wird erst nach Prüfung des tatsächlichen Produktivlaufs behauptet.
Native Anwendung: `run.cmd --take --all` ohne Zusatzflag.
