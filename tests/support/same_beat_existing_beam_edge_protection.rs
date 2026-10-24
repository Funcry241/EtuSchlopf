include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/synthetic_score.rs"));

#[derive(Clone)]
struct Event {
    onset: i64,
    duration: i64,
    beams: Vec<&'static str>,
}
fn event(onset: i64, duration: i64, beams: &[&'static str]) -> Event {
    Event {
        onset,
        duration,
        beams: beams.to_vec(),
    }
}
fn four() -> Vec<Event> {
    ["begin", "continue", "continue", "end"]
        .into_iter()
        .enumerate()
        .map(|(n, s)| event(n as i64 * 24, 24, &[s, s]))
        .collect()
}
fn pair() -> Vec<Event> {
    vec![
        event(288, 48, &[]),
        event(336, 24, &[]),
        event(360, 24, &[]),
    ]
}
fn score(events: &[Event], voice: &str, label: &str) -> String {
    let mut body = String::new();
    let mut cursor = 0;
    for e in events {
        if e.onset > cursor {
            body.push_str(&format!(
                "<note><rest/><duration>{}</duration><voice>{voice}</voice><staff>2</staff></note>",
                e.onset - cursor
            ));
        }
        let kind = match e.duration {
            12 => "32nd",
            24 => "16th",
            _ => "eighth",
        };
        let beams = e
            .beams
            .iter()
            .enumerate()
            .map(|(n, s)| format!("<beam number=\"{}\">{s}</beam>", n + 1))
            .collect::<String>();
        body.push_str(&format!("<note><pitch><step>D</step><octave>3</octave></pitch><duration>{}</duration><voice>{voice}</voice><type>{kind}</type><staff>2</staff>{beams}</note>",e.duration));
        cursor = e.onset + e.duration;
    }
    if cursor < 384 {
        body.push_str(&format!(
            "<note><rest/><duration>{}</duration><voice>{voice}</voice><staff>2</staff></note>",
            384 - cursor
        ));
    }
    format!(
        "<score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"{label}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves></attributes>{body}</measure></part></score-partwise>"
    )
}
fn combined(mut a: Vec<Event>, b: Vec<Event>) -> Vec<Event> {
    a.extend(b);
    a
}



fn shift_bench_rejects(writer: impl Fn(&str) -> String) -> bool {
    let xml = score(&combined(four(), pair()), "2", "11");
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_shift_invariant("bench", &xml, &writer);
    }))
    .is_err()
}
#[test]
fn v2_bench_detects_a_writer_bound_to_the_first_position() {
    let first_only = |xml: &str| xml.replacen("</measure>", "<!--written--></measure>", 1);
    assert!(shift_bench_rejects(first_only));
}
#[test]
fn v3_bench_detects_a_writer_bound_to_its_number() {
    let label = format!("number=\"{}\">", 11);
    let number_bound = move |xml: &str| xml.replace(&label, &format!("{label}<!--written-->"));
    assert!(shift_bench_rejects(number_bound));
}
#[test]
fn v4_bench_accepts_a_purely_local_writer() {
    let local = |xml: &str| xml.replace("<type>16th</type>", "<type>16th</type><!--written-->");
    assert!(!shift_bench_rejects(local));
}
