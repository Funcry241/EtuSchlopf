# r1002 NotationVoiceVisibleChordOccupancyAdapterClosure

## Parent, Failure Evidence und Scope

r1002 baut direkt auf dem weiterhin autoritativen r998 `NativeInventoryTrustAnchorWarningDebtClosure` auf. r999, r1000 und r1001 bleiben rejected Failure Evidence und sind niemals Parent: r999 scheiterte vor Installation an der Take-Tree-Integritaet wegen eines unzulaessigen Root-`Cargo.lock`; r1000 schloss diese Paketgrenze, scheiterte aber im Strict-Clippy-Compile an einer unnoetigen Referenz auf den privaten `FinalChordProjectionProvenanceRow`; r1001 schloss den Compilefehler, erreichte Build/Clippy/Strict-Tests, blockierte jedoch im produktiven Korpuslauf an einer zu starken XML->VisibleOccupancy-Adapterannahme.

Der Take ist `RefactorOnly` und weiterhin vollstaendig read-only gegenueber Musik und Notensatz. Er reappliziert den Pre-Serialization-Constraint-Audit aus r1001 auf r998 und aendert ausschliesslich die Adaptersemantik fuer `<chord/>` an der sichtbaren Occupancy-Grenze sowie die dazugehoerige Audit-Evidenz und Tests. Es entsteht kein NotationVoice-Writer, keine MusicXML-Mutation, keine Repair-/Selection-/Release-Authority und keine neue musikalische Regel.

## Widerlegte Adapterannahme

r1001 verlangte beim Aufbau atomarer Occupancy-Gruppen fuer jedes `<chord/>`-Element einen unmittelbar vorausgehenden gepitchten atomaren Anchor. Bei einem Chord-Member nach einem Rest oder einem anderen nicht-gepitchten Anchor brach der Adapter deshalb bereits mit `chord member has no preceding pitched atomic anchor` ab.

Diese Bedingung war fuer den neuen Audit zu stark. `<chord/>` ist technische MusicXML-Syntax und darf nicht allein sichtbare Notationsbelegung autorisieren. Insbesondere muessen unsichtbare technische Chord-Carrier ohne sichtbaren gepitchten Anchor den Audit passieren koennen; sichtbare Chord-Member ohne gepitchten Anchor duerfen nicht verschluckt werden, sondern muessen als eigene sichtbare Occupancy-Evidenz erhalten bleiben. Gerade ein sichtbarer Rest und sichtbare Chord-Member im selben Zeitbereich sind Untersuchungsdaten, kein Parserfehler.

## Neue atomare Occupancy-Grenze

r1002 fuehrt dafuer einen kleinen read-only `PreSerializationAtomicGroupTracker` ein. Er unterscheidet:

- sichtbaren gepitchten Non-Chord-Anchor,
- unsichtbaren gepitchten Non-Chord-Anchor,
- sichtbaren Rest-Anchor,
- unsichtbaren Rest-Anchor,
- sonstigen Non-Chord-Anchor,
- Chord-Member ohne gepitchten Anchor.

Ein normaler Akkord bleibt atomar: Chord-Member hinter einem gepitchten Anchor verwenden dessen Gruppe weiter. Chord-Member hinter einem nicht-gepitchten Anchor erzeugen dagegen eine eigene, fuer die zusammenhaengende Chord-Folge stabile `ChordWithoutPitchedAnchor`-Gruppe. Dadurch kann eine sichtbare Rest-Gruppe zeitlich neben einer sichtbaren gepitchten Chord-Gruppe bestehen und vom Audit als Evidenz beobachtet werden, ohne dass der Adapter den Projektlauf abbricht.

Unsichtbare Chord-Member dieser Art bleiben fuer Source-/Attack-/Occurrence-Transport vorhanden, erhalten aber keine sichtbare Occupancy-Authority. Sichtbare Chord-Member ohne gepitchten Anchor werden explizit als `VISIBLE_CHORD_WITHOUT_PITCHED_ANCHOR` berichtet. Ueberlappt eine solche Gruppe einen sichtbaren Rest auf demselben Staff, wird zusaetzlich `VISIBLE_REST_ORPHAN_CHORD_OVERLAP` als read-only Evidenz ausgegeben. Diese Zeilen sind keine automatische Fehler-, Voice- oder Repair-Entscheidung.

## Unveraenderter NotationVoice-Vertrag

Der eigentliche `NOTATION-VOICE-PRE-SERIALIZATION-CONSTRAINT-CLOSURE-AUDIT` bleibt ohne Schreibrecht. Er prueft weiterhin kanonische Occurrence->SourceEventId->SoundingAttackId-Bindung, Injektivitaet, semantische Dauer, konkrete LanePressure-Blocker-Kanten und sichtbare Occupancy. Beam-Witness-Provenienz und Materialisierung bleiben absichtlich offen; maximaler positiver Status bleibt `BLOCKED_INCOMPLETE_BEAM_WITNESS_PROVENANCE_AND_MATERIALIZATION`.

Pitch, semantischer Onset und semantische Dauer bleiben harte musikalische Invarianten. Current-Voice-Labels sind Diagnose und keine Proof-Authority. Der bekannte visuelle Fehlerkatalog bleibt Acceptance-Oracle und niemals Selektor. Etuede 6 Takt 1 bleibt ausschliesslich Witness/Regressionstest.

## Regressionen

Zusaetzlich zu den fuenf generischen r1001-Audit-Tests kommen drei Adaptergrenzen hinzu:

1. hidden `<chord/>` nach Rest bleibt als technischer Carrier erhalten und fuehrt nicht zum Adapterabbruch,
2. visible `<chord/>` nach Rest erhaelt eine separate gepitchte Evidenzgruppe; sichtbare Rest-Ueberdeckung bleibt als Evidenz erhalten,
3. normaler sichtbarer Akkord mit gepitchtem Anchor bleibt genau eine atomare Gruppe.

Ausgehend von r998 mit 1.303 Strict-Tests werden damit **1.311 Strict-Tests** erwartet.

## Acceptance

Autoritativ bleibt:

```powershell
./run.cmd --take --all --inventory
```

Acceptance verlangt Build und Strict-Clippy gruen, 1.311 Tests, einen vollstaendig verarbeiteten 10er-Korpus ohne Adapterabbruch wegen `<chord/>`, keine neue MusicXML-/PDF-/Klangmutation sowie einen auswertbaren Pre-Serialization-Audit fuer die realen Kandidaten. `BLOCKED_INCOMPLETE_*` und echte read-only Occupancy-Evidenz sind erlaubt; problematische Ereignisse duerfen nicht durch Ignorieren oder katalogbasierte Sonderfaelle verschwinden.
