# Canonical Measure Rest Single Owner Architecture

## Ziel

Vollstaendige, klanglose Taktstille einer sichtbaren Single-Staff-Lane wird genau einmal als kanonische MusicXML-Ganztaktpause materialisiert. Der Operator ist allgemein und kennt weder Projekt- noch Taktnummern.

## Owner

Produktiver Owner ist der bestehende `BehindBarsRestAndTieAudit` innerhalb der Initial-Render-Transaktion. Diese Grenze liegt bereits vor der PDF-Source-Auswahl, besitzt bereits eine gebundene Rest-Write-Authority und ist bereits als legitime Render-Ursache im vorhandenen Release-/PDF-Ledger bekannt.

Der fruehere `RenderXmlPianisticRestRepairTrial` verliert die Ganztaktstille-Entscheidung. Er bleibt fuer echte lokale Restketten-Konsolidierung zustaendig.

Es wird kein neuer Writer-Entrypoint, kein neuer Proof-Reentry-Reader und kein neuer PDF-/Manifest-Pfad eingefuehrt.

## Kanonische Regel

Eine Lane darf zu genau einer `<rest measure="yes"/>`-Pause werden, wenn alle folgenden Bedingungen gleichzeitig gelten:

- die Lane deckt die vollstaendige erwartete Taktdauer ab,
- sie enthaelt keine klingende Note,
- alle Rest-Noten gehoeren zu derselben Voice und demselben Staff,
- mindestens eine sichtbare Rest-Notiz existiert,
- technische versteckte Rest-Carrier oder Forward-Zeit duerfen innerhalb genau dieser Lane konsumiert werden,
- keine Rest-Notiz traegt eine sichtbare semantische Abhaengigkeit,
- dieselbe Voice ist nicht cross-staff klingend,
- auf demselben Staff existiert keine sichtbare klingende Gegenstimme; dieser Fall bleibt Auxiliary-Lane-Suppression und wird nicht als Ganztaktpause materialisiert.

Eine bereits kanonische einzelne `measure="yes"`-Pause bleibt unveraendert.

Die erwartete Taktdauer ist dabei eine metrische Scope-Wahrheit aus Divisions und Taktart und darf nicht von der aktuellen Rest-/Forward-Carrierform abhaengen. Eine lokale Dauerableitung aus Note-Event-Enden ist nur Fallback, wenn fuer den Scope keine metrische Dauer vorliegt; `forward`-getragene Zeit darf die erwartete Taktdauer nicht kuenstlich verkuerzen.

## Authority und Fail-Closed

Die vorhandene `RestChainRewriteCapability` bleibt der physische XML-Delta-Owner. Sie darf die erste sichtbare Rest-Notiz in die kanonische Ganztaktpause umschreiben und redundante Rest-/Forward-Fragmente entfernen.

Die bestehende physische Writer-Grenze bleibt erhalten, aber ab v0.10.1740 r01 werden darin zwei semantische Transaktionen getrennt abgenommen:

- der bestehende Behind-Bars-Rest/Voice/Duplicate-Stem-Teil mit seiner bisherigen globalen Invariantenpruefung,
- `RestOnlyStaffEmptyMeasureReadability` mit eigener lokaler Invariantenpruefung fuer Ganztaktpause und Auxiliary-Lane.

Lehnt die RestChain-Authority oder die lokale Ganztakt-Invariante den gebundenen Replacement-Batch ab, wird nur diese Rest-only-Transaktion verworfen. Ein sicherer Legacy-Behind-Bars-Kandidat darf weiterhin geschrieben werden. Umgekehrt kann eine sichere Ganztaktpause geschrieben werden, auch wenn eine andere Behind-Bars-Reparatur ihren eigenen Invariantencheck nicht besteht. Physisch geschrieben wird trotzdem nur einmal ueber den bereits vorhandenen `write_behind_bars_rest_tie_audit_candidate_authorized`-Entrypoint. Vor diesem einen Write bleibt ein gemeinsamer finaler Fail-closed-Invariantencheck bestehen.

## Invarianten

Der Write bleibt Render-MusicXML-only. Master-MusicXML wird nicht veraendert.

Vor Adoption muessen insbesondere erhalten bleiben:

- klingende Signatur,
- Anzahl sichtbarer und versteckter klingender Pitch-Events im jeweils zulaessigen Duplicate-Stem-Vertrag,
- parsebares MusicXML,
- Backup-/Forward-Zeitbilanz.

Eine Reduktion versteckter Rest-Events ist nur in der exakt gebundenen, lokal erfolgreich autorisierten Ganztaktkanonisierung zulaessig und wird separat bilanziert. Fuer sichtbare Rest-Events gilt zusaetzlich die exakte Bilanz: konsumierte sichtbare Restfragmente minus eine erhaltene kanonische Measure-Rest pro uebernommenem Ganztaktkandidaten muessen der sichtbaren Restreduktion entsprechen.

Die lokale Rest-only-Transaktion arbeitet immer auf dem bereits akzeptierten Legacy-Kandidaten oder – falls dieser verworfen wurde – wieder auf dem unveraenderten Eingang. Damit gibt es keine Abhaengigkeit von einem nicht adoptierten Zwischenstand.

## Akzeptanzbeispiele aus dem gruenen v0.10.1739/r04-Korpus

Diese Bereiche sind Test-/Evidence-Beispiele, keine Selektoren:

- Etuede 1: Takt 3 wird als vollstaendige obere Single-Staff-Stille erkannt.
- Etuede 5: unter anderem Takt 16 und 17 werden als vollstaendige untere Single-Staff-Stille erkannt.
- Etuede 7: vollstaendige untere Stille wird erkannt; Takt 5 wird wegen echter Cross-Staff-Nutzung nicht als Ganztaktpause selektiert.

## Nicht Teil dieses Takes

- keine Lockerung von Semantic-Capability-Diff,
- keine Aenderung an Proof-Reentry-Readern,
- keine Aenderung am Corpus-Manifest-SelfCheck,
- keine Launcher-/Recovery-/Watch-Aenderung,
- keine automatische `--visual-preflight`-Implizierung fuer `--all`,
- keine PDF-UI-Aenderung.

## v0.10.1741 – FinalRenderRawCursorMeasureFallbackClosure

Der reale v0.10.1740-Lauf zeigte, dass die CanonicalMeasureRest-Takte selbst nicht die
PDF-Adoption blockierten. Die spaete Render-Quelle war in E5 an zwei anderen Takten
und in E7 am bestehenden Cross-Staff-Stresstakt raw-cursor-rot. Die Release-Selection
hat diese Quelle deshalb korrekt auf Master zurueckgewiesen.

Die Closure liegt nun am bereits vorhandenen letzten FinalNotation-Checkpoint: ein
RawCursor-roter Render-Takt darf lokal auf seinen korrespondierenden raw-cursor-gruenen
Master-Takt zurueckfallen, wenn Taktnummer, erwartete Dauer und die gesamte
nicht-timeline-relevante Taktoberflaeche uebereinstimmen. Der Vergleich entfernt nur
`note`-, `backup`- und `forward`-Elemente aus der Oberflaechensignatur; Attribute,
Directions, Layout-/Print- und Barline-Kontext muessen daher weiterhin gleich bleiben.

Das ist ein Fail-closed Rollback, kein neuer Repair-Operator: gruene Render-Takte werden
nicht angefasst, unsichere lokale Render-Notation wird auf die kanonische Master-Wahrheit
zurueckgenommen, und der bestehende RawCursor-Release-Gate bleibt unveraendert die
letzte PDF-Freigabe. Kein Takt-/Projektselektor, keine neue Writer-Authority, kein
MasterWrite und kein neues Report-/Manifest-Protokoll.
