# CrossStaffLowerDualEdgeBeamPlanAdoption

## r03 – Strict-Clippy-Performance-Closure

Der r02-Gate-Lauf wurde vor jeder Installation durch `clippy::manual_contains`
geschlossen. Die BeamPlan-Eingangspruefung auf ungueltige Balkenebene 0 nutzt
nun direkt `beam_levels.contains(&0)` statt einer semantisch identischen
`iter().any(...)`-Suche. Es aendern sich weder Annahme/Ablehnung noch erzeugte
Edits, Hooks, Serialisierung, Readback oder produktives Schreibrecht.

## r02 – Strict-Clippy-CrossFile-Closure

Der r01-Gate-Lauf wurde vor jeder Installation durch einen einzelnen
Compilerfehler geschlossen. Nach der Umstellung von
`lower_dual_edge_beat_beam_states` auf `BTreeMap<u8, String>` verwendete der
read-only `BeatAlignedCrossStaffTail`-Referenzdetektor fuer seine lokalen
Erwartungspaare noch `usize`. r02 typisiert dort ausschliesslich die
Balkenlevel als `u8`; Werte, Zustandsvergleich und Klassifikationssemantik
bleiben identisch. Es entsteht keine produktive Mutation und kein neues
Schreibrecht.

## Ausgangspunkt

Der verbliebene produktive Cross-Staff-/Lower-Dual-Edge-Komplex besass zwei
parallele Balkenzielmodelle innerhalb derselben Render-Schicht:

- `LocalVoiceRewriteBatch` plante pro XML-Note verschachtelte
  `BTreeMap<u8, String>`-Zielzustaende einschliesslich Forward-/Backward-Hooks.
- `SingleVoiceCrossStaffLaneFold` erzeugte die Balkenzustaende jeder
  zusammenhaengenden Beat-Gruppe erneut als lokale Stringmap.
- Die Lower-Dual-Edge-Doppelbruchreparatur beschrieb Vorher-, Schreib- und
  Readbackzustand durch getrennte Stringarrays.

Der historische `BeatAlignedCrossStaffTail`-Direktwriter ist bereits
pensioniert und bleibt read-only Referenzdetektor; er ist kein produktiver
Writer dieses Takes.

## Aenderung

`BeamState` bildet nun zusaetzlich `ForwardHook` und `BackwardHook` typisiert
ab. `BeamPlan::from_event_levels_with_continuity()` erzeugt aus Balkentiefe und
Nachbarschaftskontinuitaet genau einen vollständigen Plan:

- primaerer Balken ueber die komplette geordnete Ereignisfolge,
- hoehere Ebenen als zusammenhaengende Laeufe,
- einzelne hoehere Ereignisse als vorwaerts- oder rueckwaertsgerichtete Hooks,
- fail-closed bei weniger als zwei Ereignissen, Nulltiefe oder unpassender
  Kontinuitaetslaenge.

`LocalVoiceBatchPlan` traegt diesen Plan einmal. Seine Knotenpatches speichern
nur noch das zugehoerige Ereignisordinal. Schreiben und Beam-only-Delta lesen
die Zielzustaende aus demselben Plan; die lokale Ziel-Stringmap entfaellt.

`SingleVoiceCrossStaffLaneFold` bildet jede zusammenhaengende Beat-Gruppe ueber
`BeamPlan::from_complete_levels()`. Auch nicht lueckenlose vorhandene
Balkenlevelmengen bleiben erhalten; Carrier-Serialisierung und Readback erhalten
die Zustaende aus dem Plan statt aus einem lokalen `state`-String.

Die Lower-Dual-Edge-Reparatur erzeugt einen typisierten Vorherplan und einen
typisierten Zielplan. Kandidatenpruefung, die zwei `continue`-Writes und der
exakte Schlussreadback verwenden diese beiden Planobjekte statt drei getrennten
Stringsequenzen.

## Nicht geaendert

- keine Kandidaten-, Staff-, Voice-, Beat- oder Taktregel,
- keine Voice-/Lane-Umsortierung,
- keine Rest-, Forward-, Backup- oder Cursorentscheidung,
- keine Handspannensperre und keine Rollenuebertragung,
- kein XML-Writer, Checkpoint-Sink oder Pipeline-Order,
- keine Pitch-, Onset-, Dauer-, Staff-, Voice-, Pausen- oder Notenzahlaenderung,
- keine neue Lintunterdrueckung, Audit- oder History-Schicht.

Die weiterhin vorhandenen Stringmaps im pensionierten
`DistinctSourceLaneVoiceSplitTransactionalApply` gehoeren zu einem explizit
deaktivierten Voice-/Lane-Transaktionspfad und werden nicht als produktive
Balkenautoritaet dieses Takes umetikettiert.

## Statische Aequivalenz

- 200.000 zufaellige Local-Voice-Gruppen mit zwei bis zwanzig Ereignissen,
  Balkentiefe eins bis vier und beliebigen Kontinuitaetsunterbrechungen:
  identische Level, `begin/continue/end` und Hook-Zustaende.
- 200.000 zufaellige Lane-Fold-Gruppen mit einem bis zwoelf Ereignissen und
  beliebigen vorhandenen Balkenlevelmengen: identische Zielzustaende.
- 200.000 korrekte und gestoerte Lower-Dual-Edge-Readbacks: identische
  Vorher-/Nachher-Annahme oder -Ablehnung.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Bis zur realen gruenen Abnahme bleibt v0.10.1604 autoritativ.
