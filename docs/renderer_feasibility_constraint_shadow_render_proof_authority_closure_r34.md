# Renderer Feasibility Constraint Shadow Render Proof Authority Closure r34

Parent: `0.10.1839 r32`

Rejected predecessor: `r33 RendererFeasibilityConstraintShadowRenderProof`

ChangeClass: `RefactorOnly`

## Gate-Befund aus r33

r33 passierte den Pre-Take-Diff-Gate und wurde transaktional installiert. Beim
Reexec stoppte jedoch der autoritative Architecture-Build-Gate fail-closed mit:

`HARD PRODUCTIVE WRITE BYPASS SHRINK REGISTER VIOLATION`

und identifizierte als neue externe Schreiboberflaeche
`src/audit_only/renderer_system_break_readback.rs::renderer_feasibility_constraint_shadow_render_proof_projection`.

Ursache war die direkte Anlage der target-only Shadow-MusicXML per
`std::fs::write`. Dass das Ziel nur unter `target/` lag, aendert den
Architecture-Vertrag nicht: MusicXML-Dateischreibvorgaenge duerfen keine neue
externe Oberflaeche ausserhalb `structural_write_authority` eroeffnen. Der
Rollback auf r32 war daher korrekt.

## r34-Closure

r34 lockert weder Gate noch Baseline. Stattdessen wird die Shadow-Seed-Schreib-
operation in die bestehende zentrale `verified_render_candidate_sink_authority`
eingebunden. Ein neuer Effekt `RendererFeasibilityShadow` besitzt eine enge
Pfadbindung:

`target/etuschlopf_renderer_feasibility_shadow/<project>/constraint_candidate.render.musicxml`

Der Pfad darf keine `.`- oder `..`-Komponenten enthalten; Dateiname, Shadow-Root
und `target`-Parent muessen exakt passen. Wie die bereits existierenden
Verified-Render-Candidate-Sinks akzeptiert der Effekt nur vollstaendiges,
parsebares `score-partwise`-MusicXML, bindet Pfad und Candidate per Fingerprint,
schreibt zentral und fordert exakten Readback.

Der Audit ruft danach weiterhin ausschliesslich den bereits autorisierten
Horizontal-Layout-Materializer auf der Shadow-Datei auf. Es entsteht weder ein
zweiter Break-Writer noch eine neue produktive Selection-Authority.

## Unveraenderter fachlicher Proof

Fuer Projekte mit beobachtetem Renderer-Systemdrift:

1. DemandBinding und Planner-Envelope werden fail-closed validiert.
2. Der r32-Constraint-Kandidat wird deterministisch erneut berechnet.
3. Das produktive Render-MusicXML wird bytegenau gelesen.
4. Die Shadow-Basis wird ueber die zentrale target-scoped Authority angelegt.
5. Der bestehende autorisierte Horizontal-Materializer materialisiert den
   Candidate auf der Shadow-Datei.
6. Nur der im selben produktiven Lauf bereits erfolgreiche MuseScore-Pfad und
   exakt dieselbe bekannte Command-Variante duerfen wiederverwendet werden.
7. MuseScore rendert nur in das Shadow-Verzeichnis.
8. Der bestehende PDF-System-Break-Readback prueft Plan gegen Shadow-PDF.
9. Master-MusicXML, produktives Render-MusicXML und produktives PDF muessen
   byteidentisch geblieben sein.

Moegliche Status bleiben `NOT_REQUIRED`, `RENDERER_CONFIRMED`,
`RENDERER_REJECTED` und `UNAVAILABLE`.

Immer gelten:

- `CandidateOnly ja`
- `SelectionAuthority nein`
- `ProductiveWriterAuthority nein`
- `PolicyWrite nein`

## Harte Grenzen

r34 aendert nicht:

- Demand-Formel oder Measure-Demands,
- Capacity oder Thresholds,
- PageCount-Preference,
- produktiven Joint Plan,
- produktive Horizontal-Materialisierung,
- Renderer-Discovery oder produktive Renderer-Command-Reihenfolge,
- musikalische Semantik.

Die Productive-Write-Bypass-Baseline bleibt unveraendert. Der r33-Gate-Treffer
wird durch Zentralisierung geschlossen, nicht durch Registrierung einer neuen
externen Ausnahme.

## Regression

Die drei bereits in r33 vorgesehenen Tests bleiben Bestandteil des Schnitts:
Candidate-Grenzen -> Materializer-Direktiven, exakte Wiederverwendung einer
bekannten Renderer-Command-Form und Bindung der Runtime-Evidence an eine reale
MuseScore-Datei. Erwarteter Gesamtbestand: **759/759**.

## Naechster Schritt

Zuerst `--take "Etüde Nr. 6"`. Nur wenn Architecture-Gate, Build, Clippy,
759/759 Tests, produktiver Lauf und Shadow-Renderer-Proof gruen sind, folgt
`--all`. Kein `--force`. Kein lokaler Taktfix. Kein Threshold-Tuning.
