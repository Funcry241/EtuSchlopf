# MetricPartialRestNotationAuthority

Stand: v0.10.1747 r01

## Ausgangsbefund

Der menschliche Reproduktionsanker ist Etuede 1, Takt 2, oberes System. Der
bewiesene stille Voice-Abschnitt reicht dort in der aktuellen 96er Zeitbasis
von `0` bis `336`. Master und Render schreiben ihn identisch als

```text
96 + 96 + 96 + 48
```

also als drei Viertelpausen und eine Achtelpause. Die anschliessenden beiden
klingenden Sechzehntel beginnen unveraendert bei `336` und `360`.

Die Fundstelle ist ausschliesslich Regressionadresse. Es existiert keine
Etueden-, Takt-, Staff- oder Onset-Sonderregel.

## Gefundene Ownership-Ursache

Die Fragmentierung entsteht bereits in `apply_visible_rhythm_ledger()` im
Master. Dort verwendeten sichtbare Pausen und klingende Noten denselben
`split_duration_for_metric_notation`-Pfad. Dieser schneidet jeden Abschnitt an
jeder Viertelschlaggrenze. Fuer den stillen Abschnitt `0..336` ist daher
`96+96+96+48` zwangsläufig.

Spaetere Render-Stufen besitzen daneben eigene Dauerentscheidungen:

- die `FragmentedRestChain`-Konsolidierung bleibt absichtlich beat-lokal und
  darf den ersten und zweiten Viertelschlag nicht zu einer Halben verbinden;
- die Lower-Staff-Compaction zerlegte laengere Restketten bisher greedy ueber
  die groessten verfuegbaren Werte. Ein Abschnitt `0..336` konnte dadurch zu
  `288+48` werden und damit als punktierte Halbe die innere 4/4-Struktur
  verdecken;
- Missing-Rest- und Lower-Rest-Lane-Materialisierung benutzten dieselbe greedy
  Zerlegung fuer aus `forward` abgeleitete sichtbare Pausen.

Damit wurde dieselbe fachliche Frage - wie ein bereits bewiesener Stillespann
sichtbar notiert wird - an mehreren Stellen unterschiedlich entschieden.

## Neue Owner-Grenze

`model::metric_rest_notation::plan_partial_visible_rest_durations` ist ein reiner,
writer-freier Planungs-Owner. Er erhaelt ausschliesslich einen bereits
bewiesenen Stillespann sowie Takt- und Beat-Dauer und liefert die sichtbare
Dauerfolge. Er entscheidet weder Voice-Existenz noch Staff/Voice-Zuordnung und
schreibt kein MusicXML.

In der produktiven EtuSchlopf-Zeitbasis gilt divisions-per-quarter=96. Weil die
bestehenden Rest-Materialisierer ihre MusicXML-Typabbildung auf genau diese
Dauerwerte binden, entscheidet der neue Planner bei einer anderen Beat-Basis
fail-closed.

Fuer den aktuellen einfachen Viererschlag gilt:

- eine an einer Takthaelfte ausgerichtete, vollstaendig abgedeckte
  Zwei-Schlag-Einheit darf als Halbe Pause erscheinen;
- danach bzw. davor bleiben relevante Schlaggrenzen sichtbar;
- ein Stillespann von `0..336` wird daher `192+96+48`;
- `0..288` wird `192+96`, nicht `288`;
- ein Abschnitt `96..288` bleibt `96+96`, weil eine Halbe von Schlag 2 bis 3
  die Taktmitte ueberschreiten wuerde;
- ein vollstaendig stiller Takt wird hier bewusst nicht kanonisiert. Seine
  einzige produktive Endentscheidung bleibt der bestehende
  `CanonicalMeasureRestSingleOwner` / `BehindBarsRestAndTieAudit`.

Andere Taktlaengen werden derzeit beat-explizit zerlegt. Die neue Klasse
beansprucht damit keine noch nicht bewiesene allgemeine Meter-Theorie.

## Materialisierung

Die vorhandenen Writer bleiben bestehen. Auf dieselbe Dauerplanung gebunden
werden:

1. das Master-`VisibleRhythmLedger` fuer sichtbare Rest-Luecken und bereits
   vorhandene sichtbare Restwerte;
2. die bestehende Lower-Staff-Restketten-Compaction;
3. die bestehende Missing-Rest-Materialisierung aus gebundener `forward`-Zeit;
4. die bestehende Lower-Staff-Visible-Rest-Lane-Planung;
5. die beat-lokale `FragmentedRestChain` fuer die Entscheidung, ob eine
   vorhandene Mikrokette als genau eine Pause darstellbar ist. Ihr bisheriger
   Cross-Beat-Schutz bleibt unveraendert.

Die alte pitched-note Zerlegung bleibt separat. Klingende Noten behalten ihre
beat-lokale Tie-/Notenwert-Policy; Pausen erhalten keinen Einfluss auf diese
Entscheidung.

`FragmentedRestChain` bleibt als nachgelagerte beat-lokale Reparatur bestehen,
bezieht aber ihre Ein-Pausen-Dauerentscheidung aus demselben Planner und
erhaelt kein neues Cross-Beat-Schreibrecht. Die vertikale Rest-Placement-
Autoritaet wird nicht veraendert.

## Invarianten

Die Aenderung darf nicht veraendern:

- klingende Pitch-Ereignisse,
- Onsets klingender Ereignisse,
- Dauern klingender Ereignisse,
- Staff- oder Voice-Zuordnung klingender Ereignisse,
- Taktgesamtdauer,
- Voice-Timeline-Abdeckung,
- Backup-/Forward-Balance,
- Cross-Staff-Semantik,
- Ganztaktpausen-Ownership,
- Rest-Placement-Ownership.

Nicht beweisbare oder nicht in der kanonischen 96er Zeitbasis liegende Faelle
werden nicht geplant.

## Tests

Der neue reine Planner prueft semantisch:

- `0+336 -> 192+96+48` im 4/4-Raster;
- kein Halbe-Pausen-Merge ueber die Taktmitte von einem schwachen Schlag aus;
- kompletter Takt bleibt beat-explicit fuer den bestehenden Ganztakt-Owner;
- nicht-vierzaehlige Takte fallen auf beat-explicite Notation zurueck;
- ungueltige, ueber den Takt hinausreichende oder nicht kanonisch skalierte
  Spans fail-closed.

## Erwarteter Regressionsbeweis

Etüde 1, Takt 2 oben soll im finalen PDF aus demselben Stillespann sichtbar

```text
Halbe Pause + Viertelpause + Achtelpause + zwei Sechzehntel
```

erhalten. Die beiden klingenden Sechzehntel und ihre Zeitpositionen bleiben
identisch.

Erster produktiver Nachweis:

```powershell
.\run.cmd --take --strict "Etüde Nr. 1" --display
```

Erst nach gruenem Einzelprojekt und visueller PDF-Bestaetigung ist ein einziger
Korpuslauf sinnvoll.
