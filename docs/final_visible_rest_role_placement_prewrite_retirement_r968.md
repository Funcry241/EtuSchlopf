# r968 – FinalVisibleRestRolePlacementPrewriteRetirement

Parent: `v0.10.1839 r967 ExactMoveExpectedTransitionExperimentRetirement`

ChangeClass: `RefactorOnly`

## Ausgangslage

Die AIC-1-Auswertung des r967-Snapshots zeigte im Rest-/Visible-Lane-Slice zunaechst mehrere scheinbar konkurrierende spaete Restpfade. Die Source-Pruefung trennt davon zwei Faelle klar ab: `StrictStaffLocalRelease` arbeitet auf Lanes mit sichtbarem Pitch, waehrend `SilentVoiceVisibleMeasureCoverage` pitchlose Silent-Lanes behandelt. Diese beiden Pfade sind damit keine Doppel-Authority und werden in r968 nicht zusammengelegt.

Ein engerer Re-entry liegt in `apply_final_visible_rest_role_purge_and_vertical_placement_write`: Der bestehende Role-Classifier entscheidet kanonisch zwischen echter sichtbarer Handpause, unabhaengiger Gegenstimme, geschuetztem MixedReview und Auxiliary-Shadow-Bookkeeping. In drei Keep-Zweigen normalisiert derselbe fruehe Pfad zusaetzlich bereits `display-step`/`display-octave`.

Spaeter existiert jedoch bereits der gemeinsame kanonische Rest-Glyph-Konvergenzpunkt `build_piano_final_rest_glyph_canonical_height_write`. Dieser ist weiterhin erforderlich, weil nach dem fruehen Role-Pfad weitere Rest-Materializer neue sichtbare `<rest/>`-Fragmente ohne vertikale Glyph-Koordinaten erzeugen. Die fruehe Placement-Arbeit ist deshalb keine notwendige Authority, sondern eine vorgezogene Teilwiederholung derselben spaeteren Glyph-Normalisierung.

## r968-Cut

r968 entfernt ausschliesslich die vorgezogene vertikale Pausenplatzierung aus dem FinalVisibleRestRole-Pfad:

- keine `display-step`/`display-octave`-Normalisierung mehr in den drei Keep-Zweigen;
- kein lokaler `placement_normalized`-Zaehler;
- kein `rest_placement_normalized_rows`-Auditfeld;
- keine `*_VISUAL_ENVELOPE_PLACEMENT`-Auditzeilen;
- keine `RestPlacementNormalized`-Projektion im Done-/Pipeline-Text.

Bewusst erhalten bleiben:

- die atomare Rollenklassifikation;
- Rest->Forward fuer Auxiliary-Shadow-Bookkeeping;
- die bestehende Overlap-/`occupied`-Sicherung dieses Rewrite-Pfads;
- `apply_final_visible_rest_role_rewrite_authorized`;
- `build_piano_final_rest_glyph_canonical_height_write` als gemeinsamer spaeter Placement-Owner;
- dessen eigene `occupied`-Sicherung;
- der nachgelagerte final-clef-gebundene Rest-Rebind;
- StrictStaff- und SilentCoverage-Planung samt ihren disjunkten Kandidatendomänen.

Es entsteht keine neue Authority, keine neue Transition-, Receipt-, Registry-, Ledger- oder Proof-Schicht.

## Erwartete Bilanz

Produktive Rust-Bilanz gegen r967:

```text
src/pipeline_shards/staff_voice_rest_glyph_topology_tail.rs   -44
-----------------------------------------------------------------
Netto                                                         -44 Rust-Zeilen
```

## Erfolgskriterium

Der Take ist nur erfolgreich, wenn die vorhandenen r967-Invarianten und der Vollkorpus erhalten bleiben. Insbesondere duerfen Rollenentscheidung, Shadow-Rest-Purge, sichtbare/hidden Pitch-Bilanzen, Backup/Forward-Konsistenz, Parsebarkeit und finale Artefakte nicht regressieren.

Ein unerwartetes Musik- oder Kandidatendelta ist kein Anlass fuer eine Folge-Reparaturschicht. In diesem Fall wird r968 verworfen und die Annahme der redundanten Vorab-Platzierung erneut bewertet.
