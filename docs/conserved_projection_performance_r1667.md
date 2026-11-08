# r1667 – Weniger Allokationen im Korpus-Solver

Parent: nativ angenommener r1666 / 0.10.1839. ChangeClass: RefactorOnly.

## Gemessener Schwerpunkt

Grundlage ist `assistant_export_latest(20261107-230313).zip` des angenommenen
r1666. Der Korpus dauerte 853 Sekunden (14:13), bei drei parallelen Projekten.
Die Hauptphasen summieren sich über die Projekte wie folgt:

| Hauptphase | Summe der Projektlaufzeiten |
| --- | ---: |
| ArrangementSolver | 964,835 s |
| InitialRenderTransaction | 629,417 s |
| MetricBeamAndFinalNotation | 308,714 s |
| MasterNotationAndSalience | 233,281 s |
| ReleaseReportAndExit | 103,733 s |

Das sind überlappende Laufzeiten, keine CPU-Sekunden. Unterhalb des Solvers
entfallen 587,956 s auf GuidedSearch; darin liegen unter anderem
451,403 s LocalBestRefinement. Diese geschachtelten Zeiten werden nicht addiert.

Eine separate CPU-Stichprobe am vollständigen synthetischen Kandidatenpfad
zeigt `conserved_content_projection` in 21,1 % seiner erfassten Stacks.
Die SSC-Belegadoption liegt inklusiv bei 37,3 %; beide Anteile überlappen.
Die Stichprobe lokalisiert Arbeit und ist kein Korpus-Geschwindigkeitsvergleich.

## Änderung und Gleichheit

Nur der bestehende Owner `conserved_content_projection` erzeugt seine
Prüfzeichenkette effizienter. Kindtexte und Tie-Typen werden aus dem gelesenen
Dokument geliehen. Formatierte Felder werden direkt in die Ergebniszeichenkette
geschrieben; temporäre Feld-, Pitch- und Zeilenzeichenketten sowie der temporäre
Tie-Join entfallen.

Die Projektion besitzt dieselbe Reihenfolge, Trennzeichen, booleschen Werte,
Standardwerte, Textnormalisierung und lexikografische Tie-Sortierung einschließlich
Duplikaten. XML-Parser und Ablehnung ungültigen XMLs bleiben erhalten.
Alle Aufrufe, Belege, Custody-, SSC- und Erhaltungsprüfungen bleiben erhalten.
Es gibt keinen zusätzlichen Cache und keine Änderung der Kandidatenzahl,
Suchbudgets, Auswahlreihenfolge oder musikalischen Ausgabe.

Die alte und neue tatsächliche Projektionsfunktion liefern für alle 20
Master-/Render-MusicXML-Dateien des angenommenen Korpus exakt gleiche Bytes.
Die neue feste Randfallprobe prüft zusätzlich Namensräume, XML-Entitäten,
Pitch/Rest/Unpitched, leere und doppelte Kindtexte, sortierte doppelte Tie-Typen,
Chord/Grace/Cue, Attribute, Backup/Forward, Barline-Standards, mehrere Takte und
Parts, ein leeres Dokument und ungültiges XML.

Auf genau diesen 20 Dateien sinken die gezählten Allokations- und
Reallokationsaufrufe innerhalb der gesamten Projektionsfunktion einschließlich
XML-Parsing von 231.226 auf 43.042 (-81,4 %). Die angeforderten Bytes sinken von
177.162.516 auf 175.266.840; das ist kein Messwert für Spitzen- oder Nettospeicher.

## Reproduzierbarer Kandidatenvergleich

Der ignorierte manuelle Test
`pipeline::arrangement_variant_representation_transport_tests::arrangement_variant_performance_probe`
führt je 24 vollständige Kandidatenbewertungen mit 16 und 64 synthetischen
Takten durch. Die Eingaben entstehen vor der gemessenen Bewertung. Die Probe
prüft auch die resultierende Bindung und Lineage; sie enthält keinen flüchtigen
Zeitgrenzwert. Sie ist gezielt mit `--ignored --exact` aufzurufen.

Der Vergleich benötigt getrennte Testprogramme für den alten und neuen
Produktionscode mit derselben Probe und denselben Compileroptionen. Beide
werden abwechselnd ohne Sampling-Profiler ausgeführt. Der lokale Vergleich
verwendet Rust 1.98, das Strict-Profil mit aktivierten Overflow-Prüfungen und
lokaler Optimierungsstufe 1. Die gemessenen synthetischen Zeiten sind keine
Prognose für den nativen Windows-Korpus.

Die native Gesamtwirkung ist mit `run.cmd --take --all` und dem folgenden
Assistant-Export zu prüfen. Die 20 geprüften XML-Dateien sind bestehende
r1666-Ausgaben; sie ersetzen keinen neuen kompletten Korpuslauf mit r1667.

## Lokales Ergebnis und Abnahmegrenze

Eine Aufwärmpaarung und sechs gemessene Alt-/Neu-Paarungen wurden abwechselnd
auf demselben CPU-Kern ohne parallele Kompilierung ausgeführt. Tabelle:
Median der 24 vollständigen Kandidatenbewertungen pro Testlauf, einschließlich
der zusätzlichen Bindungs-/Lineage-Kontrolle der Probe.

| Synthetische Takte | r1666-Produktionscode | r1667 | Rückgang |
| --- | ---: | ---: | ---: |
| 16 | 0.573059 s | 0.546582 s | 4.6 % |
| 64 | 2.570343 s | 2.314659 s | 9.9 % |

Die Einzelläufe schwanken; diese Werte belegen einen lokalen synthetischen
Vergleich und sind kein garantierter Gewinn im gesamten Korpus.

Lokal bestanden 2.199 Hauptprogrammtests, 4 Tests bleiben bewusst ignoriert
(darunter die neue manuelle Zeitprobe). Die alten und neuen Produktionsprogramme
für die Messung verwenden unveränderte Checks und dieselben Testeingaben.
Auch Produktions-Clippy für sämtliche Binaries mit `-D warnings` und die
statische Take-Prüfung gegen r1666 bestanden. Ein neuer nativer Korpuslauf sowie
dessen vollständiger MusicXML-/PDF-Vergleich stehen aus.

Die isolierte Projektion auf den 20 vorhandenen Korpus-XMLs benötigt lokal
im Median 1,248776 s zuvor und 1,121500 s danach (-10,2 %), bei jeweils zwölf
Wiederholungen über alle Dateien pro Zeitmessung und sechs abwechselnden
Alt-/Neu-Paaren auf demselben CPU-Kern ohne parallele Kompilierung.
