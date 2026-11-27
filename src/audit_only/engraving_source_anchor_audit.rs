//! Source marks for one bounded synthetic engraving experiment.

use super::{
    Kind, Mark, MarkedNativeScore, Page, Shape, child, paint_value,
    pair_inventory_audit::{ExpectedMark, ObservedMark, Role, affected_pairs, exhaustive_pairs},
    rendering_agrees, same_viewport,
    stem_pair_audit::marked_shape,
};
use roxmltree::{Document, Node};

#[path = "engraving_split_tie_inventory_audit.rs"]
pub(crate) mod split_tie;

const SOURCE: &str = r#"<score-partwise version="4.0">
  <part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
  <part id="P1">
    <measure number="@first-measure@">
      <attributes><divisions>1</divisions><time><beats>2</beats><beat-type>4</beat-type></time>
        <staves>2</staves><clef number="1"><sign>G</sign><line>2</line></clef>
        <clef number="2"><sign>F</sign><line>4</line></clef></attributes>
      <note><rest/><duration>1</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>
      <note@0@><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><tie type="start"/>
        <voice>1</voice><type>quarter</type><stem@1@>@tie-direction@</stem><staff>1</staff>
        <notations><tied type="start"@6@/></notations></note>
      <backup><duration>2</duration></backup>
      <note@4@><pitch><step>E</step><octave>3</octave></pitch><duration>2</duration>
        <voice>2</voice><type>half</type><stem@5@>@direction@</stem><staff>2</staff></note>
    </measure>
    <measure number="@next-measure@">@layout-break@
      <note@2@><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><tie type="stop"/>
        <voice>1</voice><type>quarter</type><stem@3@>@tie-direction@</stem><staff>1</staff>
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
    score_in_layout(direction, "down", first_measure, paint, "")
}

fn score_in_layout(
    direction: &str,
    tie_direction: &str,
    first_measure: u16,
    paint: bool,
    layout_break: &str,
) -> String {
    ANCHORS.iter().enumerate().fold(
        SOURCE
            .replace("@direction@", direction)
            .replace("@tie-direction@", tie_direction)
            .replace("@layout-break@", layout_break)
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
    for kind in ["Note", "Stem", "Tie"] {
        let authored = ANCHORS.iter().filter(|(_, name, _)| *name == kind).count();
        let imported = root
            .descendants()
            .filter(|node| node.has_tag_name(kind))
            .count();
        (imported == authored).then_some(())?;
    }
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

struct SvgInk<'a>(&'a Shape);

impl PartialEq for SvgInk<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.same_ink(other.0)
            && self.0.filled == other.0.filled
            && self.0.even_odd == other.0.even_odd
    }
}

fn inventory_role(kind: Kind) -> Option<Role> {
    match kind {
        Kind::Stem => Some(Role::Stem),
        Kind::Head => Some(Role::Head),
        Kind::Tie => Some(Role::Tie),
        _ => None,
    }
}

fn expected_roster() -> Vec<ExpectedMark> {
    ANCHORS
        .iter()
        .enumerate()
        .map(|(index, (_, _, mark))| ExpectedMark {
            id: u32::try_from(index).unwrap(),
            role: inventory_role(mark.kind()).unwrap(),
        })
        .collect()
}

fn svg_inventory<'a>(
    plain: &'a Page,
    painted: &Page,
    native: &Page,
) -> Option<Vec<ObservedMark<SvgInk<'a>>>> {
    same_viewport(painted, native).then_some(())?;
    for page in [plain, painted, native] {
        let relevant = page
            .shapes
            .iter()
            .filter(|shape| {
                Kind::from_svg(&shape.class)
                    .and_then(inventory_role)
                    .is_some()
            })
            .count();
        (relevant == ANCHORS.len()).then_some(())?;
    }
    let marks = MarkedNativeScore {
        xml: String::new(),
        marks: ANCHORS
            .map(|(color, _, mark)| (color.to_owned(), mark))
            .into(),
    };
    let mut result = Vec::with_capacity(ANCHORS.len());
    let mut used_plain = std::collections::BTreeSet::new();
    let mut used_native = std::collections::BTreeSet::new();
    for ((color, _, _), expected) in ANCHORS.into_iter().zip(expected_roster()) {
        let uncolored = marked_shape(plain, painted, &marks, color)?;
        let direct_ink = single_color(painted, color)?;
        let native_ink = single_color(native, color)?;
        used_plain
            .insert(std::ptr::from_ref(uncolored))
            .then_some(())?;
        used_native
            .insert(std::ptr::from_ref(native_ink))
            .then_some(())?;
        rendering_agrees(direct_ink, native_ink).then_some(())?;
        result.push(ObservedMark {
            id: Some(expected.id),
            role: expected.role,
            page: 1,
            geometry: SvgInk(uncolored),
        });
    }
    Some(result)
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
    let (mut before, mut before_ink, mut before_native) = read(before);
    let (mut after, mut after_ink, mut after_native) = read(after);
    assert_eq!(
        before.format, after.format,
        "same coordinates for both exports"
    );
    let old = svg_inventory(&before, &before_ink, &before_native)
        .expect("complete inventory in the first export");
    let new = svg_inventory(&after, &after_ink, &after_native)
        .expect("complete inventory in the second export");
    let old_head = old[4].geometry.0;
    let old_stem = old[5].geometry.0;
    let new_head = new[4].geometry.0;
    let new_stem = new[5].geometry.0;
    let mid_height = |shape: &Shape| shape.bounds.top.midpoint(shape.bounds.bottom);
    assert!(
        mid_height(old_stem) > mid_height(old_head),
        "the lower stem points down in the first import"
    );
    assert!(
        mid_height(new_stem) < mid_height(new_head),
        "the lower stem points up in the second import"
    );
    let roster = expected_roster();
    let pairs = affected_pairs(&roster, &old, &roster, &new).unwrap();
    assert_eq!(pairs, exhaustive_pairs(&old, &new));
    assert!(pairs.contains(&(5, 0)) && pairs.contains(&(5, 6)));
    assert!(
        affected_pairs(&roster, &old, &roster, &old)
            .unwrap()
            .is_empty()
    );

    for page in [
        &mut before,
        &mut before_ink,
        &mut before_native,
        &mut after,
        &mut after_ink,
        &mut after_native,
    ] {
        page.shapes
            .retain(|shape| Kind::from_svg(&shape.class) != Some(Kind::Tie));
    }
    assert!(svg_inventory(&before, &before_ink, &before_native).is_none());
    assert!(svg_inventory(&after, &after_ink, &after_native).is_none());
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
