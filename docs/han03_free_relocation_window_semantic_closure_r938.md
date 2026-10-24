# r938 – Han03FreeRelocationWindowSemanticClosure

## Parent und ChangeClass

Autoritativer Parent ist der installierte r937 `CanonicalStrictValidationBasisFailClosedCompileClosure`.

`ChangeClass: MusicMutation`.

Der Take aendert keine Note, keinen Pitch, keine Dauer, keinen Staff, keine Voice, keine MusicXML-/PDF-Writer-Authority und keine Release-Grenze. Er aendert bewusst die produktive HAN03-Regelsemantik und wird deshalb nicht als Refactor versteckt.

## Ausgangsbefund

Nach r931/r932 besitzen HAN03 und alle Downstream-Consumer dieselbe kanonische Endpoint- und TemporalEdge-Identity. Der verbleibende Etuede-7-Fall ist dadurch eindeutig:

- Intervall: 24 Halbtoene,
- Attack A -> Attack B: 96 divisions,
- A-Dauer: 48 divisions,
- Release A -> Attack B: +48 divisions,
- `direct_attack_successor=false`.

Die bestehende HAN03-Emission benutzt fuer `fast` nur den Attack-Abstand und die Intervallweite. Damit wird eine positive endpoint-lokale Release->Attack-Zeit zwar bereits kanonisch transportiert, aber fuer die Warnungsentscheidung ignoriert.

## Fachlicher Schnitt

HAN03 bleibt Warnungsowner fuer grosse Spruenge innerhalb seines bestehenden Attack-Fensters nur dann, wenn A bis zum B-Angriff keine positive endpoint-lokale Release->Attack-Zeit hinterlaesst:

```text
release_to_attack_gap <= 0  -> HAN03 darf weiterhin warnen
release_to_attack_gap >  0  -> keine HAN03-FastJump-Warnung
```

Das ist **keine Behauptung**, ein gapped leap sei menschlich sicher oder spielbar. Im Gegenteil: Sobald eine positive endpoint-lokale Release->Attack-Zeit existiert, braucht eine Aussage ueber "zu schnell" Tempo- und Capability-Evidenz. Diese Score-Demand-Seite ist bereits getrennt vorhanden:

- r608 `PianisticTemporalTransitionDemandFrontierAudit`: reale Attack-Zeitfenster ueber TempoMap,
- r610 `PianisticKeyboardStateTransitionFrontierAudit`: raeumlich-zeitliche Keyboard-State-Demand inklusive Geometrie/Zeit-Quotient.

r938 erfindet deshalb keine neue ms-Grenze, keine Sprunggeschwindigkeit und kein Pianistenprofil.

Die positive A->B-Luecke beweist insbesondere **nicht**, dass die ganze Hand frei ist. HAN03 besitzt dafuer weder Handzuordnung noch komplette Keyboard-State-Occupancy-Authority. Falls andere gleichzeitig gebundene Tasten die Relokation einschraenken, bleibt das eine getrennte Whole-Configuration-/Transition-Witness-Frage und darf nicht aus zwei HAN03-Endpunkten erraten werden.

## Bewusst erhalten

Unveraendert bleiben:

- Mindestintervall `19` Halbtoene,
- bestehendes Attack-Fenster `onset_delta <= measure.divisions`,
- HAN03 Severity/Gewicht,
- kanonische `Han03JumpIdentity`,
- kanonische `Han03TemporalEdge`,
- BehindBars-Consumer,
- alle Repair-/Writer-/Renderer-Pfade,
- Public-Artifact-Commit `0 Blocker / 0 Warnungen`.

Kontiguous grosse Spruenge (`gap=0`) warnen weiterhin. Ueberlappende grosse Spruenge (`gap<0`) warnen ebenfalls weiterhin. Nur der bisher nicht unterschiedene Fall `gap>0` verlaesst die release-blockierende HAN03-Klasse.

## Regression

Neue generische Tests pruefen ohne Etueden-/Takt-Sondercode:

1. 24-Halbton-Sprung, Attack-Abstand 96, Release->Attack +48 -> keine HAN03-Warnung.
2. Derselbe Sprung kontigu, Release->Attack 0 -> genau eine HAN03-Warnung mit kanonischer Identity.
3. Derselbe Sprung ueberlappend, Release->Attack -48 -> genau eine HAN03-Warnung.

## Falsifikation / erwarteter Nutzer-Gate

Der Take ist abzulehnen, wenn Build, Clippy oder Tests scheitern oder wenn der reale Korpus neue Blocker/Warnungen erzeugt.

Nach erfolgreichem `--take "Etüde Nr. 3"` ist der entscheidende fachliche Beweis ein normaler Lauf von `Etüde Nr. 7`:

- Clippy/Test/Build duerfen ihre exakten Receipts weiterverwenden,
- der Musik-/PDF-Lauf bleibt real,
- der bisherige Takt-25-G#2->G#4-Fall darf **nicht mehr** als HAN03-Warnung erscheinen,
- der Ergebnisbericht muss dadurch formal `0 Blocker / 0 Warnungen` erreichen,
- der Public-Artifact-Commit muss dann erfolgreich sein statt Recovery/FEHLER auszufuehren.

Wenn ein anderer produktiver Validator dabei eine neue reale Restschuld zeigt, wird sie nicht weggefiltert; dann ist r938 nicht automatisch der Abschluss.
