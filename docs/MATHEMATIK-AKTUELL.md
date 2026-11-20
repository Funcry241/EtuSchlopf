# Mathematik in EtuSchlopf

**Wie lange soll eine gespielte Note im Notenbild stehen?** EtuSchlopf leitet die Dauer einer frei gespielten Klaviernote aus den Einsätzen, der Artikulation und den gleichzeitig laufenden Stimmen ab. Im Windows-Vollkorpuslauf r1965 stimmen bei Mozarts KV 331 alle 474 regulären Einsätze samt Tonhöhe und 460 ihrer geschriebenen Dauern mit der annotierten Einspielung überein. Die nachstehenden Formeln zeigen, wie die Entscheidung entsteht und welche Eigenschaft die MusicXML-Ausgabe tatsächlich belegt. [Mozart-Notenbild r1965](examples/mozart-k331-p01-r1965.pdf) · [Windows-Vollkorpus r1965](evidence/r1965/README.md)

| Frage | Wo die Antwort steht |
| --- | --- |
| Wie gelangt eine frei gespielte Note in den gedruckten Takt? | [Vom Anschlag zur geschriebenen Dauer](#vom-anschlag-zur-geschriebenen-dauer) |
| Welche Zusammenhänge beschreibt das Modell für Stimmen, Register und Seiten? | [Verfahrensfamilien](#verfahrensfamilien-im-aktuellen-quellstand) und [Formeln](#zentrale-formeln-und-ihre-beweisgrenzen) |
| Was wurde mit welchen Stücken geprüft? | [Windows-Vollkorpus r1965](evidence/r1965/README.md), [Stimmfortsetzungen im Vollkorpus r1961](#stimmfortsetzungen-im-vollkorpus-r1961) und [Prüfstände](#prüfstände-und-verbleibende-grenzen) |
| Woran ließe sich als Nächstes arbeiten? | [Nächste mathematische Vorhaben](#nächste-mathematische-vorhaben) |

## Wie die Aussagen zu lesen sind

| Einordnung | Bedeutung |
|---|---|
| Bekanntes Verfahren | Zum Beispiel Intervallgraphen, dynamische Programmierung, Goertzel-Rekurrenz oder lexikografische Ordnung. |
| Projekteigene Modellierung | Unsere Ereignisdefinitionen, Zulässigkeitsbedingungen, Kosten und Prioritäten. Ihre Allgemeingültigkeit folgt nicht aus der Bekanntheit des Verfahrens. |
| Projekteigene Zusammenstellung | Ein Projektbegriff verbindet mehrere Bedingungen. Daraus folgt keine wissenschaftliche Neuheitsbehauptung. |

| Rolle | Was daraus folgt |
|---|---|
| Entscheidet | Ein bestehender fachlicher Owner kann das Ergebnis unter seinen weiteren Bedingungen für eine Auswahl oder Materialisierung verwenden. Nicht jede Auswertung wird übernommen. |
| Misst / prüft | Liefert Beobachtungen, Gegenbeispiele oder eine Prüfung; erhält dadurch keine musikalische Schreibbefugnis. |
| Technische Erhaltung | Verändert Aufwand oder Aufbewahrung unter einem Äquivalenznachweis, ohne eine neue musikalische Präferenz einzuführen. |
| Extern / historisch | Ein Diagnoseexperiment oder ein alter Messstand, kein aktueller Produktnachweis. |
| Entwurf | Noch keine produktive Implementierung der beschriebenen Gesamtaufgabe. |

`UNKNOWN`, eine unvollständige Suche und eine bewiesene Unzulässigkeit sind unterschiedliche Ergebnisse. Fehlende Information wird weder zu Nullkosten noch zu einer Erlaubnis. „Optimal“ bedeutet höchstens optimal im ausdrücklich gebundenen Modell und Suchraum.

## Die drei Projektbegriffe

### SchlopfFormel für belegte Übergänge

Unter diesem Namen fasst diese Seite die Bedingungen für einen erlaubten, belegten Übergang zusammen. Es gibt im Code keine einzelne kanonische Gesamtfunktion namens SchlopfFormel.

Ein ausführbarer Teilkern ist die Entscheidung über die Einstufung **eines** Änderungsbelegs:

```math
A(e)=[p(e)=1]\land[m(e)+d(e)+i(e)=0].
```

$p$ zählt erlaubende Einstufungen, $m,d,i$ zählen fehlende, doppelte und ungültige Belege. Die Zähler sind keine Wahrscheinlichkeiten. Der Kern steht in `adoption_decision` in [ssc_evidence_proof.rs](../src/ssc_evidence_proof.rs). Er ist allein noch keine vollständige Freigabeformel.

Die Zusammenstellung verlangt außerdem die zuständige Authority, die Herkunft des Belegs, den gebundenen Vor- und Nachzustand, die jeweils erforderlichen Erhaltungsbedingungen und den Schutz gegen unzulässige Wiederverwendung. Die konkreten Prüfungen und ihre Zuständigkeit bleiben in den ausführbaren Verträgen. Master und Render besitzen getrennte Herkunftsketten.

**Grenze:** Eine zulässige Transformation ist damit nicht automatisch musikalisch gut. Ein Fingerprint ersetzt keine Ereignisidentität und kein musikalisches Qualitätsurteil.

### Schlöpfchen-Formel für die Seitenfüllung

Der Entwurf ordnet zulässige Seitenpläne nach drei Kriterien.

```math
x^\star\in\underset{x\in\mathcal F}{\mathrm{argmin}_{\mathrm{lex}}}
 \bigl(N_{\mathrm{Seiten}}(x),D(x),U(x)\bigr).
```

$D$ bezeichnet übermäßige Dehnung, $U$ die Unausgewogenheit der Seitenfüllung. Der lexikografische Vergleich entscheidet zuerst über die Seitenzahl und bei Gleichstand über Dehnung, dann Ausgewogenheit.

$\mathcal F$ müsste die tatsächlich lesbaren und materialisierbaren Kandidaten enthalten. Lesbarkeit ist darin Voraussetzung. Horizontale Taktprofile, vertikale Systemhüllen und Seitenbelegung sind getrennte Größen; die Systemreihenfolge bleibt erhalten.

**Status:** Diese Gesamtformel ist nicht als vollständiger Seitenoptimierer implementiert. Die horizontale Systemplanung und der begrenzte gemeinsame Seiten- und Systemplan sind bereits implementiert. Für eine allgemeine Seitenverdichtung fehlen belegte sichtbare Mindestabstände und eine durchgängige Vergleichsgrundlage. Aus einer großen beobachteten Lücke folgt weder ein Mindestabstand noch automatisch die Erlaubnis zum Zusammenrücken.

[Fehlerklasse zur unnötig beibehaltenen Seite](selector_derivation_contract_redundant_page_retention.md) · [Implementierter Seiten- und Systemplan](../src/pipeline_shards/horizontal_layout_global_page_system.rs)

### Symbolische Spielformel für Hand und Finger

```math
\exists a:\ S(a)\land K(a)\land U(a)\land W(a).
```

$a$ belegt physische Verpflichtungen mit Hand-/Finger-Plätzen. $S$ steht für Strukturbedingungen, $K$ für statische Konfigurationen, $U$ für Übergänge und $W$ für Wiederanschläge gleicher Tasten. Anforderungen der Musik und Fähigkeiten eines Pianistenmodells bleiben getrennt.

**Status:** Audit. Die Formel und ein struktureller Beleg können vollständig sein, während die Fähigkeiten eines konkreten Menschen weiterhin nicht modelliert sind. Ein `MODEL_RELATIVE_ONLY`-Urteil beweist weder ein bestimmtes Spieltempo noch menschliche Spielbarkeit. Eine fehlende Fähigkeitseinstufung rechtfertigt keine Notenlöschung.

Quelle: [pianistic_symbolic_capability_reachability.rs](../src/audit_only/pianistic_symbolic_capability_reachability.rs) und die dort konsumierten Fähigkeitsmodelle.

## Vom Anschlag zur geschriebenen Dauer

### Das Zeitraster einer freien Einspielung

Die WAV-Aufnahme stützt die zeitliche Zuordnung zur MIDI-Einspielung. Wenn der direkte MIDI-Import kein geeignetes Raster liefert, sucht EtuSchlopf einen von den Anschlüssen, Wiederholungen und dem örtlichen Puls getragenen Taktverlauf. Mehrdeutige Taktarten oder eine unzureichende Übereinstimmung mit dem Audio können die Übernahme verhindern. Im Notenplan entsprechen 96 Einheiten einer Viertelnote. [Automatische Zeitplanung](../src/free_performance_timing.rs) · [Einbindung in den Projektlauf](../src/pipeline_shards/pipeline_orchestrator_project_setup_stages.rs)

Angenommen, zwei benachbarte Gruppen setzen in der Aufnahme bei den Zeiten $t_a,t_b$ ein. Ihre bereits gewählten Positionen im Notenraster seien $q_a,q_b$. Lässt der Pianist eine Taste zum Zeitpunkt $r$ dazwischen los, ergibt die örtliche lineare Zeitabbildung

```math
q(r)=q_a+(q_b-q_a)\frac{r-t_a}{t_b-t_a},\qquad t_a\le r\le t_b.
```

Der Abstand zum Notenanfang liefert einen Hinweis auf die geschriebene Länge. Ein staccato gespieltes Viertel kann dieselbe Tastendauer haben wie ein gebundenes Achtel. Deshalb fließen auch die folgenden Einsätze und andere Töne desselben Akkords ein. Nach dem letzten Anschlag setzt das Modell das örtliche Tempo für die Auswertung der letzten Loslasszeiten fort. [Dauerherleitung](../src/free_performance_timing.rs)

### Folgeeinsatz oder Akkordende

Für jeden regulären Anschlag $i$ vergleicht das Modell mögliche Folgeeinsätze $j$. Die Kosten $c_{ij}$ bewerten unter anderem das Verhältnis von beobachteter Dauer zum Abstand der Einsätze, den Tonhöhenabstand und die Notierbarkeit. Zusätzliche freie Spalten erlauben ein eigenes Ende. Jeder reale Folgeeinsatz steht höchstens einmal zur Verfügung. Für die gewählten Zuordnungen $x_{ij}$ gilt

```math
\min_x \sum_{i,j}c_{ij}x_{ij},\qquad
x_{ij}\in\{0,1\},\quad \sum_jx_{ij}=1,\quad \sum_ix_{ij}\le1.
```

Ein Akkordton kann auch dann enden, wenn eine andere Stimme weiterläuft. Für einen Ton ohne eigenen Folgeeinsatz prüft EtuSchlopf die Loslasszeiten der Akkordpartner auf derselben Spur und demselben Kanal. Mit der Streuung $w$ der **Anschlagszeiten** und den **Loslasszeiten** $r_i,r_j$ gilt für einen möglichen Zeugen

```math
w=\max_k t_k-\min_k t_k,\qquad
r_i,r_j>\max_k t_k,\qquad |r_i-r_j|\le w.
```

Die passenden Partner mit eigenem Folgeeinsatz müssen sich über das geschriebene Ende einig sein. Ein erneuter Anschlag derselben Taste vor diesem Ende schließt die Übernahme aus. Ein weiterer Fall betrifft die Pause im gleichmäßigen Akkordpuls. Dort müssen an zwei vorherigen Pulsen alle Akkordtöne unabhängig dieselbe Unterteilung bestätigen, die nächste Attacke muss eine Lücke lassen und die gemessene Artikulation muss im zuvor beobachteten Bereich liegen. Offen gebliebene Dauern werden nach der Zuordnung zum Notensystem im Kontext der benachbarten Einsätze entschieden. [Akkordenden und Puls](../src/free_performance_timing.rs) · [Offene Dauern im Notensystem](../src/midi_seed.rs)

### Ein Quellton über mehrere geschriebene Fragmente

Ein regulärer Ton soll das halboffene Intervall von $a$ bis $b$ belegen. Die aus demselben Anschlag erzeugten Fragmente müssen eine lückenlose Partition bilden. Für ihre Anfänge $s_k$ und Enden $e_k$ gilt

```math
s_1=a,\qquad e_m=b,\qquad s_k<e_k,\qquad e_k=s_{k+1}\quad(1\le k<m).
```

Dadurch hat ein bei 72 beginnender Ton mit Dauer 96 auch dann sein Ende bei 168, wenn auf demselben System bei 120 eine andere Note beginnt. EtuSchlopf kann dafür einen weiteren MusicXML-Stimmkanal verwenden. Bei einer Teilung behält jedes Fragment die Quellidentität und Tonhöhe. Nur das erste trägt den Anschlag, die folgenden schließen mit Haltebögen an. Ein Vorschlag hat eigene Regeln für Nullzeit und Hauptnote. Der Schreibpfad prüft die Zeiten gegen die tatsächlich ausgegebenen Noten und Cursorbewegungen. [Fragmentplanung und Ausgabekontrolle](../src/midi_seed/bound_duration.rs)

| Mozart KV 331, Andante grazioso, Einspielung p01 | r1963 | r1964 | r1965 |
| --- | ---: | ---: | ---: |
| Reguläre Noten mit passendem Einsatz und passender Tonhöhe | 474 von 474 | 474 von 474 | 474 von 474 |
| Passende geschriebene Dauer einschließlich Haltefortsetzungen | 438 von 474 | 460 von 474 | 460 von 474 |
| Abweichende geschriebene Dauer | 36 | 14 | 14 |

Der Vergleich verwendet MIDI und WAV als Programmeingaben. Die annotierte Zuordnung aus dem Vienna-4×22-Korpus dient erst zum Prüfen von Master- und Render-MusicXML. Die 14 Differenzen begrenzen die Aussage über die Dauererkennung an dieser Aufnahme. Der Erhalt der gewählten Intervalle und die Übereinstimmung dieser Wahl mit der musikalischen Referenz sind zwei getrennt messbare Eigenschaften. [Annotierte Einspielung](https://github.com/wergo/vienna4x22/blob/53872cd670182b039ff118d27abcba30de1e0785/match/Mozart_K331_1st-mov_p01.match) · [Windows-Einzellauf r1964](examples/mozart-k331-p01-r1964.pdf) · [Windows-Vollkorpus r1965](examples/mozart-k331-p01-r1965.pdf)

## Verfahrensfamilien im aktuellen Quellstand

Die Quellstellen nennen die jeweiligen Implementierungsstellen, nicht automatisch einen universellen mathematischen Satz über das gesamte Programm. Wissenschaftliche Grundlagen, Originalarbeiten und ihr jeweiliger Bezug stehen gesondert im Abschnitt [Quellen](#quellen).

| # | Aufgabe und Mathematik | Rolle und wichtigste Grenze | Quellstelle |
|---|---|---|---|
| 1 | Exakte Zeitrechnung: rationale Zahlen, ggT/kgV, gemeinsame Zeitachsen | Entscheidet / prüft. Geschriebene und klingende Zeit bleiben verschieden. | [Metrische Pausen](../src/model/metric_rest_notation.rs), [temporale Beobachtung](../src/clef_readability_observation/temporal.rs) |
| 2 | Ereignisintervalle, Akkordanker und Fortsetzungen | Grundlage für Entscheidungen und Prüfungen. Ein Pausenanker kann klingendes Material tragen. | [serialisierte Zeitprüfung](../src/audit_only/serialized_temporal_replay.rs), [ursprüngliche Erfassung](../src/pipeline_shards/actual_beam_capture_core.rs) |
| 3 | Materialbilanz und getrennte Herkunfts-/Sichtbarkeitsmaße | Misst. Anzahlen sind kein individueller Identitätsnachweis. | [material_conservation.rs](../src/audit_only/material_conservation.rs) |
| 4 | R1–R5: Zustandsautomaten für Schlüssel, Oktavierungen und Haltebögen | Misst. Diese Regeln sind kein vollständiger Notationsbeweis. | [notation_invariants.rs](../src/audit_only/notation_invariants.rs) |
| 5 | Koinzidenzen: Gleichheit von Einsatz und Taste unter Quellbindung | Misst. Kein universeller Platzierungs- oder Löschauftrag. | [material_coincidence.rs](../src/audit_only/material_coincidence.rs) |
| 6 | Stimmenbedarf im Intervallmodell: Cliquenzahl und Färbung | Misst eine modellgebundene Mindestzahl, keine Erlaubnis zum Zusammenlegen musikalischer Stimmen. | [notation_invariants.rs](../src/audit_only/notation_invariants.rs) |
| 7 | Gleichheitsklassen, Trennbedingungen und Widerspruchspfade der geschriebenen Zuordnung | Readback / Graphprüfung, kein neuer Allokator. Weitere Beweispflichten bleiben ausdrücklich offen. | [written_assignment_graph.rs](../src/structural_write_authority/written_assignment_graph.rs), [Probe](../src/structural_write_authority/written_assignment_graph_probe.rs) |
| 8 | Lexikografische Kandidatenwahl | Entscheidet. Frühere Kriterien können durch spätere Vorteile nicht aufgewogen werden. | `CandidateDecisionKey` in der Pipeline |
| 9 | Verteilung verborgener Akkordtöne: quadratische Kosten plus diskrete Strafen | Entscheidet unter weiteren Zulassungsprüfungen. Gewichte sind Modellierung, keine kalibrierte Wahrnehmung. | [hidden_chord_reduction_texture_rebuild_tail.rs](../src/pipeline_shards/hidden_chord_reduction_texture_rebuild_tail.rs) |
| 10 | Gesten- und Kettenvarianten der Verteilung | Kandidatenentscheidungen. Die konkrete Variante und ihre musikalischen Wirkungen müssen einzeln nachgewiesen werden. | dieselbe Verteilungsfamilie und ihre bestehenden Owner |
| 11 | Hilfsliniengeometrie und gradueller Druck | Beobachtung und Kosten des separaten Schlüsselplaners. Kein universelles Vier-Hilfslinien-Verbot. | [clef_readability_observation.rs](../src/clef_readability_observation.rs), [Schlüsselplanung](../src/structural_write_authority/rustig_clef_planning.rs) |
| 12 | Separater Schlüsselverlauf: dynamische Programmierung | Entscheidet im jeweils zuständigen Pfad. Bei gebundenem gemeinsamen Kurs darf keine unabhängige spätere Entscheidung dessen Darstellung ersetzen. | [rustig_clef_planning.rs](../src/structural_write_authority/rustig_clef_planning.rs) |
| 13 | C4: gemeinsamer Schlüssel-/Oktavkurs mit exakter Abschnittsdauer | Entscheidet in vollständig belegten, unterstützten Domänen. Systemübergreifende Kopplung bleibt derzeit eine mögliche UNKNOWN-Grenze. | [Suchkern](../src/pipeline_shards/joint_reading_c3_course.rs), [Adapter](../src/pipeline_shards/joint_reading_c3_adapter.rs) |
| 14 | Vergleich vollständiger gemeinsamer Registerspannen | Entscheidet nur bei vollständigem Ausgangsplan, Anschluss und zulässiger Verbesserung. | [gemeinsame_spanne.rs](../src/registernotation/gemeinsame_spanne.rs) |
| 15 | Tonart- und Schreibweisenbewertung: gewichtete Tonklassen, Tonika-/Dominantbeiträge, Vorzeichenkosten | Audit und daran gebundene produktive Schreibweisenentscheidung. Klingende Tonhöhe und Darstellung sind zu unterscheiden. | [key_signature_enharmonic_report_core.rs](../src/pipeline_shards/key_signature_enharmonic_report_core.rs) |
| 16 | Harmonik-Kandidaten: Nichtdominanz / Pareto-Vergleich | Kandidatenauswahl innerhalb vorhandener Owner. Ein nichtdominierter Kandidat muss nicht übernommen werden. | [sheaf_noether_projection_core.rs](../src/pipeline_shards/sheaf_noether_projection_core.rs) |
| 17 | Phrasen- und Grenzbeobachtung aus Rhythmus, Metrik, Kontur und tonalem Kontext | Audit. Grenzkorrelation allein erteilt keine Bogen- oder Abschnittsautorität. | [phrase_boundary_evidence.rs](../src/audit_only/phrase_boundary_evidence.rs) |
| 18 | Exakte Wiederholungsdeskriptoren; Transpositions- und Augmentationsinvarianz | Audit. Nur die tatsächlich definierten Merkmale sind invariant; kein allgemeiner Ähnlichkeits- oder Phrasenbeweis. | dieselbe Phrasenbeobachtung |
| 19 | Physische Handkonfigurationen und endliche Fähigkeitszuordnung | Audit. Ein unbelegter Fähigkeitswert bleibt unmodelliert. | [pianistic_configuration_constraint_kernel.rs](../src/audit_only/pianistic_configuration_constraint_kernel.rs) |
| 20 | Fingerplatz-Belegung als Ungleichungsgraph; Degenerationsordnung und Färbungszeuge | Audit zur Spielformel. Ein struktureller Färbungszeuge ist keine vollständige pianistische Lösung. | [symbolische Spielformel](../src/audit_only/pianistic_symbolic_capability_reachability.rs) |
| 21 | Statische und zeitliche Anforderungsprofile, Überlappungen, Wiederanschläge, Quantile | Audit. Verteilungen beschreiben Anforderungen und kalibrieren keine menschliche Grenze. | [temporale Anforderungen](../src/audit_only/pianistic_temporal_transition_demand_frontier.rs), [statische Frontier](../src/audit_only/pianistic_static_capability_frontier.rs) |
| 22 | WAV-Angriff/Puls/Metrik: Energie- und Transientenverläufe, lokale Spitzen, Tempo-/Phasenhypothesen | Audit. Audiohypothesen werden vor ihrer Bewertung gegen MIDI eingefroren. | [wav_attack_pulse_metric.rs](../src/audit_only/wav_attack_pulse_metric.rs) |
| 23 | Zeitabbildung zwischen nominalem Raster und Audio; Residuen und Quantile | Audit. Eine gute Anpassung ist kein Herkunftsnachweis für zusätzliche Quelltöne. | dieselbe WAV-Pulsanalyse |
| 24 | Frequenzenergie und harmonische Beziehungen: Goertzel-Rekurrenz | Audit. Spektrale Energie ist kein sicherer unabhängiger Anschlagsnachweis. | [wav_pitch_harmonic_evidence.rs](../src/audit_only/wav_pitch_harmonic_evidence.rs) |
| 25 | Basic Pitch: fremdes neuronales Transkriptionsmodell und unabhängiger Sensorvergleich | Separater Sensor / Audit. Keine automatische Übernahme in die kanonische musikalische Quelle. | [Sensorqualifikation](../src/audit_only/wav_pitch_sensor_qualification.rs), [Sensorbenchmark](../src/audit_only/wav_pitch_external_sensor_benchmark.rs) |
| 26 | Horizontaler Systemumbruch: endliche Partitionen, Kapazität und lexikografische Kosten | Entscheidet im vorhandenen Layoutpfad. Seitenumfang und echte Lesbarkeit sind damit nicht global optimiert. | [horizontal_layout_global_reflow.rs](../src/pipeline_shards/horizontal_layout_global_reflow.rs) |
| 27 | Renderer-Geometrie: Hüllen, Rechteckschnitt, Abstände und Umbruch-Readback | Misst / prüft nur unterstützte, eindeutig zugeordnete Geometrie. Kein vollständiger automatischer Schönheitsbeweis. | [Seitenabstände](../src/audit_only/renderer_page_clearance.rs), [Systemumbrüche](../src/audit_only/renderer_system_break_readback.rs) |
| 28 | Lexikografische Seitenpackung nach Schlöpfchen | Entwurf; von der vorhandenen Systemplanung getrennt. | [Seitenfehlerklasse](selector_derivation_contract_redundant_page_retention.md) |
| 29 | Präfixsummen und binäre Intervallsuche für dieselben additiven Kosten | Technische Erhaltung. Beschleunigung, keine neue musikalische Zielgröße. | [Schlüsselplanung](../src/structural_write_authority/rustig_clef_planning.rs), [Äquivalenzvertrag r1652](selector_derivation_contract_exact_clef_passage_prefix_costs_r1652.md) |
| 30 | Indizierte Existenzprüfung von Bogenendpunkten | Technische Erhaltung im bestehenden Owner. Kein allgemeines optimales bipartites Bogenmatching. | [Nachweis r1666](indexed_slur_endpoint_pairing_r1666.md) |
| 31 | Erreichbarkeit im Aufrufgraph, Capability- und Herkunftsprüfungen | Technische Zulassungsprüfungen. Pfad- und Quelltextprüfungen bestehen daneben weiter. | [semantic_capability_diff.rs](../src/build_gate/semantic_capability_diff.rs), [take_change_control.rs](../src/take_change_control.rs) |
| 32 | Beobachtungsäquivalenz und reduzierte Aufbewahrung | Technische Erhaltung unter nachgewiesenen Leserabhängigkeiten. | [solver_candidate_retention.rs](../src/pipeline_shards/solver_candidate_retention.rs) |
| 33 | Fingerprints und Hashfunktionen: FNV-1a, SHA-256 | Integritäts- und Bindungsprüfungen mit jeweiligen Kollisionsannahmen. Kein eigenständiger musikalischer Identitätsbeweis. | [SHA-256](../src/build_gate/sha256.rs), [Herkunft](../src/master_state_identity.rs) |
| 34 | Externe Versionsvergleiche, vereinfachte Weglass-Untergrenzen und Regressionsmodelle | Historische Diagnosewerkzeuge. Keine produktive musikalische Autorität. | historische r1556-Auswertung der früheren Seitenfassung |
| 35 | Notationszeit, Taktgrenzen und exakt gebundene Haltefortsetzungen | Entscheidet im MIDI-Seed unter Quell- und Metrikbindung. | [Notationszeit](../src/notation_time.rs), [MIDI-Seed](../src/midi_seed.rs) |
| 36 | Wiederholungsakkorde und belegte Handzuordnung über klingende Zeitintervalle | Entscheidet bei eindeutiger Wiederholung und zulässiger Tastenbelegung. | [Handzuordnung](selector_derivation_contract_repeated_accompaniment_hand_implementation.md), [Warnungsprüfung](../src/rules.rs) |
| 37 | Begrenzter gemeinsamer Seiten- und Systemplan mit Renderer-Rückmeldung | Entscheidet und prüft innerhalb zugelassener Seitenzahlen und Kapazitäten. | [Planung](../src/pipeline_shards/horizontal_layout_global_page_system.rs), [Partitionssuche](../src/pipeline_shards/renderer_exact_partition_search.rs) |
| 38 | Speicherabschätzung für parallele Korpusprojekte | Plant Prozesse und Solver-Worker mit festgesetzten Schätzwerten. | [Korpus-Speicherplanung](../src/pipeline_shards/corpus_memory_capacity.rs) |
| 39 | Lokale Stimmfortsetzung unter Zeit-, Rollen- und Randbedingungen | Prüft vollständige Belegungen einer kleinen Passage und liefert Zeugen für einzelne Beziehungen. | [Exakte Gegenprobe](../src/structural_write_authority/exact_voice_passages.rs) · [Modell und Suchgrenzen](selector_derivation_contract_exact_voice_passages.md) |

## Lokale Entscheidungen zu Pausen und Stimmen

| Aufgabe | Ausgewertete Größen | Entscheidung |
| --- | --- | --- |
| Pausen im geschriebenen Takt | Rationale Start- und Endzeiten, metrische Gruppen sowie belegte Zeitintervalle der sichtbaren Stimme | Der Pausenplan bestimmt die freie Zeit innerhalb des lokalen Takts und bewertet ihre Zerlegung anhand der metrischen Gruppierung. [Metrische Pausen](../src/model/metric_rest_notation.rs) · [Lokale Intervalle](selector_derivation_contract_final_local_rest_intervals.md) |
| Notenhals am Stimmeinsatz | Stimme, gleichzeitige Nachbarstimmen und bereits gebundene Balkengruppen | Die Richtung folgt den sichtbaren Beziehungen am Einsatz und innerhalb der gebundenen Gruppe. [Notenhälse](../src/notation_final_stems.rs) · [Vertrag](selector_derivation_contract_final_stem_direction.md) |
| Registerkontinuität | Benachbarte Abschnitte, gebundene Stimmzuordnung und Lesbarkeit des Schlüsselverlaufs | Die Entscheidung berücksichtigt den Anschluss zwischen Abschnitten. Der Nachweis bindet die Darstellung an die musikalischen Ereignisse. [Register und Stimmeinsatz](selector_derivation_contract_voice_entry_and_register_continuity.md) |

Den automatisch ermittelten 6/8-Takt, die regulären Anschläge und die geschriebene Dauer dokumentiert der Einzellauf zu Mozart. [Mozart KV 331, r1964](examples/mozart-k331-p01-r1964.pdf)

## Zentrale Formeln und ihre Beweisgrenzen

### Bilanz, Identität und Sichtbarkeit

```math
N_{\mathrm{Quelle}}=N_{\mathrm{aktiv}}+N_{\mathrm{inaktiv}},\qquad
N_{\mathrm{aktiv}}=N_{\mathrm{sichtbar}}+N_{\mathrm{verborgen}}.
```

Diese Gleichheiten gelten nur für die im jeweiligen Bericht sauber abgegrenzte Grundgesamtheit. Quellanschläge, geschriebene Fragmente und sichtbare Köpfe sind nicht austauschbare Nenner.

Eine bestandene Anzahlbilanz ist notwendig für die betreffende Buchführung, aber kein hinreichender Nachweis individuellen Erhalts: Eine Löschung und eine andere Einfügung können sich zahlenmäßig ausgleichen. Der aktuelle Bericht bezeichnet diese Projektion ausdrücklich als `cardinality_only_not_a_late_source_binding`. Herkunft und musikalische Eigenschaften brauchen eigene gebundene Nachweise.

`CanonicalSourceRetention` und `ArrangementVisibleRetention` beantworten verschiedene Fragen. Eine verborgene Note ist nicht allein deshalb entfernt. Aus ihrem Verbergen folgt weder ein nachgewiesener Spielbarkeitskonflikt noch ein Beweis, dass die verbleibende Darstellung menschlich spielbar ist.

### Intervalle und Stimmenbedarf

Für ein geschriebenes Ereignis gilt das halboffene Intervall $[t,t+d)$. Bei einem Sweep werden Enden vor gleichzeitigen Anfängen verarbeitet. Im vereinfachten Modell werden gleiche Einsätze und Dauern je Part/System/Takt zu einem Ereignis zusammengefasst.

Für den dadurch erzeugten Intervallgraphen gilt:

```math
\chi(G)=\omega(G).
```

Die maximale Überlappung ist damit die minimale Farbzahl **dieses Graphen**. Musikalische Linien, ursprüngliche Balkengruppen, Pausenpflichten, Akkordbindungen und technische Importkanäle besitzen zusätzliche Bedingungen. Aus der Graphzahl allein folgt keine erlaubte Zusammenlegung solcher Rollen. Der aktuelle `VoiceModel`-Bericht nennt diese Grenze ausdrücklich.

### Welche Balkenpaare bleiben erhalten?

Für die Mengen verbundener ursprünglicher Primärbalkenpaare im Referenzmaster $R$ und im Vergleichsmaster $V$ zählt jedes identifizierte Notenpaar einzeln. Quellnote, Anschlag, Fragment und der Ausgabebeleg identifizieren seine beiden Endpunkte auch bei einer Änderung der Stimmnummer.

| Mastervergleich an der Fuge BWV 847 | Erhalten $\lvert R\cap V\rvert$ | Gewonnen $\lvert V\setminus R\rvert$ | Verloren $\lvert R\setminus V\rvert$ |
|---|---:|---:|---:|
| Gleichstandsversuch r1958 gegen seine Referenz | 399 | 3 | 3 |
| Produktiver Stimmenwechsel r1960 gegen r1959 | 402 | 6 | 0 |

Im ersten Versuch zeigen beide Master 402 Verbindungen. Erst der Paarvergleich macht die drei verlorenen Anschlüsse sichtbar. Im zweiten gilt $R\subseteq V$. Der gemeinsame Stimmenwechsel bewahrt jede der 402 Verbindungen und gewinnt sechs hinzu. Ob der Leser einen Balken sieht, wird am Render-MusicXML und am PDF geprüft. [Master-Rücklesung](../src/structural_write_authority/primary_beam_pair_readback.rs) · [Gemeinsamer Stimmenwechsel](selector_derivation_contract_original_countervoice_exchange_r1960.md)

### Welche Stimmfortsetzung ist erzwungen?

Drei Linien enden auf den Noten A, B und C. An derselben Grenze setzen X, Y und Z ein. Für A allein kommen alle drei Anschlüsse infrage. Für B und C erlauben die gebundenen Rollen jeweils nur Y und Z. Jede ankommende Linie erhält genau eine Fortsetzung und jede neue Note gehört genau einer Linie. Eine solche vollständige Eins-zu-eins-Zuordnung heißt *perfektes Matching*.

| Zulässige Paarung | Anschluss von A | Anschluss von B | Anschluss von C |
|---|---|---|---|
| 1 | X | Y | Z |
| 2 | X | Z | Y |

B und C benötigen zusammen Y und Z. Deshalb ist A → X erzwungen. B → Y ist möglich, und die zweite Paarung belegt seine Mehrdeutigkeit. Überspannt eine Note mehrere Einsätze, muss derselbe Plan die Zeit- und Randbedingungen der ganzen Passage erfüllen.

Sei $P$ die Menge aller im lokalen Modell zulässigen Stimmpläne und $E(p)$ die Menge der darin realisierten Anschlüsse. Für eine Beziehung $e$ gilt

```math
e\text{ ist im Modell erzwungen}\quad\Longleftrightarrow\quad P\ne\varnothing\ \land\ \forall p\in P:\ e\in E(p).
```

r1961 prüft konkret, ob zwei gebundene Noten denselben Ausgabekanal belegen. Die Suche hält die Umgebung fest und berücksichtigt notierte und klingende Zeit, Rollen, Pausen, vorhandene Verbindungen und gebundene Randnachbarn. Zwei gegensätzliche Zeugen belegen Mehrdeutigkeit. Für einen Zwangsnachweis braucht es einen positiven Zeugen und eine abgeschlossene Suche nach Gegenplänen. Eine leere Lösungsmenge oder ein ausgeschöpftes Suchbudget lässt die Frage nach dem Zwang offen. [Suchkern und synthetische Gegenprobe](../src/structural_write_authority/exact_voice_passages.rs) · [Modellgrenzen](selector_derivation_contract_exact_voice_passages.md)

### Notationszeit, Haltebögen und Handzuordnung

Bei einem gebundenen Anschlag mit Einsatz $t$, Ende $e$ in der Notationszeit und einer Taktgrenze $b$ mit $t<b<e$ kann der MIDI-Seed das Zeitintervall in zwei Fragmente teilen:

```math
[t,e)=[t,b)\cup[b,e),\qquad e-t=(b-t)+(e-b).
```

Der Haltebogen verbindet beide Fragmente mit demselben Anschlag. Der Pfad prüft unter anderem exakte Notenwerte, die belegte Taktgrenze und konkurrierende Anschläge derselben Tonhöhe. In Schuberts Auftakt ergibt die gebundene Dauer $240=96+144$ Ticks, also eine Viertel am Auftakt und eine punktierte Viertel am Beginn des Folgetakts. [MIDI-Seed](../src/midi_seed.rs) · [Windows-Notenbild](examples/schubert-d783-no15-p18-r1872.pdf).

Für einen durch die bisherige Handgrenze geteilten Akkord vergleicht r1872 die Menge der MIDI-Tasten mit zeitlich direkt angrenzenden Akkordgruppen. Ein Zeuge muss vor der Korrektur vollständig derselben Hand zugeordnet sein. Widersprechen sich die Zeugen, bleibt die Einteilung bestehen. Für jede Hand $h$ und jede geprüfte Anschlagsgrenze $u$ verlangt der Code für die klingende Tastenmenge $K_h(u)$:

```math
|K_h(u)|\le 5,\qquad \max K_h(u)-\min K_h(u)\le 12\quad\text{für }K_h(u)\ne\varnothing.
```

Sind beide Hände belegt, muss die höchste linke Taste unter der tiefsten rechten liegen. Konkurrierende Anschläge derselben Akkordtaste schließen den Wechsel aus. Die Grenzwerte gehören zum Modell der Handzuordnung. Sie liefern weder einen Fingersatz noch einen Nachweis individueller Spielbarkeit. [Regel für Wiederholungsakkorde](selector_derivation_contract_repeated_accompaniment_hand_implementation.md) · [Quellcode](../src/midi_seed.rs).

Die Sprungwarnung prüft zusätzlich, ob sich beide Tonhöhengrenzen aufeinanderfolgender Anschläge in dieselbe Richtung bewegen und jeweils mindestens 19 Halbtöne zurücklegen. Der bestehende Test der Zeitkante bleibt Teil der Warnung. Ein Akkord kann sich aufspreizen oder zusammenziehen, ohne dass beide Grenzen einen solchen Sprung ausführen. [Prüfregel](../src/rules.rs).

### Kandidatenauswahl und Pareto-Front

Für zwei zulässige Lösungen entscheidet beim lexikografischen Vergleich der erste abweichende Eintrag. Der Auswahlschlüssel für die allgemeine Kandidatenbewertung ordnet die Größen so

```math
K=(B,C,H,W,I,\mathrm{rev}(R),M).
```

$B$ zählt Blocker, $C$ kritische Fälle, $H$ ist die größte kritische Härte, $W$ und $I$ zählen Warnungen und Hinweise. Es folgen der absteigend geordnete Notenerhalt $R$ und die Anzahl $M$ verschobener Noten. Wenn ein Kandidat einen zusätzlichen Blocker hat, können ihm weniger Warnungen bei diesem Vergleich keinen Vorrang verschaffen. [Kandidatenauswahl](../src/pipeline_shards/staff_voice_move_restify_atomic_patch_tail.rs)

Die lokale Harmonikbewertung untersucht auch die Pareto-Front. Im neundimensionalen Kostenvektor $z$ gehen die beiden Nutzenbeiträge mit negativem Vorzeichen ein. Kandidat $a$ dominiert $b$ genau dann, wenn er in jedem Kriterium mindestens so gut und in wenigstens einem besser ist

```math
a\prec b\;\Longleftrightarrow\;
\left(\forall k\colon z_k(a)\le z_k(b)\right)
\land\left(\exists k\colon z_k(a)<z_k(b)\right).
```

Die erste Front besteht aus allen Kandidaten ohne dominierenden Konkurrenten. Bei konkurrierenden Vorteilen können mehrere Kandidaten auf dieser Front liegen. [Harmonikvergleich](../src/pipeline_shards/sheaf_noether_projection_core.rs)

### Wiederholung bei anderer Tonhöhe und anderem Tempo

Die Phrasenanalyse beschreibt eine Passage anhand der Abstände aufeinanderfolgender Einsätze, der klingenden Dauern und der Unterschiede zwischen den Tonhöhenschwerpunkten. Für den positiven ganzzahligen Zeitvektor $v$ des Abschnitts verwendet sie

```math
g=\gcd(v_1,\ldots,v_n),\qquad \rho(v)=v/g,\qquad
\Delta p_i=p_{i+1}-p_i.
```

Verdoppeln sich alle Zeitwerte, bleibt $\rho$ gleich. Eine gemeinsame Transposition verändert die Schwerpunktdifferenzen $\Delta p_i$ ebenfalls nicht. Der Deskriptor erfasst zusätzlich die Akkordbesetzung und setzt ein gültiges Zeitmodell voraus. Ein Treffer beschreibt eine strukturelle Wiederholung. Ob der Pianist beide Stellen gleich artikuliert oder eine Stimme über die Stelle hinaus hält, braucht eigene Evidenz. [Phrasenanalyse](../src/audit_only/phrase_boundary_evidence.rs)

### Quadratische Verteilungskosten

Die implementierte lokale Bewertung enthält:

```math
E(t,s)=3\Delta t^2+24\Delta p^2+42^2[\text{System verschieden}]
+12^2[\text{Stimme verschieden}]+2(\Delta t-24k)^2+12000[\text{Platz bereits verwendet}].
```

Zeit wird hier in der vorhandenen 96-Ticks-pro-Viertel-Projektion gemessen, Tonhöhe in Halbtönen. Der Code begrenzt zusätzlich den Kandidatenraum und prüft je nach Aufruf die Zielbelegung. Die Formel allein ist weder vollständige Zulässigkeit noch Erhaltungsbeweis. Der Name „Gauss“ bezeichnet hier quadratische Kosten; er behauptet keine Gauß-Verteilung der Musikdaten, kein statistisches Maximum-Likelihood-Modell und keine globale optimale Zuordnung aller Töne.

Gewichte, Kandidatengrenzen und Strafen sind Projekteinstellungen. Ein Kandidaten-Score ist kein gemessener menschlicher Lese- oder Spielaufwand.

### Tonartprofil und Schreibweise

Die aktuelle Profilbewertung summiert Beiträge pro Ton. Vereinfacht mit dem vom Owner bestimmten Gewicht $w_n$, Tonklasse $c_n$ und Profil $P$ lautet der im Code sichtbare Kern:

```math
\begin{aligned}
T(P)&=\sum_n w_n\bigl(3S_n-4(1-S_n)+2T_n+D_n+L_n\bigr).
\end{aligned}
```

Die Indikatoren stehen für Skalenmitgliedschaft $S_n$, Tonika $T_n$, Dominante $D_n$ und den erhöhten Leitton in Moll $L_n$. Sie nehmen die Werte 0 oder 1 an.

Dies ist eine Projekteinstellung und kein statistisch kalibrierter Tonartbeweis. Die tatsächliche Schreibweisenentscheidung verlangt zusätzliche Bedingungen zu Geltungsbereich, Stabilität, konkurrierenden Lesarten und Vorzeichenkosten. Enharmonische Umbenennung darf die klingende Tonhöhe nicht verändern. Ein Profilwert allein erteilt weder eine neue Tonhöhe noch eine neue Abschnittsgrenze.

### Separater Schlüsselplan und gemeinsamer Registerkurs sind verschiedene Zielfunktionen

Der separate Lesbarkeitskurs verwendet den graduellen Druck

```math
p(l)=l^2+8\max(l-3,0)
```

und Kosten für Schlüsselwechsel, Redundanzen und kurze Ausflüge. Seine bekannten 24 Einheiten je Wechsel beziehungsweise kurzem Ausflug sind Setzungen dieses Modells. Daraus folgt keine universelle Grenze von drei Hilfslinien.

Die historische C3-Policy verwendete die eingefrorene **lexikografische** Reihenfolge

```math
(h_8,h_7,h_6,h_5,h_4,k_o,u_k,k_s,g_b,h_3,c_h,h_2,h_1).
```

Die versionierte [C4-Policy](joint_register_dp_policy_c4_v1.md) vertauscht ausschließlich die benachbarten Prioritäten $h_3$ und $c_h$:

```math
(h_8,h_7,h_6,h_5,h_4,k_o,u_k,k_s,g_b,c_h,h_3,h_2,h_1).
```

Damit kann ein Kurs mit weniger Registerzeichen trotz zusätzlicher Drei-Hilfslinien-Köpfe gewinnen. Schwerere Hilfslinienklassen, die Kopf-Schutzregel, Gruppengrenzen und alle tatsächlichen Schreibbedingungen behalten ihren Vorrang. Es gibt keine neue Gewichts- oder Schwellenkalibrierung. Der Dateiname des bestehenden Suchkerns und die C3-Präfixe einiger Transporttypen bleiben aus Kompatibilitätsgründen erhalten.

| Größe | Bedeutung |
|---|---|
| $h_1,\ldots,h_7$ | Sichtbare Köpfe mit genau dieser Zahl an Hilfslinien; $h_8$ bündelt acht oder mehr. Das ungekappte Maximum wird getrennt berichtet. |
| $k_o$ | Gleichzeitiger Schlüssel- und wirksamer Oktavwechsel an einer Grenze. |
| $u_k$ | Köpfe unter den als unkonventionell bewerteten Kombinationen G mit Abwärtsoktavierung beziehungsweise F mit Aufwärtsoktavierung. Teuer, nicht pauschal verboten. |
| $k_s$ | Maximale Abschnitte eines kombinierten Zustands, die kürzer als eine nominelle Taktlänge sind. |
| $g_b$ | Grenzen innerhalb der positiv gebundenen Gruppen. Musikalische und materielle Verbote gelten daneben unverändert. |
| $c_h$ | Echte Registerzeichen-Operationen. Reine Erinnerungszeichen werden getrennt behandelt. |

Die nominalen Zustände sind G2 oder F4 kombiniert mit loco, 8va, 8vb, 15ma oder 15mb. Mit Oktavanzahl $q\in\{0,1,-1,2,-2\}$ und diatonischer Tonposition $d$ gilt für die geschriebene Position $d_{\mathrm{geschrieben}}=d-7q$. Deshalb ist die Schreibweise relevant; MIDI-Tasten allein bestimmen die Hilfslinien nicht.

Nominale Zustände sind noch keine Schreibbefugnis. Im aktuellen Adapter unterstützt der obere Owner nur seine tatsächlich angebotenen Zustände; zusätzliche F-Schlüsselalternativen im oberen System folgen nicht aus der nominalen Zehnermenge. Anfang, Ende, Zustandsalter, Zeitachse, Gruppen und Anschluss müssen gebunden sein. Für gekoppelte Systeme darf ein unabhängiger Einzelsystemkurs nicht als gemeinsames Optimum ausgegeben werden.

Die Schutzregel lautet für jeden sichtbaren Kopf $n$:

```math
\neg\big(L_z(n)\ge4\ \land\ L_z(n)>L_0(n)\big).
```

Ein Kandidat darf also keine neue oder verschärfte Belastung ab vier Hilfslinien erzeugen. Ein bereits belasteter Ausgangszustand wird nicht durch Behauptung unsichtbar. Unter den zulässigen vollständigen Kursen zählt die erste abweichende Kostenkomponente; Gleichstand erhält den Ausgang.

**Wichtige Folge:** Weniger Umschaltungen gewinnt nicht immer. Zum Beispiel kann die Vermeidung eines Vier-Hilfslinien-Kopfes einen kurzen Schlüsselwechsel rechtfertigen. Diese auch in C4 erhaltene Priorität ist nicht identisch mit der alten skalaren Druckfunktion und kein universelles Lesbarkeitsgesetz.

Der Suchkern trägt zusätzlich zum Registerzustand das genaue, bei einer Taktlänge gesättigte Abschnittsalter. Rationale Taktkoordinaten behandeln Takt- und Divisionswechsel. Ein Ressourcenabbruch oder eine unbekannte Abhängigkeit beweist kein Optimum. Die [C3-Policy](joint_register_dp_policy_v1.md) bleibt als unveränderte historische Spezifikation erhalten; im aktuellen Suchkern gilt die [C4-Policy](joint_register_dp_policy_c4_v1.md).

Ein dynamischer Planer kann mehrere zulässige Schlüsselverläufe vergleichen, ohne alle vollständigen Verläufe aufzubewahren. Bezeichnet $D_i(z)$ die beste bis Abschnitt $i$ erreichbare Bewertung mit Endzustand $z$, folgt das Verfahren dem Bellman-Prinzip

```math
D_i(z)=\min_{y\to z}\bigl(D_{i-1}(y)+c_i(y,z)\bigr).
```

Der Zustand muss alle Informationen enthalten, von denen künftige Anschlüsse abhängen. Beim Registerkurs können dazu bereits begonnene Abschnitte oder Gruppenpflichten gehören. Die konkrete Kostenordnung und die zulässigen Übergänge legt der jeweilige Planer fest. [Schlüsselplanung](../src/structural_write_authority/rustig_clef_planning.rs) · [Gemeinsamer Registerkurs](../src/pipeline_shards/joint_reading_c3_course.rs)

### Graphzeugen der Spielformel

Die symbolische Struktur enthält Ungleichungen zwischen Belegungsvariablen. Der Code bildet einen Graphen, erzeugt eine Smallest-last-/Degenerationsordnung und prüft die daraus gewonnene Zuordnung gegen die tatsächlichen Kanten. Eine passende Clique kann umgekehrt einen strukturellen Konflikt belegen.

Seien $V$ die im Modell zu belegenden Ereignisse, $E$ die gleichzeitig unverträglichen Paare und $F$ die verfügbaren Plätze. Ein Beleg für diese Teilaufgabe ist eine geprüfte Zuordnung

```math
a\colon V\to F,\qquad (u,v)\in E\ \Longrightarrow\ a(u)\ne a(v).
```

Ein struktureller Zeuge schließt nur diese Ungleichungen. Handkonfiguration, Übergang, Wiederanschlag, Fähigkeiten und Zeitmodell bleiben zusätzliche Verpflichtungen. Weder die maximale Gleichzeitigkeit noch ein gefärbter Graph allein beweist einen ausführbaren Fingersatz.

### Audio: Pulsanalyse und Goertzel-Rekurrenz

Die WAV-Pulsanalyse erzeugt Hypothesen aus Energie- und Transientenanstiegen, lokalen Spitzen sowie Tempo und Phase. Eine zeitliche Abbildung und Residuen beschreiben Abweichungen vom nominellen Raster. Die Audiohypothesen werden vor der späteren MIDI-Evaluation festgehalten; die Vergleichsquelle darf nicht unbemerkt das unabhängige Messergebnis erzeugen.

Für ausgewählte Frequenzen verwendet die harmonische Evidenzprobe tatsächlich eine Goertzel-Rekurrenz:

```math
s_j=x_j+2\cos(\omega)s_{j-1}-s_{j-2},\qquad \omega=2\pi f/f_s.
```

Für ein Fenster von $N$ Mono-Samples und die letzten beiden Zustände liefert der Code die normierte Energie

```math
P(f)=\max\left(0,\frac{s_{N-1}^2+s_{N-2}^2-2\cos(\omega)s_{N-1}s_{N-2}}{N^2}\right).
```

Die Frequenzzuordnung der Probe verwendet $f(m)=440\cdot2^{(m-69)/12}$: gleichstufige Stimmung mit A4 = 440 Hz ist hier eine Modellannahme. Der Code normiert die berechnete Energie durch das Quadrat der Fensterlänge. Dies ist gezielte Frequenzanalyse, keine Behauptung, dass im Projekt eine allgemeine FFT-Pipeline implementiert sei. Ein hoher Wert kann auch von Obertönen oder überlagerten Klängen stammen. Basic Pitch ist zusätzlich ein fremder neuronaler Sensor; beide Beobachtungen erhalten keine automatische musikalische Schreibbefugnis.

### Horizontaler Umbruch und begrenzte gemeinsame Planung

Der horizontale Reflow vergleicht endliche Systempartitionen unter seinen Kapazitätsbedingungen. Sein Vergleichsschlüssel betrachtet zuerst Systemzahl, Ein-Takt-Systeme, erzwungene Überkapazitäts-Einzelplätze und quadratische Restkapazität. Ein deterministischer Partitionsvergleich löst Gleichstände. [Horizontaler Reflow](../src/pipeline_shards/horizontal_layout_global_reflow.rs).

Sei $L_j$ der aufsummierte Bedarf des Systems $j$ und $C$ seine Kapazität in den Einheiten des Layoutplaners. Für eine zulässige Partition mit $S$ Systemen, $E$ Ein-Takt-Systemen, $U$ erzwungenen Ein-Takt-Systemen über Kapazität und dem Vektor $n$ der Taktzahlen je System minimiert der horizontale Pfad

```math
\left(S,E,U,\sum_{j=1}^{S}\max(0,C-L_j)^2,\ n\right)_{\mathrm{lex}}.
```

Bleiben über zwei Systeme vier Kapazitätseinheiten frei, kostet die Verteilung 0 und 4 insgesamt $0^2+4^2=16$. Die Verteilung 2 und 2 kostet $2^2+2^2=8$ und gewinnt bei gleichen vorrangigen Kriterien. Für die 31 Takte der Bach-Fuge reproduzierte eine unabhängige Gegenrechnung mit Kapazität 429 und höchstens vier Takten je System alle zwölf Umbruchstellen. Das Optimum für diese Eingaben umfasst 13 Systeme, null Ein-Takt-Systeme und eine quadratische Restsumme von 154 635. Die sichtbaren Abstände im PDF erfordern eine eigene Messung.

Für einen gebundenen Bereich zulässiger Seitenzahlen kann der Renderer-Pfad auch Seiten- und Systemgrenzen gemeinsam planen. Er berücksichtigt Systemlast, Abstandsanforderungen, das Limit für Systeme je Seite und bereits verworfene Partitionen. Der Schubert-Lauf r1872 bestätigte eine Kombination aus zwei Seiten und sieben Systemen. Diese Bestätigung betrifft die konkret gerenderte Partition. [Gemeinsame Planung](../src/pipeline_shards/horizontal_layout_global_page_system.rs) · [Exakte Partitionssuche](../src/pipeline_shards/renderer_exact_partition_search.rs).

Die geometrischen Readbacks untersuchen tatsächliche Hüllen und Abstände, soweit PDF-Syntax und Zuordnung unterstützt sind. Daraus folgt weder eine vollständige Kollisionsfreiheit aller denkbaren Zeichen noch eine implementierte Schlöpfchen-Seitenoptimierung.

### Beschleunigung mit gleichen Entscheidungen

Für additive Passagenkosten können Präfixsummen dieselbe Summe liefern:

```math
\sum_{i=a}^{b-1}c_i=P_b-P_a.
```

Die Indizes entstehen aus der exakten halboffenen Zeitbedingung, nicht aus einer geratenen Notennähe. Die Umordnung muss Kosten, Kandidaten und Gleichstandsregeln erhalten.

Die Bogenprüfung verwendet einen anderen Index: Für jede Nummer beantworten erster sichtbarer Anfang und letzter sichtbarer Schluss dieselben Existenzfragen wie die frühere Vollsuche. Das ist kein allgemeines globales Matching aller möglichen Bögen. Eine solche Erweiterung wäre eine eigene musikalische Entscheidung.

In r1965 vergleicht EtuSchlopf wiederkehrende Phrasen mit dynamischer Programmierung. Für $n$ Einsatzgruppen hat die Tabelle einschließlich ihres Randes die Breite $m=n+1$. r1964 hielt jede Bewertung als 8-Byte-Zahl und jede Rückrichtung als 4-Byte-Zahl fest. In r1965 genügen die vorherige und die aktuelle Bewertungszeile. Für den späteren Rückweg bleiben alle Richtungen gespeichert, mit einem Byte je Zelle. Für die Nutzdaten dieser beiden Tabellen ergibt sich

```math
B_{1964}(m)=12m^2,\qquad B_{1965}(m)=16m+m^2\quad\text{Byte}.
```

Bei **1024 Einsatzgruppen** ($m=1025$) sinkt der Bedarf von **12.607.500 auf 1.067.025 Byte**. Bei der Suche nach dem Metrum berechnet EtuSchlopf außerdem die festen Sprungkosten einmal pro Phasenmodell. Die Reihenfolge gleichwertiger Kandidaten bleibt erhalten.

Im isolierten MIDI/WAV-Lauf für die Aufführungsanalyse von Mozarts KV 331 sank die mediane **Prozess-CPU-Zeit von 0,399568 auf 0,266895 Sekunden** (acht Läufe pro Version, abwechselnd auf demselben Linux-Rechner). Die Messung umfasst das Einlesen im Prüfprogramm und die Inferenz bis zum Zeitplan. Der anschließende Notensatz und PDF-Export liegen außerhalb dieser Messung. [Quellstand r1965 und Äquivalenznachweis](selector_derivation_contract_performance_trace_equivalence.md)

### Speicherplanung für den Korpus

Im Automatikmodus schätzt EtuSchlopf die Zahl paralleler Projekte und Solver-Worker. Für $p$ Projekte und den ermittelten verfügbaren Speicher $M$ in MiB lautet die speicherbezogene Worker-Grenze pro Projekt:

```math
w_{\mathrm{Speicher}}(p)=
\left\lfloor
\frac{\max\!\left(\left\lfloor\max(M-512,0)/p\right\rfloor-1536,0\right)}{256}
\right\rfloor.
```

Die Planung probiert die Projektzahl absteigend innerhalb der CPU-Grenze und begrenzt die Worker zusätzlich durch das CPU-Budget. Bei unbekanntem verfügbarem Speicher oder fehlender passender Aufteilung startet sie mit einem Projekt und einem Solver-Worker. Die drei Speichergrößen sind Schätzwerte für Reserve, Projektprozess und zusätzlichen Solver-Worker. Der Kandidatenumfang besitzt ein getrenntes Budget. [Speichermodell](../src/pipeline_shards/corpus_memory_capacity.rs) · [Korpusplanung](../src/pipeline_shards/rest_glyph_retention_batch_corpus_tail.rs).

### Projektion und Integrität

Eine Aufbewahrungsprojektion $\pi$ ist nur dann ausreichend, wenn spätere Beobachtungen tatsächlich durch sie faktorisieren. Präziser als ein bloßer Größenvergleich ist die Bedingung

```math
\pi(D_1)=\pi(D_2)\Rightarrow O_L(D_1)=O_L(D_2)
```

für die betroffenen erreichbaren Leser $L$ und ihre Beobachtung $O_L$, einschließlich der relevanten Fortsetzungen. Eine identische aktuelle Ausgabe allein genügt nicht für künftige Mutationen. Die ausführbare Aufbewahrungspolitik bleibt der konkrete Nachweis.

Für die Quelltext-Prüfsummen verarbeitet FNV-1a die Bytes $b_i$ nacheinander. Die 64-Bit-Variante kombiniert den vorigen Zustand und das Byte mit bitweisem XOR und multipliziert anschließend mit der FNV-Primzahl

```math
h_{i+1}=((h_i\mathbin{\mathrm{XOR}}b_i)\cdot1099511628211)\bmod 2^{64}.
```

Die Take-Prüfung vergleicht die so deklarierten Dateiänderungen mit dem tatsächlichen Paket. Kryptografische Bindungen des Pakets und seiner Herkunft verwenden gesondert SHA-256. [Prüfung des Paketdiffs](../src/take_change_control.rs)

Hashes sind davon getrennt: Sie binden Bytes unter ihren jeweiligen Kollisionsannahmen. FNV-1a ist kein kryptografischer Beweis; auch SHA-256 ist mathematisch nicht injektiv über beliebig lange Eingaben. Provenienz, tatsächlich mitgeführte Ereignisidentität und bytebezogene Integritätsprüfung erfüllen verschiedene Aufgaben.

## Quellzeit, Pausen und Balken in r1938

Ein Ton belegt das halboffene Quellzeitintervall $[s,e)$. Der MIDI-Seed hält den Quellanschlag und die Loslasszeit zusammen. Für eine gehaltene Oberstimme prüft er, ob sich deren Fortsetzung im vorhandenen metrischen Zeitfenster eindeutig bestimmen lässt. Jeder Folgeanschlag muss an die belegte Loslasszeit des vorigen Tons anschließen. Die Dauern müssen sich im Notenraster exakt darstellen lassen und das Fenster vollständig ausfüllen. Ein erneuter Anschlag derselben Tonhöhe innerhalb des Halteintervalls verhindert die Trennung. Wenn die neue Zuordnung uneindeutig ist, gilt der bisherige Ableitungsweg. [MIDI-Seed](../src/midi_seed.rs) · [Vertrag der zeitlich belegten Fortsetzung](selector_derivation_contract_timed_held_upper_voice_implementation.md)

Für das metrische Einsatzfenster $M$ einer geschriebenen Stimme sammelt EtuSchlopf deren klingende Intervalle $I_j$ systemübergreifend. Bereits sichtbare Belegung im Zielsystem sei $V$. Kandidaten für sichtbare Pausen sind die zusammenhängenden Teilintervalle von

```math
M \setminus \left(V \cup \bigcup_j I_j\right).
```

Der zuständige Planer übernimmt nur Teilintervalle, deren Anfang und Ende sich exakt im metrischen Raster schreiben lassen. Gleichzeitige Töne zählen bei der Intervallvereinigung einmal. [Planer für sichtbare Pausen](../src/pipeline_shards/composer_beat_scoped_rest_completion_planning_core.rs) · [Vertrag der exakten Pausenprojektion](written_voice_entry_rest_exact_projection.md)

Ein Balkenplan verbindet metrisch zusammengehörige Anschläge. Auf höheren Ebenen prüft die Rücklesung zusammenhängende Segmente gegen die gebundene Vorlage. Ein fehlendes Segment kann am Anfang, im Inneren oder am Ende auftreten. Eine Pause kann zwei freie Anschläge verbinden, wenn beide jeweils nur diesen passenden Nachbarn haben. Vollständige Balkengruppen behalten ihre Zugehörigkeit. [Metrische Balkenplanung](../src/pipeline_shards/composer_metric_beam_planning_core.rs) · [Rücklesung](../src/render/final_beam_observation_planning_core.rs)

Der Korpusnachweis unterscheidet zwischen einer beobachteten Render-Materialisierung und einem belegten direkten PDF-Export aus der Master-Quelle. Mozart KV 331 belegt den zweiten Weg. [Manifestprüfung](../src/corpus_run_manifest.rs)

## Stimmfortsetzungen im Vollkorpus r1961

Von 5.751 ursprünglichen Primärbalkenpaaren fanden sich im Master 5.229 verbunden und 520 auf verschiedene Stimmen verteilt. Bei zwei weiteren Paaren blieb die Rücklesung offen, je eines in Chopin op. 10 Nr. 3 und Etüde Nr. 6. Für die 520 getrennten Paare prüfte die Gegenprobe die Beziehung „gleicher Ausgabekanal“. Alle 14 Projektberichte stammen aus demselben Windows-Lauf. [Prüfbeleg r1961](evidence/r1961/README.md)

| Befund für getrennte Paare | Bach BWV 847 | Übrige 13 Stücke | Gesamt |
|---|---:|---:|---:|
| Im lokalen Modell erzwungen | 4 | 0 | 4 |
| Im lokalen Modell ausgeschlossen | 8 | 15 | 23 |
| Im lokalen Modell mehrdeutig | 0 | 0 | 0 |
| Offen | 7 | 486 | 493 |
| **Getrennte Paare** | **19** | **501** | **520** |

Bei allen vier positiven Bach-Fällen ließ die bestehende Stimmzuordnung den jeweiligen Einzelplan als Vergleich zu. Jeder Plan gewann das untersuchte Master-Paar und bewahrte die 408 zuvor verbundenen Paare. Es waren die Achtelpaare B–A in Takt 4, As–As in Takt 11, Es–D in Takt 21 und H–C in Takt 28. Im bestehenden PDF waren alle vier bereits gebalkt. Alle drei Bach-Seiten blieben im Vergleich zu r1960 bei 300 dpi pixelgleich. Eine gemeinsame Übernahme der vier Pläne müsste als eigener Kandidat geprüft werden. [Prüfbeleg und Notenbild](evidence/r1961/README.md)

Auch außerhalb der Fuge lieferte die Suche einen musikalisch lesbaren Befund. In Etüde Nr. 8, Takt 44, stehen auf der vierten Zählzeit die vier Sechzehntel C5–E5–G5–C6. Im lokalen Modell gehören C5–E5 und E5–G5 jeweils demselben Kanal an. Ein gemeinsamer Kanal für G5 und C6 ist unter denselben Bedingungen ausgeschlossen. Der Master zeigt die ersten drei Töne in Stimme 1 und das letzte C in Stimme 3.

Die 493 offenen Zielpaare verteilen sich auf 244 Fälle mit Akkordträger oder fehlender eindeutiger Trägerbindung, 174 mit einer vom Modell nicht unterstützten oder unvollständig gebundenen Quelle und 75 mit leerer lokaler Lösungsmenge. Die Zahl der offenen Fälle beschreibt die Abdeckung der Gegenprobe. Die Schreibentscheidung bleibt beim bestehenden musikalischen Verfahren. [Suchraum und Prüfung positiver Zeugen](selector_derivation_contract_exact_voice_passages.md)

## Prüfstände und verbleibende Grenzen

| Referenz | Veröffentlichter Windows-Prüfbeleg |
|---|---|
| Bach, Fuge c-Moll BWV 847 | r1749, Exitcode 0, 3.041 bestandene und vier ignorierte Tests. Der frühere Abgleich von r1745 erfasste 754 Notenansätze, geschriebene Notenwerte und Haltebögen. [Abnahme r1749](evidence/r1749/README.md). |
| Chopin, Etüde op. 10 Nr. 3, p01 | r1792, Exitcode 0, 0 Warnungen, 100 % im Notenerhaltsmaß und ein gerendertes Notenbild. Die MIDI-WAV-Zeitzuordnung blieb wegen geringer gemeinsamer Anschlagsdeckung offen. [Abnahme r1792](evidence/r1792/README.md). |
| Schubert, Deutscher Tanz D. 783 Nr. 15, p18 | r1872, Exitcode 0, 0 Blocker, 0 Warnungen und 100 % im Notenerhaltsmaß. 3.097 Tests bestanden, vier waren ignoriert. Der Windows-Renderer gab das zweiseitige Notenbild frei. [Abnahme r1872](evidence/r1872/README.md). |
| Mozart, KV 331, Andante grazioso, p01 | r1964, Windows-Einzellauf mit MIDI und WAV, 6/8 und zwei PDF-Seiten. 460 der 474 regulären Noten haben die Dauer der Korpusreferenz. [Notenbild](examples/mozart-k331-p01-r1964.pdf). |

Der [Windows-Vollkorpus r1965](evidence/r1965/README.md) umfasst 14 Stücke auf 57 PDF-Seiten. Frühere Prüfstände derselben 14 Stücke sind [r1938](evidence/r1938/README.md) und [r1961](evidence/r1961/README.md). Die vorbereitete [Ballade op. 38](examples/chopin-op38-p01.md) ist ein separater Stresstest. Das Notenerhaltsmaß folgt der im Lauf definierten Grundgesamtheit. Es bewertet weder den Abgleich aller Notenwerte mit der Druckvorlage noch das Notenbild als Ganzes.

### Historischer Vollkorpuslauf r1683

Der r1683-Export bestätigt Build und Projekt-Clippy mit Exit 0, 2.992 bestandene Tests bei vier ignorierten Tests sowie zehn formal erfolgreiche Projekte. Diese Abnahme beweist die geprüften Verträge, nicht allgemeine menschliche Spielbarkeit oder ein fehlerfreies Notenbild.

| Projekte im damaligen Korpus | Übernommener C3-Kurs nach r1683 | Einordnung |
|---|---|---|
| 5, 9, 10 | Beide Systeme | Unteres System durch r1683 neu verfügbar. |
| 7, 8 | Beide Systeme | Bereits vorher übernommen. |
| 1, 2, 3, 4, 6 | Kein gemeinsamer Kurs übernommen | Systemübergreifende Gruppenabhängigkeit nicht geschlossen; Ausgang bleibt erhalten. |

Die Tabelle ist Evaluation, kein produktiver Selektor. r1683 führt ursprünglich vorhandene Pausenanker getrennt von klingenden Balkenköpfen bis zur gemeinsamen Entscheidung mit. Ein fehlender Fortsetzungsbeleg bleibt unbekannt; der erhaltene Akkordton darf dabei seinen negativen Abhängigkeitsbeleg nicht verlieren. Eine Pause wird dadurch weder zu einem klingenden Kopf noch zu einer positiven Balkenentscheidung.

Gegenüber r1682 bleiben alle zehn Master-Notenfolgen strukturell gleich. Die Render-Noteninhalte bleiben bis auf zwei Balkengruppierungen und sieben Pausenglyphen-Positionen erhalten; Tonhöhen, Zeitwerte, Stimmen, Systeme, Sichtbarkeit und Bogeninhalte sind unverändert. Die PDF-Seitenzahl bleibt 39; 30 Seiten sind im durchgeführten Pixelvergleich bei 144 dpi identisch, neun Seiten ändern sich. Die geänderten Seiten wurden zusätzlich gesichtet. Das ist ein konkreter Artefaktvergleich und kein universeller Qualitätsbeweis.

Historische Zahlen aus r1556 oder r1559 werden nicht als heutige Kennzahlen fortgeschrieben. Das alte Speichermodell $M\approx0{,}0054FS+0{,}0061GS$ bleibt eine historische Regressionshypothese: Die abgedruckte Form besitzt zwei Koeffizienten; die frühere Textangabe „drei Parameter“ ist ohne zusätzlichen Modellterm nicht nachvollziehbar. Ohne Rohdaten und erneute Anpassung wird daraus kein aktueller Speicherbeweis abgeleitet.

Auch die frühere Weglass-Untergrenze gilt nur unter ihrem vereinfachten Modell aus monotoner Handaufteilung, gewählter Spannweite und begrenzter Fingerzahl. Sie ist keine allgemeine Untergrenze für notwendige Notenlöschungen beim menschlichen Klavierspiel.

## Nächste mathematische Vorhaben

| Vorhaben | Mathematische Frage | Woran sich ein Fortschritt messen lässt |
| --- | --- | --- |
| Notendauern freier Einspielungen | Welche Stimmfortsetzungen bleiben bei plausibler Änderung von Artikulation und örtlichem Tempo zulässig? | Für unabhängige, synthetische Stimmen kurze gehaltene, kurz gespielte lange und überlappende Noten unterscheiden. Auf zurückgehaltenen Einspielungen Dauern, neu gewonnene und verlorene Treffer getrennt ausweisen. |
| Stimme und Hand über eine Passage | Welche Kanten eines zeitlichen Belegungsgraphen sind in jeder zulässigen Stimmenführung enthalten? | Für eine erzwungene Beziehung einen vollständigen Plan und den Ausschluss der Gegenbelegung zeigen. Bei Mehrdeutigkeit zwei zulässige Gegenpläne nachweisen. |
| Lesbare Seitenfüllung | Welche geordneten Seiten- und Systempartitionen halten gemessene Mindestabstände für Noten, Text und Bögen ein? | An tatsächlich gerenderten Seiten engste sichtbare Abstände und ungenutzte Höhe pro Seite ausweisen. Erst unter den zulässigen Plänen die Seitenfüllung vergleichen. |
| Rechenzeit im gemeinsamen Registerplan | Welche Zustände lassen sich zusammenfassen, wenn sie für alle noch erreichbaren Anschlussentscheidungen dieselben Kosten und Bedingungen haben? | Gegen die vollständige Suche denselben Kurs und dieselbe Gleichstandsentscheidung nachweisen. Laufzeit und aufgesuchte Zustände mit identischen Eingaben vergleichen. |

## Herkunft und Eigenanteil

Standardverfahren bleiben Standardverfahren: dynamische Programmierung, lexikografische Ordnung, Intervallgraphen, Färbung, Nichtdominanz, rationale Arithmetik, quadratische Kosten, Spektralrekurrenzen, Deskriptivstatistik und Hashfunktionen wurden nicht für EtuSchlopf erfunden.

Unser Eigenanteil liegt in den musikalischen Modellen, dem Zusammenspiel der Bedingungen, den gebundenen Owner-Übergängen und den gewählten Prioritäten. Die drei Projektbegriffe benennen solche Zusammenstellungen. Weder deren wissenschaftliche Neuheit noch die Behauptung „so gibt es das sonst nirgends“ ist hier nachgewiesen.

Die Literaturbezüge stehen im folgenden eigenen Quellenabschnitt. Die Implementierungslinks weiter oben zeigen, wo EtuSchlopf die jeweiligen Verfahren tatsächlich verwendet. Beides gehört zur Dokumentation: Literatur erklärt das bekannte Verfahren; Code und Projektverträge belegen seine konkrete Umsetzung und seine Grenzen.

## Quellen

Die Grundlagenwerke erklären die Verfahren. Die verlinkten Implementierungen und Prüfläufe belegen den jeweiligen Einsatz in EtuSchlopf.

### Mathematische und algorithmische Grundlagen

- **Quadratische Ausgleichsverfahren:** C. F. Gauss, *Theoria motus corporum coelestium* (1809); A.-M. Legendre, *Nouvelles méthodes pour la détermination des orbites des comètes* (1805; [Originalausgabe im BnF-Katalog](https://catalogue.bnf.fr/ark:/12148/cb30775990h)). Historischer Bezug zur Methode der kleinsten Quadrate. Die quadratischen Kosten und ihre Gewichte in EtuSchlopf sind eigene Modellentscheidungen; die Referenzen begründen weder eine Normalverteilung der Musikdaten noch diese konkreten Gewichte.
- **Dynamische Programmierung:** Richard Bellman, *Dynamic Programming*, Princeton University Press (1957; [Verlagsseite](https://press.princeton.edu/books/paperback/9780691146683/dynamic-programming)). Grundlage für mehrstufige Optimierung. Die konkreten Registerzustände, Gruppenpflichten und Kosten werden durch die Projektverträge definiert.
- **Optimale Pfade in Zustandsfolgen:** Andrew J. Viterbi, „Error Bounds for Convolutional Codes and an Asymptotically Optimum Decoding Algorithm“, *IEEE Transactions on Information Theory* 13(2), 260–269 (1967), [DOI: 10.1109/TIT.1967.1054010](https://doi.org/10.1109/TIT.1967.1054010). Methodischer Bezug zu dynamischen Pfadverfahren; EtuSchlopf behauptet damit weder eine Implementierung des ursprünglichen Kanaldecoders noch dessen statistische Voraussetzungen.
- **Intervallgraphen und perfekte Graphen:** Martin Charles Golumbic, *Algorithmic Graph Theory and Perfect Graphs*, Academic Press (1980; [Originalausgabe beim Verlag](https://www.sciencedirect.com/science/book/9780122892608)). Grundlage für die Beziehung zwischen maximaler Überlappung und Farbzahl im beschriebenen Intervallmodell. Zusätzliche musikalische Bindungen folgen nicht aus dem Graphsatz.
- **Gezielte Frequenzauswertung:** Gerald Goertzel, „An Algorithm for the Evaluation of Finite Trigonometric Series“, *The American Mathematical Monthly* 65(1), 34–35 (1958), [DOI: 10.2307/2310304](https://doi.org/10.2307/2310304). Literaturbezug zur oben angegebenen Spektralrekurrenz. Frequenzraster, Fenster, Normalisierung und Interpretation bleiben die konkreten Entscheidungen der Audioprobe.

### Programmnachweise und Integrität

- **Programminvarianten:** C. A. R. Hoare, „An Axiomatic Basis for Computer Programming“, *Communications of the ACM* 12(10), 576–580, 583 (1969), [DOI: 10.1145/363235.363259](https://doi.org/10.1145/363235.363259). Grundlage für Aussagen über Vorbedingungen, Programme und Nachbedingungen. Ein Projekt-Gate ist dadurch noch kein vollständiger formaler Beweis des Programms.
- **Beweistragender Code:** George C. Necula, „Proof-Carrying Code“, *POPL* (1997), [DOI: 10.1145/263699.263712](https://doi.org/10.1145/263699.263712). Konzeptionelle Verwandtschaft der SchlopfFormel mit expliziten, maschinell prüfbaren Übergangsbelegen. Die projektinternen Receipts und Authorities sind keine behauptete vollständige Implementierung dieses PCC-Systems.
- **SHA-256:** National Institute of Standards and Technology, *Secure Hash Standard*, FIPS PUB 180-4 (2015), [NIST-Publikation](https://csrc.nist.gov/pubs/fips/180-4/upd1/final), [DOI: 10.6028/NIST.FIPS.180-4](https://doi.org/10.6028/NIST.FIPS.180-4). Referenz für den Hashalgorithmus; Bytebindung ersetzt keine Herkunfts- oder musikalische Identitätsaussage.
- **FNV-Hash:** Glenn Fowler, Landon Curt Noll und Phong Vo, [FNV-1/FNV-1a: Beschreibung und Referenzcode der Autoren](https://www.isthe.com/chongo/tech/comp/fnv/index.html). Bezug für die nichtkryptografischen Fingerprints. FNV wird ausdrücklich nicht als kryptografischer Integritätsbeweis eingeordnet.

### Umbruch, Audiomodell und Spielbarkeitskontext

- **Umbruchprobleme:** Donald E. Knuth und Michael F. Plass, „Breaking Paragraphs into Lines“, *Software: Practice and Experience* 11(11), 1119–1184 (1981), [DOI: 10.1002/spe.4380111102](https://doi.org/10.1002/spe.4380111102). Konzeptionelle Verwandtschaft der Schlöpfchen-Formel und endlicher Umbruchvergleiche mit global bewerteten Partitionen. Absatzumbruch und musikalische Seitenoptimierung haben verschiedene Zulässigkeitsbedingungen; ein vollständiger Seitenoptimierer wird daraus nicht abgeleitet.
- **Tatsächlich verwendetes Fremdmodell Basic Pitch:** Rachel M. Bittner, Juan José Bosch, David Rubinstein, Gabriel Meseguer-Brocal und Sebastian Ewert, „A Lightweight Instrument-Agnostic Model for Polyphonic Note Transcription and Multipitch Estimation“, *ICASSP* (2022), [Autorenversion: arXiv:2203.09893](https://arxiv.org/abs/2203.09893). Bezug des externen neuronalen WAV-Sensors. Die Benchmark-Werte des Papers sind keine EtuSchlopf-Messwerte und erzeugen keine musikalische Schreibbefugnis.
- **Handspanne und alternative Klaviatur:** Craig Turner et al., „An alternative 6-inch octave keyboard reduces forearm muscle activation level and improves finger postures in expert pianists with smaller hand spans“, *Applied Ergonomics* 133, 104709 (2026; online 2025), [DOI: 10.1016/j.apergo.2025.104709](https://doi.org/10.1016/j.apergo.2025.104709). Empirischer Kontext für die Trennung von Musikanspruch, Instrumentengeometrie und individueller Fähigkeit; keine Kalibrierung unserer Spielformel.
- **Handspanne, Klaviaturgröße und EMG:** Ju-Yang Chi et al., „Interaction between hand span and different sizes of keyboards on EMG activity in pianists: An observational study“, *Applied Ergonomics* 97, 103518 (2021), [DOI: 10.1016/j.apergo.2021.103518](https://doi.org/10.1016/j.apergo.2021.103518). Weiterer empirischer Kontext; daraus folgt kein universeller Spannweiten- oder Löschgrenzwert für EtuSchlopf.

Die konkrete Einbindung des Audiomodells und der beiden Handspannenstudien ist zusätzlich unter [Forschungsbezüge und ihre tatsächliche Rolle](https://github.com/Funcry241/EtuSchlopf/blob/main/docs/RESEARCH_CONTEXT.md) dokumentiert. Die Projektbegriffe und selbst gesetzten Kosten werden durch diese Literatur nicht nachträglich zu extern bestätigten mathematischen oder musikalischen Gesetzen.
