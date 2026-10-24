# r1478: spaete RestBridge-Schreibung in E048 stillgelegt

Parent: r1477. Class: GovernedChange, music_effect declared. Autor: Dachs.
Freigabe: Wolfgang am 22.10.2026 fuer die vier unten genannten Stellen.

## Ursache

r1476 sperrt in E050 (`apply_structural_rest_bridge_beam_variant_a_render_write`)
vorgeschlagene Pausenbruecken, deren Mitglieder im eingefrorenen Stimmenplan
verschiedene Stimmen haben. Nativ blieb das ohne Wirkung: Der spaete Teil von
`close_structural_beam_gestures` (E048, Trace-Name `external_feedback_closure`) nutzte
dieselbe Kandidatenwahl `choose_structural_rest_bridge_candidate_groups` ohne Sperre und
setzte genau die gesperrten Bruecken wieder.

Beleg (Konsole `LowerRestBridgeCells`): r1473 und r1475 in allen zehn Etueden 0;
r1476 genau 4 (E3 1, E5 2, E6 1), identisch mit den gesperrten Stellen. Der spaete Teil
hatte im angenommenen Korpus also keine eigene Wirkung und wirkte in r1476 nur als
Ausgleich zur Sperre.

## Aenderung

Die Schleife ueber `choose_structural_rest_bridge_candidate_groups` in
`close_structural_beam_gestures_in_segment` und ihre `lower_states` entfallen. E050 bleibt
der einzige Schreiber von Pausenbruecken. Unveraendert: Signatur und Exklusion E048,
obere Balkenschliessung, Quittung, E050 samt Sperre, Kandidatenwahl, alle Gates.
Der Zaehler `LowerRestBridgeCells` bleibt im Bericht und ist ab jetzt immer 0.

Neue Tests (tests/support/bridge_frozen_voice_compatibility.rs): E050 gefolgt von E048
stellt eine gesperrte Bruecke nicht wieder her und laesst eine zugelassene unveraendert;
beides zusaetzlich durch den Verschiebungs-Pruefstand.

## Erwartete Wirkung (aus r1476 abgeleitet, nicht gemessen)

Nur Render-Balken, Ebene 1 und 2, in Richtung Master, an
E3/T6, E5/T36, E5/T37 und E6/T34: das zweite Paarmitglied wird `end`, die
Erweiterungsnote verliert ihren Balken. Masters, Noten, Pausen, Dauern, Stimmen,
Haltebogen und Boegen unveraendert; die uebrigen sieben Render-XMLs bytegleich zu r1477.

## Abnahme

- `LowerRestBridgeCells` in allen zehn Etueden 0.
- XML-Diff nur `<beam>` an den vier Stellen.
- Pruefer fuer Primaergruppen ueber mehrere Stimmen: nur noch E6/T32.
- Lauf mit `--writer-trace`: `external_feedback_closure` ohne Eintrag an diesen Takten.
- Weicht das Ergebnis ab: Befund, kein Nachschieben.
