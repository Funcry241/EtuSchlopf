# UpperInitialMetricRestCoalescingClosure

Stand: v0.10.1748 r02

## Anlass

v0.10.1747 loest den primaeren Human-Fund Etuede 1, Takt 2 oben allgemein: Master und Render schreiben den Anfangsstillespann `0..336` als `192+96+48`. Der 1746->1747-Gegenvergleich zeigt keine Aenderung an Pitch, Onset, Dauer, Staff oder Voice klingender Ereignisse.

Im selben Gegenvergleich bleibt Etuede 1, Takt 17 oben im finalen Render jedoch als `96+96+24` sichtbar. Fuer den bewiesenen Anfangsstillespann erlaubt der zentrale `MetricRestNotation`-Owner `192+24`. Takt 17 ist nur Regressionadresse.

## Bestehender Writer, unveraenderte Writer-Identitaet

Der vorhandene Writer `apply_upper_staff_initial_rest_and_beat_local_coalescing` bleibt in v0.10.1748 r02 unter exakt demselben Symbolnamen bestehen. Es entsteht kein neuer Writer-Root, kein Writer wird umbenannt und keine Structural-Write-Authority wird veraendert.

Der Coalescing-Teil erhaelt lediglich eine zweite, zentral bewiesene Zulassungsbedingung neben der bestehenden Mikrokonsolidierung zweier Sechzehntel innerhalb eines Beats:

- zwei zusammenhaengende sichtbare Pausen derselben Upper-Staff-Voice;
- die erste Pause beginnt bei Taktzeit `0`;
- beide Pausen enden vor dem ersten sichtbaren klingenden Event derselben Voice;
- `MetricRestNotation` plant die gesamte Paarspanne als genau einen sichtbaren Pausenwert;
- die bestehende MusicXML-Typabbildung kann diesen Wert materialisieren.

Damit darf zum Beispiel `96+96` am Taktanfang zu `192` werden. Ein schwach ausgerichtetes Paar von Schlag 2 bis Schlag 4 bleibt getrennt, weil der zentrale Planner dafuer keinen einzelnen Halbe-Restwert liefert.

## Vollstaendige Taktruhe bleibt getrennt

Die Erweiterung verlangt ein spaeteres sichtbares klingendes Event derselben Voice. Eine vollstaendig stille Voice erhaelt dadurch kein neues Paar-fuer-Paar-Rewrite. Die bestehende Ganztaktpausen-Policy bleibt alleiniger Owner. Cross-Staff-, Timeline- und vertikale Rest-Placement-Ownership bleiben unveraendert.

## Warum r02

r01 wurde vom Semantic-Capability-Diff vor Build und vor Musikmutation korrekt abgelehnt. Ursache war ausschliesslich die unnoetige Umbenennung des bestehenden `apply_*`-Writer-Symbols. Dadurch aenderte sich seine Writer-Root-Identitaet und damit der WriterReachability-Fingerprint bis zum Audit-Orchestrator. r02 verwirft diese Umbenennung und behaelt die bestehende Writer-Identitaet vollstaendig bei. Das Gate wird nicht gelockert und keine Capability-Policy wird angepasst.

## Invarianten

Unveraendert bleiben muessen Pitch, Onset, Dauer, Staff und Voice aller klingenden Ereignisse, Takt-/Voice-Timeline-Abdeckung, Backup-/Forward-Balance, Raw-Cursor, Cross-Staff-Semantik, Ganztaktpausen-Ownership, vertikale Rest-Placement-Ownership und die bestehenden Writerwurzeln. Wenn der zentrale Planner keinen einzelnen metrisch zulaessigen Restwert liefert, bleibt das Paar unveraendert.

## Einzelprojektbeweis

Etuede 1 muss Takt 2 weiterhin als `192+96+48+24+24` zeigen. Takt 17 soll aus `96+96+24` zu `192+24` werden. Erst danach ist ein Vollkorpuslauf sinnvoll.
