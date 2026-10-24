# SourceSnapshotV1 Projection Boundary – v0.10.1725

Take 2A der Observability Session Architecture fuehrt noch keinen Transport ein.

Der interne `SourceState` bleibt Runtime-Domain. Ein separater transportneutraler Baustein
`src/observation_protocol.rs` projiziert daraus `SourceSnapshotV1`. Der Wire-Vertrag ist
versioniert (`ProtocolVersion 1`), deterministisch, vollstaendig pro Source und klein genug
fuer die spaetere Loopback-Datagrammgrenze. Der interne State wird nicht direkt serialisiert.

Der Snapshot umfasst Session-ID, Source/Parent, SourceRole, Revision, Status, Stage,
optionale Activity, ProjectLabel und Progress. `last_seen` und `last_state_change` gehoeren
weiterhin nicht in EtuSchlopf; sie entstehen spaeter lokal in der Watch.

Die binaere v1-Kodierung besitzt Magic + ProtocolVersion, laengenpraefigierte UTF-8-Felder,
explizite Enum-Codes und optionale Progresswerte. Sie enthaelt keine Artefakte, MusicXML,
PDF-Bytes, Reports oder Solverdaten. Gleicher Snapshot erzeugt bytegleiche Wire-Daten.
Decoder und Encoder validieren die Role/Stage-Kombination und akzeptieren keine
Trailing-Bytes.

Dieser Take fuehrt ausdrücklich noch nicht ein:

- UDP oder irgendein Socket-I/O,
- Reannouncement,
- Transportqueue,
- MQTT,
- Watch/GUI,
- neue Konsolenausgabe,
- musikalische Mutation.

Damit kann Take 2B den best-effort Sender gegen einen bereits gruenen und separat
bewiesenen Protocol Contract anbinden.
