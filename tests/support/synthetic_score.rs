// Verschiebungs-Pruefstand: ein synthetischer Einzeltakt wird umnummeriert,
// verschoben, mit anderen Divisions umgeben, umbenannt, mit Metadaten versehen
// und verdoppelt. Das Ergebnis des Writers muss im Takt selbst gleich bleiben,
// alle umgebenden Takte unveraendert. Taktnummern entstehen nur aus Variablen.

struct ShiftedScore {
    label: &'static str,
    xml: String,
    case_slots: Vec<usize>,
    case_form: Box<dyn Fn(&str) -> String>,
}

fn shift_split(xml: &str) -> (String, String, String) {
    let (head, rest) = xml
        .split_once("<measure ")
        .expect("synthetic score has a measure");
    let (_, rest) = rest.split_once('>').expect("measure tag closes");
    let (inner, tail) = rest.split_once("</measure>").expect("measure closes");
    assert!(!tail.contains("<measure "), "pruefstand expects one measure");
    (head.to_owned(), inner.to_owned(), tail.to_owned())
}

fn shift_measures(xml: &str) -> Vec<String> {
    xml.split("<measure ")
        .skip(1)
        .map(|chunk| {
            let (_, rest) = chunk.split_once('>').expect("measure tag closes");
            rest.split_once("</measure>")
                .expect("measure closes")
                .0
                .to_owned()
        })
        .collect()
}

fn shift_element(text: &str, name: &str) -> String {
    let (_, rest) = text
        .split_once(&format!("<{name}>"))
        .expect("element present");
    rest.split_once(&format!("</{name}>"))
        .expect("element closes")
        .0
        .to_owned()
}

fn shift_rest_measure(divisions: i64) -> String {
    let whole = divisions * 4;
    format!(
        "<attributes><divisions>{divisions}</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves></attributes><note><rest measure=\"yes\"/><duration>{whole}</duration><voice>1</voice><staff>1</staff></note>"
    )
}

fn shift_without_attributes(inner: &str) -> String {
    let (before, rest) = inner
        .split_once("<attributes>")
        .expect("measure carries attributes");
    let (_, after) = rest.split_once("</attributes>").expect("attributes close");
    format!("{before}{after}")
}

fn shift_assemble(head: &str, tail: &str, measures: &[String], first_ordinal: usize) -> String {
    let body = measures
        .iter()
        .enumerate()
        .map(|(offset, inner)| {
            let ordinal = first_ordinal + offset;
            format!("<measure number=\"{ordinal}\">{inner}</measure>")
        })
        .collect::<String>();
    format!("{head}{body}{tail}")
}

fn shifted_scores(xml: &str) -> Vec<ShiftedScore> {
    let (head, inner, tail) = shift_split(xml);
    let divisions = shift_element(&inner, "divisions")
        .parse::<i64>()
        .expect("numeric divisions");
    let voice = shift_element(&inner, "voice");
    let swapped = (voice.parse::<u32>().expect("numeric voice") + 4).to_string();
    let part = head
        .split_once("<part id=\"")
        .and_then(|(_, rest)| rest.split_once('"'))
        .expect("part id")
        .0
        .to_owned();
    let relabeled_head = head.replace(&format!("id=\"{part}\""), &format!("id=\"Shifted-{part}\""));
    let metadata_head = head.replacen(
        "<part",
        "<work><work-title>Pruefstand</work-title></work><identification><creator type=\"composer\">Synthetik</creator></identification><part",
        1,
    );
    let combined_head = relabeled_head.replacen(
        "<part",
        "<work><work-title>Pruefstand</work-title></work><part",
        1,
    );
    let same = || -> Box<dyn Fn(&str) -> String> { Box::new(|text: &str| text.to_owned()) };
    let rest_same = shift_rest_measure(divisions);
    let rest_half = shift_rest_measure((divisions / 2).max(1));
    let rest_large = shift_rest_measure(divisions * 5);
    let stripped = shift_without_attributes(&inner);
    let voice_from = format!("<voice>{voice}</voice>");
    let voice_to = format!("<voice>{swapped}</voice>");
    let voice_swapped = inner.replace(&voice_from, &voice_to);
    let renumber_start = 40 + 1;
    vec![
        ShiftedScore {
            label: "renumbered",
            xml: shift_assemble(&head, &tail, std::slice::from_ref(&inner), renumber_start),
            case_slots: vec![0],
            case_form: same(),
        },
        ShiftedScore {
            label: "prepended",
            xml: shift_assemble(
                &head,
                &tail,
                &[rest_same.clone(), rest_same.clone(), rest_same.clone(), inner.clone()],
                1,
            ),
            case_slots: vec![3],
            case_form: same(),
        },
        ShiftedScore {
            label: "prepended_other_divisions",
            xml: shift_assemble(&head, &tail, &[rest_half.clone(), rest_large, inner.clone()], 1),
            case_slots: vec![2],
            case_form: same(),
        },
        ShiftedScore {
            label: "inherited_attributes",
            xml: shift_assemble(&head, &tail, &[rest_same.clone(), stripped], 1),
            case_slots: vec![1],
            case_form: Box::new(shift_without_attributes),
        },
        ShiftedScore {
            label: "appended",
            xml: shift_assemble(&head, &tail, &[inner.clone(), rest_same.clone()], 1),
            case_slots: vec![0],
            case_form: same(),
        },
        ShiftedScore {
            label: "twice",
            xml: shift_assemble(&head, &tail, &[inner.clone(), inner.clone()], 1),
            case_slots: vec![0, 1],
            case_form: same(),
        },
        ShiftedScore {
            label: "part_relabeled",
            xml: shift_assemble(&relabeled_head, &tail,std::slice::from_ref(&inner), 1),
            case_slots: vec![0],
            case_form: same(),
        },
        ShiftedScore {
            label: "metadata",
            xml: shift_assemble(&metadata_head, &tail, std::slice::from_ref(&inner), 1),
            case_slots: vec![0],
            case_form: same(),
        },
        ShiftedScore {
            label: "voice_relabeled",
            xml: shift_assemble(&head, &tail, &[voice_swapped], 1),
            case_slots: vec![0],
            case_form: Box::new(move |text: &str| text.replace(&voice_from, &voice_to)),
        },
        ShiftedScore {
            label: "combined",
            xml: shift_assemble(
                &combined_head,
                &tail,
                &[rest_half, inner.clone(), rest_same, inner],
                renumber_start,
            ),
            case_slots: vec![1, 3],
            case_form: same(),
        },
    ]
}

fn assert_shift_invariant(fixture: &str, xml: &str, apply: impl Fn(&str) -> String) {
    let base_post = shift_split(&apply(xml)).1;
    for shifted in shifted_scores(xml) {
        let before = shift_measures(&shifted.xml);
        let after = shift_measures(&apply(&shifted.xml));
        assert_eq!(before.len(), after.len(), "{fixture}/{}", shifted.label);
        for (slot, (input, output)) in before.iter().zip(&after).enumerate() {
            let expected = if shifted.case_slots.contains(&slot) {
                (shifted.case_form)(&base_post)
            } else {
                input.clone()
            };
            assert_eq!(
                output, &expected,
                "Verschiebung {fixture}/{} veraendert Takt an Position {slot}",
                shifted.label
            );
        }
    }
}
