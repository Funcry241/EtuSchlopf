# ProductCodeSmallMechanicalCleanup

## Ziel

Reiner Hygiene-Take nach dem grueneren `v0.10.1743 r01`-Pausenarchitekturstand.
Es wird keine Musik-, Writer-, PDF-, Release-, Launcher-, Watch-, Manifest- oder
Recovery-Semantik geaendert.

## Aenderungen

- `render::rest_placement`: private Slot-Varianten werden von
  `MiddleLine/UpperLine/TopLine` auf `Middle/Upper/Top` gekuerzt. Die
  staff-relative Slot-Policy und alle MusicXML-Anker bleiben byte-semantisch gleich.
- `rustig_clef_planning`: eine Clippy-gemeldete `filter_map(... bool::then ...)`-Form
  wird in die aequivalente `filter(...).map(...)`-Form ueberfuehrt.
- Initial-Render- und Release-Shards: drei sofort dereferenzierte Doppel-Referenzen
  werden entfernt.

## Erwarteter Effekt

Die fuenf zuletzt gemeldeten Small-Mechanical-Warnungen des Hauptbinaries werden
beseitigt. Die sieben verbleibenden Warnungen liegen ausschliesslich in Build-/Bootstrap-/
Launcher-Sicherheitsoberflaechen und werden in diesem Take absichtlich nicht beruehrt,
damit keine Pin-/Trust-Anchor-Kette fuer reine Kosmetik geoeffnet wird.

## Invarianten

- keine neue oder entfernte Writer-Authority
- keine neue Capability-Oberflaeche
- keine Aenderung an RestPlacement-Entscheidungen oder Display-Ankern
- keine Aenderung an Clef-Planung oder Clef-Kosten
- keine Aenderung an Render-/PDF-Quellenauswahl
- keine Etueden- oder Taktselektoren

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 5" --display --visual-preflight
```

Ein Vollkorpuslauf ist fuer diesen rein mechanischen Cleanup nicht erforderlich.
