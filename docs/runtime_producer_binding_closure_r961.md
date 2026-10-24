# r961 – RuntimeProducerBindingClosure

## Authority und PhysicalStart

Fachlicher AuthorityParent bleibt der in r960 selbst dokumentierte akzeptierte r956 `FinalChordCanonicalIdentityDirectCarryRetirement`. Der eingebettete aktuelle Source-Snapshot ist r960 `FinalChordSameAnchorPeerRegressionExpectationRecovery` und bezeichnet sich selbst als `r960_unbuilt_source`. r961 erhebt r960 dadurch nicht nachträglich zum AuthorityParent.

Der deklarationspflichtige Diff dieses Candidates wird gegen den aktuellen r960-PhysicalStart gebildet. Die r960-Musik-/Planner-Bytes werden byteidentisch bewahrt und in diesem KodexOnly-Take nicht erweitert.

## Ziel

Den tatsächlichen Produktprozess vor seinem Start an die bereits existierende Build-Reuse-Wahrheit zu binden, ohne eine zweite Fingerprint-, Source- oder Revision-Authority zu erzeugen.

Verwendet werden ausschließlich bereits vorhandene Owner:

- `source_build_signature()` für die aktuelle Source-Signatur,
- `file_fnv1a64()` für den Executable-Fingerprint,
- `profile_build_artifact_is_verified()` für die bestehende Version+SourceSignature+Executable-FNV-Verifikation.

## Runtime-Producer-Binding

Nur `ObservedTarget`, also der tatsächlich ausgeführte EtuSchlopf-Produktprozess, erhält die neue interne Provenienzprojektion. Cargo-, Renderer- und sonstige `Inherit`-Subprozesse erhalten sie nicht.

Binding-Zustände:

- `bound`: bestehende Build-Stamps binden Version, aktuelle Source-Signatur und die tatsächlich zu startenden Executable-Bytes gemeinsam;
- `partially_bound`: die Executable-Bytes sind identifizierbar, ihre Source-Herkunft ist aber nicht vollständig bewiesen;
- `unbound`: selbst der ausführbare Produktartefakt-Fingerprint ist nicht hinreichend verfügbar bzw. der Pfad ist kein verifizierbares Produktprofil.

Interne Transportfelder:

- `ETUSCHLOPF_RUNTIME_PRODUCER_BINDING_STATE` wird immer für den ObservedTarget gesetzt;
- `ETUSCHLOPF_RUNTIME_PRODUCER_BINARY_FINGERPRINT` wird nur gesetzt, wenn die tatsächlich gestartete Executable gelesen und identifiziert werden kann;
- `ETUSCHLOPF_RUNTIME_PRODUCER_SOURCE_SIGNATURE` wird nur bei `bound` gesetzt;
- `ETUSCHLOPF_RUNTIME_PRODUCER_VERSION` wird nur bei `bound` gesetzt.

Damit kann ein bloß vorhandener aktueller Source-Baum niemals als Producer-Source ausgegeben werden, wenn die bestehende Build-Verifikation ihn nicht an genau dieses Binary bindet. Nicht belegte optionale Felder werden explizit aus der Child-Umgebung entfernt, damit keine geerbten Altwerte überleben.

## Bewusst nicht enthalten

- keine `ProducerRevision`; hierfür existiert in diesem Take noch kein revisionssicherer Build-Bindungsbeweis,
- keine Native Metrics,
- kein `CanonicalMeasurementSnapshotV1`,
- keine Assistant-Export-Erweiterung,
- kein AIC/EIC,
- keine MusicMutation,
- keine Writer-/Repair-/Release-Authority.

## Regression-Witnesses

Die Launcher-Unit-Tests belegen:

- `bound`, `partially_bound` und `unbound` bleiben getrennt und fail-closed,
- ein fremder/nicht produktiver Target-Pfad kann kein Runtime-Producer-Binding beanspruchen,
- die vier internen Transportnamen bleiben explizit und stabil.

Take 2 darf diese Provenienz anschließend konsumieren; r961 selbst erzeugt noch keinen Measurement Snapshot.
