# NeuroEvidenceProducerSiblingTrustDomainBoundaryV1

EtuSchlopf erzeugt nach einem erfolgreichen normalen Projektlauf ein strikt separates Machine-Evidence-Artefakt fuer NeuroSchlopf r40. Der Producer wird direkt vom normalen Launcherabschluss aufgerufen; das Assistant-Export-Modul ist weder Parent noch Datenquelle des Machine-Evidence-Producers.

Ziel:

```text
C:\Users\wmair\Documents\Programmieren\EtuSchlopfTransfer
```

Dateiname:

```text
etuschlopf_neuro_evidence_<UTC-Zeitstempel>.zip
```

Der v1-Container enthaelt exakt:

```text
manifest.json
decisions.jsonl
```

`decisions.jsonl` enthaelt ausschliesslich numerische, bereits maschinell bestimmte Summary-Evidence des bestehenden `TonalSegmentationEvidenceAudit`. Es enthaelt keinen Assistant-Kontext, keine Uebergabe, kein README, keine Logs und keinen Diagnosefreitext. NeuroSchlopf r40 interpretiert diese Payload noch nicht semantisch.

Manifestvertrag:

```text
artifact-kind = etuschlopf_neuro_evidence
schema-version = 1
producer = EtuSchlopf
corpus-id = etuschlopf-tonal-audit
decision-class = tonal_segmentation_boundary_audit_v1
decision-contract-fingerprint = fnv64:4f2c765d00c527e6
```

`producer-build` wird aus der realen Cargo-Version plus `ZIP_KIND.txt`-Revision gebildet. `created-at-utc`, Bytelaenge und SHA-256 werden aus den tatsaechlich publizierten Bytes erzeugt. Vor dem atomaren Rename wird das temporaere ZIP erneut gelesen und auf exakt zwei flache Allowlist-Dateien sowie byteidentische Manifest-/Payload-Inhalte und Payload-SHA-256 geprueft.

Ein fehlerhafter oder unvollstaendiger Evidence-Export wird nicht publiziert. Er blockiert aber nicht die bereits deterministisch abgeschlossene EtuSchlopf-Musikpipeline: Neuro-Evidence ist fail-closed hinsichtlich der Evidence-Autorisierung und fail-independent hinsichtlich EtuSchlopf.

Human Context und Machine Evidence bleiben verschiedene Artefakte; `assistant_export_*.zip` wird niemals in das Neuro-Evidence-ZIP aufgenommen.

Seit v0.10.1839 r61 gilt zusaetzlich fuer das Verzeichnis selbst: `EtuSchlopfTransfer` ist die Machine-Evidence-Domain. Der Assistant-Export publiziert dort keine Archivkopie und besitzt keinen Transfer-Writer mehr.

Trust-Domain-Topologie:

```text
normaler Launcherabschluss
|- create_assistant_export(...)          -> Human / Assistant Domain
`- publish_neuro_evidence_(...)          -> Machine Evidence Domain
```

`assistant_export.rs` enthaelt keine Referenz auf `neuro_evidence_export`; der Machine-Evidence-Producer liest keine Assistant-Export-Datei.
