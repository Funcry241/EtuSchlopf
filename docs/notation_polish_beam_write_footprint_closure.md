# NotationPolishBeamWriteFootprintClosure

## Basis

Autoritative Basis ist `v0.10.1776 r02 HigherBeamRecoveryAuthorityRetirement`, Vollkorpus 10/10 dunkelgruen. Die produktive HigherBeam-Recovery ist pensioniert; der historische HigherBeam-Schadensselektor bleibt nur noch als committed-state Regression-Gate.

## Forensischer Befund

`notation::polish_measure()` besitzt einen lokalen metrischen `BeamPlan`, materialisierte bisher aber ein wesentlich groesseres physisches Write-Footprint: Fuer jede Note des Takts wurden vorhandene `<beam>`-Tags entfernt. Nur Noten mit positivem lokalen BeamPlan bekamen anschliessend Beam-Tags zurueck. Damit bedeutete "kein positiver Plan" implizit einen destruktiven negativen Write auf fremde Beam-Wahrheit.

Die read-only Projektion auf dem autoritativen 1776-Korpus zeigt die Groesse dieses Effekts. Wird die alte Politur erneut auf die zehn finalen Master-Artefakte projiziert, entstehen 245 note-lokale Beam-Deltas: 215 beamtragende Noten in lokal blockierten Gruppen werden entbalkt, 20 beamtragende Singletons werden entbalkt und 10 positiv geplante Noten werden kanonisch neu geschrieben. Nach dem unmittelbar folgenden Partial-/Beat-Scoped-Beam-Owner bleiben von der alten destruktiven Politur noch sechs Deltas uebrig.

Wird dagegen dieselbe lokale Planung nur auf ihre positiven `BeamPlan`-Noten materialisiert und jede andere Note byteidentisch bewahrt, gilt in derselben read-only Projektion fuer alle zehn Master- und alle zehn Render-Artefakte:

`B o P_scoped = B`

mit `B = PartialBeatPostRest + BeatScopedBeaming`. In der Korpusprojektion bleiben 0 residuale Beam-Deltas nach diesem unmittelbaren Owner.

## Architekturregel

Ein fehlender positiver Plan ist kein Write-Recht.

Fuer `NotationPolish` gilt ab diesem Take:

`WriteSet(Polish) subseteq PositiveBeamPlanDecisionScope(Polish)`

Noten ausserhalb dieses Scopes werden von der Beam-Materialisierung byteidentisch uebernommen. Das betrifft ausschliesslich Beam-Tags. Pitch, Onset, Dauer, Staff, Voice, Chord-/Reststruktur und Cursor bleiben unberuehrt.

## Produktive Aenderung

In `polish_measure()` wird `strip_beam_tags()` nur noch fuer Noten aufgerufen, die in `beam_state_by_note` einen positiven typisierten Plan besitzen. Nicht geplante Noten werden unveraendert kopiert. `removed_beams` zaehlt dadurch nur noch tatsaechlich im eigenen Decision-Scope entfernte Beam-Tags.

Die Gruppierung, `is_safe_beam_group`, Clef-Segmentierung, PartialBeatPostRest-Erkennung, `BeamPlan::from_level_groups`, Layoutsignale `beam_groups`/`beamed_notes`, Passreihenfolge und alle spaeteren Beam-Owner bleiben unveraendert.

## Tests

Ein synthetischer Test bindet eine positiv geplante metrische Gruppe und daneben eine bereits bebeamte, lokal nicht sichere Gruppe. Die geplante Gruppe wird kanonisch geschrieben; die nicht geplanten Noten muessen ihre vorhandenen Beam-Tags byteinhaltlich behalten.

Ein zweiter Test fordert unmittelbare Idempotenz der scoped Politur.

## ChangeClass

`MusicMutation`.

Der Wurf veraendert bewusst produktive Beam-Mutationssemantik und wird daher nicht als RefactorOnly deklariert, obwohl die Korpusprojektion eine starke Absorption durch den nachfolgenden Owner zeigt. Ein realer Zielsystemlauf entscheidet ueber die finale Output-Invarianz.

## Abnahme

Historisch und strukturell geeigneter erster Stressfall:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Bei gruenem Lauf folgt ohne Zwischentake:

```powershell
.\run.cmd --strict --all
```

Zu vergleichen gegen 1776:

- Master-/Render-MusicXML semantisch bzw. beam-lokal,
- `FinalBeamLostBegin` Candidates/Writes,
- `FinalCompleteBeatPrimaryBeamContinuity` Writes,
- `PartialBeatPostRestTripleBeamFinalRepair`,
- committed-state Gates,
- 57 PDF-Seiten pixelweise.

Ziel ist nicht eine bestimmte Repair-Zahl. Jeder verbleibende oder neu sichtbare Beam-Delta braucht eine eigene Owner-Erklaerung.
