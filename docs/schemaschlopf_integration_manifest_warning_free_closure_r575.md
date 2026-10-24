# r575 – SchemaSchlopfIntegrationManifestWarningFreeClosure

## Parent und Anlass

Autoritativer Parent ist EtuSchlopf `0.10.1839 r574` `SchemaSchlopfDependencyAdmissionBoundary`, real gruen installiert.

Der erste Runtime-Kandidat fuer r575 wurde im Pre-Copy-Gate korrekt fail-closed abgewiesen. Ursache war nicht der SchemaSchlopf-Validator, sondern die in dessen upstream `Cargo.toml` enthaltene Zeile `warnings = "warn"`. EtuSchlopfs Warning-Free-Build-Integrity erlaubt diese historische Warning-Debt ausschliesslich im EtuSchlopf-Root; ein neu eingebrachtes Cargo-Manifest darf sie nicht neu einfuehren. Der verworfene Runtime-Kandidat ist keine Authority.

## Entscheidung

Die Warning-Free-Grenze wird nicht gelockert. Insbesondere entsteht kein Exempt fuer `tools/qualified_components/`.

Die SchemaSchlopf-r020-Bindung unterscheidet ab r575 zwischen den qualifizierten r020-Rust-/Versions-/Provenienzbytes und dem EtuSchlopf-lokalen Cargo-Integrationsmanifest. Alle gebundenen SchemaSchlopf-Rustdateien, `VERSION` und `SOURCE_MANIFEST.sha256` bleiben exakt auf den bereits in r574 festgelegten r020-Bytes. Nur das lokale aktive Komponentenmanifest wird gegenueber dem upstream r020-Manifest in genau einer Richtung gehaertet:

```toml
[lints.rust]
warnings = "deny"
```

Der restliche Manifestinhalt bleibt identisch. Das ist keine Weiterentwicklung des SchemaSchlopf-Validators und keine neue Validator-Autoritaet, sondern eine EtuSchlopf-seitige Build-/Packaging-Haertung.

## Provenienz

`SOURCE_MANIFEST.sha256` bleibt absichtlich byteidentisch zum finalen r020-Export und dokumentiert weiterhin das upstream SchemaSchlopf-Artefakt, darunter dessen originales `Cargo.toml` mit SHA-256 `909f10f3c8964f96d771c274d47de13eb04a07b0d965fe529ce2de36cfe27453`.

Das aktive EtuSchlopf-Integrationsmanifest ist separat exakt gebunden:

- FNV1A64: `a0c084e022a67925`
- SHA-256: `fef4aef554b438c73e5fd2ced9387790a18fc7c07de5f195d5f906e8749ee912`

Es unterscheidet sich vom upstream Manifest ausschliesslich durch `warnings = "deny"` statt `warnings = "warn"`.

## Neuer geschlossener Admission-Marker

Der in r574 vorbereitete Marker `..._v1` wird fuer neue Takes nicht mehr akzeptiert. Der nachfolgende Runtime-Take muss exakt verwenden:

```text
schemaschlopf_r020_dependency_admission=closed_exact_local_path_0_1_0_r020_warning_deny_v2
```

Der Projection-Gate erwartet fuer `tools/qualified_components/schemaschlopf_r020/Cargo.toml` exakt den gehaerteten Hash und zusaetzlich explizit die Zeile `warnings = "deny"`. Andere Manifestabweichungen bleiben fail-closed.

## Scope

r575 ist `KodexOnly`. Es aendert weder Root `Cargo.toml`/`Cargo.lock` noch produktive Pipeline, MusicXML, SchemaSchlopf-Rustcode, normative XSD-Bytes oder Musik-/Writer-/Repair-/Retry-/Recovery-Semantik. Erst ein nachfolgender eigener `ValidationOnly`-Take darf die gehaertete lokale Komponentenprojektion materialisieren und das Persisted-Byte-XSD-Gate aktivieren.
