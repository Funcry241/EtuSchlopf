# r1480: exakte gemeinsame Zulassung des FinalNotation-Belegtransports

Parent: vorbereiteter r1479, E050ExtensionEvidenceOutputAdmission.
Klasse: KodexOnly. Der technische Name muss nach dem bestehenden Parent-Vertrag
`SscK2GovernanceRevisionKodex` sein. Keine produktive Quell- oder Musikaenderung.
Native Annahme von r1479 ist fuer die Anwendung Voraussetzung, noch nicht behauptet.

## Warum der Zwei-Take-Plan korrigiert werden muss

Der Reparaturentwurf mit der r1479-Ausgabe erzeugt den echten E050-Beleg und fuehrt
ihn ueber die Metrik-Owner bis zur FinalNotation. Dort sind drei weitere explizite
Uebergaenge erforderlich: LowerClef, CompleteBeat und CrossingRest unmittelbar vor
der typisierten FinalChord-Uebergabe. Das private Stage-Input braucht dazu einen
veraenderlichen kanonischen Plan statt eines ausschliesslich lesenden Borrows.

Der unveraenderte r1479-Parent lehnt diesen konkreten Entwurf vor dem Kopieren ab:

`Checkpoint58 exact prepared successor changed: src/pipeline_shards/pipeline_orchestrator_final_notation_support.rs`

`render_checkpoint_prepare_kodex.rs` bindet den gesamten FinalNotation-Einstieg
und `structural_write_authority/mod.rs` an exakte vorbereitete Bytes. r1479 laesst
nur die E050-Rueckgabe zu und aendert diese separate Grenze nicht. Ein Transport
allein anhand des spaeteren XMLs waere kein Ersatz fuer die ausgelassenen Owner.
Die zusaetzliche Freigabe folgt damit einem konkreten Parent-Gate-Befund.

## Eng begrenzte neue Zulassung

Genau folgende beiden vorbereiteten Bytebilder werden nur gemeinsam als weitere
Nachfolger ihrer bestehenden Pins zugelassen:

| Pfad | SHA-256 |
| --- | --- |
| src/pipeline_shards/pipeline_orchestrator_final_notation_support.rs | 6efa601f56f8b8948bd99e723318d8a222fa4ec4311fbfc3b21a24298210c06b |
| src/structural_write_authority/mod.rs | b4035a8ad5c0b35e934e6aef38d737863238a7d4f19a2877a25d537bde98ed68 |

Das erste Bild aendert nur den privaten Plan-Borrow, fuegt die drei genannten
Belegfortsetzungen ein und haelt danach den bestehenden lokalen Plan-Borrow
explizit lesend. Kein Writer, Sink, Receipt-Replay, Shadow/Main-Commit, Gate,
Rueckfallpfad oder Musikselektor dieses geschuetzten Einstiegs wird ersetzt.
Das zweite Bild fuegt genau den bestehenden SerializationPlan-Exportnamen
`RestBridgeExtensionOccurrenceEvidence` zur kanonischen Exportliste hinzu.
Die Implementierung dieses Typs ist ein spaeterer, separat gepruefter Bestandteil.

Teilpaare, kopierte Digests an anderen Pfaden, Drift, doppelte oder zusaetzliche
Zeilen erteilen keine Zulassung. Bestehende Parent-/E007-Nachfolger bleiben
unveraendert erlaubt; alle anderen Checkpoint-Pins bleiben exakt. Insbesondere
werden `render_adopted_state.rs`, InitialRender, SSC-Commit und #58 nicht geaendert.
Die SHA-Werte sind im Parent-Code geschlossen, keine beschreibbare Konfiguration.
Der einzige Trust-Anchor wird auf die endgueltigen Gate-Bytes aktualisiert.

Diese Zulassung behauptet weder Compilergruennheit noch Sicherheit der gesamten
kuenftigen Reparatur. Sie oeffnet nur diese zwei konkreten Bytebilder. Die anderen
Parent-Gates, produktiven Budgets, Selector-/Identitaetsvertraege und Compiler
muessen den vollstaendigen spaeteren Take weiterhin unabhaengig akzeptieren.

## Aktualisierte Reihenfolge

1. r1479: E050 darf den separaten typisierten Erweiterungsbeleg ausgeben.
2. r1480: die zwei geschuetzten FinalNotation-Transportstellen werden exakt zugelassen.
3. r1481: Writer-Beleg, Adoption, gepruefte Fortsetzungen und beam-only Ruecknahme
   vor der bereits gebundenen spaeten Stimmenzuteilung. Noch kein gelieferter Take.

Evaluationsziel bleibt E6/T32: urspruengliches Paar 192/216, Note 264 ohne Balken,
keine Stimmenaenderung. Die vier r1478-Korrekturen bleiben erhalten. Der Korpus
bleibt Evaluation und wird weder zum Selektor noch zu einer normativen Testfixture.

## Pruefung und native Abnahme dieses Takes

Sieben neue Gate-Tests pruefen die exakte Paarzulassung, Reihenfolge, unveraenderten
Parent, Teilpaare/Drift, fremde Pfade, Zusatz-/Duplikatzeilen und alle bestehenden
Pins am echten kopierten Parent-Quellbaum. Keine Korpus-MusicXML wird benutzt.
Zusaetzlich: Parent-Take-Gates auf Baum und entpacktem ZIP, normaler Strict-Clippy,
Quellvergleich aller produktiven Rust-Dateien. Diese Ergebnisse werden separat
berichtet; Tests sind keine native Annahme.

Nach nativer Annahme von r1479 mit genau einem Befehl anwenden:

`.\run.cmd --take --all --writer-trace`

Erwartet: alle 20 XMLs bytegleich und alle 50 PDF-Seiten pixelgleich zum Parent.
T32 ist in r1480 ausdruecklich noch nicht repariert.
