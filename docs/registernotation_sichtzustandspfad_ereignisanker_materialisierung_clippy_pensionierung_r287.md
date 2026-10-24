# r287 – Registernotation: Sichtzustandspfad-Ereignisanker-Materialisierung und Feedbackidentitäts-Abschluss

## Ausgangslage

Der autoritative Elternstand ist r283 mit 1000/1000 Prüfungen. r283 hat für Etüde 6 das schreibfreie musikalische Zielbild geschlossen:

- 22 semantische Registerpläne,
- 4 sichere Kontaktfusionen,
- 2 bestätigte stille Kontinuitätsbrücken,
- 1 bestätigter direkter `15ma -> 8va`-Zustandswechsel,
- 16 sichtbare Zielspannen,
- 0 offene musikalische Sichtentscheidungen.

Die beiden r284-Pakete wurden vor Installation wegen Paketbaum- bzw. Same-Revision-Verträgen abgewiesen. r285 wurde ebenfalls vor Installation abgewiesen. Der `Semantic-Capability-Diff` meldete exakt die Entfernung des geschützten Musik-Rückmeldekanals:

```text
removed FeedbackIdentityChannel
src/pipeline_shards/notation_endpoint_layout_core.rs::raise_octave_shift_default_y_in_measure#1
detail=measure_number
```

Damit war nicht die neue Sichtmaterialisierung falsifiziert, sondern die Vermischung zweier Aufgaben: Die alte Funktion besaß zugleich taktbezogene Feedbackidentität und Oktav-`default-y`-Schreibwirkung.

## 1. Geschützte Identität bleibt erhalten

r287 behält exakt die bestehende private Funktionsidentität

```text
raise_octave_shift_default_y_in_measure(... measure_number ...)
```

und damit den `FeedbackIdentityChannel` `measure_number`.

Die Funktion übt aber kein Oktav-Geometrieschreibrecht mehr aus. Sie löst die identifizierte Taktstelle im aktuellen XML auf und liefert genau dann Erfolg, wenn diese Identität noch vorhanden ist. Beide bisherigen Aufrufer verwenden das Ergebnis als fail-closed Identitätsinvariante.

Die historische `wanted_y`-Eingabe bleibt nur Herkunftsevidenz. Sie wird nicht mehr auf MusicXML projiziert.

Damit gilt:

```text
Feedbackidentität bleibt erhalten
!=
Geometrieschreibrecht bleibt erhalten
```

## 2. Sichtzustandspfad wird unverändert materialisiert

Der produktive Registerpfad bleibt gegenüber dem fachlichen r285-Kandidaten unverändert:

```text
22 semantische Pläne
- 4 Kontaktfusionen
- 2 stille Brücken
= 16 sichtbare Linien
```

Der Writer übernimmt ausschließlich dieses bereits in r283 bewiesene Zielbild. Er trifft keine neue musikalische Entscheidung.

Start und Stop werden an reale Tonanker in MusicXML-Dokumentreihenfolge gebunden. Es gibt keinen taktweisen Richtungsanker mit großem `<offset>` mehr.

Der Writer erzeugt:

- kein `<offset>`,
- kein `default-y`,
- `number` aus realer gleichzeitiger Überlappung,
- `placement="above"` für 8va/15ma,
- `placement="below"` für 8vb/15mb.

## 3. Späte Export-Geometrie wird entmachtet

Bei der Prüfung von r285 wurde ein weiterer alter Pfad sichtbar, der die neue Projektion später wieder überschrieben hätte:

```text
normalize_octave_shift_direction_types
→ placement pauschal "above"
→ default-y pauschal 54
```

Dieser Pfad widerspricht der Vier-Domänen-Trennung und insbesondere der bereits feststehenden Unterseitenprojektion für 8vb/15mb.

r287 entfernt diese nachträgliche Oktav-Geometrieautorität aus der finalen Exportnormalisierung. Der Export darf nur noch prüfen, nicht musikalisch oder grafisch umentscheiden.

Der neue Exportwächter verlangt:

```text
type="down"  <-> placement="above"
type="up"    <-> placement="below"
default-y     -> verboten
```

Ein Verstoß führt fail-closed zu `OCTAVE_PLACEMENT_AMBIGUOUS` beziehungsweise `OCTAVE_GEOMETRY_AUTHORITY_CONFLICT`.

## 4. Invarianten

r287 verändert nicht:

- Tonhöhen,
- Einsätze,
- Dauern,
- Stimmen,
- Staff-Zuordnung,
- r283-Sichtentscheidung,
- Sicherheits-/Bootstrap-/Rollback-Infrastruktur,
- Auditbesitzer,
- allgemeine Staff-/Systemabstände.

Der unmittelbare Materialisierungsbeweis verlangt weiterhin:

- exakte 16 Starts und 16 Stops,
- identischen semantischen Planfingerabdruck,
- identischen Sichtfingerabdruck,
- keine Offsets,
- kein `default-y`,
- Byte-Invarianz außerhalb der neu eingefügten Oktavrichtungen.

## 5. Prüfungen

Gegenüber r283 kommen fünf produktionsnahe Prüfungen hinzu:

1. Liniennummer entsteht aus Überlappung statt Systemnummer.
2. Nicht überlappende Linien dürfen dieselbe Nummer wiederverwenden.
3. Untere Oktavprojektion wird ohne Offset/default-y geschrieben.
4. Finale Exportprüfung lässt eine untere Oktavlinie bytegleich unten und ohne default-y.
5. Finale Exportprüfung verweigert nachträgliche `default-y`-Oktavgeometrie.

Erwartung: **1005/1005**.

## 6. Prozess nach Installation

Nach erfolgreichem Take ausschließlich:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Danach menschliche PDF-Sichtprüfung von Etüde 6. Kein Korpuslauf, solange das Einzelstück nicht sichtbar plausibel sauber ist.


## Strict-Clippy-Pensionierungsabschluss

Der nie installierte r286-Kandidat erreichte nach bestandenem Semantic-Capability-Diff den realen Strict-Clippy-Bau. Dort wurden ausschliesslich Alt-Autoritaeten sichtbar, die durch die neue Registernotationsmaterialisierung vollstaendig abgekoppelt worden waren. `#![deny(unused_imports)]` meldete die drei verwaisten Re-Exports; `#![deny(dead_code)]` meldete die dazugehoerigen nicht mehr erreichbaren `default-y`-/Placement-Writer und ihre privaten Token/Helper. Die uebrigen Clippy-Meldungen waren Warnungen im bestehenden diagnostischen Budget und nicht blockierend.

r287 wendet den bereits mit r274 abgesicherten Vertrag fuer die Pensionierung abgekoppelter Music-Writer-Roots an: Die toten Alt-Wurzeln werden vollstaendig entfernt und nicht durch `#[allow(dead_code)]`, Scheinaufrufe, Stubs oder semantische Relocation konserviert. Pensioniert werden ausschliesslich:

- `raise_octave_shift_default_y_authorized` samt privater Token-/Rewrite-Familie,
- `force_direction_placement_authorized` samt `DirectionPlacementCapability`,
- `force_octave_shift_default_y_authorized` samt `OctaveDefaultYCapability`,
- die danach verwaisten Hilfsfunktionen und Re-Exports,
- der historische Unit-Test, der gerade das nun verbotene nachtraegliche Oktav-`default-y`-Schreiben verlangte.

Der geschuetzte `FeedbackIdentityChannel` `raise_octave_shift_default_y_in_measure(... measure_number ...)` bleibt dagegen bestehen und bleibt schreibfrei. Die neue Ereignisanker-Materialisierung, das 16-Linien-Zielbild und die Exportwaechterlogik bleiben gegenueber r286 fachlich unveraendert.
