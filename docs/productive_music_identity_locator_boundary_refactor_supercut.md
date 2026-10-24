# ProductiveMusicIdentityLocatorBoundaryRefactorSuperCut

Version: `v0.10.1768 r01`  
Change-Klasse: `RefactorOnly`  
Basis: bytegenau verifizierter, real gruener `v0.10.1765 r01`-Take (`SHA-256 5c17e8e6fc3bf01f13b7a426a63997696708429d479b1ede37d6209dfd721246`).

## Ziel

Dieser Wurf schliesst drei bereits vorhandene produktive Identity-/Locator-Grenzen, ohne eine neue Musikregel, Gewichtung, Schwelle, Passreihenfolge oder Writerfamilie einzufuehren. Die roten Zwischenstaende 1766/1767 sind ausdruecklich keine Basis dieses Takes.

## 1. Extern aktivierbaren Layout-Relief-Writer pensionieren

Der bisherige `VisualLayoutReliefControlledRuntimeRenderWrite` konnte ueber Prozess-Environment aktiviert werden und erhielt seinen Measure-Locator sowie die gewuenschte Delta-Art ebenfalls aus Environment-Werten. Im normalen Lauf war der Pfad deaktiviert, besass aber produktives Render-Schreibrecht.

Entfernt werden deshalb die komplette Controlled-Runtime-Writerfamilie, ihre Locator-/Delta-Parser und Statusprojektion sowie die ausschliesslich dafuer vorhandene `LayoutRelief`-Sink-Capability. Der audit-only `VisualPdfLayoutReliefRulePreflight` bleibt erhalten.

## 2. Human-Potential-Target strukturell binden

Der bisherige Human-Potential-Pfad erzeugte einen strukturell gefundenen Target-Measure zunaechst korrekt aus dem aktuellen MusicXML, serialisierte ihn danach jedoch in `human_potential_candidate_details` und parste den Reporttext spaeter wieder zu rohen `usize`-Selektoren.

`HumanPotentialStructuralTarget` wird nun direkt aus derselben `safe_sets`-Entscheidung erzeugt. Seine Felder sind privat. Der Korpus darf die vom Arrangement-Owner erzeugten Targets sortieren, pruefen und als opaken Wert zurueckreichen, kann aber keine rohe Taktnummer als Human-Potential-Ziel konstruieren. `human_potential_candidate_details` bleibt reine Reportprojektion und besitzt kein Rueckfuehrungsrecht mehr.

## 3. Direction-Writer nur mit strukturellem Measure-Locator

`insert_direction_into_measure_authorized()` akzeptierte bisher einen frei konstruierbaren `&str`-Measure-Locator. Die neue Grenze `StructuralMeasureLocator` besitzt private Felder und wird nur aus der aktuell uebergebenen Score-Struktur erzeugt (`first_structural_measure_locator` bzw. `fractional_structural_measure_locator`). Der Writer akzeptiert ausschliesslich diesen gebundenen Locator.

Die bestehende Expression-Polish-Positionierung bleibt fachlich identisch: erster Takt sowie dieselben strukturellen Fraktionen `0.40`, `0.60`, `0.75`, `0.84`. Es wird keine neue Position eingefuehrt.

## Invarianten

- kein neuer Writer;
- keine neue Musikregel;
- keine Etueden-, Takt-, Pitch- oder Feedback-Sonderregel;
- keine Gate-Lockerung;
- keine Security-/Build-Control-Datei wird ausser dem deklarativen `take_change_class.txt` veraendert;
- der Key-Signature-Span-Pfad des gruene 1765-Basisstands bleibt unveraendert;
- erwarteter Standardlauf gegen 1765: Master-/Render-MusicXML bis auf Versionsmetadaten invariant, PDF invariant.

## Sicherheitsarchitektur

Dieser `RefactorOnly`-Wurf ist absichtlich noch nicht der globale Provenance-Gate. Er schliesst zuerst die bekannten Produktgrenzen auf dem autoritativ gruene SourceTree. Der anschliessende getrennte `SecurityOnly`-Wurf darf danach dieselben Fehlerklassen global und adversariell verhindern, ohne Produkt- und Security-Aenderungen zu vermischen.
