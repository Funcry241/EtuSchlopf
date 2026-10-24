# Retired Beam Reference Layer Removal

## Ausgangspunkt

Seit der tokengebundene `AuthorizedCursorNeutralWriter` die strukturelle
Cross-Staff-Transformation kontrolliert, besitzt der historische
`BeatAlignedCrossStaffTail`-Direktwriter kein produktives Schreibrecht mehr.
Der verbliebene Shard enthielt trotzdem noch:

- einen zweiten read-only Strukturdetektor,
- ein fest auf `written = false` gesetztes Audit,
- einen test-only XML-Direktwriter mit eigener Balkenzustandsbildung,
- PDF-Quellen- und Pipeline-Plumbing fuer den pensionierten Pfad,
- eine Paritaetskopplung des aktiven Snapshot-Adapters an den alten Detector.

## Entfernung

Der gesamte pensionierte Shard wird geloescht. Der Orchestrator fuehrt sein
Audit nicht mehr durch und transportiert keinen unmoeglichen Writerzustand mehr
bis zur PDF-Quellenentscheidung. Der aktive Snapshot-Adapter bewertet nur noch
seine eigene typisierte Detektion; die Referenzzaehler und der
`bind_reference_detector`-Schritt entfallen.

Die Writer-Grenze bleibt fail-closed. Sie prueft weiterhin, dass es im
Orchestrator keinen direkten Aufruf des alten Writer-Namens gibt, benoetigt
aber keine eingebettete Quelltextkopie des geloeschten Shards mehr.

## Unveraenderte produktive Autoritaeten

- `StructuralSnapshot` und identitaetsfreier Detector,
- semantische Ereignis- und Metamorphosebeweise,
- move-only Autorisierungstoken,
- `AuthorizedCursorNeutralWriter`,
- gemeinsames `BeamPlan` einschliesslich Hooks,
- produktive Partial-Beat-, Silent-Voice-, Cross-Staff- und
  Lower-Dual-Edge-Regeln,
- Writer-Sinks und Pipeline-Reihenfolge.

## Erwartete Abnahme

- Strict Clippy Exit 0,
- keine neue Lintunterdrueckung,
- Master- und Render-MusicXML gegen v0.10.1605 nur mit geaenderter
  Werkzeugversion,
- sechs PDF-Seiten pixelidentisch,
- keine neue Musikmutation.
