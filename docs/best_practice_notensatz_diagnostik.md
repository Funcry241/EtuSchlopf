> **ARCHIVHINWEIS v0.10.1449:** Die folgenden Etueden-, Takt- und M005-Angaben sind historische Diagnose- und Antiselektorbeispiele. Sie sind kein aktueller Projektstatus, keine aktuelle Diagnose und keine Reparaturprioritaet. Aktive musikalische Arbeit erfordert frische Evidenz aus aktuellen Artefakten oder einem konkreten Nutzerbefund.

# EtuSchlopf Best Practice: Notensatz-Diagnostik vor Write

Stand: v0.10.1053  
Auslöser: langer RegisterPressure-/M005-Fall in Etüde 5.

## Leitsatz

EtuSchlopf wird nicht schneller, indem früher geschrieben wird. EtuSchlopf wird schneller, indem früher die richtige Divergenz gemessen wird.

Bei hartnäckigen Notensatzfällen ist der erste Schritt daher kein Fix-Write, sondern ein Audit, der zeigt, auf welcher Ebene zwei Wahrheiten auseinanderlaufen.

## 1. Adresse ist erlaubt, Taktlogik ist verboten

Interne Adressen wie `M005` sind erlaubt und erwünscht.

- `M` bedeutet Measure/Takt.
- `005` bedeutet Takt 5, dreistellig gepolstert.
- `M005` ist eine Report-Adresse, keine Regelbedingung.

Erlaubt:

```text
Kandidat: M005 (Takt 5), Staff 2
Auslöser: staff_local_register_pressure + stable_return_anchor
```

Verboten:

```text
if project_name == "Etüde 5" && measure_number == 5 {
    setze G-Schlüssel;
}
```

Die Taktadresse sagt, wo der Fall liegt. Sie darf nie erklären, warum eine Notationsentscheidung getroffen wird.

## 2. Fünf-Schichten-Diagnose

Jeder neue hartnäckige Fall wird zuerst über diese fünf Schichten geprüft:

| Schicht | Leitfrage | Beispiel aus M005 |
|---|---|---|
| Adresse | Wo liegt der Fall? | `M005 (Takt 5), Staff 2` |
| Prädikat | Warum ist es ein Kandidat? | Registerdruck, Lesbarkeit, alternativer G-Schlüssel-Gewinn, stabiler Rückkehranker |
| Quelle | Welche XML-Welt sieht den Kandidaten? | Master, Render-XML, FreshFinalExport, SelectedPdfSource |
| Write-Anker | Wo würde konkret eingefügt und zurückgeschaltet? | TrebleInsertStart, BassReturnStart |
| Readback | Ist der Effekt wirklich sichtbar? | Render-XML-Marker + PDF-Glyph `G2` + Rückkehr `F4` |

Erst wenn die Schichten widerspruchsfrei sind, darf ein Write diskutiert werden.

## 3. Standard-Audit-Zeile für neue Fälle

Für neue Clef-/Register-/Layoutfälle soll früh eine kompakte Zeile entstehen:

```text
Measure | Staff | ActiveClefByPredicate | ActiveClefByWriteProbe | ActiveClefByScopedXmlNode | Candidate | WouldWrite | Written | VisibleReadback | BlockReason
```

Diese Zeile hätte den M005-Fehler deutlich früher sichtbar gemacht, weil der alte Probe-Pfad `G` meldete, während Predicate und staff-skopierter XML-Knoten korrekt `F4` sahen.

## 4. Keine privaten Mini-Parser für musikalisch wichtige Entscheidungen

Der M005-Durchbruch kam, weil ein alter Probe-/Anchor-Pfad einen ganzen `<attributes>`-Block las und dadurch den oberen G-Schlüssel auf Staff 2 kontaminierte.

Regel:

- Staff, Voice, Clef, Measure, Chord und Duration dürfen nicht über lokale String-Scanner entschieden werden.
- Für entscheidungsrelevante Logik muss eine gemeinsame, staff-skopierte Leseschicht verwendet werden.
- Lokale String-Scanner sind nur als reine Diagnose erlaubt und müssen dann als Legacy/Probe gekennzeichnet sein.

## 5. PreWrite und PostWrite nicht vermischen

`WouldWrite` ist nur vor dem Write eine sinnvolle Erfolgsfrage.

Nach einem erfolgreichen Write kann `WouldWrite nein` korrekt sein, weil der Zielzustand bereits materialisiert ist.

Daher immer trennen:

```text
PreWrite:  WouldWrite ja/nein
PostWrite: AlreadyMaterialized ja/nein
Readback: VisibleGlyph ja/nein
```

Ein PostWrite-Readback darf einen alten PreWrite-Blocker überstimmen, muss ihn aber als Historie markieren.

## 6. Vier Gate-Arten getrennt halten

Ein roter Kurzstatus darf nicht verschleiern, welche Ebene wirklich betroffen ist.

| Gate-Art | Prüft | Darf nicht verwechselt werden mit |
|---|---|---|
| Musik-Gate | Pitch, Onset, Dauer, Staff, Voice, Notationsentscheidung | Renderer-Exit |
| Render-Gate | MuseScore/PDF geschrieben, RendererExit, Glyph sichtbar | musikalischer Entscheidung |
| Report-Gate | historische Audits korrekt eingeordnet | Musikfehler |
| Launcher-Gate | Farbe, Exit-Code, Kurzstatus, Logtail | Notensatzlogik |

M005 war musikalisch gelöst, bevor der Kurzstatus sauber war. Solche Ebenen müssen künftig getrennt diagnostiziert werden.

## 7. Drei-Phasen-Regel für hartnäckige Fälle

Neue komplizierte Fälle laufen maximal über diese Reihenfolge:

1. **Divergence-Audit**  
   Welche Schicht widerspricht welcher?

2. **Scoped-Fix**  
   Nur die bewiesene defekte Schicht reparieren. Keine Musikverschiebung, kein Shortcut.

3. **Visible-Readback-Closure**  
   Nicht nur logisch plausibel: Render-XML und PDF/Glyph müssen den Effekt bestätigen.

Report-Kosmetik kommt danach, nicht davor.

## 8. Abnahmekriterium für wiederverwendbare Regeln

Ein akzeptierter Einzelfall darf nur dann verallgemeinert werden, wenn der Beweis ohne Projektname und ohne Taktzahl formuliert ist.

Für M005 war der allgemeine Beweis:

```text
staff_local_register_pressure
+ active_clef_readability_penalty
+ alternate_same_staff_clef_gain
+ stable_register_span
+ return_anchor_proof
+ visible_readback
```

Nicht der Beweis war:

```text
Etüde 5, Takt 5
```

## 9. Praktische Kurzform

Wenn ein neuer Fall zäh wird, lautet die Reihenfolge:

```text
Adresse finden.
Prädikat beweisen.
XML-Quelle vergleichen.
Write-Anker staff-skopiert prüfen.
Render/PDF sichtbar zurücklesen.
Erst dann schreiben.
```

Das ist die M005-Lehre.
