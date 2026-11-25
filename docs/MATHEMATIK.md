# Mathematik in EtuSchlopf

**Wie lange soll eine gespielte Note im Notenbild stehen?** EtuSchlopf leitet die Dauer einer frei gespielten Klaviernote aus den Einsätzen, der Artikulation und den gleichzeitig laufenden Stimmen ab. Beim Vergleich mit einer Druckvorlage lassen sich Einsatz, Tonhöhe und geschriebene Dauer einzeln prüfen. Die folgenden Formeln beschreiben diese Entscheidungen und die Grenzen der Auswertung. [Aktuelle Referenzen](examples/README.md#aktuelle-referenzen) · [Aktueller Windows-Prüfbeleg](evidence/r2005/README.md)

Diese Seite bündelt die Mathematik des Projekts. Für einen Verweis genügt die Kapitelnummer, etwa [Kapitel 3.2 · Folgeeinsatz oder Akkordende](#kapitel-3-2). Bestehende Kapitel behalten ihre Nummer.

<details>
<summary>Inhaltsverzeichnis mit allen Kapiteln</summary>

- [1 Wie die Aussagen zu lesen sind](#kapitel-1)
- [2 Die drei Projektbegriffe](#kapitel-2)
  - [2.1 SchlopfFormel für belegte Übergänge](#kapitel-2-1)
  - [2.2 Schlöpfchen-Formel für die Seitenfüllung](#kapitel-2-2)
  - [2.3 Symbolische Spielformel für Hand und Finger](#kapitel-2-3)
- [3 Vom Anschlag zur geschriebenen Dauer](#kapitel-3)
  - [3.1 Das Zeitraster einer freien Einspielung](#kapitel-3-1)
  - [3.2 Folgeeinsatz oder Akkordende](#kapitel-3-2)
  - [3.3 Ein Quellton über mehrere geschriebene Fragmente](#kapitel-3-3)
- [4 Verfahrensfamilien im aktuellen Quellstand](#kapitel-4)
- [5 Lokale Entscheidungen zu Pausen und Stimmen](#kapitel-5)
- [6 Zentrale Formeln und ihre Beweisgrenzen](#kapitel-6)
  - [6.1 Bilanz, Identität und Sichtbarkeit](#kapitel-6-1)
  - [6.2 Intervalle und Stimmenbedarf](#kapitel-6-2)
  - [6.3 Welche Balkenpaare bleiben erhalten?](#kapitel-6-3)
  - [6.4 Welche Stimmfortsetzung ist erzwungen?](#kapitel-6-4)
  - [6.5 Notationszeit, Haltebögen und Handzuordnung](#kapitel-6-5)
  - [6.6 Kandidatenauswahl und Pareto-Front](#kapitel-6-6)
  - [6.7 Wiederholung bei anderer Tonhöhe und anderem Tempo](#kapitel-6-7)
  - [6.8 Quadratische Verteilungskosten](#kapitel-6-8)
  - [6.9 Tonartprofil und Schreibweise](#kapitel-6-9)
  - [6.10 Separater Schlüsselplan und gemeinsamer Registerkurs sind verschiedene Zielfunktionen](#kapitel-6-10)
  - [6.11 Graphzeugen der Spielformel](#kapitel-6-11)
  - [6.12 Audio: Pulsanalyse und Goertzel-Rekurrenz](#kapitel-6-12)
  - [6.13 Horizontaler Umbruch und begrenzte gemeinsame Planung](#kapitel-6-13)
  - [6.14 Beschleunigung mit gleichen Entscheidungen](#kapitel-6-14)
  - [6.15 Speicherplanung für den Korpus](#kapitel-6-15)
  - [6.16 Projektion und Integrität](#kapitel-6-16)
  - [6.17 Phrasierungsbögen und Konturabstände](#kapitel-6-17)
- [7 Quellzeit, Pausen und Balken](#kapitel-7)
- [8 Stimmfortsetzungen prüfen](#kapitel-8)
- [9 Korpusprüfung und Grenzen](#kapitel-9)
  - [9.1 Vergleich der eigenen Etüden](#kapitel-9-1)
- [10 Nächste mathematische Vorhaben](#kapitel-10)
  - [10.1 Notendauern freier Einspielungen](#kapitel-10-1)
  - [10.2 Stimme und Hand über eine Passage](#kapitel-10-2)
  - [10.3 Lesbare Seitenfüllung](#kapitel-10-3)
  - [10.4 Rechenzeit im gemeinsamen Registerplan](#kapitel-10-4)
  - [10.5 Pausenhöhe und Seitenlage](#kapitel-10-5)
- [11 Herkunft und Eigenanteil](#kapitel-11)
- [12 Quellen](#kapitel-12)
  - [12.1 Mathematische und algorithmische Grundlagen](#kapitel-12-1)
  - [12.2 Programmnachweise und Integrität](#kapitel-12-2)
  - [12.3 Umbruch, Audiomodell und Spielbarkeitskontext](#kapitel-12-3)
  - [12.4 Forschung zu Musiktranskription und Notensatz](#kapitel-12-4)
  - [12.5 Arbeiten aus der OpenAI-Mathematiksammlung](#kapitel-12-5)
  - [12.6 Datensatz und Projektnachweise](#kapitel-12-6)

</details>

| Frage | Wo die Antwort steht |
| --- | --- |
| Wie gelangt eine frei gespielte Note in den gedruckten Takt? | [Kapitel 3 · Vom Anschlag zur geschriebenen Dauer](#kapitel-3) |
| Welche Zusammenhänge beschreibt das Modell für Stimmen, Register und Seiten? | [Kapitel 4 · Verfahrensfamilien](#kapitel-4) und [Kapitel 6 · Formeln](#kapitel-6) |
| Was wurde mit welchen Stücken geprüft? | [Kapitel 9 · Korpusprüfung](#kapitel-9) · [Windows-Vollkorpus r2005](evidence/r2005/README.md) |
| Woran ließe sich als Nächstes arbeiten? | [Kapitel 10 · Nächste mathematische Vorhaben](#kapitel-10) |

<a name="kapitel-1"></a>
<a name="wie-die-aussagen-zu-lesen-sind"></a>

## 1 Wie die Aussagen zu lesen sind

Hochgestellte Ziffern führen von einer Aussage zu ihrer Fundstelle in [Kapitel 12](#kapitel-12). Dort führt der Pfeil mit der Kapitelnummer zur selben Textstelle zurück. Quellcode und Prüfberichte sind zusätzlich direkt bei den Projektbefunden verlinkt.

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
| Entwurf | Noch keine produktive Implementierung der beschriebenen Gesamtaufgabe. |

`UNKNOWN`, eine unvollständige Suche und eine bewiesene Unzulässigkeit sind unterschiedliche Ergebnisse. Fehlende Information wird weder zu Nullkosten noch zu einer Erlaubnis. „Optimal“ bedeutet höchstens optimal im ausdrücklich gebundenen Modell und Suchraum.

<a name="kapitel-2"></a>
<a name="die-drei-projektbegriffe"></a>

## 2 Die drei Projektbegriffe

<a name="kapitel-2-1"></a>
<a name="schlopfformel-herkunft-zuständigkeit-und-übergangsnachweis"></a>
<a name="schlopfformel-für-belegte-übergänge"></a>

### 2.1 SchlopfFormel für belegte Übergänge

Unter diesem Namen fasst diese Seite die Bedingungen für einen erlaubten, belegten Übergang zusammen. Es gibt im Code keine einzelne kanonische Gesamtfunktion namens SchlopfFormel.

Ein ausführbarer Teilkern ist die Entscheidung über die Einstufung **eines** Änderungsbelegs:

```math
A(e)=[p(e)=1]\land[m(e)+d(e)+i(e)=0].
```

$p$ zählt erlaubende Einstufungen, $m,d,i$ zählen fehlende, doppelte und ungültige Belege. Die Zähler sind keine Wahrscheinlichkeiten. Der Kern steht in `adoption_decision` in [ssc_evidence_proof.rs](../src/ssc_evidence_proof.rs). Er ist allein noch keine vollständige Freigabeformel.

Die Zusammenstellung verlangt außerdem die zuständige Authority, die Herkunft des Belegs, den gebundenen Vor- und Nachzustand, die jeweils erforderlichen Erhaltungsbedingungen und den Schutz gegen unzulässige Wiederverwendung. Die konkreten Prüfungen und ihre Zuständigkeit bleiben in den ausführbaren Verträgen. Master und Render besitzen getrennte Herkunftsketten. Vor- und Nachbedingungen haben eine formale Tradition bei Floyd und Hoare.<a name="beleg-2-1-bedingungen"></a><sup><a href="#quelle-hoare">[16]</a></sup> Für die getrennte Prüfung eines mitgelieferten Belegs bietet Neculas *Proof-Carrying Code* einen Vergleich.<a name="beleg-2-1-pruefung"></a><sup><a href="#quelle-necula">[17]</a></sup>

**Grenze:** Eine zulässige Transformation ist damit nicht automatisch musikalisch gut. Ein Fingerprint ersetzt keine Ereignisidentität und kein musikalisches Qualitätsurteil.

<a name="kapitel-2-2"></a>
<a name="schlöpfchen-formel-die-angestrebte-seitenfüllung"></a>
<a name="schlöpfchen-formel-für-die-seitenfüllung"></a>

### 2.2 Schlöpfchen-Formel für die Seitenfüllung

Der Entwurf ordnet zulässige Seitenpläne nach drei Kriterien.

```math
x^\star\in\underset{x\in\mathcal F}{\mathrm{argmin}_{\mathrm{lex}}}
 \bigl(N_{\mathrm{Seiten}}(x),D(x),U(x)\bigr).
```

$D$ bezeichnet übermäßige Dehnung, $U$ die Unausgewogenheit der Seitenfüllung. Der lexikografische Vergleich entscheidet zuerst über die Seitenzahl und bei Gleichstand über Dehnung, dann Ausgewogenheit.

$\mathcal F$ müsste die tatsächlich lesbaren und materialisierbaren Kandidaten enthalten. Lesbarkeit ist darin Voraussetzung. Horizontale Taktprofile, vertikale Systemhüllen und Seitenbelegung sind getrennte Größen; die Systemreihenfolge bleibt erhalten. Knuth und Plass bewerten Umbruchpläne für ganze Textabsätze. Renz untersucht im Notensatz, wie die Wahl der Systemumbrüche zugleich die Seitenfüllung beeinflusst.<a name="beleg-2-2-umbruch"></a><sup><a href="#quelle-knuth">[23]</a> <a href="#quelle-renz">[25]</a></sup>

**Status:** Diese Gesamtformel ist nicht als vollständiger Seitenoptimierer implementiert. Die horizontale Systemplanung und der begrenzte gemeinsame Seiten- und Systemplan sind bereits implementiert. Für eine allgemeine Seitenverdichtung fehlen belegte sichtbare Mindestabstände und eine durchgängige Vergleichsgrundlage. Aus einer großen beobachteten Lücke folgt weder ein Mindestabstand noch automatisch die Erlaubnis zum Zusammenrücken.

[Fehlerklasse zur unnötig beibehaltenen Seite](selector_derivation_contract_redundant_page_retention.md) · [Implementierter Seiten- und Systemplan](../src/pipeline_shards/horizontal_layout_global_page_system.rs)

<a name="kapitel-2-3"></a>
<a name="symbolische-spielformel-anforderungen-und-fähigkeiten"></a>
<a name="symbolische-spielformel-für-hand-und-finger"></a>

### 2.3 Symbolische Spielformel für Hand und Finger

```math
\exists a:\ S(a)\land K(a)\land U(a)\land W(a).
```

$a$ belegt physische Verpflichtungen mit Hand-/Finger-Plätzen. $S$ steht für Strukturbedingungen, $K$ für statische Konfigurationen, $U$ für Übergänge und $W$ für Wiederanschläge gleicher Tasten. Anforderungen der Musik und Fähigkeiten eines Pianistenmodells bleiben getrennt. Wie Handspanne und Klaviaturgröße die Muskelaktivität beim Spielen beeinflussen, untersuchen die Forschungsgruppen um Ju-Yang Chi und Craig Turner.<a name="beleg-2-3-handspanne"></a><sup><a href="#quelle-chi">[34]</a> <a href="#quelle-turner">[33]</a></sup>

**Status:** Audit. Die Formel und ein struktureller Beleg können vollständig sein, während die Fähigkeiten eines konkreten Menschen weiterhin nicht modelliert sind. Ein `MODEL_RELATIVE_ONLY`-Urteil beweist weder ein bestimmtes Spieltempo noch menschliche Spielbarkeit. Eine fehlende Fähigkeitseinstufung rechtfertigt keine Notenlöschung.

Quelle: [pianistic_symbolic_capability_reachability.rs](../src/audit_only/pianistic_symbolic_capability_reachability.rs) und die dort konsumierten Fähigkeitsmodelle.

<a name="kapitel-3"></a>
<a name="vom-anschlag-zur-geschriebenen-dauer"></a>

## 3 Vom Anschlag zur geschriebenen Dauer

<a name="kapitel-3-1"></a>
<a name="das-zeitraster-einer-freien-einspielung"></a>

### 3.1 Das Zeitraster einer freien Einspielung

Die WAV-Aufnahme stützt die zeitliche Zuordnung zur MIDI-Einspielung. Wenn der direkte MIDI-Import kein geeignetes Raster liefert, sucht EtuSchlopf einen von den Anschlüssen, Wiederholungen und dem örtlichen Puls getragenen Taktverlauf. Mehrdeutige Taktarten oder eine unzureichende Übereinstimmung mit dem Audio können die Übernahme verhindern. Im Notenplan entsprechen 96 Einheiten einer Viertelnote. Für wiederkehrende Passagen vergleicht das Programm die Mengen absoluter MIDI-Tonnummern mit Jaccards Koeffizienten. Die lokale Folgenausrichtung folgt dem Prinzip von Smith und Waterman.<a name="beleg-3-1-wiederholungen"></a><sup><a href="#quelle-jaccard">[2]</a> <a href="#quelle-smith">[3]</a></sup> [Automatische Zeitplanung](../src/free_performance_timing.rs) · [Einbindung in den Projektlauf](../src/pipeline_shards/pipeline_orchestrator_project_setup_stages.rs)

Für einen möglichen Puls glättet die Zeitplanung gemessene Einsatzabstände mit Gauß-Kernen. Die beiden Kurven über jeweils zwei und drei Intervalle entsprechen bei fester Bandbreite bis auf einen Faktor einer Kerndichteschätzung.<a name="beleg-3-1-pulssuche"></a><sup><a href="#quelle-parzen">[14]</a></sup> Bei der Auftaktwahl vergleicht das Programm geglättete Anteile von Basswechseln nach der Laplace-Regel.<a name="beleg-3-1-auftakt"></a><sup><a href="#quelle-laplace">[15]</a></sup>

Angenommen, zwei benachbarte Gruppen setzen in der Aufnahme bei den Zeiten $t_a,t_b$ ein. Ihre bereits gewählten Positionen im Notenraster seien $q_a,q_b$. Lässt der Pianist eine Taste zum Zeitpunkt $r$ dazwischen los, ergibt die örtliche lineare Zeitabbildung

```math
q(r)=q_a+(q_b-q_a)\frac{r-t_a}{t_b-t_a},\qquad t_a\le r\le t_b.
```

Der Abstand zum Notenanfang liefert einen Hinweis auf die geschriebene Länge. Ein staccato gespieltes Viertel kann dieselbe Tastendauer haben wie ein gebundenes Achtel. Deshalb fließen auch die folgenden Einsätze und andere Töne desselben Akkords ein. Nach dem letzten Anschlag setzt das Modell das örtliche Tempo für die Auswertung der letzten Loslasszeiten fort. Auch die Transkriptionsarbeit *Rubato* unterscheidet die Zeit der Aufführung von der Position in der Partitur.<a name="beleg-3-1-auffuehrungszeit"></a><sup><a href="#quelle-rubato">[35]</a></sup> [Dauerherleitung](../src/free_performance_timing.rs)

<a name="kapitel-3-2"></a>
<a name="folgeeinsatz-oder-akkordende"></a>

### 3.2 Folgeeinsatz oder Akkordende

Für jeden regulären Anschlag $i$ vergleicht das Modell mögliche Folgeeinsätze $j$. Die Kosten $c_{ij}$ bewerten unter anderem das Verhältnis von beobachteter Dauer zum Abstand der Einsätze, den Tonhöhenabstand und die Notierbarkeit. Zusätzliche freie Spalten erlauben ein eigenes Ende. Jeder reale Folgeeinsatz steht höchstens einmal zur Verfügung. Für die gewählten Zuordnungen $x_{ij}$ gilt

```math
\min_x \sum_{i,j}c_{ij}x_{ij},\qquad
x_{ij}\in\{0,1\},\quad \sum_jx_{ij}=1,\quad \sum_ix_{ij}\le1.
```

Für diese Zuordnung verwendet EtuSchlopf die ungarische Methode mit zusätzlichen Spalten für ein eigenes Tonende. Kuhn und Munkres beschreiben die Verfahrensfamilie. Welche musikalischen Eigenschaften einen Anschluss günstig machen, legt EtuSchlopf in den Kosten fest.<a name="beleg-3-2-zuordnung"></a><sup><a href="#quelle-zuordnung">[4]</a></sup>

Ein Akkordton kann auch dann enden, wenn eine andere Stimme weiterläuft. Für einen Ton ohne eigenen Folgeeinsatz prüft EtuSchlopf die Loslasszeiten der Akkordpartner auf derselben Spur und demselben Kanal. Mit der Streuung $w$ der **Anschlagszeiten** und den **Loslasszeiten** $r_i,r_j$ gilt für einen möglichen Zeugen

```math
w=\max_k t_k-\min_k t_k,\qquad
r_i,r_j\gt\max_k t_k,\qquad |r_i-r_j|\le w.
```

Die passenden Partner mit eigenem Folgeeinsatz müssen sich über das geschriebene Ende einig sein. Ein erneuter Anschlag derselben Taste vor diesem Ende schließt die Übernahme aus. Ein weiterer Fall betrifft die Pause im gleichmäßigen Akkordpuls. Dort müssen an zwei vorherigen Pulsen alle Akkordtöne unabhängig dieselbe Unterteilung bestätigen, die nächste Attacke muss eine Lücke lassen und die gemessene Artikulation muss im zuvor beobachteten Bereich liegen. Offen gebliebene Dauern werden nach der Zuordnung zum Notensystem im Kontext der benachbarten Einsätze entschieden. Auch Nakamura, Yoshii und Dixon werten den mehrstimmigen Kontext für die Wahl des Notenwerts aus. Ihre Arbeit verwendet dafür ein Markov-Zufallsfeld.<a name="beleg-3-2-notenwert"></a><sup><a href="#quelle-nakamura-yoshii-dixon">[36]</a></sup> [Akkordenden und Puls](../src/free_performance_timing.rs) · [Offene Dauern im Notensystem](../src/midi_seed.rs)

<a name="kapitel-3-3"></a>
<a name="ein-quellton-über-mehrere-geschriebene-fragmente"></a>

### 3.3 Ein Quellton über mehrere geschriebene Fragmente

Ein regulärer Ton soll das halboffene Intervall von $a$ bis $b$ belegen. Die aus demselben Anschlag erzeugten Fragmente müssen eine lückenlose Partition bilden. Für ihre Anfänge $s_k$ und Enden $e_k$ gilt

```math
s_1=a,\qquad e_m=b,\qquad s_k\lt e_k,\qquad e_k=s_{k+1}\quad(1\le k\lt m).
```

Dadurch hat ein bei 72 beginnender Ton mit Dauer 96 auch dann sein Ende bei 168, wenn auf demselben System bei 120 eine andere Note beginnt. EtuSchlopf kann dafür einen weiteren MusicXML-Stimmkanal verwenden. Bei einer Teilung behält jedes Fragment die Quellidentität und Tonhöhe.<a name="beleg-3-3-fragment"></a><sup><a href="#quelle-zeit">[44]</a></sup> Nur das erste trägt den Anschlag, die folgenden schließen mit Haltebögen an. Ein Vorschlag hat eigene Regeln für Nullzeit und Hauptnote. Der Schreibpfad prüft die Zeiten gegen die tatsächlich ausgegebenen Noten und Cursorbewegungen. [Fragmentplanung und Ausgabekontrolle](../src/midi_seed/bound_duration.rs)

Die annotierte Zuordnung aus dem Vienna-4×22-Korpus dient beim Vergleich zur Prüfung der Ausgabe.<a name="beleg-3-3-korpus"></a><sup><a href="#quelle-goebl">[43]</a></sup> Der Erhalt der gewählten Intervalle und die Übereinstimmung dieser Wahl mit der musikalischen Referenz sind getrennt messbar.

<a name="kapitel-4"></a>
<a name="verfahrensfamilien-der-bestandsaufnahme"></a>
<a name="verfahrensfamilien-im-aktuellen-quellstand"></a>

## 4 Verfahrensfamilien im aktuellen Quellstand

Die Quellstellen nennen die jeweiligen Implementierungsstellen, nicht automatisch einen universellen mathematischen Satz über das gesamte Programm. Zu einzelnen Verfahren führen die Detailkapitel, etwa [zur Stimmfärbung](#kapitel-6-2), [zur Pareto-Front](#kapitel-6-6) und [zur Frequenzauswertung](#kapitel-6-12). Wissenschaftliche Grundlagen, Originalarbeiten und ihr jeweiliger Bezug stehen gesondert im Abschnitt [Kapitel 12 · Quellen](#kapitel-12).

| # | Aufgabe und Mathematik | Rolle und wichtigste Grenze | Quellstelle |
|---|---|---|---|
| 1 | Exakte Zeitrechnung: rationale Zahlen, ggT/kgV, gemeinsame Zeitachsen<a name="beleg-4-verfahren-1"></a><sup><a href="#quelle-euklid">[1]</a></sup> | Entscheidet / prüft. Geschriebene und klingende Zeit bleiben verschieden. | [Metrische Pausen](../src/model/metric_rest_notation.rs), [temporale Beobachtung](../src/clef_readability_observation/temporal.rs) |
| 2 | Ereignisintervalle, Akkordanker und Fortsetzungen | Grundlage für Entscheidungen und Prüfungen. Ein Pausenanker kann klingendes Material tragen. | [serialisierte Zeitprüfung](../src/audit_only/serialized_temporal_replay.rs), [ursprüngliche Erfassung](../src/pipeline_shards/actual_beam_capture_core.rs) |
| 3 | Materialbilanz und getrennte Herkunfts-/Sichtbarkeitsmaße<a name="beleg-4-verfahren-3"></a><sup><a href="#quelle-bilanzen">[46]</a></sup> | Misst. Anzahlen sind kein individueller Identitätsnachweis. | [material_conservation.rs](../src/audit_only/material_conservation.rs) |
| 4 | R1–R5: Zustandsautomaten für Schlüssel, Oktavierungen und Haltebögen | Misst. Diese Regeln sind kein vollständiger Notationsbeweis. | [notation_invariants.rs](../src/audit_only/notation_invariants.rs) |
| 5 | Koinzidenzen: Gleichheit von Einsatz und Taste unter Quellbindung | Misst. Kein universeller Platzierungs- oder Löschauftrag. | [material_coincidence.rs](../src/audit_only/material_coincidence.rs) |
| 6 | Stimmenbedarf im Intervallmodell: Cliquenzahl und Färbung<a name="beleg-4-verfahren-6"></a><sup><a href="#quelle-golumbic">[7]</a></sup> | Misst eine modellgebundene Mindestzahl, keine Erlaubnis zum Zusammenlegen musikalischer Stimmen. | [notation_invariants.rs](../src/audit_only/notation_invariants.rs) |
| 7 | Gleichheitsklassen, Trennbedingungen und Widerspruchspfade der geschriebenen Zuordnung | Readback / Graphprüfung, kein neuer Allokator. Weitere Beweispflichten bleiben ausdrücklich offen. | [written_assignment_graph.rs](../src/structural_write_authority/written_assignment_graph.rs), [Probe](../src/structural_write_authority/written_assignment_graph_probe.rs) |
| 8 | Lexikografische Kandidatenwahl | Entscheidet. Frühere Kriterien können durch spätere Vorteile nicht aufgewogen werden. | `CandidateDecisionKey` in der Pipeline |
| 9 | Verteilung verborgener Akkordtöne: quadratische Kosten plus diskrete Strafen | Entscheidet unter weiteren Zulassungsprüfungen. Gewichte sind Modellierung, keine kalibrierte Wahrnehmung. | [hidden_chord_reduction_texture_rebuild_tail.rs](../src/pipeline_shards/hidden_chord_reduction_texture_rebuild_tail.rs) |
| 10 | Gesten- und Kettenvarianten der Verteilung | Kandidatenentscheidungen. Die konkrete Variante und ihre musikalischen Wirkungen müssen einzeln nachgewiesen werden. | dieselbe Verteilungsfamilie und ihre bestehenden Owner |
| 11 | Hilfsliniengeometrie und gradueller Druck | Beobachtung und Kosten des separaten Schlüsselplaners. Kein universelles Vier-Hilfslinien-Verbot. | [clef_readability_observation.rs](../src/clef_readability_observation.rs), [Schlüsselplanung](../src/structural_write_authority/rustig_clef_planning.rs) |
| 12 | Separater Schlüsselverlauf: dynamische Programmierung<a name="beleg-4-verfahren-12"></a><sup><a href="#quelle-bellman">[12]</a></sup> | Entscheidet im jeweils zuständigen Pfad. Bei gebundenem gemeinsamen Kurs darf keine unabhängige spätere Entscheidung dessen Darstellung ersetzen. | [rustig_clef_planning.rs](../src/structural_write_authority/rustig_clef_planning.rs) |
| 13 | C4: gemeinsamer Schlüssel-/Oktavkurs mit exakter Abschnittsdauer | Entscheidet in vollständig belegten, unterstützten Domänen. Systemübergreifende Kopplung bleibt derzeit eine mögliche UNKNOWN-Grenze. | [Suchkern](../src/pipeline_shards/joint_reading_c3_course.rs), [Adapter](../src/pipeline_shards/joint_reading_c3_adapter.rs) |
| 14 | Vergleich vollständiger gemeinsamer Registerspannen | Entscheidet nur bei vollständigem Ausgangsplan, Anschluss und zulässiger Verbesserung. | [gemeinsame_spanne.rs](../src/registernotation/gemeinsame_spanne.rs) |
| 15 | Tonart- und Schreibweisenbewertung: gewichtete Tonklassen, Tonika-/Dominantbeiträge, Vorzeichenkosten | Audit und daran gebundene produktive Schreibweisenentscheidung. Klingende Tonhöhe und Darstellung sind zu unterscheiden. | [key_signature_enharmonic_report_core.rs](../src/pipeline_shards/key_signature_enharmonic_report_core.rs) |
| 16 | Harmonik-Kandidaten: Nichtdominanz / Pareto-Vergleich<a name="beleg-4-verfahren-16"></a><sup><a href="#quelle-pareto">[10]</a></sup> | Kandidatenauswahl innerhalb vorhandener Owner. Ein nichtdominierter Kandidat muss nicht übernommen werden. | [sheaf_noether_projection_core.rs](../src/pipeline_shards/sheaf_noether_projection_core.rs) |
| 17 | Phrasen- und Grenzbeobachtung aus Rhythmus, Metrik, Kontur und tonalem Kontext | Audit. Grenzkorrelation allein erteilt keine Bogen- oder Abschnittsautorität. | [phrase_boundary_evidence.rs](../src/audit_only/phrase_boundary_evidence.rs) |
| 18 | Exakte Wiederholungsdeskriptoren; Transpositions- und Augmentationsinvarianz | Audit. Nur die tatsächlich definierten Merkmale sind invariant; kein allgemeiner Ähnlichkeits- oder Phrasenbeweis. | dieselbe Phrasenbeobachtung |
| 19 | Physische Handkonfigurationen und endliche Fähigkeitszuordnung | Audit. Ein unbelegter Fähigkeitswert bleibt unmodelliert. | [pianistic_configuration_constraint_kernel.rs](../src/audit_only/pianistic_configuration_constraint_kernel.rs) |
| 20 | Fingerplatz-Belegung als Ungleichungsgraph; Degenerationsordnung und Färbungszeuge<a name="beleg-4-verfahren-20"></a><sup><a href="#quelle-matula">[9]</a></sup> | Audit zur Spielformel. Ein struktureller Färbungszeuge ist keine vollständige pianistische Lösung. | [symbolische Spielformel](../src/audit_only/pianistic_symbolic_capability_reachability.rs) |
| 21 | Statische und zeitliche Anforderungsprofile, Überlappungen, Wiederanschläge, Quantile | Audit. Verteilungen beschreiben Anforderungen und kalibrieren keine menschliche Grenze. | [temporale Anforderungen](../src/audit_only/pianistic_temporal_transition_demand_frontier.rs), [statische Frontier](../src/audit_only/pianistic_static_capability_frontier.rs) |
| 22 | WAV-Angriff/Puls/Metrik: Energie- und Transientenverläufe, lokale Spitzen, Tempo-/Phasenhypothesen<a name="beleg-4-verfahren-22"></a><sup><a href="#quelle-dixon">[28]</a></sup> | Audit. Audiohypothesen werden vor ihrer Bewertung gegen MIDI eingefroren. | [wav_attack_pulse_metric.rs](../src/audit_only/wav_attack_pulse_metric.rs) |
| 23 | Zeitabbildung zwischen nominalem Raster und Audio; Residuen und Quantile | Audit. Eine gute Anpassung ist kein Herkunftsnachweis für zusätzliche Quelltöne. | dieselbe WAV-Pulsanalyse |
| 24 | Frequenzenergie und harmonische Beziehungen: Goertzel-Rekurrenz<a name="beleg-4-verfahren-24"></a><sup><a href="#quelle-goertzel">[30]</a></sup> | Audit. Spektrale Energie ist kein sicherer unabhängiger Anschlagsnachweis. | [wav_pitch_harmonic_evidence.rs](../src/audit_only/wav_pitch_harmonic_evidence.rs) |
| 25 | Basic Pitch: fremdes neuronales Transkriptionsmodell und unabhängiger Sensorvergleich<a name="beleg-4-verfahren-25"></a><sup><a href="#quelle-basic">[32]</a></sup> | Separater Sensor / Audit. Keine automatische Übernahme in die kanonische musikalische Quelle. | [Sensorqualifikation](../src/audit_only/wav_pitch_sensor_qualification.rs), [Sensorbenchmark](../src/audit_only/wav_pitch_external_sensor_benchmark.rs) |
| 26 | Horizontaler Systemumbruch: endliche Partitionen, Kapazität und lexikografische Kosten<a name="beleg-4-verfahren-26"></a><sup><a href="#quelle-hegazy">[24]</a></sup> | Entscheidet im vorhandenen Layoutpfad. Seitenumfang und echte Lesbarkeit sind damit nicht global optimiert. | [horizontal_layout_global_reflow.rs](../src/pipeline_shards/horizontal_layout_global_reflow.rs) |
| 27 | Renderer-Geometrie: Hüllen, Rechteckschnitt, Abstände und Umbruch-Readback | Misst / prüft nur unterstützte, eindeutig zugeordnete Geometrie. Kein vollständiger automatischer Schönheitsbeweis. | [Seitenabstände](../src/audit_only/renderer_page_clearance.rs), [Systemumbrüche](../src/audit_only/renderer_system_break_readback.rs) |
| 28 | Lexikografische Seitenpackung nach Schlöpfchen | Entwurf; von der vorhandenen Systemplanung getrennt. | [Seitenfehlerklasse](selector_derivation_contract_redundant_page_retention.md) |
| 29 | Präfixsummen und binäre Intervallsuche für dieselben additiven Kosten | Technische Erhaltung. Beschleunigung, keine neue musikalische Zielgröße. | [Schlüsselplanung](../src/structural_write_authority/rustig_clef_planning.rs), [Äquivalenzvertrag](selector_derivation_contract_exact_clef_passage_prefix_costs_r1652.md) |
| 30 | Indizierte Existenzprüfung von Bogenendpunkten | Technische Erhaltung im bestehenden Owner. Kein allgemeines optimales bipartites Bogenmatching. | [Verfahrensnachweis](indexed_slur_endpoint_pairing_r1666.md) |
| 31 | Erreichbarkeit im Aufrufgraph, Capability- und Herkunftsprüfungen | Technische Zulassungsprüfungen. Pfad- und Quelltextprüfungen bestehen daneben weiter. | [semantic_capability_diff.rs](../src/build_gate/semantic_capability_diff.rs), [take_change_control.rs](../src/take_change_control.rs) |
| 32 | Beobachtungsäquivalenz und reduzierte Aufbewahrung | Technische Erhaltung unter nachgewiesenen Leserabhängigkeiten. | [solver_candidate_retention.rs](../src/pipeline_shards/solver_candidate_retention.rs) |
| 33 | Fingerprints und Hashfunktionen: FNV-1a, SHA-256<a name="beleg-4-verfahren-33"></a><sup><a href="#quelle-fnv">[21]</a> <a href="#quelle-nist">[22]</a></sup> | Integritäts- und Bindungsprüfungen mit jeweiligen Kollisionsannahmen. Kein eigenständiger musikalischer Identitätsbeweis. | [SHA-256](../src/build_gate/sha256.rs), [Herkunft](../src/master_state_identity.rs) |
| 35 | Notationszeit, Taktgrenzen und exakt gebundene Haltefortsetzungen<a name="beleg-4-verfahren-35"></a><sup><a href="#quelle-zeit">[44]</a></sup> | Entscheidet im MIDI-Seed unter Quell- und Metrikbindung. | [Notationszeit](../src/notation_time.rs), [MIDI-Seed](../src/midi_seed.rs) |
| 36 | Wiederholungsakkorde und belegte Handzuordnung über klingende Zeitintervalle | Entscheidet bei eindeutiger Wiederholung und zulässiger Tastenbelegung. | [Handzuordnung](selector_derivation_contract_repeated_accompaniment_hand_implementation.md), [Warnungsprüfung](../src/rules.rs) |
| 37 | Begrenzter gemeinsamer Seiten- und Systemplan mit Renderer-Rückmeldung<a name="beleg-4-verfahren-37"></a><sup><a href="#quelle-renz">[25]</a></sup> | Entscheidet und prüft innerhalb zugelassener Seitenzahlen und Kapazitäten. | [Planung](../src/pipeline_shards/horizontal_layout_global_page_system.rs), [Partitionssuche](../src/pipeline_shards/renderer_exact_partition_search.rs) |
| 38 | Speicherabschätzung für parallele Korpusprojekte<a name="beleg-4-verfahren-38"></a><sup><a href="#quelle-speicher">[48]</a></sup> | Plant Prozesse und Solver-Worker mit festgesetzten Schätzwerten. | [Korpus-Speicherplanung](../src/pipeline_shards/corpus_memory_capacity.rs) |
| 39 | Lokale Stimmfortsetzung unter Zeit-, Rollen- und Randbedingungen<a name="beleg-4-verfahren-39"></a><sup><a href="#quelle-backtrack">[8]</a></sup> | Prüft vollständige Belegungen einer kleinen Passage und liefert Zeugen für einzelne Beziehungen. | [Exakte Gegenprobe](../src/structural_write_authority/exact_voice_passages.rs) · [Modell und Suchgrenzen](selector_derivation_contract_exact_voice_passages.md) |

<a name="kapitel-5"></a>
<a name="lokale-entscheidungen-zu-pausen-und-stimmen"></a>

## 5 Lokale Entscheidungen zu Pausen und Stimmen

Der jeweilige Notationsschritt wertet die am Einsatz belegten Stimmen und ihre Nachbarn aus.<a name="beleg-5-notation"></a><sup><a href="#quelle-lokal">[45]</a></sup>

| Aufgabe | Ausgewertete Größen | Entscheidung |
| --- | --- | --- |
| Pausen im geschriebenen Takt | Rationale Start- und Endzeiten, metrische Gruppen sowie belegte Zeitintervalle der sichtbaren Stimme | Der Pausenplan bestimmt die freie Zeit innerhalb des lokalen Takts und bewertet ihre Zerlegung anhand der metrischen Gruppierung. [Metrische Pausen](../src/model/metric_rest_notation.rs) · [Lokale Intervalle](selector_derivation_contract_final_local_rest_intervals.md) |
| Notenhals am Stimmeinsatz | Stimme, gleichzeitige Nachbarstimmen und bereits gebundene Balkengruppen | Die Richtung folgt den sichtbaren Beziehungen am Einsatz und innerhalb der gebundenen Gruppe. [Notenhälse](../src/notation_final_stems.rs) · [Vertrag](selector_derivation_contract_final_stem_direction.md) |
| Registerkontinuität | Benachbarte Abschnitte, gebundene Stimmzuordnung und Lesbarkeit des Schlüsselverlaufs | Die Entscheidung berücksichtigt den Anschluss zwischen Abschnitten. Der Nachweis bindet die Darstellung an die musikalischen Ereignisse. [Register und Stimmeinsatz](selector_derivation_contract_voice_entry_and_register_continuity.md) |

<a name="kapitel-6"></a>
<a name="zentrale-formeln-und-ihre-beweisgrenzen"></a>

## 6 Zentrale Formeln und ihre Beweisgrenzen

<a name="kapitel-6-1"></a>
<a name="bilanz-identität-und-sichtbarkeit"></a>

### 6.1 Bilanz, Identität und Sichtbarkeit

```math
N_{\mathrm{Quelle}}=N_{\mathrm{aktiv}}+N_{\mathrm{inaktiv}},\qquad
N_{\mathrm{aktiv}}=N_{\mathrm{sichtbar}}+N_{\mathrm{verborgen}}.
```

Diese Gleichheiten gelten nur für die im jeweiligen Bericht sauber abgegrenzte Grundgesamtheit. Quellanschläge, geschriebene Fragmente und sichtbare Köpfe sind nicht austauschbare Nenner.<a name="beleg-6-1-bilanz"></a><sup><a href="#quelle-bilanzen">[46]</a></sup>

Eine bestandene Anzahlbilanz ist notwendig für die betreffende Buchführung, aber kein hinreichender Nachweis individuellen Erhalts: Eine Löschung und eine andere Einfügung können sich zahlenmäßig ausgleichen. Der aktuelle Bericht bezeichnet diese Projektion ausdrücklich als `cardinality_only_not_a_late_source_binding`. Herkunft und musikalische Eigenschaften brauchen eigene gebundene Nachweise.

`CanonicalSourceRetention` und `ArrangementVisibleRetention` beantworten verschiedene Fragen. Eine verborgene Note ist nicht allein deshalb entfernt. Aus ihrem Verbergen folgt weder ein nachgewiesener Spielbarkeitskonflikt noch ein Beweis, dass die verbleibende Darstellung menschlich spielbar ist.

<a name="kapitel-6-2"></a>
<a name="intervalle-und-stimmenbedarf"></a>

### 6.2 Intervalle und Stimmenbedarf

Für ein geschriebenes Ereignis gilt das halboffene Intervall $[t,t+d)$. Bei einem Sweep werden Enden vor gleichzeitigen Anfängen verarbeitet. Im vereinfachten Modell werden gleiche Einsätze und Dauern je Part/System/Takt zu einem Ereignis zusammengefasst.

Für den dadurch erzeugten Intervallgraphen gilt:

```math
\chi(G)=\omega(G).
```

Die maximale Überlappung ist damit die minimale Farbzahl **dieses Graphen**. Das lässt sich auch mit Dilworths Satz lesen: Ein überlappungsfreier Kanal ist eine Kette, gleichzeitig belegte Intervalle bilden eine Antikette.<a name="beleg-6-2-faerbung"></a><sup><a href="#quelle-dilworth">[6]</a> <a href="#quelle-golumbic">[7]</a></sup> Musikalische Linien, ursprüngliche Balkengruppen, Pausenpflichten, Akkordbindungen und technische Importkanäle besitzen zusätzliche Bedingungen. Aus der Graphzahl allein folgt keine erlaubte Zusammenlegung solcher Rollen. Der aktuelle `VoiceModel`-Bericht nennt diese Grenze ausdrücklich.

<a name="kapitel-6-3"></a>
<a name="welche-balkenpaare-bleiben-verbunden"></a>
<a name="welche-balkenpaare-bleiben-erhalten"></a>

### 6.3 Welche Balkenpaare bleiben erhalten?

Für die Mengen verbundener ursprünglicher Primärbalkenpaare im Referenzmaster $R$ und im Vergleichsmaster $V$ zählt jedes identifizierte Notenpaar einzeln. Quellnote, Anschlag, Fragment und der Ausgabebeleg identifizieren seine beiden Endpunkte auch bei einer Änderung der Stimmnummer.

Eine unveränderte Anzahl von Verbindungen kann verdecken, dass einzelne Balkenpaare verloren gingen. Der Planer prüft deshalb die gebundene Paarmenge.<a name="beleg-6-3-balken"></a><sup><a href="#quelle-beams">[47]</a></sup> Ein Stimmenwechsel erfüllt den Erhalt genau dann, wenn alle ursprünglichen Verbindungen in der Ausgabe vorkommen, also $R\subseteq V$.

**Balkenverbindung bei vollständigen Pausen.** Blockiert eine sichtbare Pause die technische Stimmzuordnung eines bereits verbundenen Balkens, prüft EtuSchlopf das gesamte ursprüngliche Schweigeintervall $I=[a,b)$. Die Quelllücken $G_i$ und die sichtbaren notierten Pausen $P_j$ müssen jeweils eine lückenlose und überlappungsfreie Zerlegung desselben Intervalls bilden.

```math
I = \bigsqcup_i G_i = \bigsqcup_j P_j.
```

Die Zerlegungen dürfen unterschiedliche innere Grenzen haben. Am rechten Rand $b$ bindet ein belegter Einsatz die Fortsetzung derselben musikalischen Linie. Ihr technischer Kanal muss auf $I$ frei sein und beide Balkengrenzen wahren. EtuSchlopf prüft die Balkenzuordnung und alle zugehörigen Pausen gemeinsam und übernimmt ihre technischen Stimmenzuordnungen als einen Satz von Änderungen. [Herleitung und Voraussetzungen](selector_derivation_contract_original_beam_companion_rest_partition.md)

**Abgenommen in r1998.** Der Windows-Vollkorpus bestätigt die Korrektur der Balkenverbindung in der Bach-Fuge BWV 847. Die vier Änderungen im Render-MusicXML betreffen die technischen Stimmenzuordnungen von zwei Noten und zwei Pausen. [Windows-Prüfbeleg](evidence/r1998/README.md)

<a name="kapitel-6-4"></a>
<a name="welche-stimmfortsetzung-ist-erzwungen"></a>

### 6.4 Welche Stimmfortsetzung ist erzwungen?

Drei Linien enden auf den Noten A, B und C. An derselben Grenze setzen X, Y und Z ein. Für A allein kommen alle drei Anschlüsse infrage. Für B und C erlauben die gebundenen Rollen jeweils nur Y und Z. Jede ankommende Linie erhält genau eine Fortsetzung und jede neue Note gehört genau einer Linie. Eine solche vollständige Eins-zu-eins-Zuordnung heißt *perfektes Matching*.

| Zulässige Paarung | Anschluss von A | Anschluss von B | Anschluss von C |
|---|---|---|---|
| 1 | X | Y | Z |
| 2 | X | Z | Y |

B und C benötigen zusammen Y und Z. Deshalb ist A → X erzwungen. Hier schöpfen B und C ihre gemeinsame Nachbarmenge aus, wie im endlichen Heiratssatz von Philip Hall.<a name="beleg-6-4-hall"></a><sup><a href="#quelle-hall">[5]</a></sup> B → Y ist möglich, und die zweite Paarung belegt seine Mehrdeutigkeit. Überspannt eine Note mehrere Einsätze, muss derselbe Plan die Zeit- und Randbedingungen der ganzen Passage erfüllen.

Sei $P$ die Menge aller im lokalen Modell zulässigen Stimmpläne und $E(p)$ die Menge der darin realisierten Anschlüsse. Für eine Beziehung $e$ gilt

```math
e\text{ ist im Modell erzwungen}\quad\Longleftrightarrow\quad P\ne\varnothing\ \land\ \forall p\in P:\ e\in E(p).
```

Die lokale Suche prüft, ob zwei gebundene Noten denselben Ausgabekanal belegen. Die Suche hält die Umgebung fest und berücksichtigt notierte und klingende Zeit, Rollen, Pausen, vorhandene Verbindungen und gebundene Randnachbarn. Zwei gegensätzliche Zeugen belegen Mehrdeutigkeit. Für einen Zwangsnachweis braucht es einen positiven Zeugen und eine abgeschlossene Suche nach Gegenplänen. Eine leere Lösungsmenge oder ein ausgeschöpftes Suchbudget lässt die Frage nach dem Zwang offen. Der Suchkern erweitert Teilpläne schrittweise und verwirft sie bei einem Widerspruch. Diese Suche mit Rückkehr zum letzten Entscheidungspunkt heißt Backtracking.<a name="beleg-6-4-suche"></a><sup><a href="#quelle-backtrack">[8]</a></sup> [Suchkern und synthetische Gegenprobe](../src/structural_write_authority/exact_voice_passages.rs) · [Modellgrenzen](selector_derivation_contract_exact_voice_passages.md)

<a name="kapitel-6-5"></a>
<a name="notationszeit-haltebögen-und-handzuordnung"></a>

### 6.5 Notationszeit, Haltebögen und Handzuordnung

Bei einem gebundenen Anschlag mit Einsatz $t$, Ende $e$ in der Notationszeit und einer Taktgrenze $b$ mit $`t \lt b \lt e`$ kann der MIDI-Seed das Zeitintervall in zwei Fragmente teilen:

```math
[t,e)=[t,b)\cup[b,e),\qquad e-t=(b-t)+(e-b).
```

Der Haltebogen verbindet beide Fragmente mit demselben Anschlag. Der Pfad prüft unter anderem exakte Notenwerte, die belegte Taktgrenze und konkurrierende Anschläge derselben Tonhöhe.

Für einen durch die bisherige Handgrenze geteilten Akkord vergleicht die Prüfung die Menge der MIDI-Tasten mit zeitlich direkt angrenzenden Akkordgruppen. Ein Zeuge muss vor der Korrektur vollständig derselben Hand zugeordnet sein. Widersprechen sich die Zeugen, bleibt die Einteilung bestehen. Für jede Hand $h$ und jede geprüfte Anschlagsgrenze $u$ verlangt der Code für die klingende Tastenmenge $K_h(u)$:

```math
|K_h(u)|\le 5,\qquad \max K_h(u)-\min K_h(u)\le 12\quad\text{für }K_h(u)\ne\varnothing.
```

Sind beide Hände belegt, muss die höchste linke Taste unter der tiefsten rechten liegen. Konkurrierende Anschläge derselben Akkordtaste schließen den Wechsel aus. Die Grenzwerte gehören zum Modell der Handzuordnung. Gemessene Unterschiede zwischen Handspannen und Klaviaturen sind in den ergonomischen Studien dokumentiert.<a name="beleg-6-5-ergonomie"></a><sup><a href="#quelle-chi">[34]</a> <a href="#quelle-turner">[33]</a></sup> Sie liefern weder einen Fingersatz noch einen Nachweis individueller Spielbarkeit. [Regel für Wiederholungsakkorde](selector_derivation_contract_repeated_accompaniment_hand_implementation.md) · [Quellcode](../src/midi_seed.rs).

Die Sprungwarnung prüft zusätzlich, ob sich beide Tonhöhengrenzen aufeinanderfolgender Anschläge in dieselbe Richtung bewegen und jeweils mindestens 19 Halbtöne zurücklegen. Der bestehende Test der Zeitkante bleibt Teil der Warnung. Ein Akkord kann sich aufspreizen oder zusammenziehen, ohne dass beide Grenzen einen solchen Sprung ausführen. [Prüfregel](../src/rules.rs).

<a name="kapitel-6-6"></a>
<a name="kandidatenauswahl-und-pareto-front"></a>

### 6.6 Kandidatenauswahl und Pareto-Front

Für zwei zulässige Lösungen entscheidet beim lexikografischen Vergleich der erste abweichende Eintrag. Der Auswahlschlüssel für die allgemeine Kandidatenbewertung ordnet die Größen so

```math
K=(B,C,H,W,I,\mathrm{rev}(R),M).
```

$B$ zählt Blocker, $C$ kritische Fälle, $H$ ist die größte kritische Härte, $W$ und $I$ zählen Warnungen und Hinweise. Es folgen der absteigend geordnete Notenerhalt $R$ und die Anzahl $M$ verschobener Noten. Wenn ein Kandidat einen zusätzlichen Blocker hat, können ihm weniger Warnungen bei diesem Vergleich keinen Vorrang verschaffen. [Kandidatenauswahl](../src/pipeline_shards/staff_voice_move_restify_atomic_patch_tail.rs)

Die lokale Harmonikbewertung untersucht auch die Pareto-Front. Im neundimensionalen Kostenvektor $z$ gehen die beiden Nutzenbeiträge mit negativem Vorzeichen ein. Kandidat $a$ dominiert $b$ genau dann, wenn er in jedem Kriterium mindestens so gut und in wenigstens einem besser ist

```math
a\prec b\;\Longleftrightarrow\;
\left(\forall k\colon z_k(a)\le z_k(b)\right)
\land\left(\exists k\colon z_k(a)\lt z_k(b)\right).
```

Die erste Front besteht aus allen Kandidaten ohne dominierenden Konkurrenten.<a name="beleg-6-6-dominanz"></a><sup><a href="#quelle-pareto">[10]</a></sup> Bei konkurrierenden Vorteilen können mehrere Kandidaten auf dieser Front liegen. [Harmonikvergleich](../src/pipeline_shards/sheaf_noether_projection_core.rs)

<a name="kapitel-6-7"></a>
<a name="wiederholung-bei-anderer-tonhöhe-und-anderem-tempo"></a>

### 6.7 Wiederholung bei anderer Tonhöhe und anderem Tempo

Die Phrasenanalyse beschreibt eine Passage anhand der Abstände aufeinanderfolgender Einsätze, der klingenden Dauern und der Unterschiede zwischen den Tonhöhenschwerpunkten. Für den positiven ganzzahligen Zeitvektor $v$ des Abschnitts verwendet sie

```math
g=\gcd(v_1,\ldots,v_n),\qquad \rho(v)=v/g,\qquad
\Delta p_i=p_{i+1}-p_i.
```

Verdoppeln sich alle Zeitwerte, bleibt $\rho$ gleich. Zum Kürzen der Zeitfolge dient der größte gemeinsame Teiler, wie ihn Euklid in Buch VII der *Elemente* bestimmt.<a name="beleg-6-7-ggt"></a><sup><a href="#quelle-euklid">[1]</a></sup> Eine gemeinsame Transposition verändert die Schwerpunktdifferenzen $\Delta p_i$ ebenfalls nicht. Der Deskriptor erfasst zusätzlich die Akkordbesetzung und setzt ein gültiges Zeitmodell voraus. Ein Treffer beschreibt eine strukturelle Wiederholung. Ob der Pianist beide Stellen gleich artikuliert oder eine Stimme über die Stelle hinaus hält, braucht eigene Evidenz. [Phrasenanalyse](../src/audit_only/phrase_boundary_evidence.rs)

<a name="kapitel-6-8"></a>
<a name="quadratische-verteilungskosten"></a>

### 6.8 Quadratische Verteilungskosten

Die implementierte lokale Bewertung enthält:

```math
E(t,s)=3\Delta t^2+24\Delta p^2+42^2[\text{System verschieden}]
+12^2[\text{Stimme verschieden}]+2(\Delta t-24k)^2+12000[\text{Platz bereits verwendet}].
```

Zeit wird hier in der vorhandenen 96-Ticks-pro-Viertel-Projektion gemessen, Tonhöhe in Halbtönen. Der Code begrenzt zusätzlich den Kandidatenraum und prüft je nach Aufruf die Zielbelegung. Die Formel allein ist weder vollständige Zulässigkeit noch Erhaltungsbeweis. Der Name „Gauss“ bezeichnet hier quadratische Kosten; er behauptet keine Gauß-Verteilung der Musikdaten, kein statistisches Maximum-Likelihood-Modell und keine globale optimale Zuordnung aller Töne.

Gewichte, Kandidatengrenzen und Strafen sind Projekteinstellungen. Die Bewertung verwendet Quadratsummen im Sinne der kleinsten Quadrate.<a name="beleg-6-8-quadratsumme"></a><sup><a href="#quelle-quadrate">[11]</a></sup> Ein Kandidaten-Score ist kein gemessener menschlicher Lese- oder Spielaufwand.

<a name="kapitel-6-9"></a>
<a name="tonartprofil-und-schreibweise"></a>

### 6.9 Tonartprofil und Schreibweise

Die aktuelle Profilbewertung summiert Beiträge pro Ton. Vereinfacht mit dem vom Owner bestimmten Gewicht $w_n$, Tonklasse $c_n$ und Profil $P$ lautet der im Code sichtbare Kern:

```math
\begin{aligned}
T(P)&=\sum_n w_n\bigl(3S_n-4(1-S_n)+2T_n+D_n+L_n\bigr).
\end{aligned}
```

Die Indikatoren stehen für Skalenmitgliedschaft $S_n$, Tonika $T_n$, Dominante $D_n$ und den erhöhten Leitton in Moll $L_n$. Sie nehmen die Werte 0 oder 1 an.

Dies ist eine Projekteinstellung und kein statistisch kalibrierter Tonartbeweis. Die tatsächliche Schreibweisenentscheidung verlangt zusätzliche Bedingungen zu Geltungsbereich, Stabilität, konkurrierenden Lesarten und Vorzeichenkosten. Enharmonische Umbenennung darf die klingende Tonhöhe nicht verändern. Ein Profilwert allein erteilt weder eine neue Tonhöhe noch eine neue Abschnittsgrenze. Als Vergleich untersuchen Bouquillard und Jacquemard, wie enharmonische Schreibweisen sowie lokale und globale Tonarten gemeinsam geschätzt werden können.<a name="beleg-6-9-tonart"></a><sup><a href="#quelle-bouquillard-jacquemard">[39]</a></sup>

<a name="kapitel-6-10"></a>
<a name="separater-schlüsselplan-und-gemeinsamer-registerkurs-sind-verschiedene-zielfunktionen"></a>

### 6.10 Separater Schlüsselplan und gemeinsamer Registerkurs sind verschiedene Zielfunktionen

Der separate Lesbarkeitskurs verwendet den graduellen Druck

```math
p(l)=l^2+8\max(l-3,0)
```

und Kosten für Schlüsselwechsel, Redundanzen und kurze Ausflüge. Seine bekannten 24 Einheiten je Wechsel beziehungsweise kurzem Ausflug sind Setzungen dieses Modells. Daraus folgt keine universelle Grenze von drei Hilfslinien.

Der gemeinsame Kurs bewertet zulässige Pläne nach der lexikografischen Reihenfolge der [C4-Policy](joint_register_dp_policy_c4_v1.md).

```math
(h_8,h_7,h_6,h_5,h_4,k_o,u_k,k_s,g_b,c_h,h_3,h_2,h_1).
```

Ein Kurs mit weniger Registerzeichen kann trotz zusätzlicher Drei-Hilfslinien-Köpfe gewinnen. Die Zulässigkeitsbedingungen und die vorherigen Kostenkomponenten haben Vorrang.

| Größe | Bedeutung |
|---|---|
| $h_1,\ldots,h_7$ | Sichtbare Köpfe mit genau dieser Zahl an Hilfslinien; $h_8$ bündelt acht oder mehr. Das ungekappte Maximum wird getrennt berichtet. |
| $k_o$ | Gleichzeitiger Schlüssel- und wirksamer Oktavwechsel an einer Grenze. |
| $u_k$ | Köpfe unter den als unkonventionell bewerteten Kombinationen G mit Abwärtsoktavierung beziehungsweise F mit Aufwärtsoktavierung. Teuer, nicht pauschal verboten. |
| $k_s$ | Maximale Abschnitte eines kombinierten Zustands, die kürzer als eine nominelle Taktlänge sind. |
| $g_b$ | Grenzen innerhalb der positiv gebundenen Gruppen. Musikalische und materielle Verbote gelten daneben unverändert. |
| $c_h$ | Echte Registerzeichen-Operationen. Reine Erinnerungszeichen werden getrennt behandelt. |

Die Platzierung einer Oktavlinie über ein ganzes System beschreibt Elaine Gould. Daran knüpft die projektinterne Regel für Erinnerungszeichen an.<a name="beleg-6-10-oktavlinie"></a><sup><a href="#quelle-gould">[27]</a></sup>

Die nominalen Zustände sind G2 oder F4 kombiniert mit loco, 8va, 8vb, 15ma oder 15mb. Mit Oktavanzahl $`q\in\lbrace 0,1,-1,2,-2\rbrace`$ und diatonischer Tonposition $d$ gilt für die geschriebene Position $d_{\mathrm{geschrieben}}=d-7q$. Deshalb ist die Schreibweise relevant; MIDI-Tasten allein bestimmen die Hilfslinien nicht.

Nominale Zustände sind noch keine Schreibbefugnis. Im aktuellen Adapter unterstützt der obere Owner nur seine tatsächlich angebotenen Zustände; zusätzliche F-Schlüsselalternativen im oberen System folgen nicht aus der nominalen Zehnermenge. Anfang, Ende, Zustandsalter, Zeitachse, Gruppen und Anschluss müssen gebunden sein. Für gekoppelte Systeme darf ein unabhängiger Einzelsystemkurs nicht als gemeinsames Optimum ausgegeben werden.

Die Schutzregel lautet für jeden sichtbaren Kopf $n$:

```math
\neg\big(L_z(n)\ge4\ \land\ L_z(n)\gt L_0(n)\big).
```

Ein Kandidat darf also keine neue oder verschärfte Belastung ab vier Hilfslinien erzeugen. Ein bereits belasteter Ausgangszustand wird nicht durch Behauptung unsichtbar. Unter den zulässigen vollständigen Kursen zählt die erste abweichende Kostenkomponente; Gleichstand erhält den Ausgang.

**Wichtige Folge:** Weniger Umschaltungen gewinnt nicht immer. Zum Beispiel kann die Vermeidung eines Vier-Hilfslinien-Kopfes einen kurzen Schlüsselwechsel rechtfertigen. Die Priorität gehört zur Zielfunktion des gemeinsamen Kurses.

Der Suchkern trägt zusätzlich zum Registerzustand das genaue, bei einer Taktlänge gesättigte Abschnittsalter. Rationale Taktkoordinaten behandeln Takt- und Divisionswechsel. Ein Ressourcenabbruch oder eine unbekannte Abhängigkeit beweist kein Optimum.

Ein dynamischer Planer kann mehrere zulässige Schlüsselverläufe vergleichen, ohne alle vollständigen Verläufe aufzubewahren. Bezeichnet $D_i(z)$ die beste bis Abschnitt $i$ erreichbare Bewertung mit Endzustand $z$, folgt das Verfahren dem Bellman-Prinzip

```math
D_i(z)=\min_{y\to z}\bigl(D_{i-1}(y)+c_i(y,z)\bigr).
```

Der Zustand muss alle Informationen enthalten, von denen künftige Anschlüsse abhängen.<a name="beleg-6-10-rekurrenz"></a><sup><a href="#quelle-bellman">[12]</a></sup> Eine verwandte Pfadrekurrenz findet sich bei Viterbi für die Decodierung einer übertragenen Symbolfolge.<a name="beleg-6-10-pfad"></a><sup><a href="#quelle-viterbi">[13]</a></sup> Beim Registerkurs können dazu bereits begonnene Abschnitte oder Gruppenpflichten gehören. Die konkrete Kostenordnung und die zulässigen Übergänge legt der jeweilige Planer fest. [Schlüsselplanung](../src/structural_write_authority/rustig_clef_planning.rs) · [Gemeinsamer Registerkurs](../src/pipeline_shards/joint_reading_c3_course.rs)

<a name="kapitel-6-11"></a>
<a name="graphzeugen-der-spielformel"></a>

### 6.11 Graphzeugen der Spielformel

Die symbolische Struktur enthält Ungleichungen zwischen Belegungsvariablen. Der Code bildet einen Graphen, erzeugt eine Smallest-last-/Degenerationsordnung und prüft die daraus gewonnene Zuordnung gegen die tatsächlichen Kanten.<a name="beleg-6-11-ordnung"></a><sup><a href="#quelle-matula">[9]</a></sup> Eine passende Clique kann umgekehrt einen strukturellen Konflikt belegen.

Seien $V$ die im Modell zu belegenden Ereignisse, $E$ die gleichzeitig unverträglichen Paare und $F$ die verfügbaren Plätze. Ein Beleg für diese Teilaufgabe ist eine geprüfte Zuordnung

```math
a\colon V\to F,\qquad (u,v)\in E\ \Longrightarrow\ a(u)\ne a(v).
```

Ein struktureller Zeuge schließt nur diese Ungleichungen. Handkonfiguration, Übergang, Wiederanschlag, Fähigkeiten und Zeitmodell bleiben zusätzliche Verpflichtungen. Weder die maximale Gleichzeitigkeit noch ein gefärbter Graph allein beweist einen ausführbaren Fingersatz.

<a name="kapitel-6-12"></a>
<a name="audio-pulsanalyse-und-goertzel-rekurrenz"></a>

### 6.12 Audio: Pulsanalyse und Goertzel-Rekurrenz

Die WAV-Pulsanalyse sucht Anstiege im mittleren Betrag der Samples und ihrer aufeinanderfolgenden Differenzen. Aus den zeitlichen Abständen der Spitzen gewinnt sie Puls- und Phasenhypothesen. Einen verwandten Ansatz über Anschlagsabstände beschreibt Dixon.<a name="beleg-6-12-puls"></a><sup><a href="#quelle-dixon">[28]</a></sup> Eine zeitliche Abbildung und Residuen beschreiben Abweichungen vom nominellen Raster. Bei mindestens vier Stützstellen glättet ein gleitender Dreiermedian die inneren Zeitversätze.<a name="beleg-6-12-median"></a><sup><a href="#quelle-tukey">[29]</a></sup> Die Audiohypothesen werden vor der späteren MIDI-Evaluation festgehalten; die Vergleichsquelle darf nicht unbemerkt das unabhängige Messergebnis erzeugen.

Für ausgewählte Frequenzen verwendet die harmonische Evidenzprobe die Rekurrenz von Gerald Goertzel.<a name="beleg-6-12-rekurrenz"></a><sup><a href="#quelle-goertzel">[30]</a></sup>

```math
s_j=x_j+2\cos(\omega)s_{j-1}-s_{j-2},\qquad \omega=2\pi f/f_s.
```

Für ein Fenster von $N$ Mono-Samples und die letzten beiden Zustände liefert der Code die normierte Energie

```math
P(f)=\max\left(0,\frac{s_{N-1}^2+s_{N-2}^2-2\cos(\omega)s_{N-1}s_{N-2}}{N^2}\right).
```

Die Frequenzzuordnung der Probe verwendet $f(m)=440\cdot2^{(m-69)/12}$: gleichstufige Stimmung mit A4 = 440 Hz ist hier eine Modellannahme. Der Code normiert die berechnete Energie durch das Quadrat der Fensterlänge. Die Frequenz einer Klaviertaste kann zwischen zwei Frequenzpunkten der diskreten Fouriertransformation liegen. Sysel und Rajmic behandeln die Goertzel-Auswertung für diesen Fall.<a name="beleg-6-12-frequenz"></a><sup><a href="#quelle-sysel">[31]</a></sup> Dies ist gezielte Frequenzanalyse, keine Behauptung, dass im Projekt eine allgemeine FFT-Pipeline implementiert sei. Ein hoher Wert kann auch von Obertönen oder überlagerten Klängen stammen. Basic Pitch liefert als externer neuronaler Sensor eine weitere WAV-Beobachtung.<a name="beleg-6-12-basic-pitch"></a><sup><a href="#quelle-basic">[32]</a></sup> Über die geschriebene Note entscheidet der Notationsschritt. Die von OpenAI untersuchten exakten Fourier-Schaltungen bilden einen Literaturvergleich. Ob ein numerisches Verfahren die Klaviertasten schneller auswertet, erfordert eine Messung am selben Audiomaterial.<a name="beleg-6-12-fourier"></a><sup><a href="#quelle-openai-130">[42]</a></sup>

<a name="kapitel-6-13"></a>
<a name="systemplanung-ist-nicht-seitenoptimierung"></a>
<a name="horizontaler-umbruch-und-begrenzte-gemeinsame-planung"></a>

### 6.13 Horizontaler Umbruch und begrenzte gemeinsame Planung

Der horizontale Reflow vergleicht endliche Systempartitionen unter seinen Kapazitätsbedingungen. Hegazy und Gourlay untersuchten den optimalen Zeilenumbruch eigens für Musik.<a name="beleg-6-13-systeme"></a><sup><a href="#quelle-hegazy">[24]</a></sup> Sein Vergleichsschlüssel betrachtet zuerst Systemzahl, Ein-Takt-Systeme, erzwungene Überkapazitäts-Einzelplätze und quadratische Restkapazität. Ein deterministischer Partitionsvergleich löst Gleichstände. [Horizontaler Reflow](../src/pipeline_shards/horizontal_layout_global_reflow.rs).

Sei $L_j$ der aufsummierte Bedarf des Systems $j$ und $C$ seine Kapazität in den Einheiten des Layoutplaners. Für eine zulässige Partition mit $S$ Systemen, $E$ Ein-Takt-Systemen, $U$ erzwungenen Ein-Takt-Systemen über Kapazität und dem Vektor $n$ der Taktzahlen je System minimiert der horizontale Pfad

```math
\left(S,E,U,\sum_{j=1}^{S}\max(0,C-L_j)^2,\ n\right)_{\mathrm{lex}}.
```

Bleiben über zwei Systeme vier Kapazitätseinheiten frei, kostet die Verteilung 0 und 4 insgesamt $0^2+4^2=16$. Die Verteilung 2 und 2 kostet $2^2+2^2=8$ und gewinnt bei gleichen vorrangigen Kriterien. Für die 31 Takte der Bach-Fuge reproduzierte eine unabhängige Gegenrechnung mit Kapazität 429 und höchstens vier Takten je System alle zwölf Umbruchstellen. Das Optimum für diese Eingaben umfasst 13 Systeme, null Ein-Takt-Systeme und eine quadratische Restsumme von 154 635. Die sichtbaren Abstände im PDF erfordern eine eigene Messung.

Für einen gebundenen Bereich zulässiger Seitenzahlen kann der Renderer-Pfad auch Seiten- und Systemgrenzen gemeinsam planen. Ein verwandtes Verfahren für die Seitenfüllung beschreibt Renz.<a name="beleg-6-13-seiten"></a><sup><a href="#quelle-renz">[25]</a></sup> Er berücksichtigt Systemlast, Abstandsanforderungen, das Limit für Systeme je Seite und bereits verworfene Partitionen. Der Schubert-Lauf r1872 bestätigte eine Kombination aus zwei Seiten und sieben Systemen. Diese Bestätigung betrifft die konkret gerenderte Partition. [Gemeinsame Planung](../src/pipeline_shards/horizontal_layout_global_page_system.rs) · [Exakte Partitionssuche](../src/pipeline_shards/renderer_exact_partition_search.rs).

Die geometrischen Readbacks untersuchen tatsächliche Hüllen und Abstände, soweit PDF-Syntax und Zuordnung unterstützt sind. Daraus folgt weder eine vollständige Kollisionsfreiheit aller denkbaren Zeichen noch eine implementierte Schlöpfchen-Seitenoptimierung.

<a name="kapitel-6-14"></a>
<a name="exakte-beschleunigung-ist-keine-neue-musikentscheidung"></a>
<a name="beschleunigung-mit-gleichen-entscheidungen"></a>

### 6.14 Beschleunigung mit gleichen Entscheidungen

Für additive Passagenkosten können Präfixsummen dieselbe Summe liefern:

```math
\sum_{i=a}^{b-1}c_i=P_b-P_a.
```

Die Indizes entstehen aus der exakten halboffenen Zeitbedingung, nicht aus einer geratenen Notennähe. Die Umordnung muss Kosten, Kandidaten und Gleichstandsregeln erhalten.

Die Bogenprüfung verwendet einen anderen Index: Für jede Nummer beantworten erster sichtbarer Anfang und letzter sichtbarer Schluss dieselben Existenzfragen wie die frühere Vollsuche. Das ist kein allgemeines globales Matching aller möglichen Bögen. Eine solche Erweiterung wäre eine eigene musikalische Entscheidung.

EtuSchlopf vergleicht wiederkehrende Phrasen mit dynamischer Programmierung.<a name="beleg-6-14-wiederholungen"></a><sup><a href="#quelle-smith">[3]</a></sup> Für $n$ Einsatzgruppen hat die Tabelle einschließlich ihres Randes die Breite $m=n+1$. Zwei Bewertungszeilen mit je acht Byte pro Zelle und eine Rückrichtung mit einem Byte pro Zelle benötigen zusammen

```math
B(m)=16m+m^2\quad\text{Byte}.
```

Bei der Metrumsuche berechnet EtuSchlopf feste Sprungkosten einmal pro Phasenmodell. Die Reihenfolge gleichwertiger Kandidaten bleibt erhalten. [Äquivalenzvertrag](selector_derivation_contract_performance_trace_equivalence.md)

<a name="kapitel-6-15"></a>
<a name="speicherplanung-für-den-korpus"></a>

### 6.15 Speicherplanung für den Korpus

Im Automatikmodus schätzt EtuSchlopf die Zahl paralleler Projekte und Solver-Worker. Für $p$ Projekte und den ermittelten verfügbaren Speicher $M$ in MiB lautet die speicherbezogene Worker-Grenze pro Projekt:

```math
w_{\mathrm{Speicher}}(p)=
\left\lfloor
\frac{\max\!\left(\left\lfloor\max(M-512,0)/p\right\rfloor-1536,0\right)}{256}
\right\rfloor.
```

Die Planung probiert die Projektzahl absteigend innerhalb der CPU-Grenze und begrenzt die Worker zusätzlich durch das CPU-Budget. Bei unbekanntem verfügbarem Speicher oder fehlender passender Aufteilung startet sie mit einem Projekt und einem Solver-Worker. Die drei Speichergrößen sind Schätzwerte für Reserve, Projektprozess und zusätzlichen Solver-Worker.<a name="beleg-6-15-speicher"></a><sup><a href="#quelle-speicher">[48]</a></sup> Der Kandidatenumfang besitzt ein getrenntes Budget. [Speichermodell](../src/pipeline_shards/corpus_memory_capacity.rs) · [Korpusplanung](../src/pipeline_shards/rest_glyph_retention_batch_corpus_tail.rs).

<a name="kapitel-6-16"></a>
<a name="projektion-und-integrität"></a>

### 6.16 Projektion und Integrität

Eine Aufbewahrungsprojektion $\pi$ ist nur dann ausreichend, wenn spätere Beobachtungen tatsächlich durch sie faktorisieren. Präziser als ein bloßer Größenvergleich ist die Bedingung

```math
\pi(D_1)=\pi(D_2)\Rightarrow O_L(D_1)=O_L(D_2)
```

für die betroffenen erreichbaren Leser $L$ und ihre Beobachtung $O_L$, einschließlich der relevanten Fortsetzungen. Eine identische aktuelle Ausgabe allein genügt nicht für künftige Mutationen. Die ausführbare Aufbewahrungspolitik bleibt der konkrete Nachweis.

Für die Quelltext-Prüfsummen verarbeitet FNV-1a die Bytes $b_i$ nacheinander. Die 64-Bit-Variante kombiniert den vorigen Zustand und das Byte mit bitweisem XOR und multipliziert anschließend mit der FNV-Primzahl

```math
h_{i+1}=((h_i\mathbin{\mathrm{XOR}}b_i)\cdot1099511628211)\bmod 2^{64}.
```

Die Take-Prüfung vergleicht die so deklarierten Dateiänderungen mit dem tatsächlichen Paket. Kryptografische Bindungen des Pakets und seiner Herkunft verwenden gesondert SHA-256.<a name="beleg-6-16-hash"></a><sup><a href="#quelle-fnv">[21]</a> <a href="#quelle-nist">[22]</a></sup> [Prüfung des Paketdiffs](../src/take_change_control.rs)

Hashes sind davon getrennt: Sie binden Bytes unter ihren jeweiligen Kollisionsannahmen. FNV-1a ist kein kryptografischer Beweis; auch SHA-256 ist mathematisch nicht injektiv über beliebig lange Eingaben. Provenienz, tatsächlich mitgeführte Ereignisidentität und bytebezogene Integritätsprüfung erfüllen verschiedene Aufgaben.

<a name="kapitel-6-17"></a>

### 6.17 Phrasierungsbögen und Konturabstände

MuseScore zeichnet einen Bogen bei einem Systemumbruch in mehreren Segmenten. EtuSchlopf ordnet die Segmente in einer SVG-Vorschau demselben MusicXML-Bogen zu. Es zerlegt die Bézierkurven für den Vergleich in kurze Strecken und berücksichtigt die Strichbreite. Die Prüfung zählt Kontakte mit musikalischen Zeichen wie Balken, Hälsen oder Notenköpfen über alle Segmente.

Sei $K_\varepsilon(l)$ die Zahl der Kontakte in der Lage $l$ bei einem Toleranzabstand $\varepsilon$ für Kurvenapproximation und Rundung. Die Gegenlage kommt für eine Übernahme infrage, wenn

```math
K_0(l_{\mathrm{auto}})=K_\varepsilon(l_{\mathrm{auto}})>0
\qquad\text{und}\qquad
K_\varepsilon(l_{\mathrm{gegen}})=0.
```

Bei einer zulässigen Gegenlage prüft EtuSchlopf zusätzlich Seitenformat, Zeichenzuordnung und Abstände zwischen benachbarten Zeichen, deren relative Lage sich ändert. Vorschläge auf verschiedenen Seiten müssen sich auch im gemeinsamen Rendervergleich bestätigen.

Die gemeinsame Vorschau bindet die Bewertung an die tatsächlich gezeichnete Kontur. Bleibt ein Kontakt oder lässt sich die Zuordnung über den Umbruch nicht eindeutig herstellen, behält EtuSchlopf die Ausgangslage. [Verfahren und Prüfkriterien](selector_derivation_contract_phrase_slur_clearance_implementation.md)

<a name="kapitel-7"></a>

## 7 Quellzeit, Pausen und Balken

Ein Ton belegt das halboffene Quellzeitintervall $[s,e)$. Der MIDI-Seed hält den Quellanschlag und die Loslasszeit zusammen.<a name="beleg-7-zeit"></a><sup><a href="#quelle-zeit">[44]</a></sup> Für eine gehaltene Oberstimme prüft er, ob sich deren Fortsetzung im vorhandenen metrischen Zeitfenster eindeutig bestimmen lässt. Jeder Folgeanschlag muss an die belegte Loslasszeit des vorigen Tons anschließen. Die Dauern müssen sich im Notenraster exakt darstellen lassen und das Fenster vollständig ausfüllen. Ein erneuter Anschlag derselben Tonhöhe innerhalb des Halteintervalls verhindert die Trennung. Wenn die neue Zuordnung uneindeutig ist, gilt der bisherige Ableitungsweg. [MIDI-Seed](../src/midi_seed.rs) · [Vertrag der zeitlich belegten Fortsetzung](selector_derivation_contract_timed_held_upper_voice_implementation.md)

Für das metrische Einsatzfenster $M$ einer geschriebenen Stimme sammelt EtuSchlopf deren klingende Intervalle $I_j$ systemübergreifend. Bereits sichtbare Belegung im Zielsystem sei $V$. Kandidaten für sichtbare Pausen sind die zusammenhängenden Teilintervalle von

```math
M \setminus \left(V \cup \bigcup_j I_j\right).
```

Der zuständige Planer übernimmt nur Teilintervalle, deren Anfang und Ende sich exakt im metrischen Raster schreiben lassen. Gleichzeitige Töne zählen bei der Intervallvereinigung einmal. [Planer für sichtbare Pausen](../src/pipeline_shards/composer_beat_scoped_rest_completion_planning_core.rs) · [Vertrag der exakten Pausenprojektion](written_voice_entry_rest_exact_projection.md)

Ein Balkenplan verbindet metrisch zusammengehörige Anschläge. Auf höheren Ebenen prüft die Rücklesung zusammenhängende Segmente gegen die gebundene Vorlage. Ein fehlendes Segment kann am Anfang, im Inneren oder am Ende auftreten. Eine Pause kann zwei freie Anschläge verbinden, wenn beide jeweils nur diesen passenden Nachbarn haben. Vollständige Balkengruppen behalten ihre Zugehörigkeit. [Metrische Balkenplanung](../src/pipeline_shards/composer_metric_beam_planning_core.rs) · [Rücklesung](../src/render/final_beam_observation_planning_core.rs)

Der Korpusnachweis unterscheidet zwischen einer beobachteten Render-Materialisierung und einem belegten direkten PDF-Export aus der Master-Quelle. Mozart KV 331 belegt den zweiten Weg. [Manifestprüfung](../src/corpus_run_manifest.rs)

<a name="kapitel-8"></a>
<a name="stimmfortsetzungen-prüfen"></a>

## 8 Stimmfortsetzungen prüfen

Zwei zeitlich gebundene Noten können im selben Ausgabekanal liegen, wenn eine zulässige Stimmzuordnung beide verbindet. Die Gegenprobe hält den Kontext der Passage fest und sucht jeweils nach einer Lösung mit gleichem und mit unterschiedlichem Kanal. Ein erzwungener Anschluss verlangt einen gültigen Zeugen für die Verbindung und einen vollständigen Ausschluss der Gegenlage. Die zuständige musikalische Regel prüft die Übernahme und den Erhalt bereits gebundener Anschlüsse. [Suchraum und Prüfung positiver Zeugen](selector_derivation_contract_exact_voice_passages.md)

<a name="kapitel-9"></a>
<a name="korpusprüfung-und-grenzen"></a>

## 9 Korpusprüfung und Grenzen

Der unter Windows abgenommene Stand r2005 erzeugt aus MIDI und WAV 15 Partituren mit 59 PDF-Seiten. Die Galerie enthält die Originalausgaben dieses Laufs. Der Windows-Lauf prüft die Ausführung und den PDF-Export. Ein musikalischer Vergleich mit einer Referenz bewertet zusätzlich Tonhöhe, Einsatz, geschriebene Dauer und sichtbare Notation. [Windows-Prüfbeleg](evidence/r2005/README.md) · [Notenbilder](examples/README.md#notenbilder)

<a name="kapitel-9-1"></a>

### 9.1 Vergleich der eigenen Etüden

Der Korpus enthält zehn eigene Etüden mit 49 Seiten. Ein Rastervergleich zeigt, auf welchen Seiten sich das Notenbild geändert hat. Die musikalische Beurteilung prüft die betroffenen Stimmen, Dauern und Zeichen im Zusammenhang. [Etüden und Referenzstücke](examples/README.md#notenbilder)

<a name="kapitel-10"></a>
<a name="nächste-mathematische-vorhaben"></a>

## 10 Nächste mathematische Vorhaben

<a name="kapitel-10-1"></a>

### 10.1 Notendauern freier Einspielungen

Welche Stimmfortsetzungen bleiben bei plausibler Änderung von Artikulation und örtlichem Tempo zulässig?

Nakamura, Yoshii und Dixon untersuchen genau die Unterscheidung zwischen gespielter und notierter Dauer im mehrstimmigen Kontext.<a name="beleg-10-1-dauer"></a><sup><a href="#quelle-nakamura-yoshii-dixon">[36]</a></sup> Ein anderer Vergleich ist die direkte Übersetzung einer gesamten Performance-MIDI in Notenschrift bei Beyer und Dai.<a name="beleg-10-1-transkription"></a><sup><a href="#quelle-beyer-dai">[40]</a></sup>

Eine Gegenprobe für die Dauerentscheidung in [Kapitel 3.2](#kapitel-3-2) ist ein gehaltener Ton, über dem eine zweite Stimme früher oder später einsetzt. Dazu passt als Gegenfall ein tatsächlich kurz gespielter Ton mit anschließender notierter Pause. [Referenzzuordnung der Chopin-Einspielung p01](https://github.com/CPJKU/vienna4x22/blob/master/match/Chopin_op10_no3_p01.match)

**Gegenprobe.** Für unabhängige, synthetische Stimmen kurze gehaltene, kurz gespielte lange und überlappende Noten unterscheiden. Auf zurückgehaltenen Einspielungen Dauern, neu gewonnene und verlorene Treffer getrennt ausweisen. Eine ergänzende metamorphische Probe verdoppelt MIDI-Auflösung und Tickpositionen gemeinsam. Die rationalen Dauern müssen gleich bleiben.<a name="beleg-10-1-test"></a><sup><a href="#quelle-metamorphische-tests">[20]</a></sup>

<a name="kapitel-10-2"></a>

### 10.2 Stimme und Hand über eine Passage

Welche Kanten eines zeitlichen Belegungsgraphen sind in jeder zulässigen Stimmenführung enthalten?

Die bestehende Prüfung erzwungener Anschlüsse beschreibt [Kapitel 6.4](#kapitel-6-4). Foscarin, Karystinaios und ihre Mitautoren untersuchen Akkordzugehörigkeit, Stimmfortsetzung und Notensystem als drei unterscheidbare Beziehungen.<a name="beleg-10-2-stimmen"></a><sup><a href="#quelle-cluster-and-separate">[37]</a></sup> Eine Folgearbeit betrachtet mehrere Satzentscheidungen gemeinsam.<a name="beleg-10-2-satz"></a><sup><a href="#quelle-engravinggnn">[38]</a></sup> Das OpenAI-Manuskript zu perfekten Matchings schätzt, wie viele Paarungen möglich sind. Für die einzelne musikalische Linie interessiert uns, welche Verbindung in jedem zulässigen Plan vorkommt.<a name="beleg-10-2-matching"></a><sup><a href="#quelle-openai-113">[41]</a></sup>

**Gegenprobe.** Für eine erzwungene Beziehung einen vollständigen Plan und den Ausschluss der Gegenbelegung zeigen. Bei Mehrdeutigkeit zwei zulässige Gegenpläne nachweisen. Prüfbare Belege für Optimalität und Unlösbarkeit behandeln Koops und Mitautoren für pseudo-boolesche Optimierung sowie Flippo, Sidorov und Mitautoren für Constraint-Programme.<a name="beleg-10-2-zertifikate"></a><sup><a href="#quelle-koops">[18]</a> <a href="#quelle-flippo-sidorov">[19]</a></sup>

<a name="kapitel-10-3"></a>

### 10.3 Lesbare Seitenfüllung

Welche geordneten Seiten- und Systempartitionen halten gemessene Mindestabstände für Noten, Text und Bögen ein?

Die Schlöpfchen-Formel steht in [Kapitel 2.2](#kapitel-2-2), der implementierte Umbruch in [Kapitel 6.13](#kapitel-6-13). Wie eng beide Entscheidungen zusammenhängen, zeigt auch die Dokumentation zum Seitenumbruch in LilyPond.<a name="beleg-10-3-lilypond"></a><sup><a href="#quelle-lilypond">[26]</a></sup>

**Gegenprobe.** An tatsächlich gerenderten Seiten engste sichtbare Abstände und ungenutzte Höhe pro Seite ausweisen. Erst unter den zulässigen Plänen die Seitenfüllung vergleichen.

<a name="kapitel-10-4"></a>

### 10.4 Rechenzeit im gemeinsamen Registerplan

Welche Zustände lassen sich zusammenfassen, wenn sie für alle noch erreichbaren Anschlussentscheidungen dieselben Kosten und Bedingungen haben?

Das bestehende Planungsmodell beschreibt [Kapitel 6.10](#kapitel-6-10). Für die Zustandszusammenfassung ist Bellmans Bedingung maßgeblich: Ein gespeicherter Zustand muss alles enthalten, was die Bewertung der Fortsetzung beeinflusst.<a name="beleg-10-4-zustand"></a><sup><a href="#quelle-bellman">[12]</a></sup>

**Gegenprobe.** Gegen die vollständige Suche denselben Kurs und dieselbe Gleichstandsentscheidung nachweisen. Laufzeit und aufgesuchte Zustände mit identischen Eingaben vergleichen.

<a name="kapitel-10-5"></a>

### 10.5 Pausenhöhe und Seitenlage

**Abgenommen in r2005.** Bei einer kurzen sichtbaren Basspause bestimmt EtuSchlopf die Zielhöhe aus den gezeichneten Notenköpfen unmittelbar davor und danach. Beide Noten müssen zu derselben Stimme gehören, zeitlich an die Pause anschließen und bis auf die Rundungstoleranz dieselbe Höhe aufweisen. Aus den Höhen $y_{-}$ und $y_{+}$ bildet EtuSchlopf die Zielhöhe $y_* = (y_{-} + y_{+})/2$.

EtuSchlopf prüft bis zu fünf nahe vertikale Versuchslagen. Unter den gerenderten Lagen, die den Abstandstest bestehen und die Pause der Zielhöhe nachweislich näher bringen, minimiert es zuerst $|y-y_*|$ und dann die Verschiebung $|y-y_0|$ gegenüber der Ausgangshöhe $y_0$. Die horizontale Position bleibt bis auf die Rundungstoleranz erhalten.

#### Konservative Abstandsmessung

Umschließen $B(P)$ und $B(H)$ die gezeichnete Pause $P$ und ein Nachbarzeichen $H$ vollständig, liefert der Abstand der Rechtecke eine Untergrenze für den Abstand der Zeichen. Mit dem Notenlinienabstand $s$ und der Reserve $arepsilon$ je Zeichen lautet die Bedingung

```math
d(B(P),B(H)) le d(P,H),qquad
min_H rac{max(0,d(B(P),B(H))-2arepsilon)}{s} ge 0{,}50 .
```

Die Nachbarzeichen schließen weitere Pausen ein. Notenlinien dienen als Führungsgeometrie und dürfen die Pause kreuzen. Ein gemeinsamer Render prüft alle vorgesehenen Bewegungen, ihren Abstand zu den übrigen Zeichen und die Stabilität der übrigen Geometrie. Bei fehlendem Nachweis bleibt die bisherige Lage erhalten.

Im Windows-Vollkorpus änderten sich zwölf Pausenpositionen in drei Partituren. Alle Master-MusicXML-Dateien sind bytegleich. Im Vergleich mit r2002 blieben 55 von 59 PDF-Seiten bei 144 dpi pixelgleich. Die gemeinsame Wahl von seitlichen Positionen und Zielhöhen bei unterschiedlich hohen Nachbarn bleibt offen. [Windows-Prüfbeleg](evidence/r2005/README.md) · [Platzierungsregel](../src/structural_write_authority/rest_svg_placement.rs) · [Abstandsmessung](../src/structural_write_authority/rest_svg_geometry.rs)

<a name="kapitel-11"></a>
<a name="herkunft-und-eigenanteil"></a>

## 11 Herkunft und Eigenanteil

Standardverfahren bleiben Standardverfahren: dynamische Programmierung, lexikografische Ordnung, Intervallgraphen, Färbung, Nichtdominanz, rationale Arithmetik, quadratische Kosten, Spektralrekurrenzen, Deskriptivstatistik und Hashfunktionen wurden nicht für EtuSchlopf erfunden.

Unser Eigenanteil liegt in den musikalischen Modellen, dem Zusammenspiel der Bedingungen, den gebundenen Owner-Übergängen und den gewählten Prioritäten. Die drei Projektbegriffe benennen solche Zusammenstellungen. Weder deren wissenschaftliche Neuheit noch die Behauptung „so gibt es das sonst nirgends“ ist hier nachgewiesen.

Die Quellenziffern führen zum jeweiligen Beitrag. So lässt sich etwa die optimale Zuordnung in [Kapitel 3.2](#kapitel-3-2) bis zu den Arbeiten von Kuhn und Munkres zurückverfolgen.<a name="beleg-11-eigenanteil"></a><sup><a href="#quelle-zuordnung">[4]</a></sup> Die dort verlinkte Implementierung zeigt, wie EtuSchlopf gespielte Dauer, Abstand der Einsätze und Tonhöhenabstand in der Zuordnung bewertet.

<a name="kapitel-12"></a>
<a name="quellen"></a>

## 12 Quellen

Jeder Eintrag nennt den Beitrag der Quelle zur verlinkten Aussage. Die Pfeile führen zu den einzelnen Verwendungen. Für Messwerte und Regeln des Programms sind die zugehörigen Projektnachweise angegeben.

<a name="kapitel-12-1"></a>
<a name="mathematische-und-algorithmische-grundlagen"></a>

### 12.1 Mathematische und algorithmische Grundlagen

<a name="quelle-euklid"></a>**[1]** Euklid, *Elemente* (um 300 v. Chr.), Buch VII, Propositionen 1–3. Die ersten beiden Sätze behandeln gemeinsame Maße zweier ganzer Zahlen, der dritte erweitert das Verfahren auf drei. EtuSchlopf teilt die ganzzahligen Zeitabstände einer Phrase durch ihren größten gemeinsamen Teiler. So erhalten die Abstandsfolgen 12–24–12 und 24–48–24 dieselbe Darstellung 1–2–1. [Buch VII mit den einzelnen Propositionen](https://www.claymath.org/library/historical/euclid/book07.html) · [↑ 6.7](#beleg-6-7-ggt) · [↑ 4, Nr. 1](#beleg-4-verfahren-1)

<a name="quelle-jaccard"></a>**[2]** Paul Jaccard, *Distribution de la flore alpine dans le Bassin des Dranses et dans quelques régions voisines* (1901), S. 241–272, und *Étude comparative de la distribution florale dans une portion des Alpes et du Jura* (1901), S. 547–579. Jaccard verglich die floristische Zusammensetzung von Standorten. In der musikalischen Wiederholungssuche misst derselbe Koeffizient den Anteil gemeinsamer Tonhöhen an der Menge aller Tonhöhen zweier Anschlagsgruppen. [Erste Untersuchung](https://doi.org/10.5169/seals-266440) · [Vergleichsstudie](https://doi.org/10.5169/seals-266450) · [↑ 3.1](#beleg-3-1-wiederholungen)

<a name="quelle-smith"></a>**[3]** Temple F. Smith und Michael S. Waterman, *Identification of Common Molecular Subsequences* (1981), S. 195–197. Ihr lokaler Sequenzvergleich kann eine Teilfolge neu beginnen, eine Position überspringen oder zwei Positionen einander zuordnen. EtuSchlopf verwendet dieses Prinzip bei der Suche nach wiederkehrenden Passagen innerhalb einer Einspielung. Tonmengenähnlichkeit, Lückenkosten und Mindestabstand der Passagen sind projektspezifisch. [Originalartikel](https://doi.org/10.1016/0022-2836(81)90087-5) · [↑ 3.1](#beleg-3-1-wiederholungen) · [↑ 6.14](#beleg-6-14-wiederholungen)

<a name="quelle-zuordnung"></a>**[4]** Harold W. Kuhn, *The Hungarian Method for the Assignment Problem* (1955), S. 83–97, und James Munkres, *Algorithms for the Assignment and Transportation Problems* (1957), S. 32–38, behandeln die optimale gewichtete Zuordnung. Kuhn knüpft an Dénes König, *Graphok és mátrixok*, und Jenő Egerváry, *Mátrixok kombinatorius tulajdonságairól* (beide 1931), an. Eine frühere unabhängige Herleitung steht in Carl Gustav Jacob Jacobis posthum veröffentlichtem *De investigando ordine systematis aequationum differentialium vulgarium cujuscunque* (1865), S. 297–320. Sigismund Cohn begann die Transkription des Manuskripts. Carl Wilhelm Borchardt vollendete die Herausgabe. EtuSchlopf verwendet eine rechteckige Zuordnung mit zusätzlichen Spalten für Töne ohne Folgeeinsatz. Die musikalischen Kosten legt das Projekt fest. [Kuhn](https://doi.org/10.1002/nav.3800020109) · [Munkres](https://doi.org/10.1137/0105003) · [König und Egerváry im Originalband](https://real-j.mtak.hu/7307/1/MTA_MatematikaiEsPhysikaiLapok_38.pdf#page=22) · [Jacobi](https://doi.org/10.1515/crll.1865.64.297) · [Geschichte der Handschrift](https://www.lix.polytechnique.fr/~ollivier/PRODUCTION_SCIENT/AAECC_Jacobi/jacobi1V3.pdf) · [↑ 3.2](#beleg-3-2-zuordnung) · [↑ 11](#beleg-11-eigenanteil)

<a name="quelle-hall"></a>**[5]** Philip Hall, *On Representatives of Subsets* (1935), S. 26–30. Verschiedene Vertreter lassen sich genau dann auswählen, wenn jede Teilfamilie zusammen mindestens so viele Möglichkeiten hat, wie sie Mitglieder enthält. Im Beispiel der drei Stimmen brauchen B und C ihre beiden möglichen Anschlüsse Y und Z vollständig auf. Für A bleibt damit nur X. [Originalartikel](https://doi.org/10.1112/jlms/s1-10.37.26) · [↑ 6.4](#beleg-6-4-hall)

<a name="quelle-dilworth"></a>**[6]** Robert P. Dilworth, *A Decomposition Theorem for Partially Ordered Sets* (1950), S. 161–166, Satz 1.1. Im vereinfachten Stimmenmodell bilden zeitlich aufeinanderfolgende, überlappungsfreie Töne eine Kette. Eine Antikette besteht aus paarweise überlappenden Zeitintervallen. Dilworths Satz erklärt, warum die kleinste Zahl solcher Ketten der größten gleichzeitigen Belegung entspricht. Für musikalische Stimmführungen kommen weitere Bedingungen hinzu. [Originalartikel](https://mathweb.ucsd.edu/~asuk/dilworth.pdf) · [↑ 6.2](#beleg-6-2-faerbung)

<a name="quelle-golumbic"></a>**[7]** Martin Charles Golumbic, *Algorithmic Graph Theory and Perfect Graphs* (1980). Das Lehrbuch erläutert Intervallgraphen und ihre Färbung. Im zeitlichen Stimmenmodell entspricht eine Farbe einem überlappungsfrei belegten Kanal. [Originalausgabe](https://www.sciencedirect.com/science/book/9780122892608) · [↑ 6.2](#beleg-6-2-faerbung) · [↑ 4, Nr. 6](#beleg-4-verfahren-6)

<a name="quelle-backtrack"></a>**[8]** Robert M. Haralick und Gordon L. Elliott, *Increasing Tree Search Efficiency for Constraint Satisfaction Problems* (1980), S. 263–313. Die Autoren vergleichen klassische Backtracking-Suche mit vorausschauenden Verfahren für binäre Nebenbedingungen. Die EtuSchlopf-Passagensuche belegt jeweils einen weiteren Kanal und verwirft Teilpläne, die bereits belegten Nachbarn widersprechen. Ein Zwang ist erst belegt, wenn ein zulässiger Plan vorliegt und die Suche alle Gegenpläne ausgeschlossen hat. [Originalartikel](https://doi.org/10.1016/0004-3702(80)90051-X) · [↑ 6.4](#beleg-6-4-suche) · [↑ 4, Nr. 39](#beleg-4-verfahren-39)

<a name="quelle-matula"></a>**[9]** David W. Matula, George Marble und Joel D. Isaacson, *Graph Coloring Algorithms* (1972), S. 109–122, sowie David W. Matula und Leland L. Beck, *Smallest-Last Ordering and Clustering and Graph Coloring Algorithms* (1983), S. 417–427. Bei dieser Ordnung entfernt man schrittweise einen Knoten mit kleinstem verbliebenen Grad und färbt in umgekehrter Reihenfolge. EtuSchlopf prüft auf diese Weise, ob sich die modellierten Fingerplätze ohne Kantenkonflikt belegen lassen. [Matula, Marble und Isaacson](https://doi.org/10.1016/B978-1-4832-3187-7.50015-5) · [Matula und Beck](https://doi.org/10.1145/2402.322385) · [↑ 6.11](#beleg-6-11-ordnung) · [↑ 4, Nr. 20](#beleg-4-verfahren-20)

<a name="quelle-pareto"></a>**[10]** Francis Ysidro Edgeworth, *Mathematical Psychics* (1881), S. 20–29, beschreibt die Kontraktkurve beim Tausch zwischen zwei Parteien. Vilfredo Pareto formuliert das Wohlfahrtskriterium im *Manuale di economia politica* (1906), Kapitel VI, § 33. EtuSchlopf überträgt den Vergleich auf musikalische Merkmale. Ein Kandidat dominiert einen anderen, wenn er in keinem Merkmal schlechter und in mindestens einem besser abschneidet. Auf der ersten Front können mehrere Kandidaten verbleiben. [Edgeworths Originalausgabe](https://socserv2.mcmaster.ca/~econ/ugcm/3ll3/edgeworth/mathpsychics.pdf) · [Paretos Originaltext, Kapitel VI](https://it.wikisource.org/wiki/Manuale_di_economia_politica_con_una_introduzione_alla_scienza_sociale/Capitolo_VI) · [↑ 6.6](#beleg-6-6-dominanz) · [↑ 4, Nr. 16](#beleg-4-verfahren-16)

<a name="quelle-quadrate"></a>**[11]** Adrien-Marie Legendre, *Nouvelles méthodes pour la détermination des orbites des comètes* (1805), Anhang „Sur la méthode des moindres quarrés“, und Carl Friedrich Gauß, *Theoria motus corporum coelestium* (1809), untersuchen den Ausgleich von Beobachtungsfehlern durch Quadratsummen. In EtuSchlopf gehören Gewichte, diskrete Strafen und die Wahl der musikalischen Abstände zum Modell. [Legendre, Originalausgabe im BnF-Katalog](https://catalogue.bnf.fr/ark:/12148/cb30775990h) · [Gauß, Digitalisat](https://doi.org/10.3931/e-rara-522) · [↑ 6.8](#beleg-6-8-quadratsumme)

<a name="quelle-bellman"></a>**[12]** Richard Bellman, *On the Theory of Dynamic Programming* (1952), S. 716–719, und *Dynamic Programming* (1957). Der beste weitere Plan hängt vom erreichten Zustand ab. Welche Vorgeschichte EtuSchlopf im Registerzustand aufbewahren muss, ergibt sich aus den musikalischen Anschlussbedingungen. [Originalartikel](https://doi.org/10.1073/pnas.38.8.716) · [Monografie](https://press.princeton.edu/books/paperback/9780691146683/dynamic-programming) · [↑ 6.10](#beleg-6-10-rekurrenz) · [↑ 10.4](#beleg-10-4-zustand) · [↑ 4, Nr. 12](#beleg-4-verfahren-12)

<a name="quelle-viterbi"></a>**[13]** Andrew J. Viterbi, *Error Bounds for Convolutional Codes and an Asymptotically Optimum Decoding Algorithm* (1967), S. 260–269. Ein früher Anwendungsfall der Pfadsuche in Zustandsfolgen. EtuSchlopf bewertet den Registerverlauf anhand von Lesbarkeitskosten. [Originalartikel](https://doi.org/10.1109/TIT.1967.1054010) · [↑ 6.10](#beleg-6-10-pfad)

<a name="quelle-parzen"></a>**[14]** **Kernglättung bei der Pulssuche.** Murray Rosenblatt, *Remarks on Some Nonparametric Estimates of a Density Function* (1956), und Emanuel Parzen, *On Estimation of a Probability Density Function and Mode* (1962). EtuSchlopf glättet Zeitspannen über zwei und über drei aufeinanderfolgende Einsatzintervalle mit Gauß-Kernen. Bei fester Bandbreite ist jede Kurve proportional zu einer Kerndichteschätzung. Die Auswahl des Pulses nach der schwächeren der beiden Kurven und die Prüfung in drei Aufnahmeabschnitten sind eigene Regeln. [Rosenblatt](https://doi.org/10.1214/aoms/1177728190) · [Parzen](https://doi.org/10.1214/aoms/1177704472) · [↑ 3.1](#beleg-3-1-pulssuche)

<a name="quelle-laplace"></a>**[15]** **Glättung bei der Auftaktbestimmung.** Pierre-Simon Laplace, *Essai philosophique sur les probabilités* (1814). Edwin B. Wilson gibt in *Probable Inference, the Law of Succession, and Statistical Inference* (1927), S. 210, die allgemeine Formel $(k+1)/(n+2)$ für $k$ Erfolge bei $n$ Beobachtungen an. EtuSchlopf verwendet diesen geglätteten Anteil, um Basswechsel an möglichen Taktpositionen zu vergleichen. So erhält jede Position rechnerisch einen zusätzlichen Wechsel und einen zusätzlichen unveränderten Übergang. [Laplaces Originalausgabe](https://www.e-rara.ch/zut/content/pagetext/1090777) · [Formel bei Wilson](https://jhanley.biostat.mcgill.ca/c607/ch08/wilson_jasa_1927.pdf) · [↑ 3.1](#beleg-3-1-auftakt)

<a name="kapitel-12-2"></a>
<a name="programmnachweise-und-integrität"></a>

### 12.2 Programmnachweise und Integrität

<a name="quelle-hoare"></a>**[16]** Robert W. Floyd, *Assigning Meanings to Programs* (1967), S. 19–32, und C. A. R. Hoare, *An Axiomatic Basis for Computer Programming* (1969), S. 576–580 und 583. Floyd verknüpft Programmpunkte mit Zusicherungen. Hoare gibt Regeln für Vor- und Nachbedingungen an. Diese Unterscheidung hilft beim Lesen der projektspezifischen Übergangsbedingungen. [Floyd](https://userweb.cs.txstate.edu/~rp31/papers/Floyd.pdf) · [Hoare](https://doi.org/10.1145/363235.363259) · [↑ 2.1](#beleg-2-1-bedingungen)

<a name="quelle-necula"></a>**[17]** George C. Necula, *Proof-Carrying Code* (1997), S. 106–119. Ein Empfänger prüft einen mitgelieferten Beweis gegen eine festgelegte Sicherheitspolitik. In EtuSchlopf ist die getrennte Prüfung eines mitgelieferten Übergangsbelegs der konzeptionelle Bezug. [Originalartikel](https://doi.org/10.1145/263699.263712) · [↑ 2.1](#beleg-2-1-pruefung)

<a name="quelle-koops"></a>**[18]** Wietze Koops, Daniel Le Berre, Magnus O. Myreen, Jakob Nordström, Andy Oertel, Yong Kiam Tan und Marc Vinyals, *Practically Feasible Proof Logging for Pseudo-Boolean Optimization* (CP 2025). Für die Kandidatensuche ist die Trennung zweier Nachweise relevant. Ein gültiger Notenplan belegt, dass die Bedingungen erfüllbar sind. Eine Behauptung über seine Optimalität erfordert einen zusätzlichen Beleg. [Originalarbeit](https://doi.org/10.4230/LIPIcs.CP.2025.21) · [↑ 10.2](#beleg-10-2-zertifikate)

<a name="quelle-flippo-sidorov"></a>**[19]** Maarten Flippo und Konstantin Sidorov (gleichberechtigte Erstautoren), Tip ten Brink, Clément Pit-Claudel und Emir Demirović, *Formally Verified Certification of Constraint Programming Proofs* (CP 2026). Vorbild für die Prüfung einer behaupteten Unlösbarkeit oder Optimalität im benannten Suchraum, etwa beim Ausschluss alternativer Stimmanschlüsse. [Originalarbeit](https://doi.org/10.4230/LIPIcs.CP.2026.24) · [↑ 10.2](#beleg-10-2-zertifikate)

<a name="quelle-metamorphische-tests"></a>**[20]** Tsong Yueh Chen, Shing-Chi Cheung und Siu Ming Yiu, *Metamorphic Testing: A New Approach for Generating Next Test Cases* (HKUST-CS98-01, 1998). Den Forschungsstand fassen Tsong Yueh Chen, Fei-Ching Kuo, Huai Liu, Pak-Lok Poon, Dave Towey, T. H. Tse und Zhi Quan Zhou in *Metamorphic Testing: A Review of Challenges and Opportunities* (ACM Computing Surveys, 2018) zusammen. Für die Projektprüfung lässt sich etwa die MIDI-Auflösung samt Tickpositionen verdoppeln und anschließend prüfen, ob die rationalen Notendauern erhalten bleiben. [Originalbericht](https://www.cse.ust.hk/faculty/scc/publ/CS98-01-metamorphictesting.pdf) · [Übersichtsarbeit](https://i.cs.hku.hk/~tse/Papers/2010s/hlmtCSUR.html) · [↑ 10.1](#beleg-10-1-test)

<a name="quelle-fnv"></a>**[21]** Glenn Fowler, Landon Curt Noll und Kiem-Phong Vo entwickelten FNV. RFC 9923 dokumentiert die Beiträge und den Unterschied zwischen FNV-1 und FNV-1a. EtuSchlopf verwendet FNV-1a für nichtkryptografische Fingerprints. [Beschreibung der Entwickler](https://www.isthe.com/chongo/tech/comp/fnv/index.html) · [RFC 9923, Abschnitte 2 und 7](https://www.rfc-editor.org/rfc/rfc9923.html#section-7) · [↑ 6.16](#beleg-6-16-hash) · [↑ 4, Nr. 33](#beleg-4-verfahren-33)

<a name="quelle-nist"></a>**[22]** National Institute of Standards and Technology, *Secure Hash Standard*, FIPS PUB 180-4 (2015), Abschnitt 6.2. Der Standard spezifiziert SHA-256 für die kryptografische Bindung von Bytes. [Norm](https://doi.org/10.6028/NIST.FIPS.180-4) · [↑ 6.16](#beleg-6-16-hash) · [↑ 4, Nr. 33](#beleg-4-verfahren-33)

<a name="kapitel-12-3"></a>
<a name="umbruch-audiomodell-und-spielbarkeitskontext"></a>

### 12.3 Umbruch, Audiomodell und Spielbarkeitskontext

<a name="quelle-knuth"></a>**[23]** Donald E. Knuth und Michael F. Plass, *Breaking Paragraphs into Lines* (1981), S. 1119–1184. Die Autoren vergleichen Umbruchstellen über einen ganzen Textabsatz hinweg. Im Notensatz treten Taktgrenzen, Systemhöhen und die Geometrie der Zeichen als eigene Bedingungen hinzu. [Originalartikel](https://doi.org/10.1002/spe.4380111102) · [↑ 2.2](#beleg-2-2-umbruch)

<a name="quelle-hegazy"></a>**[24]** Wael A. Hegazy und John S. Gourlay, *Optimal Line Breaking in Music*, Ohio State University, Forschungsbericht OSU-CISRC-8/87-TR33 (August 1987). Die Arbeit überträgt den globalen Umbruch auf die Verteilung musikalischer Abschnitte auf Notensysteme. EtuSchlopfs Kosten und Gleichstandsregeln stehen in Kapitel 6.13. [Bibliografischer Nachweis der University of Illinois](https://courses.grainger.illinois.edu/ece402/webECE402-Readings.pdf) · [Darstellung bei Renz, Abschnitt 4.5.2](https://tuprints.ulb.tu-darmstadt.de/265/1/kai_renz_diss.pdf) · [↑ 6.13](#beleg-6-13-systeme) · [↑ 4, Nr. 26](#beleg-4-verfahren-26)

<a name="quelle-renz"></a>**[25]** Kai Renz, *Algorithms and Data Structures for a Music Notation System based on GUIDO Music Notation*, Dissertation, TU Darmstadt (2002), Abschnitte 4.5.2 und 4.5.3, ab S. 100 und 109. Renz erweitert den optimalen Systemumbruch um die Seitenfüllung. Sein Modell bietet einen fachlichen Vergleich für EtuSchlopfs begrenzte gemeinsame Planung und für den Entwurf der Schlöpfchen-Formel. [Dissertation im Original](https://tuprints.ulb.tu-darmstadt.de/265/1/kai_renz_diss.pdf) · [↑ 2.2](#beleg-2-2-umbruch) · [↑ 6.13](#beleg-6-13-seiten) · [↑ 4, Nr. 37](#beleg-4-verfahren-37)

<a name="quelle-lilypond"></a>**[26]** LilyPond-Projekt, *LilyPond Notation Reference 2.26*, Abschnitt *Page breaking* (2026). Das Handbuch zeigt, wie sich die Verteilung der Takte auf Systeme und der Systeme auf Seiten gemeinsam bewerten lässt. Das ist eine technische Referenz für EtuSchlopfs Umbruchplanung. Han-Wen Nienhuys und Jan Nieuwenhuizen sind die Hauptautoren des Gesamtprojekts. [Handbuch](https://lilypond.org/doc/v2.26/Documentation/notation/page-breaking) · [Projektmitwirkende](https://lilypond.org/doc/v2.26/Documentation/web/authors) · [↑ 10.3](#beleg-10-3-lilypond)

<a name="quelle-gould"></a>**[27]** Elaine Gould, *Behind Bars: The Definitive Guide to Music Notation*, Faber Music (2011), S. 29. Gould erläutert die Platzierung einer Oktavlinie über ein ganzes System und ihre Orientierungsfunktion beim Lesen. Darauf bezieht sich EtuSchlopfs Vertrag zur Darstellung von Oktavierungszeichen in Kapitel 6.10. [Originalseite in der Verlagsleseprobe](https://www.behindbarsnotation.co.uk/contents/sample_pages.pdf#page=4) · [↑ 6.10](#beleg-6-10-oktavlinie)

<a name="quelle-dixon"></a>**[28]** **Tempo aus Anschlagsabständen.** Simon Dixon, *Automatic Extraction of Tempo and Beat From Expressive Performances* (2001). Dixon leitet aus den Abständen rhythmischer Ereignisse mehrere Tempohypothesen ab. Der WAV-Audit von EtuSchlopf zählt dazu skalierte Anschlagsabstände in Zeitklassen und stimmt über die Pulsphase ab. Diese Abstimmung ist projektspezifisch. [Originalarbeit](https://doi.org/10.1076/jnmr.30.1.39.7119) · [↑ 6.12](#beleg-6-12-puls) · [↑ 4, Nr. 22](#beleg-4-verfahren-22)

<a name="quelle-tukey"></a>**[29]** **Gleitender Dreiermedian.** John W. Tukey, *Exploratory Data Analysis* (1977), Kapitel 7A „Medians of 3“, S. 210. Bei mindestens vier Stützstellen glättet der WAV-Audit die inneren Zeitversätze in einem Durchgang. Jeder neue innere Wert ist der Median aus seinem bisherigen Wert und den beiden Nachbarn. Die beiden Endpunkte bleiben erhalten. [Buch und Inhaltsverzeichnis](https://openlibrary.org/books/OL4877620M/Exploratory_data_analysis) · [↑ 6.12](#beleg-6-12-median)

<a name="quelle-goertzel"></a>**[30]** Gerald Goertzel, *An Algorithm for the Evaluation of Finite Trigonometric Series* (1958), S. 34–35. Die Rekurrenz ermöglicht die gezielte Auswertung einzelner Frequenzen aus den Samples eines Fensters. [Originalartikel](https://doi.org/10.2307/2310304) · [↑ 6.12](#beleg-6-12-rekurrenz) · [↑ 4, Nr. 24](#beleg-4-verfahren-24)

<a name="quelle-sysel"></a>**[31]** **Spektralmessung zwischen DFT-Frequenzpunkten.** Petr Sysel und Pavel Rajmic, *Goertzel algorithm generalized to non-integer multiples of fundamental frequency* (2012). Die Arbeit behandelt die gezielte Spektralauswertung bei frei gewählten Frequenzen. Die WAV-Probe setzt die Frequenz jeder Klaviertaste direkt in die Goertzel-Rekurrenz ein und wertet das durch das Quadrat der Fensterlänge normierte Betragsquadrat aus. [Originalarbeit](https://doi.org/10.1186/1687-6180-2012-56) · [↑ 6.12](#beleg-6-12-frequenz)

<a name="quelle-basic"></a>**[32]** Rachel M. Bittner, Juan José Bosch, David Rubinstein, Gabriel Meseguer-Brocal und Sebastian Ewert, *A Lightweight Instrument-Agnostic Model for Polyphonic Note Transcription and Multipitch Estimation* (ICASSP 2022). EtuSchlopf verwendet Basic Pitch als externes Modell zur WAV-Analyse. Der Sensor liefert Beobachtungen für den Vergleich. [Autorenfassung](https://arxiv.org/abs/2203.09893) · [Einbindung und Grenzen](RESEARCH_CONTEXT.md) · [↑ 6.12](#beleg-6-12-basic-pitch) · [↑ 4, Nr. 25](#beleg-4-verfahren-25)

<a name="quelle-turner"></a>**[33]** Craig Turner, Justine Pelletier, Robin Mailly, Luis Angel Rivera, Fabien Dal Maso, Mickaël Begon und Felipe Verdugo, „An alternative 6-inch octave keyboard reduces forearm muscle activation level and improves finger postures in expert pianists with smaller hand spans“, *Applied Ergonomics* 133 (2026, online 2025), Artikel 104709. Die Studie untersucht Muskelaktivität und Fingerhaltung bei kleiner Handspanne auf zwei Klaviaturgrößen. Sie liefert empirischen Kontext zur Instrumentengeometrie in Kapitel 2.3. [Publikation](https://doi.org/10.1016/j.apergo.2025.104709) · [↑ 2.3](#beleg-2-3-handspanne) · [↑ 6.5](#beleg-6-5-ergonomie)

<a name="quelle-chi"></a>**[34]** Ju-Yang Chi, Mark Halaki, Erica Booker, Rhonda Boyle und Bronwen J. Ackermann, „Interaction between hand span and different sizes of keyboards on EMG activity in pianists: An observational study“, *Applied Ergonomics* 97 (2021), Artikel 103518. Die Beobachtungsstudie untersucht das Zusammenspiel von Handspanne und Klaviaturgröße anhand gemessener Muskelaktivität. EtuSchlopfs Spielbarkeitsmodell setzt seine eigenen Bedingungen für die jeweilige Passage. [Publikation](https://doi.org/10.1016/j.apergo.2021.103518) · [↑ 2.3](#beleg-2-3-handspanne) · [↑ 6.5](#beleg-6-5-ergonomie)

<a name="kapitel-12-4"></a>

### 12.4 Forschung zu Musiktranskription und Notensatz

<a name="quelle-rubato"></a>**[35]** Nazif Can Tamer, Victoria Ebert, Guang Yang und Noah A. Smith, *Rubato: Transcribing Piano Music with Timestamps* (Preprint 2026). Das Modell gewinnt Notation mit Aufführungszeitstempeln aus Audio. Diese Darstellung ist ein Vergleich für EtuSchlopfs Zuordnung gespielter Zeitpunkte zu erschlossenen Notenpositionen. [Originalarbeit](https://arxiv.org/abs/2605.24291) · [↑ 3.1](#beleg-3-1-auffuehrungszeit)

<a name="quelle-nakamura-yoshii-dixon"></a>**[36]** Eita Nakamura, Kazuyoshi Yoshii und Simon Dixon, *Note Value Recognition for Piano Transcription Using Markov Random Fields* (IEEE/ACM TASLP 2017). Das Modell schätzt notierte Dauern aus Aufführungsdaten und dem mehrstimmigen Kontext. Damit bietet es einen Vergleich für EtuSchlopfs Entscheidung, welchen Notenwert ein real gespielter Anschlag erhält. [Originalarbeit](https://webspace.eecs.qmul.ac.uk/s.e.dixon/pub/2017/NakamuraYoshiiDixon-TASLP2017.pdf) · [↑ 10.1](#beleg-10-1-dauer) · [↑ 3.2](#beleg-3-2-notenwert)

<a name="quelle-cluster-and-separate"></a>**[37]** Francesco Foscarin und Emmanouil Karystinaios (gleichberechtigte Erstautoren), Eita Nakamura und Gerhard Widmer, *Cluster and Separate: a GNN Approach to Voice and Staff Prediction for Score Engraving* (ISMIR 2024). Für EtuSchlopfs Stimmführung liefert die Arbeit getrennte Kriterien für Akkordgruppen, Stimmfortsetzungen und die Zugehörigkeit zum Notensystem, auch bei systemübergreifenden Stimmen. [Originalarbeit](https://arxiv.org/pdf/2407.21030) · [↑ 10.2](#beleg-10-2-stimmen)

<a name="quelle-engravinggnn"></a>**[38]** Emmanouil Karystinaios und Francesco Foscarin (gleichberechtigte Erstautoren) sowie Gerhard Widmer, *EngravingGNN: A Hybrid Graph Neural Network for End-to-End Piano Score Engraving* (TENOR 2025). Die gemeinsame Vorhersage von Stimmen, Systemen, Hälsen und Schlüsseln liefert einen Vergleich für gekoppelte Satzentscheidungen in EtuSchlopf. [Originalarbeit](https://www.tenor-conference.org/proceedings/2025/06_TENOR2025_Emmanouil_Karystinaios.pdf) · [↑ 10.2](#beleg-10-2-satz)

<a name="quelle-bouquillard-jacquemard"></a>**[39]** Augustin Bouquillard und Florent Jacquemard, *Engraving Oriented Joint Estimation of Pitch Spelling and Local and Global Keys* (TENOR 2024). Die gemeinsame Schätzung von Tonbuchstabierung und Tonarten bietet einen Vergleich für die Wahl enharmonischer Schreibweisen bei wechselndem lokalem Tonartkontext. [Originalarbeit](https://www.tenor-conference.org/proceedings/2024/14_TENOR2024_Bouquillard.pdf) · [↑ 6.9](#beleg-6-9-tonart)

<a name="quelle-beyer-dai"></a>**[40]** Tim Beyer und Angela Dai, *End-to-End Piano Performance-MIDI to Score Conversion with Transformers* (ISMIR 2024). Die direkte Übersetzung einer Klavieraufführung aus MIDI in Notenschrift ist ein möglicher Vergleich für die Gesamtausgabe von EtuSchlopf. [Originalarbeit](https://zenodo.org/record/14877339/files/000033.pdf) · [↑ 10.1](#beleg-10-1-transkription)

<a name="kapitel-12-5"></a>

### 12.5 Arbeiten aus der OpenAI-Mathematiksammlung

<a name="quelle-openai-113"></a>**[41]** OpenAI, *A Fully Polynomial Randomized Approximation Scheme for Perfect Matchings in General Graphs* (23. September 2026, Familie 113). Die Arbeit approximiert die Anzahl perfekter Matchings. Für die Stimmführung ist das Zählen möglicher Zuordnungen an einer Zeitgrenze von der musikalischen Bewertung ganzer Stimmverläufe zu unterscheiden. [Manuskript](https://github.com/openai/math/tree/main/preprints/A-Fully-Polynomial-Randomized-Approximation-Scheme-for-Perfect-Matchings-in-General-Graphs-September-23-2026) · [Formalisierte Aussage](https://github.com/openai/math/blob/main/lean/docs/113.md) · [↑ 10.2](#beleg-10-2-matching)

<a name="quelle-openai-130"></a>**[42]** OpenAI, *Finite tensor savings and exact Fourier circuits* und *An explicit power saving for the exact discrete Fourier transform* (beide 25. September 2026, Familie 130). Die erste Arbeit behandelt exakte Fourier-Schaltungen für eine unbeschränkte Teilfolge von Längen, die zweite ein Verfahren für jede positive Länge mit exakter komplexer Arithmetik. Für EtuSchlopfs WAV-Analyse müssten Laufzeit und numerische Abweichung einer Alternative zu Goertzel an denselben Frequenzen und Fenstern gemessen werden. [Finite tensor savings and exact Fourier circuits](https://github.com/openai/math/tree/main/preprints/Finite-tensor-savings-and-exact-Fourier-circuits-September-25-2026) · [An explicit power saving for the exact discrete Fourier transform](https://github.com/openai/math/tree/main/preprints/An-explicit-power-saving-for-the-exact-discrete-Fourier-transform-September-25-2026) · [Formalisierte Aussagen](https://github.com/openai/math/blob/main/lean/docs/130.md) · [↑ 6.12](#beleg-6-12-fourier)

<a name="kapitel-12-6"></a>

### 12.6 Datensatz und Projektnachweise

<a name="quelle-goebl"></a>**[43]** Werner Goebl, *The Vienna 4×22 Piano Corpus*, Version 1.0 (31. Januar 2017), mdw – Universität für Musik und darstellende Kunst Wien. Zu den Aufnahmen gehören Aufführungs-MIDI und Zuordnungen zur Partitur. Die verwendeten Einspielungen, Bearbeitungen und verfügbaren Angaben zu Interpretation und Annotation sind je Stück nachgewiesen. [Aufnahmen und Zuordnungsdateien](https://repo.mdw.ac.at/projects/IWK/the_vienna_4x22_piano_corpus/) · [Katalog der mdw, DOI 10.21939/4X22](https://datasets.mdw.ac.at/datasets/dataset/98ea25fa-2468-43ff-929b-3c926e163583?view=full) · [Aufnahmen und Mitwirkende](examples/README.md) · [↑ 3.3](#beleg-3-3-korpus)

<a name="quelle-zeit"></a>**[44]** EtuSchlopf, *Gebundene Notendauer und Quellzeit*. Der MIDI-Import bindet die Ausgabe von Fragmenten und Haltefortsetzungen an den ursprünglichen Anschlag. [Fragmentprüfung](../src/midi_seed/bound_duration.rs) · [Zeitlich belegte Oberstimme](selector_derivation_contract_timed_held_upper_voice_implementation.md) · [↑ 3.3](#beleg-3-3-fragment) · [↑ 7](#beleg-7-zeit) · [↑ 4, Nr. 35](#beleg-4-verfahren-35)

<a name="quelle-lokal"></a>**[45]** EtuSchlopf, *Lokale Notationsentscheidungen*. Die Implementierungsverträge legen fest, wie das Programm sichtbare Stimmen, gemessene Pausenintervalle und Notenhälse am Einsatz behandelt. [Lokale Entscheidungen](selector_derivation_contract_final_local_notation_owners.md) · [Pausenintervalle](selector_derivation_contract_final_local_rest_intervals.md) · [↑ 5](#beleg-5-notation)

<a name="quelle-bilanzen"></a>**[46]** EtuSchlopf, *Materialbilanz und sichtbarer Notenerhalt*. Der Bericht zählt Quelltöne, geschriebene Ereignisse und sichtbare Köpfe jeweils in ihrer eigenen Grundgesamtheit. [Bericht im Quellcode](../src/audit_only/material_conservation.rs) · [↑ 6.1](#beleg-6-1-bilanz) · [↑ 4, Nr. 3](#beleg-4-verfahren-3)

<a name="quelle-beams"></a>**[47]** EtuSchlopf, *Ursprüngliche Gegenstimmen und verbundene Primärbalkenpaare*. Die Rücklesung identifiziert die beiden Endnoten jeder Verbindung vor und nach einem Stimmenwechsel. [Verfahrensvertrag](selector_derivation_contract_original_countervoice_exchange_r1960.md) · [Rücklesung](../src/structural_write_authority/primary_beam_pair_readback.rs) · [↑ 6.3](#beleg-6-3-balken)

<a name="quelle-speicher"></a>**[48]** EtuSchlopf, *Speicherabschätzung für den parallelen Korpuslauf*. Die Konstanten für Reserve, Prozess und Worker stammen aus der Projektplanung. [Berechnung und Voraussetzungen](../src/pipeline_shards/corpus_memory_capacity.rs) · [↑ 6.15](#beleg-6-15-speicher) · [↑ 4, Nr. 38](#beleg-4-verfahren-38)
