**Historischer Abschlussplan; teilweise umgesetzt, nicht pauschal erledigt.**
Die folgenden 9/10-Zahlen gehoeren zur damaligen Basis. Die aktuelle Referenz
erzeugt 10/10 PDFs ohne `-f`; Forced-Gewinner-Haertung und RAM-Worker bleiben
separate offene Punkte. Tie-Safety ist seit r1400 implementiert; der spezifische
native Abschlussbeleg wird damit nicht vorweggenommen.

Aktueller Kurzstand: [050_aktueller_stand.md](050_aktueller_stand.md).
Der historische Inhalt bleibt zur Referenz am bisherigen Pfad erhalten.

---

# 042 – Bauauftrag Abschluss: alle PDFs ohne `-f`

Advisory only. Keine Musik-, Writer-, Selector-, Proof- oder Release-Authority.
Eingefroren mit r1396 (KodexOnly). Basis-Evidenz: angenommene r1394/r1395-Exporte
(alle 20 XMLs bytegleich zwischen beiden) und ein nativer MuseScore-4.7.4-Linux-Gegenversuch.
Reale Fundstellen unten sind Evaluation, nie Selektor.

## Stand

- Frischer Korpus 10/10, FinalExit 0. **9 von 10 PDFs ohne `-f`.** Ein Projekt wird mit
  `-f -o` gerendert, gemeldet als `VariantsAttempted 1/11; FailedBeforeSuccess 0`.
- Ursache dieser Meldung (`src/render.rs::try_render_with_candidate`): Der
  RendererFeasibility-Shadow-Oracle (`pipeline_orchestrator_metric_final_notation_stage.rs`)
  ruft `render_pdf_if_possible` auf und wertet auch einen Erfolg mit `-f` als GREEN.
  Danach zieht der prozesslokale Gewinner-Cache `-f` vor. Das finale XML wird nicht mehr
  ungezwungen versucht. Klasse: `selector_derivation_contract_forced_renderer_fallback_precedence.md`.
- Nativer Gegenversuch: Das finale Render-XML dieses Projekts endet ungezwungen mit
  Exit 40 (Load-Fehler), ein Kontrollprojekt ist grün. Der Defekt ist also real.

## Ursache Exit 40 = eingefrorene Klasse r1326

`selector_derivation_contract_partwide_renderer_channel_capacity.md` beschreibt den
Mechanismus vollständig. Eine Nachrechnung der Import-Zuteilung (`allocateStaves`,
`allocateVoices`, `VoiceOverlapDetector`, MuseScore v4.7.4) auf den zehn finalen
Render-XMLs ergibt (Evaluation):

| Befund | Wirkung im Importer |
| --- | --- |
| Ein Kanal wird durch zeitgleiche **Pausen** auf dem anderen Staff partweit „overlapping“. Overlapping-Kanäle werden erst nach allen regulären Kanälen verteilt; alle 4+4 Slots sind dann belegt | Der Hauptkanal erhält keinen Slot; hunderte Chord/Rest-Einträge fehlen; Load-Fehler; ohne `-f` kein PDF |
| In zwei weiteren Projekten bleiben einzelne reguläre Kanäle ohne Slot | **Sichtbare Tonhöhen fehlen still im PDF**, obwohl ohne `-f` gerendert |

Gleichzeitig aktive registrierende Kanäle je Staff und Zeitpunkt: in allen
geprüften Projekten höchstens 2 (Profil: 4 partweit). Kapazität ist also
grundsätzlich vorhanden; das Problem ist die Zuordnung.

**Warnung aus r1326:** Partweit staffgetrennte Labels, Pausen-Label-Trennung oder
Primärpausen-Normalisierung importieren zwar ungezwungen, verloren in der damaligen
Readback-Messung aber 83–188 exakte Vorkommen je Projekt. Eine reine Umbenennung
ist **kein** Reparaturbeweis. Ungezwungener Erfolg muss mit exakter
Materialkontinuität (Tonhöhe/Einsatz/Dauer/Staff, Chord-Host-Grenzen, HARD SAME)
nachgewiesen werden.

## Zusatzgrenze ab r1398: Staff-Erhaltung im nativen Akkordobjekt

Der native Voice-only-Witness ist noch kein vollständiger Erhaltungsbeweis.
Die unabhängige consumerbasierte Klasse
`selector_derivation_contract_chord_scoped_staff_projection.md` beschreibt,
warum ein gemeinsames importiertes Akkordobjekt unterschiedliche gebundene
Staff-Obligationen verlieren kann, obwohl Slots, Tonanzahl und Ties vorhanden
bleiben. MuseScore besitzt die Staff-Bewegung am Akkord und überschreibt sie
bei der Member-Verarbeitung. Synthetische Beispiele ohne Ties oder Oktavierung
belegen diese Grenze unabhängig vom Korpus.

Vor dem Kanalzuordnungs-Take müssen deshalb sowohl die partweite Kapazität als
auch die genaue per-occurrence Staff-Erhaltung nativ belegt sein. Ein grünes
Slot-Profil, PDF-Exit 0 oder allein die klingenden Tonhöhen/Dauern genügt nicht.
Die Staff-Dimension darf nicht aus dem Materialvergleich entfernt werden.
Die bestehende HARD-SAME-/Chord-/Rollen-Authority bleibt bindend; diese neue
Definition erteilt keine Erlaubnis für Chord-Split, Handtransfer oder einen
neuen späten Writer. Die ursprüngliche Voice-only-Portierung ist solange offen,
bis eine vollständige erhaltende Realisierung unter diesen Authorities belegt
ist. Die folgende Reihenfolge benennt Arbeitsziele, keine Zusage ihrer Anzahl.

## Empfohlene Take-Reihenfolge (je ein Take, jede Klasse ist jetzt eingefroren)

1. **Kanalzuordnung (MusicMutation, Abschluss-kritisch).** Implementierungsvertrag mit
   `ParentErrorClassContract: docs/selector_derivation_contract_partwide_renderer_channel_capacity.md`,
   `SelectorRole: repair_scope`. Ort: bestehende Serialisierungs-Authority
   (`structural_write_authority/serialization_plan.rs`: `FrozenSerializedVoicePlan`,
   `bind_musicxml_voice_labels`, `label_for_source`). Kanäle aus vollständiger
   kanonischer Written-/Rollen-/Pausen-/Cursor-Evidenz ableiten. Zuordnung partweit
   profilzulässig. Vorhandenen Overlapping-Modus und Pausenlabels zunächst erhalten;
   ihre Änderung verlangt eine eigene vollständige Authority-/Staff-Erhaltungsprüfung.
   Fail closed bei fehlender Provenienz. Readback: Profil-Nachrechnung ohne
   unzugeteilte registrierende Kanäle **und** exakte Materialkontinuität.
2. **Tie-Grenze an Oktavierung (MusicMutation).** Inhalt des abgelehnten r1362
   (Ablehnung nur `reexec_exit` Exit 1 vor der Bootstrap-Stabilisierung, nicht
   inhaltlich) auf den dann gültigen Parent übertragen, mit Implementierungsvertrag auf
   `selector_derivation_contract_octave_display_boundary_incoming_tie.md`. Der enge Safety-Filter wird mit r1400 vorgezogen: unsichere ganze Registerpläne
   werden vor ihrer vorhandenen Materialisierung verweigert. Dies löst keine
   Kanalzuordnung; die tatsächliche native Abschlussabnahme bleibt nach 1 nötig.
3. **Renderer-Wahrheit (Härtung).** Implementierungsvertrag auf
   `selector_derivation_contract_forced_renderer_fallback_precedence.md`: Forced-Gewinner
   nie vorziehen; je PDF und Shadow-Render `Forced`, `UnforcedAttempted` und die erste
   ungezwungene Fehlerevidenz; Manifest `UnforcedPdfs x/N`. Entscheidung des Oracles erst
   in einem eigenen Schritt ändern.
4. **RAM-gesteuerte Default-Worker.** Implementierungsvertrag auf
   `selector_derivation_contract_runtime_worker_memory_capacity.md`. Windows-Messung ohne
   unsafe (z. B. read-only CIM), Linux `/proc/meminfo`, sonst `unknown`. Override vorrangig;
   RAM begrenzt nur Nebenläufigkeit, nie die Musikauswahl.

Reihenfolge 3/4 ist frei. Für den Vertragsabschluss nötig sind 1 und 2.

## Abnahme-Readback (ab r1397)

Jeder Result-Bericht enthaelt `IMPORT-CHANNEL-PROFILE-READBACK` mit
`ImportChannelProfileReadback: Source render; ... ProfileStatus ...`. Der
Kanalzuordnungs-Take muss dort fuer alle Projekte `UnreservedChannels none`,
`LostPitchedOccurrences 0` und `ProfileStatus admissible` zeigen. Das ist
notwendig, nicht hinreichend: Materialkontinuitaet bleibt separat zu belegen.

## Abnahme Abschluss

Frischer Korpus 10/10, FinalExit 0; **jede PDF `Aufruf: -o <pdf> <input>`**; Profil-
Readback ohne Verlust; Ties an Oktavierungsgrenzen nativ erhalten; klingende
Einsätze/Tonhöhen/Dauern taktweise unverändert.

## SSC-Abbruch: Grenze der Ursachenbehauptung

Ein abgelehnter Take mit unveraendertem Rust und gruenen Admissions ist noch
kein Beweis fuer einen Inhaltsfehler, einen Build-Absturz oder eine bestimmte
Optimierungsursache. Phase und Originalbytes zuerst hashgebunden pruefen.
Die externe Replay-Untersuchung ist in `docs/ssc_failure_replay_lessons_r1399.md`
eingeordnet. Ein nicht reproduzierbarer Denied-Witness erlaubt weder eine
Envelope-Aufweitung noch das Ueberspringen der urspruenglichen Adoption.

Kanalreparatur muss Kapazitaet und tatsaechliche Staff-Abbildung gemeinsam
belegen. Eine fuer die gebundene Staff-Erhaltung erforderliche vorhandene
Overlap-Aufloesung darf nicht blind entfernt werden; keine Heads oder Pausen
erfinden. Rollen, HARD SAME und vollständige Rest-/Cursor-Provenienz bleiben
unabhaengige Voraussetzungen.

## Arbeitsübergabe ab r1400

`docs/channel_assignment_handoff_r1400.md` bündelt die konkrete Owner-Lesereihenfolge,
die voneinander unabhängigen Kapazitäts- und Staff-Pflichten und die Grenzen der
externen Gegenprobe mit unveränderten Pausenlabels. Sie ist advisory only und
ändert keine Abnahmebedingung. Ein Staff-roter Witness bleibt rot, auch wenn seine
Pausen unverändert sind und alle Tonhöhen gezählt werden können. Die Herkunft der
Abweichung muss von der Frage nach einer zulässigen Reparatur getrennt bleiben.
