# Was EtuSchlopf tatsächlich implementiert

Stand: angenommene r1526-Quelle plus öffentliche Erläuterungen, 26.10.2026.
Diese Beschreibung ist ein Wegweiser zum Code. Sie ersetzt keinen Contract,
keine fachliche Authority und keinen Laufbeleg.

## Ein Musikprogramm und ein kontrollierter Entwicklungsprozess

EtuSchlopf verarbeitet musikalisches Projektmaterial zu einer Klavierfassung.
Der reguläre Launcher-Einstieg verbindet gleichnamige WAV-/MIDI-Dateien; die
Ausgaben sind musikalischer Master, Render-MusicXML, PDF und Ergebnisbericht.
Die private Zehner-Evaluation ist kein öffentlich mitgelieferter Datensatz.

Die zweite Ebene betrifft die Entwicklung dieses Programms mit KI. Ein Vorschlag
muss seine Ursache, Eingabebindung, Zuständigkeit, erlaubte Veränderung und
Prüfung offenlegen. Eine Etüde kann einen Fehler zeigen; ihre Identität soll die
allgemeine Regel nicht ersetzen. Architekturwissen liegt im Repository, damit
ein neuer Bearbeiter auch ohne frühere Gespräche einsteigen kann.

Der Ablauf ist konkret: Fehlerklasse vor Auswahlregel, synthetische Gegenfälle,
deklarierter Quellbaum, Prüfung gegen den unveränderten Parent, Build, Strict
Clippy, Tests, realer Lauf und Artefakt-Readback. Diese Kontrollen erschweren
bekannte Umgehungen. Ihre Reichweite hängt von den implementierten Prädikaten,
Modellen und Tests ab; sie beweisen keine Fehlerfreiheit beliebiger Programme.

Einstieg: [Kaltstart](assistant_memory/000_start_here.md),
[Essenz](assistant_memory/020_architecture_essence_v0.md),
[Selektor-Vertrag](selector_derivation_contract_template.md),
[Take-Prüfer](../src/take_change_control.rs).

## SchlopfFormel: vom Leitgedanken zu ausführbaren Bedingungen

Die SchlopfFormel gehört zur gemeinsam entwickelten mathematischen Sicht auf
zulässige Fortsetzung: Eine Veränderung braucht gebundene Herkunft, einen
zuständigen Ausführer und Nachweise für den konkreten Übergang. Hier wird dieser
Projektbegriff anhand der nachweisbaren operativen Bestandteile erläutert.

Eine einzelne kanonische Gesamtgleichung unter diesem Namen ist im geprüften
r1520-Quellbaum nicht hinterlegt. Die folgende Notation ist deshalb eine jetzt
dokumentierte Zusammenfassung vorhandener Implementierung, kein wörtliches Zitat
einer früheren Formel und kein zusätzlicher Beweis.

### Konkreter Teilkern: SSC an der Übernahmegrenze

Für ein originales Receipt `e` lässt sich die aktuelle A7-Zulassung so lesen:

$$
\mathrm{A7Permit}(e) =
[p(e)=1]\;\land\;[u(e)+d(e)+i(e)=0]\;\land\;\neg\mathrm{Replay}(e).
$$

`p` ist der Permit-Zähler der Klassifikation dieses einen Receipts, keine
Wahrscheinlichkeit. `u`, `d` und `i` zählen fehlende, doppelte und ungültige
Evidenz. Ein bereits übernommenes Receipt wird zuvor als Replay abgewiesen.
Genau diese Bedingungen stehen in `ssc_adoption_verdict` und `adoption_decision`
in [ssc_evidence_proof.rs](../src/ssc_evidence_proof.rs).

Diese Teilbedingung allein definiert noch nicht die gesamte Übernahme. Dazu
kommen unter anderem die vollständige, kontinuierliche Receipt-Abdeckung und die
Bindung an den tatsächlichen Vor-/Nachzustand. Master und Render führen ihre
Herkunftsketten getrennt; die Zustandsübernahme bindet Dokument und Lineage im
gleichen Schritt. Fehlende Herkunft wird nicht durch einen Bericht oder durch
neu konstruierte Ersatz-Receipts ersetzt.

Die produktiven Anschlussstellen stehen in
[Master-Receipt-Commit](../src/master_state_identity.rs),
[Render-Lineage](../src/master_state_identity/render_lineage.rs) und
[Master-Adoption](../src/structural_write_authority/master_adoption/adopted_state.rs).
Die letzte Instanz ist der vorhandene [StructuralWriteAuthority-Bereich](../src/structural_write_authority/).

`N_A` ist im SSC-Abschluss eine konkrete Restmenge von Legacy-Abhängigkeiten.
Der aktuelle [Abschlussbeleg](../src/build_gate/ssc_exit_closure_v1.txt) verlangt
unter anderem eine leere Restmenge und aktive verdrahtete A7-Grenzen. Das noch
vorhandene alte `exit_claim=not_closed`-Schema-Token wird dort ausdrücklich
eingeordnet; es darf nicht isoliert als aktueller Runtime-Status gelesen werden.
Eine leere Restmenge bedeutet nicht, dass sämtliche musikalischen Fragen gelöst
sind. [Der v4-Abschluss](ssc_contract_v4_closure_r1288.md) erklärt die Reichweite.

## Weitere mathematische Bestandteile

| Bestandteil | Tatsächliche Umsetzung | Geltungsbereich |
| --- | --- | --- |
| Exakte Zeitordnung | Teilungen, ggT/kgV und kanonische Zeitachsen; metrische Zerlegung stiller Intervalle | Diskrete Ereigniszeiten und Notationsdauern; keine Aussage über menschliches Timing |
| Registeroptimierung | Sichere zusammenhängende Fenster; bestehende quadratische Registerkosten, Bezeichnungsstrafe und deterministische Gleichstandsregeln | Begründete Auswahl einer Darstellung; kein allgemeines Maß musikalischer Schönheit |
| Erhaltende gemeinsame Spanne | Vergleich mit vollständigem eingefrorenem Basisplan, Pflichtabdeckung, echten Ankern und Überbindungsprüfung | r1512; konservative Ersetzung innerhalb eines geschlossenen Konfliktbereichs |
| Harmonik-Kandidaten | Kostenvektoren, Pareto-Vergleiche, lokale Kandidatengraphen und begrenzte Suche | Vorschläge werden weiterhin durch vorhandene Owner und Kandidatenprüfung begrenzt |
| Symbolische Spielanforderung | Existenzielle Belegungen physischer Verpflichtungen auf zehn Hand-/Finger-Slots und abgeleitete statische, Übergangs- und Wiederanschlagbedingungen | Audit und modellrelative Erreichbarkeit; keine fertig gewählte Fingersetzung |
| Provenienz und Kontinuität | Gebundene Identitäten, exakte Übergangsspuren, Mengen-/Bereichsprüfungen und Receipt-Ketten | Erklärbarkeit und Zulässigkeit definierter Änderungen; keine empirische Pianistenfähigkeit |

Die wichtigsten Code-Einstiege:

- [Metrische Pausenplanung](../src/model/metric_rest_notation.rs)
- [Register-Owner und Zeitachse](../src/registernotation/mod.rs)
- [Geschlossener Spannenvergleich](../src/registernotation/gemeinsame_spanne.rs)
- [Lokales Harmonik-Portfolio und Pareto-Vergleiche](../src/pipeline_shards/sheaf_noether_projection_core.rs),
  aufgerufen im [Notation-Setup](../src/pipeline_shards/pipeline_orchestrator_project_setup_stages.rs)
- [Symbolische Fähigkeits-Erreichbarkeit](../src/audit_only/pianistic_symbolic_capability_reachability.rs)
- [Originale rhythmische Repräsentationstransporte](../src/rhythmic_representation_transport.rs)

Dateinamen wie `sheaf` oder `noether` sind dabei keine unabhängigen Belege für
einen bewiesenen mathematischen Satz. Belegbar sind die konkreten Operationen,
ihre Einbindung und ihre getesteten Invarianten.

### Beispiel: die ausführbare symbolische Spielformel

Der symbolische Audit formuliert im Code ausdrücklich:

```text
Exists(slot_variables): StructuralConstraints
AND ValuationDerivedStaticAtoms
AND ValuationDerivedTransitionAtoms
AND ValuationDerivedSameKeyAtoms
```

Variablen repräsentieren physische Verpflichtungskomponenten; der Wertebereich
enthält zehn Hand-/Finger-Slots. Die aus einer Belegung abgeleiteten Anforderungen
werden von den Antworten eines Fähigkeitsmodells getrennt. Ein nicht modellierter
Fähigkeitswert wird nicht zu einer positiven Fähigkeit umgedeutet.

Das vorhandene [Referenzmodell](../src/audit_only/pianistic_reference_assumption_model.rs)
deklariert seinen Status ausdrücklich als `MODEL_RELATIVE_ONLY`. Es verwendet
vereinfachte statische Annahmen und entspannt die minimale Übergangszeit auf null.
Sein Ergebnis liefert daher keinen Beweis für das Spieltempo oder die Spielbarkeit
durch einen konkreten Menschen. Forschung zu Hand- und Klaviaturgröße motiviert
die Modelltrennung; sie kalibriert diese Annahmen nicht automatisch.

## Was die Registeränderung r1512 gezeigt hat

r1512 reproduzierte zunächst sämtliche zehn bisherigen Plan-Hashes aus den echten
Eingangsaufnahmen. Danach wurden 16 bessere gemeinsame Alternativen in sieben
Stücken ausgewählt und im nativen Lauf bestätigt. Ein synthetischer Gegenfall
hatte vorher gezeigt, dass pauschal abgeschwächte Sprunggrenzen eine zweite hohe
Insel ohne Oktavierung zurücklassen können.

Die tatsächlichen Ausgaben behalten Noten, Rhythmus, Stimmen, Balken und Bindungen.
Neben den Oktavrichtungen ändern sich bei Etüde 4 automatisch abgeleitete
Systemabstände. Die 50 PDF-Seiten bleiben erhalten; 16 Seiten zeigen Änderungen.
Bei Etüde 4 sinkt die maximale Hilfslinienzahl von fünf auf drei. Unverwandte
Probleme im unteren System bleiben offen.

Das ist ein überprüfter Fortschritt eines konkreten Arbeitsstands, kein allgemeiner
Benchmark-Sieg und kein Beweis, dass der gesamte Notensatz fertig ist. Die
[öffentliche Belegzusammenfassung](evidence/r1512/README.md) trennt Laufbelege,
redaktionelle Auswertung und die Grenzen öffentlicher Reproduzierbarkeit.

## Oktavfortsetzungen in r1520

r1520 entfernt automatisch erzeugte redundante Oktavtexte nur bei gebundener
Herkunft, tatsächlich ausgeführtem Receipt und übernommener finaler Fassung.
Der PDF-Readback prüft die aktuelle Linie und das zugehörige Oktavzeichen in
ihrem aktuellen System. Vertikale Neuverteilung durch den Renderer darf diese
Lesbarkeit erhalten; unveränderte absolute Seitenhöhen sind dafür keine geeignete
Bedingung. Fehlende oder mehrdeutige Beobachtung erlaubt keine Entfernung.

Der native Lauf hat r1520 angenommen: 2.511 Tests bestanden, Build und Clippy
erfolgreich, zehn Projekte vollständig. Gegenüber r1518 bleiben alle zehn
Master-MusicXML-Dateien bytegleich. In fünf Render-Dateien fehlen ausschließlich
neun erzeugte Wort-Wrapper; alle übrigen XML-Bytes sind erhalten. Die PDF-Ausgaben
behalten zusammen 50 Seiten. Diese Auswertung enthält keine erneute visuelle
Einzelprüfung aller Seiten und keine allgemeine Spielbarkeitsaussage.

Einstiege: [Reader-Policy](../src/registernotation/reader_policy.rs),
[PDF-Beobachtung](../src/audit_only/renderer_ottava_context.rs),
[Begründung des vertikalen Nachweises](ottava_vertical_reflow_proof_r1520.md) und
[öffentliche Abnahmebelege](evidence/r1520/README.md).

## Bedien- und Exportstand r1521

r1521 dokumentiert sämtliche vorhandenen Launcher- und Programmoptionen samt
Aliasen, Standardwerten und der lokalen Watch-GUI. Der fest eingebaute private
Transferpfad entfällt: Einzelprojekt- und Korpusexport legen Machine-Evidence
unter `target/etuschlopf_neuro_evidence/` im jeweiligen Projektordner ab.
Container, Payload, Commit-Vertrag und Retention bleiben erhalten; alte externe
Dateien werden weder verschoben noch gelöscht.

Der native Windows-Lauf hat r1521 angenommen: 2.513 Tests bestanden,
0 fehlgeschlagen, 1 ignoriert; Build und Strict Clippy erfolgreich; privater
Korpus 10/10. Alle 20 Master-/Render-XML-Dateien sind gegenüber dem angenommenen
r1520 bytegleich. Für die erneut erzeugten PDFs wird hier keine Pixelgleichheit
behauptet. Die [Abnahmebelege](evidence/r1521/README.md) und die
[Änderungsbeschreibung](project_local_evidence_r1521.md) halten den Umfang fest.

## Zeitkontext und Ergebnisschutz r1522–r1526

Die untere Schlüsselplanung bewertet Notenköpfe, wirksame Schlüssel, Zeitspannen
und Angriffe nun in musikalischer Zeit. XML-Bytepositionen bleiben gebundene
Editierstellen. Eine später serialisierte Stimme kann einen früher wirksamen
Schlüssel oder eine gleichzeitig gehaltene Überbindung enthalten. Die gemeinsame
Zeitbeobachtung berücksichtigt diese Beziehungen; r1524 ordnet getrennte,
nicht überlappende Ursprungsbeobachtungen nach bewiesener musikalischer Zeit.
Kostengewichte, bestehende Schreibpfade und Schutzgrenzen bleiben erhalten.
Ungeklärte Zeit- oder Herkunftsevidenz erlaubt keinen zusätzlichen Plan.
Eine gemeinsame Schlüssel-/Oktavoptimierung wird damit nicht behauptet.

r1525 schützt vorhandene Ergebnisse vor der Root-Bereinigung. r1526 erhält die
bestehende Vierdateien-Transaktion für reguläre Einzelaufrufe mit unterstützten
Optionen. Batch, direkte Programmaufrufe und abweichende Ausgaben bleiben außerhalb
dieses Sicherungsumfangs. Die Stage-A-Eingabedefinition bereitet einen späteren
Runtime-Schnitt vor; sie implementiert ihn noch nicht.

Der native r1526-Lauf ist angenommen: 2.538 Tests bestanden, 0 fehlgeschlagen,
1 ignoriert; Build und Strict Clippy erfolgreich; privater Korpus 10/10.
Gegen r1525 sind alle 20 XML-Dateien bytegleich und 50 PDF-Seiten bei 96 dpi
pixelgleich. Diese Gleichheit betrifft r1526 gegen r1525, nicht sämtliche
musikalischen Änderungen seit dem vorherigen öffentlichen r1521-Stand.
[Abnahme und Grenzen](evidence/r1526/README.md).

Einstiege: [Zeitbeobachtung](../src/clef_readability_observation/temporal.rs),
[Schlüssel-Owner](../src/structural_write_authority/rustig_clef_planning.rs),
[Ursprungsordnung](staff_origin_musical_order_closure_r1524.md),
[Root-Schutz](root_result_preservation_r1525.md) und
[Optionsschutz](standard_output_option_protection_r1526.md).

## Forschung und Beweisanspruch

Die [Forschungsbezüge](RESEARCH_CONTEXT.md) unterscheiden eine tatsächlich
integrierte Modellgrundlage von Literatur, die im Code als Kontext genannt wird.
Die SchlopfFormel, Owner-Verträge und Gates sind projektspezifische Konstruktionen.
Es wird kein extern begutachteter Neuheits- oder Vollständigkeitsnachweis für sie
behauptet. Eine persönliche Schätzung der Entwicklungsstunden ist keine gemessene
Projektkennzahl und kein Nachweis mathematischer oder musikalischer Qualität.
