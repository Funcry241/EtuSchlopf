# r1479: geschlossene E050-Ausgabe fuer einen Erweiterungsbeleg

Parent: r1478, aus dem vollstaendigen Source-Snapshot des nativen Exports
`assistant_export_latest(20261022-211056).zip`.
Klasse: KodexOnly. Keine Pipeline-, Writer-, Stimmen- oder Musikaenderung.

## Pruefergebnis und notwendige Schnittstelle

`OriginalBeamOriginCensus` schreibt ausdruecklich `authority=none`. Der private
`OriginalBeamVoiceGroup` transportiert authentische urspruengliche Mitgliedschaft,
aber keine vom Bruecken-Writer spaeter hinzugefuegte Verbindung. Die Differenz
zwischen Original und fertigem XML beweist keine Writer-Herkunft oder Adoption.
Sie darf nach `beam_extension_evidence_contract_r1474.md` keinen Beleg erzeugen.

E050 liefert derzeit genau Ergebnis/Audit plus einen allgemeinen PRESERVING-Beleg.
Die bestehende geschlossene Zulassung aus r1475 lehnt einen separaten typisierten
Erweiterungsbeleg als zusaetzliche Rueckgabe ab. Fuer den gewaehlten expliziten
Ausgabeweg ist daher eine im Parent angenommene Schnittstellenfreigabe notwendig.
Ein Beleg im Diagnose-Text oder eine versteckte Autoritaet im Audit wird vermieden.

## Exakte Zulassung

Nur die bestehende E050-Owner-Identitaet, ihre urspruenglich eingefrorene Signatur,
Klassifikation PRESERVING und die unveraenderten r1475-Eingaenge werden zugelassen.
Die vorhandene r1475-Rueckgabe bleibt erlaubt. Genau eine weitere Form darf hinter
dem unveraenderten Ergebnis/Audit und genau einem optionalen PRESERVING-Beleg
folgenden dritten Tuple-Eintrag liefern:

`Option<structural_write_authority::RestBridgeExtensionOccurrenceEvidence>`

Andere Typen, Referenzen, Pflichtbelege, zusaetzliche Eintraege, geaenderte Inputs,
fehlende/andere/doppelte Transformationsbelege und mehrfache Owner-Definitionen
bleiben verboten. Die Normalisierung auf die urspruengliche physische Oberflaeche
erhaelt deren Instanz-, Buffer- und Dateischreibbudgets.

Das ist eine begrenzte Erweiterung der Signaturzulassung. Sie fuehrt den genannten
Typ noch nicht ein, aendert keine produktive Signatur, erzeugt keinen Beleg und
autorisiert weder neue Writer noch eine Rekonstruktion aus dem fertigen XML.
Der bestehende Trust-Anchor bindet die endgueltigen Gate-Bytes erneut; keine
Population-, Exclusion-, Sink-, Budget- oder sonstige Pin-Tabelle wird geaendert.

## Folgetake r1480: Transport und beam-only Reparatur

Erst nach nativer Annahme dieses Takes:

1. Der echte E050-Writer erzeugt den privaten typisierten Beleg aus seinem Input,
   seiner gewaehlten Operation und seinem tatsaechlich realisierten Output.
   Er bindet Source/Attack/Fragment, ganze Notencontainer und Akkordanhaenge,
   die urspruengliche Gruppe und genau die neu geschriebene Erweiterung getrennt.
2. Nur erfolgreiche Adoption derselben Kandidatenbytes aktiviert den Beleg.
   Verworfene, fehlgeschlagene und Shadow-Kandidaten liefern keine Main-Autoritaet.
3. Jede dazwischenliegende tatsaechliche Owner-Transition wird geprueft. Identitaet,
   Timing, Akkordbindung und relevante Beam-/Lane-Zustaende bleiben getrennte
   Verpflichtungen. Nicht unterstuetzte Drift entwertet den Beleg; entfernte
   Verbindungen duerfen nicht wieder entstehen. Kein freies Rebinding aus Output.
4. Gepruefte Uebergabe an FinalChord- und PDF-Source-Continuation. Bei der spaeten
   Stimmenausrichtung wird die tatsaechliche gleichzeitige Zuteilung ausgewertet.
   Unvertraegliche belegte Erweiterungen entfallen; Stimmen, originale Mitgliedschaft
   und urspruengliche Paare bleiben unveraendert. Kein Voice-Move als Ersatzreparatur.
5. Synthetische Positiv-/Negativfaelle gemaess unveraendertem r1474-Belegvertrag;
   einschliesslich Verwurf, fremde/veraltete Identitaet, Timing-/Beam-Drift,
   vollstaendige Akkorde, Umbenennung, Verschiebung und unabhaengige Duplikation.
   Der neue Repair-Scope verweist auf die unveraenderte kausale r1474-Definition.

Evaluationsziel, niemals produktiver Selektor: E6/T32 bleibt bei Position 192/216
ein Paar, 264 ohne Balken. Die drei r1478-Korrekturen in E3/T6 und E5/T36/T37 sowie
E6/T34 bleiben erhalten. Erwartet gegen r1478: alle Masters und neun Render-XMLs
bytegleich, in E6 nur die beschriebenen Beam-Tags; keine Noten-, Pausen-, Dauer-,
Stimmen-, Haltebogen- oder Bogen-Aenderung. Zusaetzliche Wirkungen sind ein Befund
und duerfen nicht als stillschweigende Scope-Erweiterung ausgeliefert werden.

## Abnahme r1479

Fuenf neue Gate-Tests pruefen die exakte Zulassung, die Owner-/Klassenbindung,
negative Signaturvarianten, die reale Owner-Pruefung und unveraenderte Schreibbudgets.
Die bisherigen Signaturtests bleiben bestehen. Produktiver Source ist bytegleich
zum Parent. Lokal: Parent-Take-Gates auf Candidate und entpacktem Paket, Gate-Tests,
Strict-Checks und Clippy. Native Auswertung mit einem Befehl:

`.\run.cmd --take --all --writer-trace`

Erwartet: dunkelgruener Lauf, alle 20 XMLs bytegleich zu r1478 und alle 50 PDF-Seiten
pixelgleich. r1479 behauptet keine Reparatur von T32 und keine fertige Implementierung
des Erweiterungsbelegs. Vollstaendige Lauf-/Pruefergebnisse werden separat uebergeben.
