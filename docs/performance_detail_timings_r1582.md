# Beobachtender Performance-Messtake r1582

RefactorOnly auf dem nativ angenommenen r1581 / 0.10.1839.
Keine musikalische Optimierung und kein behaupteter Laufzeitgewinn.
Alle Suchbudgets, Kandidaten, Identitaeten, Receipts, Reihenfolgen,
Auswahlbedingungen, Fehlerpfade, Gates und Writer bleiben erhalten.

## Anlass und Ausgangspunkt

Der native r1581-Zehnerkorpus benoetigt 893 s gegenueber 867 s in r1580.
Alle zwanzig Master-/Render-XMLs sind bytegleich. Ein einzelner Lauf beweist
keine Regression; eine Gesamtbeschleunigung ist damit nicht nachgewiesen.
Die Indexoptimierung hat den dominanten Aufwand nicht ausreichend reduziert.

r1581-Zeiten pro Projekt, nicht zwischen parallelen Projekten addieren:

| Etuede | Solver | MasterNotationAndSalience | Projektstufen gesamt |
|---|---:|---:|---:|
| 5 | 299,326 s | 114,566 s | 481,996 s |
| 6 | 383,376 s | 74,347 s | 553,242 s |
| 7 | 187,200 s | 123,400 s | 423,021 s |

## Messstellen

Neue DetailTiming-Zeilen unterteilen die bestehenden Solverstufen in ihre
unveraenderten Such-/Reparaturpaesse und die einzelnen Reconciliation-Schritte.
SubstanceGuard, ConsensusGestureProbe und die angrenzenden Herkunftspruefungen
bleiben getrennt sichtbar. Auch bedingt uebersprungene Paesse koennen minimale
Durchlaufzeiten besitzen; eine DetailTiming-Zeile behauptet keine ausgefuehrte
musikalische Mutation.

Die Master-Notationsfunktion selbst bleibt bytegleich: Der angenommene Kodex
friert ihren gesamten Body als positive WAV-Metronom-Konstruktion ein.
Ihr unveraenderter Aufrufer setzt einen rein beobachtenden Thread-Kontext mit
Projekt und Stufe. Zulaessige Unterfunktionen messen darin Oktavierung,
untere Schluesselpolitik, Notationspolitur, Exportnormalisierung,
EditorialVoiceDecision, zertifiziertes Rewrite-Portfolio und die allgemeine
Material-Reconciliation mit bestehendem Ownerlabel. Ausserhalb dieses Kontexts
erzeugen diese Unterfunktionsmarker keine Zeilen. Der Kontext wird bei
normaler Rueckkehr und Fehlern wiederhergestellt und nicht an Worker vererbt.

Diese Unterfunktionszeiten sind inklusiv und koennen verschachtelt sein.
Insbesondere innere Reconciliation nicht erneut zu einem aeusseren Pass
addieren. Nicht separat messbar bleiben in diesem Take die eingefrorene
Expression-Funktion und die eingefrorene Layoutdatei. Ihr Aufwand sowie
sonstige Adoptions-/Auditkosten bleiben als nicht aufgeloester Rest der schon
vorhandenen gesamten MasterNotationAndSalience-Zeit sichtbar. Keine Pins,
Deskriptoren, Admission- oder Kodex-Regeln werden angepasst.

Die Profilbatches melden separate Wandzeiten fuer Seed-Vorbereitung,
ExactSeedEquivalence, Finalisierung eindeutiger Seeds und Clone/Rebind.
Die parallele Seed-Finalisierung sowie ihre Arrangement-Varianten erhalten
jeweils einen lokalen Zaehlerblock. Dadurch werden initiale Bindung, Parsing,
Checks, Baseline-Arrangement, Variantenbewertung und HumanPotential-Trials
unterscheidbar. Innerhalb der Varianten werden auch Polish, Layout,
OwnedRhythm-Transport und Receipt-Adoption getrennt gemessen.

Der AtomicPatch-Pfad misst Normalisierung, originale Owner-Fortfuehrung,
Receipt-/Auditpruefung, XML-Parsing, run_checks, Kandidatenkopie,
CompleteContext/Geburtstransport, Adoption/MaterialLineage und Abschlussstatistik.
Vorpruefung, Worker-Auswertung und Sortierung/Sammlung haben eigene Wandzeiten.

## Protokoll und Bedeutung

CandidateTiming schema=1 erscheint einmal pro betroffenem Batch, niemals
pro Kandidat oder Note. Arrays phases, phase_calls, phase_sum_us und
phase_max_us sind positionsgleich. Aufrufe fuehren lokale Zeitwerte;
erst beim Verlassen werden atomare Summen, Maxima und Zaehler zusammengefuehrt.
Es gibt keine globale Registry und keine Vermischung unabhaengiger Batches.
Projekt, Batch und vorhandene Startiteration dienen ausschliesslich der
Zuordnung im Protokoll. Die Iteration ist keine neue musikalische Identitaet.

- batch_wall_us: gesamte Lebensdauer des Messblocks, einschliesslich Vorbereitung,
  Verwaltung und Sammlung; kein exklusiver Worker-Timer.
- call_sum_us: Summe der Aufruf-Wandzeiten. Parallelzeiten ueberlappen.
- call_max_us: laengster einzelner Aufruf.
- phase_calls: Anzahl Aufrufe, die den betreffenden Abschnitt erreicht haben;
  mehrmaliges Betreten zaehlt fuer diesen Aufruf einmal.
- phase_sum_us und phase_max_us: Summe bzw. Maximum pro Abschnitt.
- calls: alle beim Verlassen erfassten Aufrufe, auch Fehler-/Fruehrueckgaben.
- tail_calls: normale Abschlussmarkierung erreicht. Kein musikalisches Urteil.
  Eine kleinere Zahl beweist keinen Fehler: erfolgreiche Fruehrueckgaben sind
  ebenso moeglich und unveraendert erlaubt.

Alle Zeiten sind monotone Instant-Messungen in ganzen Mikrosekunden. Kleine
Abschnitte koennen auf null abgerundet werden. Mess-/Ausgabeaufwand ist real;
Phasensummen muessen daher nicht exakt die Aufruf- oder Projektzeit ergeben.
Maxima verschiedener Abschnitte koennen von verschiedenen Aufrufen stammen.
Uebergeordnete und untergeordnete Zeiten niemals doppelt addieren. Es werden
keine CPU-Zeiten, RSS-Spitzen oder Wartezeitursachen behauptet.

Die neuen Beobachtungen schreiben best-effort nach stderr. Ihr Ausgabefehler
wird nicht als fachlicher Fehler propagiert. Normale Fehler-/Fruehrueckgaben
publizieren per Drop die bereits gemessene Arbeit; harter Prozessabbruch kann
die Schlusszeile verlieren. Es gibt keine veraenderte Abbruch-/Erfolgspolitik.
Kein produktiver Leser wertet Zeiten oder Zaehler fuer Entscheidungen aus.

## Abnahme

Parent-kompilierte Eingangspruefung, vollstaendige Rust-Tests und produktiver
Clippy-Scope sind vor Auslieferung erforderlich. Vier separate synthetische Proben gegen den exakten neuen Messcode pruefen
konkurrierende Zusammenfuehrung, Batch-Isolation, Abschnittserreichbarkeit,
unveraenderte Fehlerrueckgabe, erfolgreiche Fruehrueckgabe sowie verschachtelte,
threadlokale und bei Fehlern wiederhergestellte Beobachtungskontexte.

Nativ normaler Zehnerlauf ohne -f, danach assistant_export_latest.zip.
Alle zwanzig XMLs muessen bytegleich zu r1581 bleiben. Neue CandidateTiming-
und DetailTiming-Zeilen muessen im vorhandenen Konsolenexport stehen und den
teuren Stufen eindeutig zugeordnet werden koennen. Dieser Messtake soll die
naechste gezielte Optimierung begruenden; er senkt selbst kein Suchbudget.
Die musikalischen Fragen bleiben pausiert. Keine GitHub-Veroeffentlichung.

Die vier zusaetzlichen Proben werden als separates optimiertes Rust-Pruefprogramm
ausgefuehrt und im externen Pruefbericht dokumentiert. Sie sind keine neuen
Produkttests; die vollstaendige vorhandene Testserie bleibt unveraendert aktiv.
