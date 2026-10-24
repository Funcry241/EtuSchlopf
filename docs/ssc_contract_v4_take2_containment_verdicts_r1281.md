# r1281: SSC-Vertrag v4, Take 2 – Containment-Verdikte V4

Erster `GovernedChange`-Take auf accepted r1280. Die Zulassung erfolgt vollständig über die vom Parent kompilierten Arten:

- `music_effect=none`;
- `proof_rule` für `src/ssc_evidence_proof.rs`;
- 22 Zeilen `envelope_extend`;
- der Selector-Implementierungsvertrag `docs/selector_derivation_contract_ssc_containment_verdicts_r1281.md`, der auf die in r1277 eingefrorene Fehlerklasse verweist.

Der Take ist rein beobachtend: kein Enforcement, keine Writer- oder Owner-Änderung, keine Musikänderung.

## Verdikte (A2)

| V4-Verdikt | Bedingung |
|---|---|
| `Permit` | Integrität ok, Invariante bewiesen, Eingrenzung bewiesen |
| `ContainmentDenied` | Eine Differenz außerhalb der Envelope-Zeile oder eine widerlegte Invariante |
| `ContainmentUnproven` | Keine Zeile, unlesbare Bytes oder die Invariante ist nicht beweisbar |
| `OutOfScope` | Exact- und Block-Owner ohne Familie; bleibt Ausstiegsschuld bis Take 3 |

Die Notwendigkeit wird weiter gemessen, entscheidet aber nicht mehr. Die V2-Zeilen des Berichts bleiben unverändert stehen. Neue Berichtszeilen:

- `V4OutOfScope`, `V4ContainmentUnproven`, `V4ContainmentDenied`, `V4Permitted` und `V4Balance`;
- `V4Envelope<ART>` je Envelope-Art;
- `V4DeniedDescriptor<NAME>` je Descriptor; das ist die Eingabe für den Korrektur-Take nach A5.

## Eingrenzung (A3)

Die Prüfung entfernt aus dem kanonischen Strukturvergleich genau die Teile, die die Zeile als veränderlich deklariert:

- ganze Elemente;
- deklarierte Attribute und Kind-Elemente an Host-Elementen.

Ohne die Operation `change` werden ganze Elemente nicht ausgeblendet, sondern als Folge geführt. Dann ist nur Einfügen bzw. Entfernen möglich. Lane-Felder gelten nur als Kinder einer `note`. Die Prüfung ist damit höchstens so weit wie die Tabelle, nie weiter.

Die Invariante bleibt die bisherige des jeweiligen Envelope-Typs:

| Envelope-Typ | Invariante |
|---|---|
| Projection | eigene Erhaltungsprojektion des Owners |
| Insertion | Einfügegeometrie (E036) bzw. finale Barline als letztes Element des letzten Takts (E004, jetzt ohne Notwendigkeitszertifikat) |
| Lane | Struktur außer den Lane-Feldern gleich |

## Envelope-Zeilen

- **17 PRESERVING-Owner:**
  - Beam-Familie: E006, E008, E047 bis E050 und E053;
  - Clef: E052, E054 und E024;
  - E051: Beam und Clef;
  - Layout: E056, E044 und E013;
  - Direction: E055;
  - Slur und Layout: E007;
  - E057: Note/Forward-Tausch, als Content deklariert.
- **Insertion:** E004 und E036.
- **Lane:** F5, F7 und F9.

Jede Zeile liegt innerhalb der eingefrorenen Zweckklassen aus r1278. Abgeleitet sind die Zeilen aus dem Owner-Zweck und der Owner-Signatur, nicht aus dem Korpus. Ein `ContainmentDenied` im Windows-Lauf ist deshalb ein echter Befund für Take 4: Entweder ist der Envelope innerhalb des Zwecks zu erweitern, oder der Owner ist zu korrigieren, dann nur mit Nutzerfreigabe.

## Tests

- Neue Einheitstests in `ssc_evidence_proof.rs` decken ab:
  - die Zeilenbindung;
  - Beam und Tonhöhe;
  - Lane (`voice` in `forward` ist keine Lane);
  - Insertion (Änderung einer bestehenden `direction` wird erkannt);
  - Layout und Clef;
  - die Verdiktlogik, einschließlich einer Proof-Bindung an ein fremdes Event.
- Die End-to-End-Tests über echte Owner, Adoption und Bericht prüfen zusätzlich die V4-Zeilen:
  - E006, auch eine Umgruppierung ohne Notwendigkeit, jetzt `V4Permitted`;
  - E044, E036 und E004;
  - F5, F7 und F9.
