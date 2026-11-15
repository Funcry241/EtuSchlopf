struct AuthoredOpeningTempo {
    unit: &'static str,
    dotted: bool,
    rate: u16,
    words: String,
}

fn read_authored_opening_tempo(midi: &std::path::Path) -> anyhow::Result<Option<AuthoredOpeningTempo>> {
    let path = midi.with_extension("opening-tempo.txt");
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    let mut lines = text.lines();
    anyhow::ensure!(lines.next() == Some("etuschlopf-opening-tempo-v1"), "Ungültige Anfangstempo-Datei");
    let midi_sha = crate::sha256::hex(&std::fs::read(midi)?);
    anyhow::ensure!(lines.next() == Some(midi_sha.as_str()), "Anfangstempo gehört zu einer anderen MIDI-Datei");
    let (unit, dotted) = match lines.next() {
        Some("half") => ("half", false),
        Some("quarter") => ("quarter", false),
        Some("eighth") => ("eighth", false),
        Some("dotted-half") => ("half", true),
        Some("dotted-quarter") => ("quarter", true),
        Some("dotted-eighth") => ("eighth", true),
        _ => anyhow::bail!("Ungültiger Notenwert für Anfangstempo"),
    };
    let rate = lines.next().and_then(|line| line.parse::<u16>().ok())
        .filter(|rate| *rate > 0).ok_or_else(|| anyhow!("Ungültiger Wert für Anfangstempo"))?;
    let words = lines.next().unwrap_or("");
    anyhow::ensure!(lines.next().is_none() && !words.chars().any(char::is_control), "Mehrdeutige Anfangstempo-Datei");
    Ok(Some(AuthoredOpeningTempo { unit, dotted, rate, words: words.to_owned() }))
}

fn authored_opening_tempo_direction(
    xml: &str,
    mark: &AuthoredOpeningTempo,
) -> anyhow::Result<(crate::structural_write_authority::StructuralMeasureLocator, String)> {
    let score = roxmltree::Document::parse(xml)?;
    let mut parts = score.root_element().children().filter(|node| node.has_tag_name("part"));
    let part = parts.next().ok_or_else(|| anyhow!("Anfangstempo benötigt einen Partiturteil"))?;
    anyhow::ensure!(parts.next().is_none(), "Anfangstempo benötigt einen eindeutigen Partiturteil");
    let opening = part.children().find(|node| node.has_tag_name("measure"))
        .ok_or_else(|| anyhow!("Anfangstempo benötigt einen Anfangstakt"))?;
    let locator = crate::structural_write_authority::first_structural_measure_locator(xml)
        .ok_or_else(|| anyhow!("Anfangstakt ist nicht gebunden"))?;
    anyhow::ensure!(
        opening.attribute("number").and_then(|number| number.parse().ok()) == Some(locator.measure_number()),
        "Anfangstakt stimmt nicht mit der Schreibposition überein",
    );
    anyhow::ensure!(
        !opening.descendants().any(|node| node.has_tag_name("metronome") || node.attribute("tempo").is_some()),
        "Anfangstakt enthält bereits eine Tempoangabe",
    );
    let quarters = match mark.unit { "half" => 2.0, "quarter" => 1.0, _ => 0.5 };
    let augmentation = if mark.dotted { 1.5 } else { 1.0 };
    let playback = f64::from(mark.rate) * quarters * augmentation;
    let dot = if mark.dotted { "<beat-unit-dot/>" } else { "" };
    let words = mark.words.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
    let expression = if words.is_empty() { String::new() } else {
        format!("<direction-type><words>{words}</words></direction-type>")
    };
    let direction = format!(
        "<direction placement=\"above\">{expression}<direction-type><metronome><beat-unit>{}</beat-unit>{dot}<per-minute>{}</per-minute></metronome></direction-type><staff>1</staff><sound tempo=\"{playback}\"/></direction>",
        mark.unit, mark.rate,
    );
    Ok((locator, direction))
}
