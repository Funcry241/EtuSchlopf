# r610 – PianisticKeyboardStateTransitionFrontierAudit

## ChangeClass

`AuditOnly`

## Parent

`0.10.1839 r608 PianisticTemporalTransitionDemandFrontierAudit`

## Zweck

r607 hat die statische Distinct-Key-Spannweitenfrage des Scores von Same-Key-Overlap getrennt. r608 hat fuer reale Attack-Grenzen die exakte physische Zeitachse und Same-Key-Re-Attack-Demand geschlossen.

r610 verbindet jetzt ausschliesslich diese **Score-Demand-Seiten**: Fuer aufeinanderfolgende Attack-Grenzen werden die jeweils aktiven **verschiedenen physischen Klaviertasten** verglichen. Beobachtet werden Persistenz, neu hinzukommende und wegfallende Tasten, gerichtete Nearest-Key-Distanzen, die symmetrische Hausdorff-Distanz auf der MIDI-Tastenachse und deren Quotient zur exakten r292-Zeitspanne.

Der Quotient ist ein dimensionsbehafteter **Score-Demand-Index**. Er ist ausdruecklich weder Handgeschwindigkeit noch Fingerbewegung noch ein menschlicher Grenzwert.

## Strikte Grenzen

r610:

- weist keine Hand zu,
- weist keinen Finger zu,
- verwendet Staff nicht als Handwahrheit,
- fuehrt keinen Reichweiten- oder Geschwindigkeitsgrenzwert ein,
- veraendert r299 nicht,
- veraendert r607/r608 nicht,
- erzeugt keinen Execution Witness,
- behauptet weder spielbar noch unspielbar,
- besitzt keinerlei Writer-, Repair- oder MusicMutation-Autoritaet.

Nicht-identische ueberlappende Verpflichtungen auf derselben physischen Taste bleiben bewusst die eigene r608-Technik-/Zeitklasse. r610 dedupliziert MIDI-Tasten nur fuer die Geometrie und darf diese Same-Key-Frage weder verstecken noch loesen.

## Capability-Handoff

Nach r610 sind drei voneinander getrennte Score-Fragen explizit vorhanden:

1. statische Distinct-Key-Konfiguration / minimale Zwei-Hand-Spannweitenfrontier (r607),
2. exakte Zeitfenster und Same-Key-Re-Artikulationsbedarf (r608),
3. assignment-freie raeumlich-zeitliche Tastenzustandsaenderung (r610).

Erst ein spaeterer Capability-Take darf **unabhaengige** Evidenz an die bereits vorhandenen Hand-/Finger-Konfigurations- und TransitionSignatures binden. Score-Demand darf nicht selbst zur Capability-Wahrheit umgedeutet werden.


## Recovery aus r609

r609 wurde nach erfolgreichem Build und Strict-Clippy im integrierten `--bin etuschlopf`-Testgate verworfen und vollstaendig auf r608 zurueckgerollt. Ursache war ausschliesslich ein neuer report-contract Unit-Test: `build_for_score_title(None)` verlangte die bereits fachlich geltende Zeile `HandTransitionCapability NOT_BOUND`, waehrend der `None`-Fruehreturn nur `HumanPlayabilityClaim NOT_EVALUATED` materialisierte.

r610 schliesst genau diese Projektion: auch ohne Scoretitel werden HumanPlayability, HandTransitionCapability, FingerTransitionCapability und TempoFeasibility explizit als nicht bewertet bzw. nicht gebunden ausgegeben. Die Tastenzustandsgeometrie, Zeitrechnung und alle produktiven Musikpfade sind gegenueber dem rejected r609-Kandidaten unveraendert. r609 besitzt keine Authority und wird nicht wiederverwendet.
