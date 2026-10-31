# PersistedResultVocabularyTransferDomainSecurityClosureExplicitRejection

Version: 0.10.1839 r61
ChangeClass: OutcomeProjection

## Parent / rejected predecessor

Parent ist der committed DARK-GREEN r58. r59 wurde vor Copy korrekt abgelehnt, weil sieben inventarisierte Security-/Negativcheck-Symbole entfernt worden waeren. r60 wurde ebenfalls vor Copy abgelehnt, weil der wiederhergestellte `publish_assistant_export_archive`-Helper zwar fail-closed war, aber als Tail-`Err(...)` keine explizite Scanner-Rejection-Surface trug. r61 lockert das Gate nicht: derselbe pensionierte, nicht schreibende Helper verwendet explizit `return Err(...)`, sodass die bestehende SecurityNegativeCheck-Capability erhalten bleibt.

## Ziel

r61 schliesst zwei nach r58 bewusst verbliebene Oberflaechen, ohne Musik-,
Layout-, Renderer-, Planner-, Writer- oder Selection-Verhalten zu aendern.

### 1. Persistierte Result-Texte

Die produktive Page-Count-Preference ist seit r58 First-Class und kein Shadow
mehr. Der bestehende Console-Capture bleibt als interne Eingangsoberflaeche
kompatibel, aber die persistierte `.result.txt`-Projektion verwendet ab r61
First-Class-Vokabular:

- Section: `HORIZONTAL-LAYOUT-PAGE-COUNT-PREFERENCE`
- Prefix: `ETUSCHLOPF: HorizontalLayoutGlobalPageCountPreference:`
- Feld: `nonterminalThin` statt `legacyThin`

Echte historische/Regression-Surfaces bleiben unveraendert historisch benannt.
Insbesondere `RendererFeasibilityConstraintShadow*`, `ShadowRest*` und
`LegacyMusicMutation` werden nicht umetikettiert, weil sie weiterhin
ausdruecklich Shadow-, Retirement- oder Legacy-Diagnostik bezeichnen.

### 2. Assistant-Export / Transfer-Trust-Domain

Die fruehere chronologische Assistant-Export-Archivkopie nach

`%USERPROFILE%\Documents\Programmieren\EtuSchlopfTransfer`

wird vollstaendig pensioniert.

Der Human-/Assistant-Export bleibt ausschliesslich als
`assistant_export_latest.zip` im EtuSchlopf-Projekt-Root (plus internem
Diagnose-Staging unter `target/`) erhalten. `assistant_export.rs` besitzt keinen
Writer mehr in das Transferverzeichnis und keine
`ETUSCHLOPF_ASSISTANT_EXPORT_DIR`-Konfiguration.

`EtuSchlopfTransfer` bleibt damit die Machine-Evidence-Domain fuer
`etuschlopf_neuro_evidence_*.zip`.

Die bestehende Build-Security prueft zusaetzlich fail-closed, dass
`assistant_export.rs` weder einen Transferpfad noch eine produktive Archive-Publish-Surface
wieder einfuehrt. Die sieben historisch gepinnten Archive-/Negativcheck-Symbole bleiben
ausschliesslich unter `#[cfg(test)]` als Retirement-Evidenz erhalten. Der historische
`publish_assistant_export_archive`-Name bezeichnet dort eine absichtlich verweigernde
Testoberflaeche, die keine Datei oder kein Zielverzeichnis erzeugen kann. Die
Verweigerung ist als explizites `return Err(...)` formuliert, damit der
Semantic-Capability-Scanner den bestehenden Negative-Security-Beweis weiterhin
inventarisiert.

## Nicht geaendert

- keine Musikmutation
- keine Layout-/Page-/System-Entscheidung
- keine Renderer-Feasibility- oder No-Good-Semantik
- keine Writer-/Materialization-Authority
- keine Threshold-/Capacity-/Demand-Aenderung
- keine Neuro-Evidence-Payload-Aenderung
- keine Persistenz ausser der gewollten Entfernung des Human-Export-Mirrors
- keine Gate-Lockerung

## Abnahme

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Erwartet:

- Build/Clippy/Tests gruen
- Etüde 6 weiterhin Renderer `MATCH` und DemandBinding `BOUND`
- `.result.txt` enthaelt die neue First-Class Page-Count-Section
- keine datierte `assistant_export_*.zip` wird nach `EtuSchlopfTransfer` geschrieben
- `etuschlopf_neuro_evidence_*.zip` wird weiterhin separat publiziert
