# Frische physische rhythmische Auslesung teilen — r1583

RefactorOnly auf nativ angenommenem r1582. Der gemessene Hauptanteil der
Kandidatenbewertung liegt in der OwnedRhythm-Fortfuehrung. Die vollstaendige
validate_binding-Pruefung erstellt bereits physical_inventory(xml) und dessen
by_range-Index. validate_encoded liest danach dasselbe unveraenderte xml noch
einmal vollstaendig aus und erzeugt denselben Index erneut.

Die erste produktive Aenderung: validate_binding leiht den gerade aufgebauten
Index an validate_encoded. Dessen einziger produktiver Aufrufer bleibt derselbe.
Die Daten stammen aus der frischen vollstaendigen Auslesung genau dieses xml;
kein aufrufuebergreifender Cache, kein Snapshotwechsel, kein Ersatzbestand und keine neue Identitaet.
Der geliehene Index lebt nur innerhalb dieses einen Validatoraufrufs.

Die zweite Aenderung liegt im selben frischen Leser: collect_note_metas liest
bisher fuer jede Note die unveraenderten Taktattribute erneut aus. Eine lokale
OnceCell berechnet RhythmContext::default().update_from_measure(measure_text)
beim ersten tatsaechlich erzeugten NoteMeta genau einmal und kopiert danach
diesen identischen Copy-Wert. Bei leerem oder vorher abgebrochenem Notenlesen
wird weiterhin kein solcher Wert benoetigt. Kein Kontext wird aus anderen
Takten geholt; selbst das bisherige Default-Verhalten bleibt unveraendert.
Die Zelle lebt nur waehrend eines einzelnen collect_note_metas-Aufrufs.

Die wiederholte identische Auslesung, deren zweite Indexkonstruktion und die
identischen Taktattribut-Auswertungen pro weiterer Note entfallen.
Saemtliche Encoding-, Emissions-, Ancestry-, Identitaets-, Intervall-, Staff-,
Tie-, Chord- und Partitionspruefungen bleiben unveraendert. Auch forwards(xml)
wird weiter vollstaendig ausgefuehrt, einschliesslich seiner Fehlerpfade, selbst
wenn keine Encodings vorhanden sind. Validate-Binding-Reihenfolge, Memo-Bindung,
Konsolidierung und Wurzelabdeckung bleiben erhalten. Kein Suchbudget-, Auswahl-,
Schreibbefugnis-, Notations-, Kodex- oder Pin-Eingriff.

Vier testexklusive vollstaendige alte Bodies aus angenommenem r1582
(beide Validatoren und beide zugrundeliegenden Leser)
vergleichen Resultat und genaue Fehler. Positive Quellen stammen aus den echten
unveraenderten Seed-/FinalChord-/Staff-Ownern: gleichzeitige unabhaengige Koepfe,
mehrere Systeme und Takte, Encodings und vollstaendige/partielle Forward-Obligationen.
Negative Zustaende pruefen falsche Dokumentbindung, fehlende Herkunft und
Vorkommen, Emissionsdrift, verschobene Carrierbereiche, doppelte Encodings und
fremdes XML nach einem erfolgreichen Memo-Treffer.

Ein zusaetzlicher Feldvergleich prueft saemtliche abgeleiteten NoteMeta-Felder
fuer leere, einzelne und dichte Notenfolgen, mehrere Stimmen/Systeme, Backups,
Forwards und vorhandene, fehlende, ungueltige sowie nichtpositive Taktattribute.

Eine separate synthetische Testmessung vergleicht volle kalte Validatoraufrufe;
Cloning liegt ausserhalb der Messung, sechs Wiederholungen mit wechselnder Reihenfolge, oberer Median in Mikrosekunden.
Es gibt keine Zeitgrenze als Testbedingung und keinen daraus abgeleiteten nativen
Beschleunigungsanspruch. Die lokale Strict-Testkonfiguration ist nicht optimiert.

Nativ: alle zehn Projekte normal ohne -f, alle zwanzig XMLs bytegleich zu r1582.
CandidateTiming- und DetailTiming-Zeilen bleiben zur Erfolgskontrolle aktiv.
Die Oktavierung und ConsensusGestureProbe bleiben in diesem Take unveraendert.
Ein Einzel-Eingriff wird getrennt gemessen. Die musikalischen Fragen bleiben pausiert.
