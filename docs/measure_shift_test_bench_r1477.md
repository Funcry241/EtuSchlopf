# r1477: Verschiebungs-Pruefstand fuer Writer-Tests

Parent: r1476. Class: RefactorOnly. Nur Tests und Dokumentation; keine Pipeline-,
Gate-, Writer- oder Musikaenderung. Autor: Dachs, im Auftrag von Wolfgang.

## Anlass

- Die Gate-Checkliste empfahl `number="probe"` als Taktnummer in Test-XML. Das erfuellt
  die Corpus-Independence-Regel, bricht aber Binder, die Taktnummern numerisch lesen.
- Verschiebungsfestigkeit (Umnummerierung, vorangestellte Takte, Duplikate) wurde bisher
  je Writer und je Fall von Hand getestet und deckte nur einzelne Fixtures ab.

## Inhalt

`tests/support/synthetic_score.rs` stellt einen synthetischen Einzeltakt in zehn
Umgebungen: umnummeriert; drei Takte vorangestellt; vorangestellte Takte mit halben
und fuenffachen Divisions; Attribute vom Vortakt geerbt; ein Takt angehaengt; zweimal
hintereinander; Part-ID umbenannt; Metadaten ergaenzt; Stimmlabel umbenannt; kombiniert.
`assert_shift_invariant` verlangt im Falltakt dasselbe Ergebnis wie im Einzeltakt (bei
geerbten Attributen und umbenannter Stimme in entsprechend umgeformter Gestalt) und in
allen umgebenden Takten keine Aenderung. Taktnummern entstehen nur aus Variablen.

Nutzer:
- r1472-Schutzregel (`resolve_scorewide_metric_beam_continuity_candidates`): alle 15
  vorhandenen Fixtures P1–P10, P8 mit und ohne Akkordbalken, G1–G4.
- r1476-Bruecke (`apply_structural_rest_bridge_beam_variant_a_render_write` mit
  synthetischer Sperre): getrennte und gemeinsame Stimmlabels.
- Selbsttests: ein an die erste Position gebundener und ein an die Taktnummer gebundener
  Spiel-Writer muessen abgelehnt, ein rein lokaler muss angenommen werden.

Beide Einbindungen erfolgen per `include!` in den vorhandenen Support-Dateien; die
Einbindestellen in hash-gepinnten Quelldateien bleiben unveraendert.

Doku: Gate-Checkliste in 040 korrigiert (keine Wort-Taktnummern; Variablenname `scope`
loest zusammen mit Kontrollfluss die Selector-Pruefung aus); 010 Muster 31; 050 auf den
Stand r1476 gebracht, inklusive Befund zum spaeten RestBridge-Pfad.

## Abnahme

Alle 20 XMLs bytegleich zu r1476, PDFs pixelgleich, Strict-Tests und Clippy gruen.
Wird ein Verschiebungstest rot, ist das ein Befund ueber eine echte Positions- oder
Labelabhaengigkeit und kein Anlass, die Variante zu entfernen.
