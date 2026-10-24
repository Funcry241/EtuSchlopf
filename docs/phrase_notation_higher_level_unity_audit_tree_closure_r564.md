# r564 – PhraseNotationHigherLevelUnityAuditTreeClosure

## Autoritativer Parent und verworfene Kandidaten

Autoritativer Elternstand bleibt EtuSchlopf v0.10.1839 r560
`PhraseNotationInternalHierarchyAudit`.

Die Kandidaten r561 `PhraseNotationHigherLevelUnityAudit`, r562
`PhraseNotationHigherLevelUnityAuditCompileClosure` und r563
`PhraseNotationHigherLevelUnityAuditTestCompileClosure` wurden fail-closed
abgewiesen und sind nicht Bestandteil der erfolgreichen Revisionslinie.

- r561 scheiterte im Produktiv-Compile an drei ueberzaehligen
  `usize`-Dereferenzierungen in neu eingefuehrten read-only `map`-Closures.
- r562 schloss diese drei Stellen; Produktiv-Build und Strict-Clippy liefen
  daraufhin gruen. Der integrierte All-Targets-Test-Compile scheiterte jedoch an
  fehlenden expliziten Imports privater Audit-Hilfstypen im `#[cfg(test)]`-Modul.
- r563 schloss auch diese Test-Imports. Es wurde jedoch noch vor Installation
  vom Take-Change-Control abgewiesen, weil sein Vollbaum den im r560-Parent
  vorhandenen Pfad `tools/etuschlopf_watch/Cargo.lock` versehentlich ausliess.
  Das Gate sah diese unbeabsichtigte Dateiloeschung korrekt als
  `undeclared=["tools/etuschlopf_watch/Cargo.lock"]`.

## Zweck

r564 nimmt den fachlich unveraenderten Higher-Level-Unity-Audit als frischen
Kandidaten direkt auf dem sauberen r560-Elternstand erneut auf. Gegenueber dem
r563-Fachcode wird ausschliesslich die Paket-/Tree-Identitaet geschlossen:
`tools/etuschlopf_watch/Cargo.lock` wird byteidentisch aus r560 uebernommen.

Der Take bleibt `AuditOnly`.

Er untersucht fuer die bereits existierenden
`allow_structural_pending_visual`-Projektionen ausschliesslich positive
hoeherstufige Einheitsevidenz. Daraus wird weiterhin keine sichtbare
Phrasierungsentscheidung abgeleitet.

## Mechanische Compile-Closures aus den verworfenen Kandidaten

### Runtime-/Produktiv-Compile

In den drei neuen Whole-Span-`map`-Closures wird ein `&usize` genau einmal
dereferenziert:

```rust
boundaries[*index]
```

Die bereits bestehenden `filter`-Closures behalten ihre dort typkorrekte
Doppeldereferenzierung. Es gibt keine Boundary- oder Musiksemantikaenderung.

### Test-Target-Compile

Das bestehende `#[cfg(test)] mod tests` importiert explizit die fuer die neue
Audit-Regression verwendeten privaten Elemente aus dem Elternmodul:

- `Attack`,
- `JointParallelSignature`,
- `LaneKey`,
- `ParallelMelodicSignature`,
- `PhraseNotationHigherLevelUnityAuditRow`,
- `annotate_phrase_notation_higher_level_unity`.

Dies aendert ausschliesslich die Namensaufloesung des Testtargets. Die Typen und
die Annotierungsfunktion bleiben privat im bestehenden Audit-Owner; es entsteht
keine neue produktive API und keine neue Writer-Erreichbarkeit.

## Paket-/Tree-Closure

Der r560-Parent enthaelt den verwalteten Watch-Crate-Lockfile
`tools/etuschlopf_watch/Cargo.lock`. r563 liess diese Datei beim Vollbaum-Packen
versehentlich weg. r564 nimmt exakt die r560-Bytes wieder auf.

Der Pfad ist daher **keine fachliche Aenderung** und wird im
Take-Change-Control-Manifest absichtlich nicht als `change=` deklariert. Der
korrekte erwartete Zustand ist, dass er im tatsaechlichen Parent-vs.-Take-Diff
ueberhaupt nicht erscheint.

Ein Root-`Cargo.lock` wird weiterhin nicht aufgenommen.

## Higher-Level-Unity-Audit

Der Audit verwendet weiterhin nur bereits vorhandene Evidenz:

- interne `continuation_supported`-Rollen,
- vorhandene exakte `JointParallelSignature`-Deskriptoren,
- geordnete Whole-Span-Signaturfolgen,
- Same-Lane-/Cross-Lane-Kontext,
- Rollenfolgenkonsistenz,
- terminalen exakten Joint-Parallelismus.

Es gibt weiterhin:

- keine Aehnlichkeitsmetrik,
- keine Edit-Distanz,
- keine fuzzy matching rule,
- keinen UnityScore,
- keine gewichtete Summe,
- keine neue numerische Schwelle,
- keine Mindestzahl von Treffern,
- keine automatische Promotion.

`PositiveUnityDecision` bleibt `unclassified`.

`PhraseMarkSemanticCandidate` bleibt `unclassified`.

`NotationPolicyDecision` bleibt `unchanged_pending_visual`.

## Harte Nicht-Aenderungen

r564 aendert nicht:

- `PhraseBoundaryEvidence`,
- bestehende Boundary-Rollen,
- `PhraseStructure`,
- SlurIntent-Allow/Withhold-Entscheidungen,
- `PhraseNotationIntentProjection`,
- Projektionsstart oder -ende,
- Projektionslaenge,
- Writer-Reachability,
- r421-Persistenzsperre,
- MusicXML oder Render-/Master-MusicXML,
- Pitch, Onset, Duration, Staff oder Voice,
- klingende Ereignisse,
- Pass-Reihenfolge.

Der bestehende Slur-Writer bleibt geschlossen.

## Erwarteter Audit-Output

Der Report erhaelt die Summenzeile
`PhraseNotationHigherLevelUnityAudit` sowie pro bestehender Pending-Visual-
Projektion eine `PhraseNotationHigherLevelUnity[...]`-Zeile.

Alle Werte sind ausschliesslich Beobachtung. Insbesondere gilt weiterhin nicht:

- `exact whole-span match => Slur`,
- `all internal continuation => Slur`,
- `terminal repeat => Slur`,
- `no structural joint => Slur`.

## Fail-closed

Der Take muss die normalen EtuSchlopf-Gates bestehen. Bei Compile-, Test-,
Build-, Clippy-, Change-Control- oder Korpusfehlern bleibt r560 autoritativ.

r561, r562 und r563 duerfen weder als erfolgreiche Zwischenrevisionen noch als
Parent interpretiert oder wiederverwendet werden.
