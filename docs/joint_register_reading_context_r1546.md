# r1546 - JointRegisterReadingContextDefinition

Parent: accepted r1545 / 0.10.1839.
Accepted take SHA-256:
`0047faf4013a164f59841654b596581844da6d74389533277d399b3a20fd80c2`.
Current source snapshot SHA-256:
`a8cdf1f5adbb8a1fd24a8909a0e11a6d55651e9a40444444ff7b945663dab7db`.
The snapshot differs from the delivered parent archive only by its root
Cargo.lock, which is local state and is omitted from this incoming take.

ChangeClass: KodexOnly. This is the separate causal-definition stage required
by causal_class_first_v1, before new selector-sensitive implementation.

## Ausgangspunkt und bestaetigte Praeferenz

Der Nutzer bewertet die experimentelle, durchgehend im Violinschluessel
notierte untere Stimme von Etuede 3, Takte 15-18, als deutlich besser.
Im Vergleich: fuenf gesetzte Schluesselwechsel werden zu einem, vier kurze
Oktavierungsspannen entfallen. Ueber 38 sichtbare Tonobjekte steigt die
Hilfslinien-Summe von 6 auf 28; das Maximum bleibt drei je Tonobjekt.
Die Summe zaehlt je Tonobjekt, Akkordtoene einzeln, keine verschiedenen Striche.
MuseScore 4.7.4 erzeugt aus beiden vollstaendigen Fassungen bytegleiche
MIDI-Dateien. Beide bleiben vier Seiten; Seite 2 wird geringfuegig neu gesetzt.

Die bestaetigte Lese-Praeferenz lautet: Eine stabile Lesereferenz darf
zusaetzliche vertretbare Hilfslinien kosten, wenn kurze Umschaltungen entfallen.
Das ist keine pauschale Violinschluesselregel und keine Freigabe fuer einen
numerischen Grenzwert. Die manuelle Variante ist kein produktiver Take.

## Was der bestehende Code tatsaechlich zeigt

- `src/registernotation/mod.rs`, `lesbarkeitswert` und `fenster_lesbarkeitswert`,
  vergleichen fuer das untere System Bass-/Violinschluessel-Kosten ueber
  Fenster. Die tatsaechliche spaetere Schluesselfolge wird dort nicht gebunden.
- `src/registernotation/gemeinsame_spanne.rs` belaesst die untere Domaene
  ausdruecklich unveraendert: Der Kommentar und Status
  `AlternativeSchluesselAutoritaetUngeklaert` benennen den fehlenden
  gemeinsamen Schluesselkontext. Diese Schutzgrenze bleibt erhalten.
- `src/structural_write_authority/rustig_clef_planning.rs` optimiert bereits
  ganze Schluesselfolgen und berechnet reale Hilfslinien mit Wechselkosten.
  `scan_lower_pitches` konsumiert dabei die vorhandene Oktavierungsdarstellung.
  Der Schluesselplaner darf nicht nebenbei Oktavspannen entfernen.

Damit ist ein fehlender gemeinsamer Vergleich fachlich begruendet. Nicht
bewiesen ist, dass die manuelle Variante unter den bisherigen Kosten gewinnen
wuerde oder bereits durch dieselbe fruehe Authority materialisierbar ist.
Auch die Ursache jedes sichtbaren Einzelwechsels ist damit nicht bewiesen.

Die neue Definition betrifft den gemeinsamen Schluessel-/Oktavkontext.
Sie dupliziert weder die vorhandene rhythmische Gestenklasse noch die bereits
implementierte Common-Span-Suche im oberen System. Die bisherigen Vertraege
bleiben bytegleich. Es gibt noch keinen Klassifikations- oder Reparaturcode.

## Kleinster weiterer Arbeitsgang nach Annahme

1. Die tatsaechlichen Eingaben beider vorhandenen Owner auf denselben
   erhaltenen musikalischen Kontext binden; fehlende Bindung bleibt UNKNOWN.
2. Mit synthetischen Faellen einen vollstaendigen gemeinsamen Vergleich
   beschreiben, insbesondere unveraenderten Anschluss, Gegenstimmen und Ties.
3. Die Lese-Praeferenz als separate, begruendete Vergleichspolitik festlegen;
   Maximal- und Gesamt-Hilfslinienlast sowie Umschaltungen getrennt ausweisen.
4. Erst danach Korpusverteilung und musikalischen Folgetake pruefen, mit
   bestehenden Authoritaeten, unveraenderter Musik und frischem PDF-Nachweis.

Dieser Take friert ausschliesslich die Definition und diese Arbeitsnotiz ein.
Alle Rust-Programmquellen, Tests, Cargo-Dateien, Launcher, Gewichte, Gates,
Writer und musikalischen Entscheidungen sind unveraendert. Die bestehende
manuelle Vergleichs-PDF ist Evaluation, kein normativer Test und kein
automatisches Reparaturziel. Vollstaendige native Annahme steht aus.

Anwenden auf r1545:

```text
.\run.cmd --take --all
```

Der neue Export muss den hashgebundenen Annahmebeleg liefern. Der historische
r1545-Lauf meldet Build/Clippy 0, 2633 bestandene Tests, einen ignorierten Test
und zehn gruene Etueden auf 38 Seiten; dies sind Parent-Belege, keine neuen
r1546-Testergebnisse.
