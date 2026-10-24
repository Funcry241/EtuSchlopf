# Wiederholte Validierung unveraenderlicher Ursprungs-Custody

RefactorOnly auf angenommenem r1455. Kein neuer Cache, keine Identity-, Writer-,
Repair- oder Release-Authority. Der Nutzer hat diesen Quick Win auf einen Take
begrenzt; Messungen und Prototypen sind Vorarbeit.

OwnedRhythmicContinuation::receive validiert courses gegen xml vollstaendig,
bevor es die gepruefte Kopie als initial_custody: Arc speichert. Der untersuchte
Typ hat keine innere Veraenderlichkeit. Produktionskonstruktion erfolgt hier;
Fortfuehrungen klonen den Zustand und veraendern dieses Arc-Feld nicht.
Die erneute Vollvalidierung ausschliesslich dieses identischen Ursprungszustands
in validate_binding entfaellt. Alle uebrigen Pruefungen bleiben bestehen.

initial_custody bleibt ausdruecklich erhalten: validate_consolidated_origins
vergleicht seine urspruenglichen history-/coverage-Daten mit der heutigen Herkunft.
Sein Feld, seine Konstruktion, der Arc-Transport und saemtliche Konsolidierungs-,
Snapshot-, Intervall-, Origin-, Encoding- und physische Abdeckungspruefungen
bleiben bytegleich. Das Array-/Objektmodell und die Fehler-/Unknown-Weitergabe
werden nicht umgebaut. Ein neuer Konstruktionsweg oder veraenderliche Ursprungs-
Daten wuerden diese Begruendung ungueltig machen und neue Pruefung verlangen.

Ein zusaetzlicher synthetischer Test prueft, dass falscher Snapshot, fehlende
Herkunft, doppelte Course-Identitaet und unvollstaendige Unterstuetzung bereits
beim Empfang abgewiesen werden. Bestehende negative Tests fuer heutige Herkunft,
Rest-Konsolidierung und finalen Export bleiben unveraendert.

Die externe Messvorarbeit kompiliert den exakten alten Validator und den nur
um diesen Aufruf reduzierten Prototyp im selben optimierten Testprogramm.
Identische synthetische Owner-Zustaende, wechselnde Reihenfolge und wiederholte
Messungen trennen Validator-Gewinn von End-to-End-Behauptungen. Erfolgreiche und
beschaedigte aktuelle Zustaende werden gegen beide Bodies auf dieselben Resultate
und Fehlermeldungen geprueft. Der Messcode kommt nicht in den Take.

Diese Messung beweist keine bestimmte Beschleunigung des ganzen Korpus: Die
exportierten Dateien enthalten keine MIDI/WAV-Originaleingaben fuer einen lokalen
vollstaendigen Wiederholungslauf. Native Abnahme: 10/10 frisch ohne -f, alle 20
XMLs gegen r1455 bytegleich, unveraenderte Report-/Kanal-/Overlap-Readbacks.
Native Laufzeit gesondert messen und keine Mikrobenchmark-Prozente auf den
gesamten Lauf uebertragen. Suchbudget, Worker, Kandidatenreihenfolge und Auswahl
bleiben unveraendert. Custody und r1451 bleiben pausiert.
