> **COLD-START-HINWEIS:** Diese Datei ist eine fallorientierte historische Ergaenzung. Fuer neue Architekturarbeit zuerst `docs/assistant_memory/000_start_here.md`, danach `docs/assistant_memory/architecture_essence_bootstrap_v0.json` und `docs/assistant_memory/020_architecture_essence_v0.md` lesen. Contracts, Authorities und Gates des aktuellen Repositorys bleiben die einzige normative Architekturwahrheit.

> **ARCHIVHINWEIS v0.10.1449:** Die folgenden Etueden-, Takt- und M005-Angaben sind historische Diagnose- und Antiselektorbeispiele. Sie sind kein aktueller Projektstatus, keine aktuelle Diagnose und keine Reparaturprioritaet. Aktive musikalische Arbeit erfordert frische Evidenz aus aktuellen Artefakten oder einem konkreten Nutzerbefund.

# EtuSchlopf – externes Assistenten-Gedächtnis

**Zweck dieser Datei:**  
Diese Datei dient als externes Gedächtnis für ChatGPT/Assistenten in neuen Threads. Sie soll verhindern, dass bereits gelöste Diagnosewege wiederholt werden. Sie ist keine lose Dokumentation, sondern eine Arbeitsanweisung für zukünftige EtuSchlopf-Diagnosen.

**Wichtig:** Wenn diese Datei mit frischen Laufartefakten kollidiert, haben die frischen Artefakte Vorrang. Diese Datei bewahrt die stabilen Erkenntnisse, die Diagnose-Reihenfolge, Invarianten und verbotenen Abkürzungen.

---

## 1. Erst lesen, dann reparieren

Die zentrale Lehre aus dem M005-/RegisterPressure-Fall:

> EtuSchlopf wird nicht schneller, indem der Assistent früher schreibt. EtuSchlopf wird schneller, indem der Assistent früher die richtige Divergenz misst.

Bei hartnäckigen Notensatzproblemen nie sofort einen Write bauen. Zuerst klären, **auf welcher Schicht die Wahrheit auseinanderläuft**.

---

## 2. M005 ist Adresse, nicht Entscheidungslogik

`M005` bedeutet: Measure/Takt 5, dreistellig gepolstert.

Erlaubt:

```text
Kandidat gefunden bei M005 (Takt 5), Staff 2.
```

Verboten:

```text
if project == "Etüde 5" && measure == 5 {
    setze G-Schlüssel
}
```

Regel:

- `M005` darf als Diagnose-Adresse, Beweisort und Report-Koordinate erscheinen.
- `M005` darf **nicht** Auslöser, Prädikat oder Sonderregel sein.
- Der Write muss aus allgemeinen Eigenschaften folgen, zum Beispiel Registerdruck, Lesbarkeit, stabiler Span und Rückkehranker.

Gute Reportform:

```text
Kandidat: Takt 5 [M005]
Auslöser: staff_local_register_pressure + alternate_same_staff_clef_gain + stable_register_span + return_anchor_proof
Nicht-Auslöser: project_name, literal_measure_number, historical_hotspot_rank
```

---

## 3. Die fünf Diagnose-Schichten

Für jeden neuen Notensatz-Fix zuerst diese fünf Schichten prüfen:

| Schicht | Frage |
|---|---|
| Adresse | Wo liegt der Fall? Projekt, Takt/Measure, Staff, Voice, ggf. Onset. |
| Prädikat | Warum ist es ein Kandidat? Welche allgemeine Regel feuert? |
| XML-Quelle | Welche XML-Welt sieht den Kandidaten? Master, Render, FreshExport, PDF-Input. |
| Write-Anker | Wo würde konkret eingefügt, beendet oder zurückgeschaltet? |
| Visible Readback | Ist der gewünschte Effekt nach dem Write im XML/PDF/Glyph-Readback wirklich sichtbar? |

Minimaler Diagnose-Table für künftige Fälle:

```text
Measure | Staff | ActiveClefByPredicate | ActiveClefByWriteProbe | ActiveClefByScopedXmlNode | Candidate | WouldWrite | Written | AlreadyMaterialized | VisibleReadback
```

---

## 4. Drei-Phasen-Regel

Jeder schwierige Fall wird in drei Phasen gelöst:

### Phase 1 – Divergence-Audit

Nur messen, nicht schreiben.

Ziel: beweisen, welche Schicht widerspricht welcher.

Typische Fragen:

- Sieht der Generalization-Audit den Kandidaten?
- Sieht der Gould-/Akzeptanz-Audit ihn auch?
- Sieht der Write-Pfad ihn?
- Sieht die tatsächliche PDF-Quelle ihn?
- Sind Staff/Clef/Voice/Chord in allen Schichten gleich gelesen?

### Phase 2 – Scoped-Fix

Nur die bewiesene Schicht reparieren.

Beispiel aus M005: Nicht die Musik verschieben, sondern den unscharfen privaten Parser durch staff-skopierte Clef-Knotenlesung ersetzen.

### Phase 3 – Visible-Readback-Closure

Nicht behaupten, dass es logisch richtig ist. Beweisen:

- Marker im Render-XML vorhanden
- Marker in selected PDF source vorhanden
- PDF-/Glyph-Audit sieht das Symbol
- Rückkehranker vorhanden
- Guard-Projekt sauber

---

## 5. PreWrite und PostWrite strikt trennen

`WouldWrite` ist nur vor dem Write aussagekräftig.

Vor dem Write:

```text
WouldWrite ja
```

ist ein gutes Signal.

Nach erfolgreichem Write kann

```text
WouldWrite nein
```

korrekt sein, weil der Zielzustand schon materialisiert ist.

PostWrite muss daher anders heißen:

```text
AlreadyMaterialized ja
SourceReadbackAccepted ja
VisibleReadback ja
```

Kein Report darf einen erfolgreichen PostWrite-Zustand deshalb als Fehler melden, weil WouldWrite nicht erneut feuert.

---

## 6. Keine privaten Mini-Parser für musikalisch relevante Entscheidungen

Beim M005-Fall kostete ein privater Attribute-String-Parser viele Iterationen. Er las einen gemeinsamen `<attributes>`-Block, sah den oberen G-Schlüssel und kontaminierte damit die Staff-2-Schlüssel-Erkennung.

Lehre:

- Staff/Clef/Voice/Measure/Chord nie mit lokalen String-Scannern entscheiden.
- Für musikalisch relevante Entscheidungen eine gemeinsame strukturierte Leseschicht verwenden.
- String-Scanner höchstens diagnostisch, niemals als Operator-Wahrheit.

Bekanntes Fehlerbild:

```text
LegacyAnchorActiveStaff2Clef = G
PredicateActiveStaff2Clef = F4
ScopedClefNodeActiveStaff2Clef = F4
SharedAttributeUpperGStaff2FBeforeM005 = 1
```

Interpretation:

```text
legacy_attribute_segment_parser_contaminated_by_upper_staff_g_in_shared_attributes_block
```

---

## 7. Musik-Gate, Render-Gate, Report-Gate und Launcher-Gate trennen

Nicht alles, was rot wirkt, ist Musik kaputt.

Vier getrennte Ebenen:

| Ebene | Prüft |
|---|---|
| Musik-Gate | Pitch, Onset, Dauer, Staff, Voice, Hand/Rhythmus. |
| Render-Gate | RendererExit, PDF geschrieben, PDF-Quelle, Render-XML. |
| Report-Gate | Sind historische Audits richtig eingeordnet? |
| Launcher-Gate | Farbe, Kurzstatus, Exit-Code, Logtail-Darstellung. |

Fehlerbild aus 1049–1051:

- Projektläufe OK
- PDF geschrieben
- aber Kurzstatus rot wegen Aggregat-/Renderer-/Launcherklassifikation

Lehre:

- Erst feststellen, welche Ebene wirklich rot ist.
- Keine Musikreparatur bauen, wenn nur Launcher/Report hysterisch ist.
- Exit 2 kann Audit-/Preflight-Zustand sein, nicht automatisch musikalischer Fehler.

---

## 8. Gould/Bienenkönigin-Prinzip

Elaine Gould ist im Projekt die Notensatz-Autorität („Bienenkönigin“).

Für RegisterPressure-Fälle gilt:

- Wenn der Registerdruck lokal ist,
- ein gleicher-Staff-Schlüsselwechsel die Lesbarkeit verbessert,
- der Span stabil ist,
- und ein Rückkehranker sauber existiert,

ist ein temporärer Schlüsselwechsel im selben System oft besser als Oktavierung, Staff-Wechsel oder Verschieben.

Aber:

- Gould-Akzeptanz ist ein Beweis-/Akzeptanzkriterium.
- Sie ist kein Freibrief für unbewiesene Writes.
- Der konkrete Write braucht weiter scoped Anchor, Invariantenschutz und Visible Readback.

---

## 9. Invarianten und verbotene Abkürzungen

Bei Notensatz-Fixes grundsätzlich schützen:

- Pitch
- Onset
- Dauer
- Voice
- Staff
- Rhythmusabdeckung
- Hand-/Voice-Zusammenhang
- MasterMusicXML, sofern der Fix render-only sein soll
- `WrittenButNonZeroExit` mit stabilem PDF als Renderer-Guard/Regression

Verboten:

- Projektname als Operator-Prädikat
- literal_measure_number als Operator-Prädikat
- historische Hotspot-Rankings als Write-Auslöser
- Ledger-Write als Shortcut, wenn der echte Write-Pfad den Kandidaten nicht sieht
- Oktavierung oder Staff-Wechsel, wenn die akzeptierte Lösung ein gleicher-Staff-Schlüsselwechsel ist
- Report-Kosmetik als Beweis für PDF-Sichtbarkeit

---

## 10. Was beim Start eines neuen Threads zu tun ist

Wenn der Nutzer einen neuen Export hochlädt:

1. `manifest.txt` lesen.
2. `visual_preflight.result.txt` lesen.
3. relevante `Etüde *.result.txt` lesen.
4. Nur bei PDF-/Glyph-Fragen die PDFs/Renderbilder prüfen.
5. Erst dann entscheiden, ob es um Musik, Render, Report oder Launcher geht.
6. Niemals aus Erinnerung behaupten, ein Gate sei grün/rot, wenn der Export es anders sagt.

Minimaler Startsatz für den Assistenten:

```text
Ich prüfe zuerst Manifest, Visual-Preflight und die betroffenen Result-Dateien. Danach trenne ich Musik-, Render-, Report- und Launcher-Gate, bevor ich einen Fix vorschlage.
```

---

## 11. M005-Abschlusswissen

Der M005-RegisterPressure-Fall wurde fachlich geschlossen:

- Etüde 5, M005, Staff 2
- temporärer G-Schlüssel im unteren System
- Rückkehr nach F gebunden
- keine Oktavierung
- kein Staff-Wechsel
- Pitch/Onset/Dauer/Voice/Staff bleiben erhalten
- Lösung stammt aus allgemeinem RegisterPressure-/Gould-Prädikat, nicht aus Takt-Hack

Der historische Diagnoseweg:

1. XML-Stage-Divergenz vermutet
2. Predicate-/WouldWrite-Divergenz bewiesen
3. Insertion-Anchor-Trace gebaut
4. Staff-unscharfer Parserfehler gefunden
5. staff-skopierte Probe gebunden
6. render-only Write materialisiert
7. visible Glyph Readback akzeptiert
8. Report-/Launcher-Hysterese aufgeräumt

Für künftige Fälle soll diese Kette nicht wiederholt werden. Direkt mit Diagnose-Schichten und scoped Parser-/Anchor-Vergleich beginnen.

---

## 12. Kompakte Merksätze

```text
Adresse ist nicht Prädikat.
Divergenz vor Write.
Scoped Fix statt Ledger-Shortcut.
PreWrite WouldWrite, PostWrite AlreadyMaterialized.
Keine privaten Mini-Parser für Staff/Clef/Voice.
Visible Readback schlägt Report-Gerede.
Musik-, Render-, Report- und Launcher-Gate getrennt lesen.
Das typisierte Renderer-Verhalten bleibt Guard, kein Projektname wird Auswahlprädikat.
```


## v0.10.1486 r01 – TransactionalBuildStampFormalGreenDependencyPreflight

- Ausgangspunkt: real grün bestätigtes v0.10.1485 r01.
- Build-Version-/Signaturmarker werden bei Take-Rollback transaktional wiederhergestellt.
- Bericht und Public-Artifact-Commit teilen `src/build_gate/formal_green_contract.rs`.
- Billiger ChangeDependencyPreflight prüft Manifest/FNV, Pfadklasse, Pins und entfernte quellenübergreifende Struct-Feldreferenzen vor MultiRevision-/Capability-Analyse.
- Keine Musikmutation; Prüfung mit genau einem strikten Einzelprojekt-Take.
