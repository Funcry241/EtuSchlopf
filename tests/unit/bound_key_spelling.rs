    use super::*;

    fn score(fifths: i32, mode: &str, pitches: &[&str]) -> String {
        let label = 6 + pitches.len();
        let notes = pitches.iter().map(|pitch| format!(
            r#"<note><pitch>{pitch}</pitch><duration>24</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>"#
        )).collect::<String>();
        format!(r#"<score-partwise><part id="P"><measure number="{label}"><attributes><divisions>24</divisions><key><fifths>{fifths}</fifths>{mode}</key></attributes>{notes}</measure></part></score-partwise>"#)
    }

    #[test]
    fn render_owner_uses_the_bound_signature_with_no_key_rewrite() {
        let sharp = "<step>G</step><alter>1</alter><octave>4</octave>";
        let chromatic = "<step>C</step><alter>1</alter><octave>4</octave>";
        let xml = score(-3, "<mode>minor</mode>", &[sharp, chromatic, sharp]);
        let ((written, audit), receipt) = apply_key_signature_enharmonic_spelling_render_write(
            &xml, "synthetic", Path::new("synthetic.render.musicxml"));
        assert!(audit.written);
        assert_eq!(audit.diatonic_respellings, 2);
        assert_eq!(functional_explicit_key_signatures(&xml), functional_explicit_key_signatures(&written));
        assert!(written.contains(chromatic));
        assert_eq!(written.matches("<step>A</step><alter>-1</alter>").count(), 2);
        let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
        receipts.push_block(receipt, &written);
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    }

    #[test]
    fn recognizes_the_sharp_side_and_leaves_the_neutral_placeholder_unbound() {
        let g_flat = "<step>G</step><alter>-1</alter><octave>4</octave>";
        let xml = score(2, "<mode>major</mode>", &[g_flat]);
        let (plans, fifths) = bound_key_diatonic_plans(&xml, &parse_atomic_notes(&xml), None).unwrap();
        assert_eq!(fifths, 2);
        assert_eq!(plans.len(), 1);
        assert_eq!((plans[0].step, plans[0].alter), ("F", 1));
        let placeholder = score(0, "", &[g_flat]);
        assert!(bound_key_diatonic_plans(&placeholder, &parse_atomic_notes(&placeholder), None).is_none());
    }

    #[test]
    fn an_unbound_tie_or_editorial_accidental_is_preserved() {
        let pitch = "<step>G</step><alter>1</alter><octave>4</octave>";
        let xml = score(-3, "<mode>minor</mode>", &[pitch]);
        for modified in [
            xml.replace("</duration>", "</duration><tie type=\"start\"/>"),
            xml.replace("</type>", "</type><accidental cautionary=\"yes\">sharp</accidental>"),
        ] {
            let (plans, _) = bound_key_diatonic_plans(&modified, &parse_atomic_notes(&modified), None).unwrap();
            assert!(plans.is_empty());
        }
    }

    #[test]
    fn chromatic_passing_spelling_follows_both_directions() {
        let c = "<step>C</step><octave>4</octave>";
        let c_sharp = "<step>C</step><alter>1</alter><octave>4</octave>";
        let e_flat = "<step>E</step><alter>-1</alter><octave>4</octave>";
        let xml = score(-3, "<mode>minor</mode>", &[c, c_sharp, e_flat, c_sharp, c]);
        let ((written, audit), receipt) = apply_key_signature_enharmonic_spelling_render_write(
            &xml, "synthetic", Path::new("synthetic.render.musicxml"));
        assert!(audit.written);
        assert_eq!(audit.chromatic_passing_respellings, 2);
        assert_eq!(written.matches("<step>D</step><alter>-1</alter>").count(), 2);
        assert_eq!(functional_explicit_key_signatures(&xml), functional_explicit_key_signatures(&written));
        let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
        receipts.push_block(receipt, &written);
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    }

    #[test]
    fn chromatic_passing_spelling_requires_a_bound_three_note_figure() {
        let c = "<step>C</step><octave>4</octave>";
        let c_sharp = "<step>C</step><alter>1</alter><octave>4</octave>";
        let d = "<step>D</step><octave>4</octave>";
        let e_flat = "<step>E</step><alter>-1</alter><octave>4</octave>";
        let xml = score(-3, "<mode>minor</mode>", &[c, c_sharp, e_flat]);
        for rejected in [
            score(-3, "<mode>minor</mode>", &[c, c_sharp, d]),
            xml.replace("</note><note><pitch><step>E", "</note><forward><duration>24</duration></forward><note><pitch><step>E"),
            xml.replace("</note><note><pitch><step>E", "</note><note print-object=\"no\"><pitch><step>E"),
            xml.replace("</note><note><pitch><step>E", "</note><note><chord/><pitch><step>E"),
            xml.replace("</note><note><pitch><step>E", &format!("</note></measure><measure number=\"{}\"><note><pitch><step>E", xml.len())),
            xml.replace("<alter>1</alter><octave>4</octave></pitch>", "<alter>1</alter><octave>4</octave></pitch><tie type=\"start\"/>"),
        ] {
            let (plans, _) = bound_key_diatonic_plans(&rejected, &parse_atomic_notes(&rejected), None).unwrap();
            assert!(plans.is_empty(), "{rejected}");
        }
        let relocated = xml.replace(&format!("number=\"{}\"", 6 + [c, c_sharp, e_flat].len()),
            &format!("number=\"{}\"", xml.len() + 1))
            .replace("<voice>1</voice>", "<voice>7</voice>");
        let third = score(-3, "<mode>minor</mode>", &[c, c_sharp, e_flat]);
        let second_figure = third.split_once("</attributes>").unwrap().1.split_once("</measure>").unwrap().0;
        let second_measure = format!("<measure number=\"{}\">{second_figure}</measure>", relocated.len());
        let duplicated = relocated.replace("</part>", &format!("{second_measure}</part>"));
        let (both, _) = bound_key_diatonic_plans(&duplicated, &parse_atomic_notes(&duplicated), None).unwrap();
        assert_eq!(both.len(), 2);
        assert!(both.iter().all(|plan| (plan.step, plan.alter, plan.octave) == ("D", -1, 4)));

        let (plans, _) = bound_key_diatonic_plans(&relocated, &parse_atomic_notes(&relocated), None).unwrap();
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].kind, FunctionalRespellingKind::ChromaticPassing);
    }
