# Tatsächliche Pausen-Übergangsbytes — r1585

AuditOnly auf nativ angenommenem r1583. In dessen belegten Enddateien besitzt
1T13 im Master sichtbare untere Pausen, im Render keine. Auch die untere Pause
72..96 in 2T29 fehlt im Render. Die bestehenden Endepochenberichte können den
konkreten verantwortlichen Übergang nicht beweisen. Die vorhandene allgemeine
Balken-Auswahl findet für die erste untere Notengruppe eine Brücke, wenn ihre
exakte sichtbare Lückenpause vorhanden ist. Auswahl ist keine Schreibfreigabe.

Dieser Take behebt noch keine Notation. Er schließt ausschließlich die fehlende
physische Beobachtung, bevor ein musikalischer Eingriff ausgewählt wird:

* Bereits aufgerufene Lineage-Checkpoints erfassen ihre tatsächlichen XML-Bytes.
* Bereits beobachtete erfolgreiche Render-Dateischreibungen erfassen ihre
  tatsächlichen Vorher-/Nachher-Operanden, auch bei unveränderten Bytes.
* Bereits gemeldete, quittungsgestützt adoptierte FinalChord-Projektionen
  erfassen Eingang, unmittelbare Projektion und adoptierte Fortsetzung. Das
  geschieht unabhängig davon, ob die alte Kollisionsdiagnose einen Treffer hat.

Die Beobachtung läuft im normalen Projektlauf ohne neues Kommandoargument.
Der bestehende Abschnitt RENDER-EPOCH-CONTINUITY enthält anschließend den
Inhalt ab `RestBoundaryByteReadback: active.`. Er listet jede Beobachtung und ihre
konkreten Dokumentreferenzen mit SHA256. Pro Dokument bleiben alle physischen
Part-/Taktvorkommen mit Original-Bytebereichen erhalten. Bytegleiche Takttexte
werden nur für die Ausgabe dedupliziert. Jede XML-Zeile erhält das reversible
Präfix `RestReadbackXmlLine: `. Nach Entfernen dieser Präfixe ergeben die ersten
angegebenen Bytes den exakten Originaltext, einschließlich CRLF und Schlusszeile.
Die Deduplikation prüft vollständige Bytegleichheit; ein Hash allein genügt nicht.
Fehlende Eingänge und nicht parsebare Dokumente bleiben ausdrücklich UNAVAILABLE.

D/M-Nummern, SHA256, Bytebereiche, Pfad-Stream-Fingerprints und XML-Labels sind
physische Fundstellen, keine musikalischen Identitäten, Stimmenrollen oder
Pausenverpflichtungen. Gleiche Koordinaten beweisen keine Fortsetzung. Benachbarte
Beobachtungszeilen beweisen keinen Verarbeitungsschritt. Ein Sink-Label benennt
eine Persistenzgrenze und kann mehrere Owner umfassen. Fremde Threads,
Kandidaten ohne übernommenen Zustands-Checkpoint und nicht instrumentierte
Zwischenschritte bleiben außerhalb der Beobachtung; sie werden nicht geraten.

Der Zustand ist an dieselbe Thread-Session wie der vorhandene Render-Recorder
gebunden, beginnt bei Projektstart neu und wird beim Session-Ende verworfen.
Vollständige Dokumente werden bis zur Berichtsausgabe im Speicher gehalten;
bytegleiche Dokumente werden innerhalb dieser Session dedupliziert. Kein Cache
wirkt auf Validatoren, Kandidaten oder musikalische Entscheidungen. Die lokale
Zusatzlast und der größere Export sind Diagnosekosten dieses Takes.

Keine Änderung an Produktions-Ownern, Orchestrierung, Rollenplänen, Schreibrechten,
Quittungen, Gates, Pins, Suchbudgets, Cargo oder musikalischen Regeln. Die
Quellenänderungen liegen ausschließlich in audit_only und der Take-Metadatei.
Es gibt keine neue Klassifikation eines musikalischen Fehlers und keinen
Reparaturselektor nach Etüden-/Takt-, Stimmen- oder Systemnummer.

Sieben synthetische Tests prüfen konkrete Operanden und No-op, fehlendes/defektes
XML, unabhängige Verdopplung mit gleichen Labels, Hash-Kollision im Index,
Session-/Thread-Trennung sowie unveränderte nackte Forwards, versteckte Pausen
und Akkordmitglieder. Ein Integrationstest prüft den Standardlauf ohne optionales
Trace-Flag einschließlich der kollisionsfreien Adoptionsmeldung. Keine Korpusdatei wird als normative Testquelle eingebaut.

Nächster nativer Lauf: ./run.cmd --take --all, ohne -f und ohne Zusatzflag.
Erwartung: 10 Etüden erfolgreich; 20 MusikXML-Dateien bytegleich zu r1583.
Der neue Export muss tatsächliche Render-Projektionseingänge und rohe Taktbytes
enthalten. Erst die daraus belegte Ursache bestimmt den folgenden musikalischen
Take. 1T13 und 2T29 bleiben offen; 6T6 wird nicht nebenbei umgeschrieben.

## Korrektur nach dem nativen r1584-Versuch

r1584 scheiterte nach erfolgreichen nativen Tests (2893 bestanden, 3 ignoriert)
und warnungsfreiem Clippy an der Berichtserzeugung aller zehn Projekte. Die neue
unterstrichene Großbuchstaben-Überschrift wurde als nicht deklarierte Sektion des
Producers render_epoch_continuity abgelehnt; die Transaktion stellte r1583 wieder
her. Der neue Take startet deshalb weiterhin auf r1583, enthält die gleiche
Diagnose und repariert ausschließlich deren Ausgabeintegration.

Die Diagnose verwendet eine gewöhnliche Body-Zeile innerhalb der unveränderten
bestehenden Sektion. Auch eingebettetes XML erhält auf jeder physischen Zeile ein
Präfix: Originaltext kann dadurch selbst keine zusätzliche Berichtssektion öffnen.
Die exakten XML-Bytes bleiben anhand der deklarierten Bytezahl rekonstruierbar.
Zwei weitere synthetische Tests rufen die echte DraftReport/CompactedReport-
Projektion mit dem vollständigen Epoch-Body auf, reproduzieren die alte Ablehnung
und prüfen Erfolg sowie Verlustfreiheit einschließlich CRLF, Umlauten, fehlendem
Zeilenende und einer scheinbaren Überschrift innerhalb des XML. Auch die Ausgabe
ohne aktive Session wird geprüft. Kein Registry-, Parser-, Gate- oder Pin-Eingriff.
