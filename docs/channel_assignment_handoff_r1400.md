# r1400 – Arbeitsübergabe Kanalzuordnung und native Staff-Erhaltung

Parent: accepted r1399. ChangeClass: MusicMutation.
Advisory only: kein neuer Selector-, Repair-, Writer-, Receipt- oder Release-Vertrag.
Die vorhandenen kausalen Klassen bleiben bytegleich zum Parent. Der produktive
Teil dieses Takes verweigert unsichere Oktavspannen an eingehenden Tie-Grenzen;
siehe `docs/selector_derivation_contract_incoming_tie_octave_boundary_safety_r1400.md`.
Reale Fundstellen, Eingaben und Vergleichslisten werden nur im separaten
Evaluationspaket geliefert; sie sind keine normative Test- oder Produktidentität.

## Ergebnis und Grenze

Die externe Gegenprobe kann erschöpfte Import-Slots durch Umverteilung weniger
pitched Voice-Elemente freigeben, ohne ein Pausenlabel oder den bisherigen
Overlapping-Hauptkanal zu ändern. Trotzdem bleibt eine bereits beobachtete native
Staff-Abweichung an unverändert serialisierten gemischtstufigen Akkorden sichtbar.
Das widerlegt eine notwendige Verursachung durch den zuvor getesteten Pausenumzug.
Es beweist nicht direkt die ursprüngliche native Ausgabe vor jeder Kanaländerung
und nicht die musikalische oder kanonische Zulässigkeit der vorgeschlagenen Hosts.

Zwei unabhängige Pflichten bleiben bestehen:

- Vollständige partweite Import-Kapazität: keine ausgeschlossene erforderliche
  pitched Occurrence unter dem bekannten Consumer-Profil.
- Tatsächliche Staff-Erhaltung: jedes gebundene Vorkommen behält seinen Staff im
  nativen Consumer-Objekt; Tonanzahl und vorhandene Tie-Marken reichen nicht.

Die Staff-Dimension bleibt im Comparator. Keine force-Freigabe, kein Chord-Split,
kein Handtransfer, kein künstlicher Head und kein nachträglicher Output-Cleaner.
Der vollständige Materialbeleg bleibt Vorbedingung einer produktiven Portierung.

## Kürzester Owner-Pfad für den nächsten Bearbeiter

1. Unveränderte Parent-Klassen lesen:
   `docs/selector_derivation_contract_partwide_renderer_channel_capacity.md` und
   `docs/selector_derivation_contract_chord_scoped_staff_projection.md`.
2. Im bestehenden Owner `src/structural_write_authority/serialization_plan.rs`
   den Weg von `FrozenVoiceAttackEvidenceRow` über
   `build_frozen_serialization_voice_channels_for_canonical` und
   `bind_musicxml_voice_labels` zum `FrozenSerializedVoicePlan` verfolgen.
   Kanäle sind technische Ressourcen; tatsächliche Source-Rollen sind bindend.
3. `FinalPdfSourceOccurrenceContinuation` und den transportierten
   `FinalChordTransactionLocalOutputAlphaReceipt` verwenden. Source-/Attack-/Fragment-
   Identität vorwärts tragen; nicht aus spätem Pitch, Onset oder XML-Voice erfinden.
4. `FinalChordNonPitchedContinuation` prüfen: seine derzeit fehlende unabhängige
   Pausenrollenbindung ist kein Recht, diese Rolle aus dem Voice-Label abzuleiten.
   Das unveränderte Pausenlabel ist deshalb der zuerst zu untersuchende Weg.
5. Die bestehende kanonische Materialisierung und ihre EXACT-Receipt-/SSC-Adoption
   erhalten. Der vorhandene PrimaryBeamVoicePartition-Plan besitzt seinen eigenen
   Zweck; ihn nicht als Kapazitäts-Reparaturberechtigung umdeuten.
6. Die nachfolgende FinalNotation-Closure einschließlich PrimaryClef, RestPlacement,
   LowerDualEdge, ShadowRest, Beam-/Tie-Zustand und SourceOutputAlpha unverändert
   auswerten. Ein Voice-only-Byte-Diff beweist diese musikalischen Pflichten nicht.

## Zu untersuchender Lösungsraum, noch keine Freigabe

Ein vollständiges gewöhnliches, pausenfreies Kanalobligationspaket kann möglicherweise
auf vorhandene kompatible Kanäle verteilt werden, so dass sein eigener partweiter
Slot-Bedarf entfällt. Dabei bleiben Overlapping-Kanäle und alle Pausen unverändert.
Das ist eine technische Suchrichtung, kein bereits lizenzierter Repair-Selector.

Für jede vorgeschlagene Wiederverwendung sind mindestens unabhängig zu beweisen:

- Vollständige Part-/Staff-Ressourcengruppe einschließlich aller Heads, Chord-Member,
  versteckter Cursor-Träger, Pausen, forward/backup und Zeitauflösungen.
- Tatsächlich gebundene kompatible Source-Rollen; gleiche Staff-Zahl, ein freier
  Slot oder ein niedriges numerisches Label erzeugen keine Rollen-Authority.
- Keine zusätzliche Intervallkollision auf irgendeinem betroffenen Staff; kein
  neues Zusammenfallen verschiedener Consumer-Akkordobjekte mit widersprüchlichen
  Staff-Obligationen.
- Alle kanonischen HARD-SAME-/Chord-Host-Grenzen und Source-Bindungen bleiben erfüllt.
  Alle Fragmente einer Source müssen nach der bestehenden Source-Label-Struktur
  dieselbe zulässige Zuordnung tragen; fragmentweise Vorschläge sind nicht automatisch
  durch einen einzigen Source-Key darstellbar.
- Metric-Line-/Beam-Kontinuität und die tatsächlichen Tie-Verbindungen bleiben erhalten.
  Die extern verschobenen Heads können Beam-Marken besitzen; bloßes Beibehalten
  ihres XML-Texts beweist keine unveränderte native Balkengruppierung.
- Profilzulässigkeit und exakte Materialkontinuität nach der vollständigen Transaktion.

Das Evaluationsskript hat keine kanonische Rollen-/Custody-Evidenz und darf deshalb
nicht direkt in Produktcode oder normative Fixtures kopiert werden. Seine numerische
Host-Auswahl ist eine Machbarkeitsprobe, keine musikalische Entscheidung.

Ein zukünftiger Implementierungsvertrag muss seinen Repair-Scope separat von der
bereits angenommenen Fehlerklasse ableiten. Einschränkungen dürfen nur unabhängig
belegte Authority-/Safety-Grenzen besitzen. Keine Verengung auf bekannte Fundstellen.
Unabhängige Parts/Kandidaten müssen ihre Entscheidung bei Duplikation behalten;
mehrere Obligationen derselben erschöpften Ressource sind eine gemeinsame Gruppe.

## Beweisfolge und Zusammenarbeit

Zuerst vollständige sourcegebundene Planung und synthetische Nachweise für Renaming,
Relocation, Zeitskalierung, Staff-Permutation, unabhängige Duplikation und die
homogene positive Nachbarklasse. Danach die konkrete externe Native-Readback-
Gegenprobe und vor produktiver Übergabe die Gates, Tests, Clippy, Build und der
frische vollständige Korpus gemäß tatsächlicher Semantikänderung.

Die externe Musikbewertung nennt ausdrücklich die noch roten Staff-Obligationen.
Die parallele RAM-/Lessons-Arbeit kann ohne Musikänderung weitergeführt werden;
nach Annahme dieses Takes muss ein anderer Take gegen dessen angenommenen Parent
neu deklariert und geprüft werden. Keine vorbereiteten Dateien blind überschreiben.

Dieser Take repariert den eng begrenzten Registerplaner-Pfad für eingehende
Tie-Grenzen. Kapazität und Staff-Abweichung bleiben offen; vollständige native
Tie-Erhaltung im realen Endprodukt ist erst nach deren Schließung belegt.
Er macht die nächste Arbeit prüfbar und bewahrt die Abnahmegrenzen. Die Ursprungs-
frage der Staff-Abweichung bleibt von ihrer Reparaturberechtigung getrennt.
