//! Source marks for one bounded synthetic engraving experiment.

use super::{
    Mark, MarkedNativeScore, Page, Shape, child, paint_value, rendering_agrees, same_viewport,
    stem_pair_audit::marked_shape,
};
use roxmltree::{Document, Node};

const SOURCE: &str = r#"<score-partwise version="4.0">
  <part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
  <part id="P1">
    <measure number="@first-measure@">
      <attributes><divisions>1</divisions><time><beats>2</beats><beat-type>4</beat-type></time>
        <staves>2</staves><clef number="1"><sign>G</sign><line>2</line></clef>
        <clef number="2"><sign>F</sign><line>4</line></clef></attributes>
      <note><rest/><duration>1</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>
      <note@0@><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><tie type="start"/>
        <voice>1</voice><type>quarter</type><stem@1@>down</stem><staff>1</staff>
        <notations><tied type="start"@6@/></notations></note>
      <backup><duration>2</duration></backup>
      <note@4@><pitch><step>E</step><octave>3</octave></pitch><duration>2</duration>
        <voice>2</voice><type>half</type><stem@5@>@direction@</stem><staff>2</staff></note>
    </measure>
    <measure number="@next-measure@">
      <note@2@><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><tie type="stop"/>
        <voice>1</voice><type>quarter</type><stem@3@>down</stem><staff>1</staff>
        <notations><tied type="stop"/></notations></note>
      <note><rest/><duration>1</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>
      <backup><duration>2</duration></backup>
      <note><rest measure="yes"/><duration>2</duration><voice>2</voice><staff>2</staff></note>
    </measure>
  </part>
</score-partwise>"#;

const STEM: Mark = Mark::Stem {
    chord: 0,
    isolated: false,
};

const ANCHORS: [(&str, &str, Mark); 7] = [
    ("#311001", "Note", Mark::Head),
    ("#311002", "Stem", STEM),
    ("#311003", "Note", Mark::Head),
    ("#311004", "Stem", STEM),
    ("#311005", "Note", Mark::Head),
    ("#311006", "Stem", STEM),
    ("#311007", "Tie", Mark::Tie),
];

pub(crate) fn score(direction: &str, first_measure: u16, paint: bool) -> String {
    ANCHORS.iter().enumerate().fold(
        SOURCE
            .replace("@direction@", direction)
            .replace("@first-measure@", &first_measure.to_string())
            .replace("@next-measure@", &(first_measure + 1).to_string()),
        |xml, (index, (ink, _, _))| {
            let attribute = if paint {
                format!(" color=\"{ink}\"")
            } else {
                String::new()
            };
            xml.replace(&format!("@{index}@"), &attribute)
        },
    )
}

fn ink(node: Node<'_, '_>) -> Option<String> {
    let color = child(node, "color")?;
    let byte = |name| color.attribute(name)?.parse::<u8>().ok();
    (byte("a") == Some(255)).then_some(())?;
    Some(format!(
        "#{:02x}{:02x}{:02x}",
        byte("r")?,
        byte("g")?,
        byte("b")?
    ))
}

fn native_node<'a, 'input>(
    root: Node<'a, 'input>,
    color: &str,
    kind: &str,
) -> Option<Node<'a, 'input>> {
    let mut found = root
        .descendants()
        .filter(|item| ink(*item).as_deref() == Some(color));
    let first = found.next()?;
    (first.has_tag_name(kind) && found.next().is_none()).then_some(first)
}

fn native_anchors(xml: &str) -> Option<()> {
    let doc = Document::parse(xml).ok()?;
    let root = doc.root_element();
    let [start, start_stem, end, end_stem, bass, bass_stem, tie] =
        ANCHORS.map(|(color, kind, _)| native_node(root, color, kind));
    for (head, stem) in [
        (start?, start_stem?),
        (end?, end_stem?),
        (bass?, bass_stem?),
    ] {
        let chord = head.parent()?;
        (chord.has_tag_name("Chord") && stem.parent() == Some(chord)).then_some(())?;
    }
    let start = start?;
    let end = end?;
    let edge = tie?.parent()?;
    (edge.has_tag_name("Spanner")
        && edge.attribute("type") == Some("Tie")
        && edge.parent() == Some(start))
    .then_some(())?;
    child(edge, "next")?;
    let stop = end
        .children()
        .find(|node| node.has_tag_name("Spanner") && node.attribute("type") == Some("Tie"))?;
    child(stop, "prev")?;
    // This experiment contains one held chain. With more native edges the
    // mere presence of a start and an end would not identify their pairing.
    (root
        .descendants()
        .filter(|node| node.has_tag_name("Spanner") && node.attribute("type") == Some("Tie"))
        .count()
        == 2)
        .then_some(())
}

fn single_color<'a>(page: &'a Page, color: &str) -> Option<&'a Shape> {
    let mut shapes = page.shapes.iter().filter(|shape| {
        ["fill", "stroke"]
            .iter()
            .any(|kind| paint_value(shape, kind).eq_ignore_ascii_case(color))
    });
    let first = shapes.next()?;
    shapes.next().is_none().then_some(first)
}

fn svg_anchors<'a>(plain: &'a Page, painted: &Page, native: &Page) -> Option<[&'a Shape; 2]> {
    same_viewport(painted, native).then_some(())?;
    let marks = MarkedNativeScore {
        xml: String::new(),
        marks: ANCHORS
            .map(|(color, _, mark)| (color.to_owned(), mark))
            .into(),
    };
    for (color, _, _) in ANCHORS {
        marked_shape(plain, painted, &marks, color)?;
        let direct_ink = single_color(painted, color)?;
        let native_ink = single_color(native, color)?;
        rendering_agrees(direct_ink, native_ink).then_some(())?;
    }
    Some([
        marked_shape(plain, painted, &marks, ANCHORS[4].0)?,
        marked_shape(plain, painted, &marks, ANCHORS[5].0)?,
    ])
}

pub(crate) fn assert_source_pair(before: [&str; 4], after: [&str; 4]) {
    let read = |view: [&str; 4]| {
        native_anchors(view[0]).expect("source marks at native note, stem and tie");
        (
            Page::read(view[1]).unwrap(),
            Page::read(view[2]).unwrap(),
            Page::read(view[3]).unwrap(),
        )
    };
    let (before, before_ink, before_native) = read(before);
    let (after, after_ink, after_native) = read(after);
    let [old_head, old_stem] = svg_anchors(&before, &before_ink, &before_native)
        .expect("one visible glyph per source mark");
    let [new_head, new_stem] =
        svg_anchors(&after, &after_ink, &after_native).expect("one visible glyph per source mark");
    let mid_height = |shape: &Shape| shape.bounds.top.midpoint(shape.bounds.bottom);
    assert!(
        mid_height(old_stem) > mid_height(old_head),
        "the lower stem points down in the first import"
    );
    assert!(
        mid_height(new_stem) < mid_height(new_head),
        "the lower stem points up in the second import"
    );
}

#[test]
fn source_mark_refuses_missing_duplicate_and_wrong_native_role() {
    let check = |xml: &str| {
        let doc = Document::parse(xml).unwrap();
        native_node(doc.root_element(), "#311007", "Tie").is_some()
    };
    let tie = r#"<Tie><color r="49" g="16" b="7" a="255"/></Tie>"#;
    assert!(check(&format!("<Score>{tie}</Score>")));
    assert!(!check("<Score/>"));
    assert!(!check(&format!("<Score>{tie}{tie}</Score>")));
    assert!(!check(&format!(
        "<Score>{}</Score>",
        tie.replace("Tie", "Note")
    )));
}
