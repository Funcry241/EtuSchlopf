# r1594 – ungebundene Pausenergänzung vor der Balkenkorrektur stilllegen

Basis: nativ angenommener r1593 / v0.10.1839. ChangeClass: MusicMutation.

Der native Ursachenbeleg zeigt zwei tatsächliche Übergänge: D10→D11 ergänzt
in Etüde 6 eine obere Pause in Takt 40. D18→D19 verteilt bestehende obere
Balkengruppen in Takt 2 auf mehrere technische Stimmen. Nach der Projektion
erschöpft die ergänzte Pause das partweite Render-Importprofil. Die vorhandene
Originalbalkenkorrektur bricht deshalb vor der ersten Gruppenentscheidung ab.

## Produktänderung

Der bestehende BehindBars-Owner ruft die optionale Transaktion
`apply_beat_local_voice_rest_rewrite` nicht mehr auf. Dieser Writer selbst und
seine drei Unterfunktionen bleiben bytegleich. Damit werden an dieser Stelle
weder neue Randpausen aus technischen Lücken erzeugt noch versteckte Pausen
aufgrund bloßer Rhythmusgeometrie eingeblendet. Auch die interne
Backup-Ankerkorrektur dieses optionalen Teilschritts wird abgekoppelt. Das ist
bewusst der vollständige Rückbau dieses bestehenden Aufrufwegs; es gibt
keinen Ersatzwriter und keinen neuen Aufruf einer Unterfunktion.

Der alte Schritt erhält keinen kanonisch gebundenen musikalischen Auftrag für
neue sichtbare Stille. Die bereits im angenommenen Parent eingefrorene
Fehlerklasse `unbound_rest_context` und `canonical_rest_context_v1` begründen
die Verweigerung neuer Realisierungen. Vorhandene Pausen werden nicht nachträglich
wegen eines Kanalkonflikts gelöscht oder umgestimmt.

Der interne gemeinsame Aktionsrahmen bleibt mit dem unveränderten Writer
zusammen erhalten. Frei gewordene Plätze können deshalb keine nachfolgenden
Sichtbarkeitsänderungen nachrücken lassen. Alle anderen BehindBars-Transaktionen,
die Stimmenzuordnung, Balkenregeln, Importprüfung und typed writer bleiben
unverändert. Der Parent-Übergabeprüfer bestätigt den reinen Rückbau.

## Prüfgrenze

Die vorhandenen aktiven Tests bleiben bytegleich. Zusätzliche synthetische
Diagnoseproben werden nur in einer lokalen Testinstrumentierung ausgeführt.
Sie sind kein neuer produktiver Aufrufweg und werden nicht in den strengen
Retirement-Take aufgenommen. Die Instrumentierung verwendet dieselben
produktiven Writer-Bytes; deren SHA256 ist unten gebunden. Ein neuer Test mit
einem Writer-Aufruf wäre im Gate-Inventar selbst eine neue Aufrufkante und
würde den reinen Rückbau verwerfen. Kein Gate wird dafür abgeschwächt.

Die Proben prüfen den tatsächlichen BehindBars-Owner sowie positive Vorschläge
des abgekoppelten Vorgängers, bereits sichtbare und versteckte Pausen,
abweichende Stimmennamen und Systeme, Zeitmaßstab, Taktverschiebung und
unabhängige Verdoppelung. Nicht jede Variante ist eine positive Kandidatin des
alten, metrisch eingeschränkten Writers; die Preservation-Prüfung gilt auch
für dessen nicht unterstützte Eingaben.

## Native Abnahme

Wie bisher: `run.cmd --take --all` mit dem neuen Take im Eingangsordner.

Zu prüfen sind das vollständige Importprofil ohne verlorene Vorkommen und die
sichtbaren Gruppen in Etüde 6 ab Takt 2, insbesondere auch 30–33 und 40.
Gleichzeitig den gesamten Korpus auf geänderte Pausendarstellung und die Wirkung
der ebenfalls entfallenen optionalen Backup-Ankerkorrektur prüfen. Quellidentität,
Tonhöhen, Einsätze, Dauern, Systeme, Akkord- und Bindebogenpflichten bleiben
unabhängige Erhaltungsnachweise. Die vier zuvor geschlossenen Pausenbrücken
werden erneut visuell kontrolliert.

Der native Korpus- und PDF-Nachweis für r1594 steht aus. Der neue Take beseitigt
den belegten Verursacher an seinem produktiven Aufrufweg; er behauptet noch
keine vollständige sichtbare Balkenreparatur.

## Lokale Evidenz

Produktiver Owner `src/pipeline_shards/human_pdf_pianistic_quality_core.rs`:
SHA256 `ed7715b6b2257587d8e2112b4a2948498d5f954c6c4e04e312e7e8532ecf8020`.

Die lokal zusätzlich ausgeführten Diagnosefälle sind nachstehend dokumentiert.
Sie werden zur Reproduktion in einer wegwerfbaren Arbeitskopie an das bestehende
Modul `tests/unit/strict_staff_rest_context_refusal.rs` angehängt; diese Kopie
ist kein eingehender Take. Der eigentliche Take behält die Tests des Parents.

Zusatzprüfung: **4 bestanden, 0 fehlgeschlagen**.

```rust

fn beat_local_retirement_body(divisions: i32, staff: &str, voice: &str) -> String {
    let other = if staff == "1" { "2" } else { "1" };
    let short = divisions / 4;
    let pitched = |staff: &str, duration: i32| format!("<note><pitch><step>E</step><octave>4</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><type>16th</type><staff>{staff}</staff></note>");
    format!("<attributes><divisions>{divisions}</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves></attributes>{}<forward><duration>{divisions}</duration></forward>{}{}<forward><duration>{}</duration></forward><backup><duration>{}</duration></backup><note><rest/><duration>{}</duration><voice>2</voice><type>whole</type><staff>{other}</staff></note>",
        pitched(staff, divisions), pitched(other, short), pitched(staff, short), divisions * 2 - short * 2, divisions * 4, divisions * 4)
}

fn assert_beat_local_retirement_preserves(xml: &str) {
    let (_, candidate, receipts) = build_behind_bars_rule_check_rest_and_tie_audit(
        xml, "synthetic-retirement", std::path::Path::new("unused-synthetic-output"),
    ).expect("actual BehindBars owner must accept synthetic input");
    assert_eq!(candidate.as_deref().unwrap_or(xml), xml,
        "retired realization must preserve existing carriers");
    assert!(receipts.is_empty());
    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
}

#[test]
fn beat_local_retirement_preserves_cross_staff_gaps_and_existing_rests() {
    let xml = score_at(53, &beat_local_retirement_body(96, "1", "7"));
    // Establish the synthetic positive predecessor: the detached original
    // planner would actually insert a rest, with its original real receipt.
    let ((old, actions, seen), receipts) = apply_beat_local_missing_staff_edge_rest_rewrite(&xml, 12);
    assert!(seen > 0 && !actions.is_empty());
    assert_ne!(old, xml);
    assert!(!receipts.is_empty());
    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    for (scope, divisions, staff, voice) in [
        (31_usize, 96, "1", "7"),
        (829_usize, 192, "2", "RenamedCourse"),
    ] {
        let xml = score_at(scope, &beat_local_retirement_body(divisions, staff, voice));
        assert_beat_local_retirement_preserves(&xml);
    }
}

#[test]
fn beat_local_retirement_is_independent_under_duplication_and_measure_shift() {
    let body = beat_local_retirement_body(96, "1", "OpaqueCourse");
    let one = score_at(43, &body);
    let two = one.replace("</part>", &format!("<measure number=\"{}\">{body}</measure></part>", 997_usize));
    assert_beat_local_retirement_preserves(&one);
    assert_beat_local_retirement_preserves(&two);
}

#[test]
fn beat_local_retirement_does_not_reveal_hidden_rest_through_freed_budget() {
    let body = beat_local_retirement_body(96, "1", "7");
    let xml = score_at(463, &body);
    let xml = xml.replace("</measure>", "<backup><duration>384</duration></backup><forward><duration>288</duration></forward><note print-object=\"no\"><rest/><duration>96</duration><voice>3</voice><type>quarter</type><staff>1</staff></note></measure>");
    let ((old, actions, seen), receipts) = apply_beat_local_hidden_rest_visibility_rewrite(&xml, 12);
    assert!(seen > 0 && !actions.is_empty());
    assert_ne!(old, xml);
    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    assert_beat_local_retirement_preserves(&xml);
}

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/synthetic_score.rs"));

#[test]
fn beat_local_retirement_passes_the_measure_shift_bench() {
    let xml = score_at(347, &beat_local_retirement_body(96, "1", "7"));
    assert_shift_invariant("beat-local-rest-realization-retirement", &xml, |input| {
        let (_, candidate, receipts) = build_behind_bars_rule_check_rest_and_tie_audit(
            input, "synthetic-shift-retirement", std::path::Path::new("unused-synthetic-output"),
        ).expect("actual BehindBars owner");
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        candidate.unwrap_or_else(|| input.to_owned())
    });
}

```
