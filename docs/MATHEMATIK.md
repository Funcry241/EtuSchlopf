# Mathematik in EtuSchlopf

**Fachlicher Stand: nativ angenommener r1683, 8. November 2026.** Diese Überarbeitung gehört zur Dokumentationsrevision r1684. Sie verändert keine Formeln im Programm, Gewichte, Suchräume, Tests oder Schreibbefugnisse.

EtuSchlopf verwendet Mathematik für musikalische Entscheidungen, Darstellungsentscheidungen, Messungen und technische Beweise. Diese Aufgaben sind verschieden: Eine Erhaltungsbilanz beweist keine Lesbarkeit; ein lesbarer Registerkurs beweist keine menschliche Spielbarkeit; eine bestandene Prüfung erlaubt nur den Übergang, dessen Bedingungen sie tatsächlich abdeckt.

Diese Seite ist eine **nach Aufgaben gegliederte Bestandsaufnahme** der überprüften Verfahrensfamilien. Sie ersetzt weder die kanonischen Verträge noch ein vollständiges Funktionsinventar. Die frühere Liste von 21 Themen war unvollständig. Insbesondere fehlten eigenständige Beschreibungen der Tonartbewertung, Phrasen- und Wiederholungsanalyse, Belegungsgraphen, WAV-Puls- und Spektralanalyse, vorhandenen Systemumbruchplanung und exakten Suchbeschleunigungen. Auch diese erweiterte Seite behauptet nicht, jede arithmetische Operation jeder Abhängigkeit aufzulisten.

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

### SchlopfFormel: Herkunft, Zuständigkeit und Übergangsnachweis

Unter diesem Namen fasst diese Seite die Bedingungen für einen erlaubten, belegten Übergang zusammen. Es gibt im Code keine einzelne kanonische Gesamtfunktion namens SchlopfFormel.

Ein ausführbarer Teilkern ist die Entscheidung über die Einstufung **eines** Änderungsbelegs:

$$A(e)=[p(e)=1]\land[m(e)+d(e)+i(e)=0].$$

$p$ zählt erlaubende Einstufungen, $m,d,i$ zählen fehlende, doppelte und ungültige Belege. Die Zähler sind keine Wahrscheinlichkeiten. Der Kern steht in `adoption_decision` in [ssc_evidence_proof.rs](../src/ssc_evidence_proof.rs). Er ist allein noch keine vollständige Freigabeformel.

Die Zusammenstellung verlangt außerdem die zuständige Authority, die Herkunft des Belegs, den gebundenen Vor- und Nachzustand, die jeweils erforderlichen Erhaltungsbedingungen und den Schutz gegen unzulässige Wiederverwendung. Die konkreten Prüfungen und ihre Zuständigkeit bleiben in den ausführbaren Verträgen. Master und Render besitzen getrennte Herkunftsketten.

**Grenze:** Eine zulässige Transformation ist damit nicht automatisch musikalisch gut. Ein Fingerprint ersetzt keine Ereignisidentität und kein musikalisches Qualitätsurteil.

### Schlöpfchen-Formel: die angestrebte Seitenfüllung

Der bisher vereinbarte Entwurf lautet:

$$\min_{x\in\mathcal F}(\text{Seitenzahl}(x),\text{übermäßige Dehnung}(x),\text{Unausgewogenheit}(x)).$$

$\mathcal F$ müsste die tatsächlich lesbaren und materialisierbaren Kandidaten enthalten. Lesbarkeit ist darin Voraussetzung. Horizontale Taktprofile, vertikale Systemhüllen und Seitenbelegung sind getrennte Größen; die Systemreihenfolge bleibt erhalten.

**Status:** Diese Gesamtformel ist nicht als vollständiger Seitenoptimierer implementiert. Die bestehende horizontale Systemplanung ist davon zu unterscheiden. Seitenverdichtung bleibt zurückgestellt, bis sichtbare Mindestabstände und die zugehörige Vergleichsgrundlage festgelegt sind. Aus einer großen beobachteten Lücke folgt weder ein Mindestabstand noch automatisch die Erlaubnis zum Zusammenrücken.

Quelle: [Fehlerklasse zur unnötig beibehaltenen Seite](selector_derivation_contract_redundant_page_retention.md), [aktueller Arbeitsrahmen](assistant_memory/050_aktueller_stand.md). Dessen alte Revisionsangaben ersetzen keine aktuellen Laufbelege.

### Symbolische Spielformel: Anforderungen und Fähigkeiten

$$\exists a:\ S(a)\land K(a)\land U(a)\land W(a).$$

$a$ belegt physische Verpflichtungen mit Hand-/Finger-Plätzen. $S$ steht für Strukturbedingungen, $K$ für statische Konfigurationen, $U$ für Übergänge und $W$ für Wiederanschläge gleicher Tasten. Anforderungen der Musik und Fähigkeiten eines Pianistenmodells bleiben getrennt.

**Status:** Audit. Die Formel und ein struktureller Beleg können vollständig sein, während die Fähigkeiten eines konkreten Menschen weiterhin nicht modelliert sind. Ein `MODEL_RELATIVE_ONLY`-Urteil beweist weder ein bestimmtes Spieltempo noch menschliche Spielbarkeit. Eine fehlende Fähigkeitseinstufung rechtfertigt keine Notenlöschung.

Quelle: [pianistic_symbolic_capability_reachability.rs](../src/audit_only/pianistic_symbolic_capability_reachability.rs) und die dort konsumierten Fähigkeitsmodelle.

## Verfahrensfamilien im aktuellen Quellstand

Die Quellen nennen die jeweiligen Implementierungsstellen, nicht automatisch einen universellen mathematischen Satz über das gesamte Programm.

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
| 12 | Separater Schlüsselverlauf: dynamische Programmierung | Entscheidet im jeweils zuständigen Pfad. Bei gebundenem C3-Kurs darf keine unabhängige spätere Entscheidung dessen Darstellung ersetzen. | [rustig_clef_planning.rs](../src/structural_write_authority/rustig_clef_planning.rs) |
| 13 | C3: gemeinsamer Schlüssel-/Oktavkurs mit exakter Abschnittsdauer | Entscheidet in vollständig belegten, unterstützten Domänen. Systemübergreifende Kopplung bleibt derzeit eine mögliche UNKNOWN-Grenze. | [Suchkern](../src/pipeline_shards/joint_reading_c3_course.rs), [Adapter](../src/pipeline_shards/joint_reading_c3_adapter.rs) |
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

## Zentrale Formeln und ihre Beweisgrenzen

### Bilanz, Identität und Sichtbarkeit

$$N_{\mathrm{Quelle}}=N_{\mathrm{aktiv}}+N_{\mathrm{inaktiv}},\qquad
N_{\mathrm{aktiv}}=N_{\mathrm{sichtbar}}+N_{\mathrm{verborgen}}.$$

Diese Gleichheiten gelten nur für die im jeweiligen Bericht sauber abgegrenzte Grundgesamtheit. Quellanschläge, geschriebene Fragmente und sichtbare Köpfe sind nicht austauschbare Nenner.

Eine bestandene Anzahlbilanz ist notwendig für die betreffende Buchführung, aber kein hinreichender Nachweis individuellen Erhalts: Eine Löschung und eine andere Einfügung können sich zahlenmäßig ausgleichen. Der aktuelle Bericht bezeichnet diese Projektion ausdrücklich als `cardinality_only_not_a_late_source_binding`. Herkunft und musikalische Eigenschaften brauchen eigene gebundene Nachweise.

`CanonicalSourceRetention` und `ArrangementVisibleRetention` beantworten verschiedene Fragen. Eine verborgene Note ist nicht allein deshalb entfernt. Aus ihrem Verbergen folgt weder ein nachgewiesener Spielbarkeitskonflikt noch ein Beweis, dass die verbleibende Darstellung menschlich spielbar ist.

### Intervalle und Stimmenbedarf

Für ein geschriebenes Ereignis gilt das halboffene Intervall $[t,t+d)$. Bei einem Sweep werden Enden vor gleichzeitigen Anfängen verarbeitet. Im vereinfachten Modell werden gleiche Einsätze und Dauern je Part/System/Takt zu einem Ereignis zusammengefasst.

Für den dadurch erzeugten Intervallgraphen gilt:

$$\chi(G)=\omega(G).$$

Die maximale Überlappung ist damit die minimale Farbzahl **dieses Graphen**. Musikalische Linien, ursprüngliche Balkengruppen, Pausenpflichten, Akkordbindungen und technische Importkanäle besitzen zusätzliche Bedingungen. Aus der Graphzahl allein folgt keine erlaubte Zusammenlegung solcher Rollen. Der aktuelle `VoiceModel`-Bericht nennt diese Grenze ausdrücklich.

### Quadratische Verteilungskosten

Die implementierte lokale Bewertung enthält:

$$E(t,s)=3\Delta t^2+24\Delta p^2+42^2[\text{System verschieden}]
+12^2[\text{Stimme verschieden}]+2(\Delta t-24k)^2+12000[\text{Platz bereits verwendet}].$$

Zeit wird hier in der vorhandenen 96-Ticks-pro-Viertel-Projektion gemessen, Tonhöhe in Halbtönen. Der Code begrenzt zusätzlich den Kandidatenraum und prüft je nach Aufruf die Zielbelegung. Die Formel allein ist weder vollständige Zulässigkeit noch Erhaltungsbeweis. Der Name „Gauss“ bezeichnet hier quadratische Kosten; er behauptet keine Gauß-Verteilung der Musikdaten, kein statistisches Maximum-Likelihood-Modell und keine globale optimale Zuordnung aller Töne.

Gewichte, Kandidatengrenzen und Strafen sind Projekteinstellungen. Ein Kandidaten-Score ist kein gemessener menschlicher Lese- oder Spielaufwand.

### Tonartprofil und Schreibweise

Die aktuelle Profilbewertung summiert Beiträge pro Ton. Vereinfacht mit dem vom Owner bestimmten Gewicht $w_n$, Tonklasse $c_n$ und Profil $P$ lautet der im Code sichtbare Kern:

$$T(P)=\sum_n w_n\big(3[c_n\in\mathrm{Skala}(P)]-4[c_n\notin\mathrm{Skala}(P)]+2[c_n=\mathrm{Tonika}(P)]+[c_n=\mathrm{Dominante}(P)]+[P\text{ ist Moll}\land c_n=\mathrm{erhöhte\ Leittonklasse}(P)]\big).$$

Dies ist eine Projekteinstellung und kein statistisch kalibrierter Tonartbeweis. Die tatsächliche Schreibweisenentscheidung verlangt zusätzliche Bedingungen zu Geltungsbereich, Stabilität, konkurrierenden Lesarten und Vorzeichenkosten. Enharmonische Umbenennung darf die klingende Tonhöhe nicht verändern. Ein Profilwert allein erteilt weder eine neue Tonhöhe noch eine neue Abschnittsgrenze.

### Separater Schlüsselplan und C3 sind verschiedene Zielfunktionen

Der separate Lesbarkeitskurs verwendet den graduellen Druck

$$p(l)=l^2+8\max(l-3,0)$$

und Kosten für Schlüsselwechsel, Redundanzen und kurze Ausflüge. Seine bekannten 24 Einheiten je Wechsel beziehungsweise kurzem Ausflug sind Setzungen dieses Modells. Daraus folgt keine universelle Grenze von drei Hilfslinien.

C3 verwendet dagegen die eingefrorene **lexikografische** Reihenfolge

$$ (h_8,h_7,h_6,h_5,h_4,k_o,u_k,k_s,g_b,h_3,c_h,h_2,h_1). $$

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

$$\neg\big(L_z(n)\ge4\ \land\ L_z(n)>L_0(n)\big).$$

Ein Kandidat darf also keine neue oder verschärfte Belastung ab vier Hilfslinien erzeugen. Ein bereits belasteter Ausgangszustand wird nicht durch Behauptung unsichtbar. Unter den zulässigen vollständigen Kursen zählt die erste abweichende Kostenkomponente; Gleichstand erhält den Ausgang.

**Wichtige Folge:** Weniger Umschaltungen gewinnt nicht immer. Zum Beispiel kann die Vermeidung eines Vier-Hilfslinien-Kopfes einen kurzen Schlüsselwechsel rechtfertigen. Diese bewusste C3-Priorität ist nicht identisch mit der alten skalaren Druckfunktion und kein universelles Lesbarkeitsgesetz.

Der Suchkern trägt zusätzlich zum Registerzustand das genaue, bei einer Taktlänge gesättigte Abschnittsalter. Rationale Taktkoordinaten behandeln Takt- und Divisionswechsel. Ein Ressourcenabbruch oder eine unbekannte Abhängigkeit beweist kein Optimum. Maßgeblich ist [die vollständige C3-Policy](joint_register_dp_policy_v1.md).

### Graphzeugen der Spielformel

Die symbolische Struktur enthält Ungleichungen zwischen Belegungsvariablen. Der Code bildet einen Graphen, erzeugt eine Smallest-last-/Degenerationsordnung und prüft die daraus gewonnene Zuordnung gegen die tatsächlichen Kanten. Eine passende Clique kann umgekehrt einen strukturellen Konflikt belegen.

Ein struktureller Zeuge schließt nur diese Ungleichungen. Handkonfiguration, Übergang, Wiederanschlag, Fähigkeiten und Zeitmodell bleiben zusätzliche Verpflichtungen. Weder die maximale Gleichzeitigkeit noch ein gefärbter Graph allein beweist einen ausführbaren Fingersatz.

### Audio: Pulsanalyse und Goertzel-Rekurrenz

Die WAV-Pulsanalyse erzeugt Hypothesen aus Energie- und Transientenanstiegen, lokalen Spitzen sowie Tempo und Phase. Eine zeitliche Abbildung und Residuen beschreiben Abweichungen vom nominellen Raster. Die Audiohypothesen werden vor der späteren MIDI-Evaluation festgehalten; die Vergleichsquelle darf nicht unbemerkt das unabhängige Messergebnis erzeugen.

Für ausgewählte Frequenzen verwendet die harmonische Evidenzprobe tatsächlich eine Goertzel-Rekurrenz:

$$s_j=x_j+2\cos(\omega)s_{j-1}-s_{j-2},\qquad \omega=2\pi f/f_s.$$

Die Frequenzzuordnung der Probe verwendet $f(m)=440\cdot2^{(m-69)/12}$: gleichstufige Stimmung mit A4 = 440 Hz ist hier eine Modellannahme. Der Code normiert die berechnete Energie durch das Quadrat der Fensterlänge. Dies ist gezielte Frequenzanalyse, keine Behauptung, dass im Projekt eine allgemeine FFT-Pipeline implementiert sei. Ein hoher Wert kann auch von Obertönen oder überlagerten Klängen stammen. Basic Pitch ist zusätzlich ein fremder neuronaler Sensor; beide Beobachtungen erhalten keine automatische musikalische Schreibbefugnis.

### Systemplanung ist nicht Seitenoptimierung

Der vorhandene globale horizontale Reflow vergleicht endliche Systempartitionen unter seinen Kapazitätsbedingungen. Sein Vergleichsschlüssel enthält in dieser Reihenfolge Systemzahl, Einzel-Takt-Systeme, erzwungene Überkapazitäts-Einzelplätze, quadratische Restkapazität und den deterministischen Partitionsvergleich. Die Seitenzuordnung bleibt an die vorhandene Autorität gebunden.

Die geometrischen Readbacks untersuchen tatsächliche Hüllen und Abstände, soweit PDF-Syntax und Zuordnung unterstützt sind. Daraus folgt weder eine vollständige Kollisionsfreiheit aller denkbaren Zeichen noch eine implementierte Schlöpfchen-Seitenoptimierung.

### Exakte Beschleunigung ist keine neue Musikentscheidung

Für additive Passagenkosten können Präfixsummen dieselbe Summe liefern:

$$\sum_{i=a}^{b-1}c_i=P_b-P_a.$$

Die Indizes entstehen aus der exakten halboffenen Zeitbedingung, nicht aus einer geratenen Notennähe. Die Umordnung muss Kosten, Kandidaten und Gleichstandsregeln erhalten.

Die Bogenprüfung verwendet einen anderen Index: Für jede Nummer beantworten erster sichtbarer Anfang und letzter sichtbarer Schluss dieselben Existenzfragen wie die frühere Vollsuche. Das ist kein allgemeines globales Matching aller möglichen Bögen. Eine solche Erweiterung wäre eine eigene musikalische Entscheidung.

### Projektion und Integrität

Eine Aufbewahrungsprojektion $\pi$ ist nur dann ausreichend, wenn spätere Beobachtungen tatsächlich durch sie faktorisieren. Präziser als ein bloßer Größenvergleich ist die Bedingung

$$\pi(D_1)=\pi(D_2)\Rightarrow O_L(D_1)=O_L(D_2)$$

für die betroffenen erreichbaren Leser $L$ und ihre Beobachtung $O_L$, einschließlich der relevanten Fortsetzungen. Eine identische aktuelle Ausgabe allein genügt nicht für künftige Mutationen. Die ausführbare Aufbewahrungspolitik bleibt der konkrete Nachweis.

Hashes sind davon getrennt: Sie binden Bytes unter ihren jeweiligen Kollisionsannahmen. FNV-1a ist kein kryptografischer Beweis; auch SHA-256 ist mathematisch nicht injektiv über beliebig lange Eingaben. Provenienz, tatsächlich mitgeführte Ereignisidentität und bytebezogene Integritätsprüfung erfüllen verschiedene Aufgaben.

## Aktueller Nachweis und verbleibende Grenzen

Der frische r1683-Export bestätigt Annahme des ausgelieferten Kandidaten, Build und Projekt-Clippy mit Exit 0, 2.992 bestandene Tests bei vier ignorierten Tests sowie zehn formal erfolgreiche Projekte. Diese Abnahme beweist die geprüften Verträge, nicht allgemeine menschliche Spielbarkeit oder ein fehlerfreies Notenbild.

| Projekte im aktuellen Korpus | Übernommener C3-Kurs nach r1683 | Einordnung |
|---|---|---|
| 5, 9, 10 | Beide Systeme | Unteres System durch r1683 neu verfügbar. |
| 7, 8 | Beide Systeme | Bereits vorher übernommen. |
| 1, 2, 3, 4, 6 | Kein gemeinsamer Kurs übernommen | Systemübergreifende Gruppenabhängigkeit nicht geschlossen; Ausgang bleibt erhalten. |

Die Tabelle ist Evaluation, kein produktiver Selektor. r1683 führt ursprünglich vorhandene Pausenanker getrennt von klingenden Balkenköpfen bis zur gemeinsamen Entscheidung mit. Ein fehlender Fortsetzungsbeleg bleibt unbekannt; der erhaltene Akkordton darf dabei seinen negativen Abhängigkeitsbeleg nicht verlieren. Eine Pause wird dadurch weder zu einem klingenden Kopf noch zu einer positiven Balkenentscheidung.

Gegenüber r1682 bleiben alle zehn Master-Notenfolgen strukturell gleich. Die Render-Noteninhalte bleiben bis auf zwei Balkengruppierungen und sieben Pausenglyphen-Positionen erhalten; Tonhöhen, Zeitwerte, Stimmen, Systeme, Sichtbarkeit und Bogeninhalte sind unverändert. Die PDF-Seitenzahl bleibt 39; 30 Seiten sind im durchgeführten Pixelvergleich bei 144 dpi identisch, neun Seiten ändern sich. Die geänderten Seiten wurden zusätzlich gesichtet. Das ist ein konkreter Artefaktvergleich und kein universeller Qualitätsbeweis.

Historische Zahlen aus r1556 oder r1559 werden nicht als heutige Kennzahlen fortgeschrieben. Das alte Speichermodell $M\approx0{,}0054FS+0{,}0061GS$ bleibt eine historische Regressionshypothese: Die abgedruckte Form besitzt zwei Koeffizienten; die frühere Textangabe „drei Parameter“ ist ohne zusätzlichen Modellterm nicht nachvollziehbar. Ohne Rohdaten und erneute Anpassung wird daraus kein aktueller Speicherbeweis abgeleitet.

Auch die frühere Weglass-Untergrenze gilt nur unter ihrem vereinfachten Modell aus monotoner Handaufteilung, gewählter Spannweite und begrenzter Fingerzahl. Sie ist keine allgemeine Untergrenze für notwendige Notenlöschungen beim menschlichen Klavierspiel.

## Was noch offen ist

- Systemübergreifende Registerabhängigkeiten vollständig binden und vergleichen, bevor eine neue gekoppelte Optimierung behauptet wird.
- Bewertungspräferenzen und Zulässigkeit getrennt begründen; symbolische Kosten nicht als empirisch kalibrierte Lesezeit ausgeben.
- Menschliche Fähigkeiten und Tempo belastbar kalibrieren, bevor aus Anforderungsprofilen ein personenbezogenes Spielbarkeitsurteil folgt.
- Seitenverdichtung erst nach dem zurückgestellten Mindestabstands- und Lesbarkeitsvergleich angehen.
- Unterschiedliche Platzierungsfamilien und ihre Erhaltungs-/Belegungsbedingungen einzeln verfolgen; ein Audit-Zähler ist keine gemeinsame Schreibregel.
- Eine künftige globale Bogen-Zuordnung von der heutigen indizierten Existenzprüfung unterscheiden.
- Die Bestandsaufnahme bei neuen Ownern, Suchkernen oder Messverfahren aktualisieren. Neue Behauptungen erhalten einen Quellbezug und eine ausdrücklich begrenzte Beweisaussage.

## Herkunft und Eigenanteil

Standardverfahren bleiben Standardverfahren: dynamische Programmierung, lexikografische Ordnung, Intervallgraphen, Färbung, Nichtdominanz, rationale Arithmetik, quadratische Kosten, Spektralrekurrenzen, Deskriptivstatistik und Hashfunktionen wurden nicht für EtuSchlopf erfunden.

Unser Eigenanteil liegt in den musikalischen Modellen, dem Zusammenspiel der Bedingungen, den gebundenen Owner-Übergängen und den gewählten Prioritäten. Die drei Projektbegriffe benennen solche Zusammenstellungen. Weder deren wissenschaftliche Neuheit noch die Behauptung „so gibt es das sonst nirgends“ ist hier nachgewiesen.

Historische Literaturanker der bisherigen Seite sind Bellman und Viterbi für dynamische Verfahren, Golumbic für Intervallgraphen, Gauss und Legendre für quadratische Ausgleichsverfahren, Hoare für Programm-Invarianten, Necula für beweistragenden Code sowie Knuth/Plass für die Verwandtschaft mit Umbruchproblemen. Diese Verwandtschaft ersetzt keinen konkreten EtuSchlopf-Beweis. Für den externen Basic-Pitch-Sensor bleibt Bittner et al., ICASSP 2022, der benannte Modellbezug; dessen Benchmark ist kein Messwert unseres Korpus. Die vorliegende Überarbeitung ist eine Quell- und Statusprüfung, keine erneute vollständige Literaturrecherche.
