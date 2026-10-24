use crate::master_state_identity::{
    BlockTransformationReceipt, RenderDiscardReason, RenderTransitionReceipts,
};

/// One already-selected fragment-local BLOCK transformation that must become
/// part of the document draft before the real document-level consumer mints
/// its EXACT receipt. This is transport/rebasing only: it performs no
/// selection, ranking, or musical decision.
#[derive(Debug)]
pub(crate) struct DurationPromotion {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) post: String,
    pub(crate) receipt: BlockTransformationReceipt,
}

impl DurationPromotion {
    pub(crate) fn new(
        start: usize,
        end: usize,
        post: String,
        receipt: BlockTransformationReceipt,
    ) -> Self {
        Self { start, end, post, receipt }
    }
}

fn discard_promotions(promotions: Vec<DurationPromotion>) {
    let mut losing = RenderTransitionReceipts::new();
    for promotion in promotions {
        losing.push_block_fragment(
            Some(promotion.receipt),
            promotion.start,
            promotion.end,
            &promotion.post,
        );
    }
    losing.discard_unadopted(RenderDiscardReason::LosingCandidate);
}

/// Materialize selected fragment posts into a temporary document B and rebase
/// an already-selected legacy A->C edit list onto B. The returned receipt
/// chain proves only A->B. The caller MUST execute the real existing EXACT
/// consumer on B and append that receipt before any physical sink.
type SelectedDurationFragmentPost = (String, Vec<(usize, usize, String)>, RenderTransitionReceipts);

pub(crate) fn mutate_selected_duration_fragments(
    source: &str,
    mut promotions: Vec<DurationPromotion>,
    final_edits_on_a: Vec<(usize, usize, String)>,
) -> anyhow::Result<SelectedDurationFragmentPost> {
    promotions.sort_by_key(|promotion| promotion.start);

    let rebased = (|| -> anyhow::Result<Vec<(usize, usize, String)>> {
        let mut previous_end = 0usize;
        for (index, promotion) in promotions.iter().enumerate() {
            anyhow::ensure!(
                promotion.start < promotion.end,
                "duration promotion has an empty or reversed range"
            );
            anyhow::ensure!(
                source.is_char_boundary(promotion.start)
                    && source.is_char_boundary(promotion.end)
                    && source.get(promotion.start..promotion.end).is_some(),
                "duration promotion has an invalid UTF-8 source range"
            );
            if index > 0 {
                anyhow::ensure!(
                    previous_end <= promotion.start,
                    "duration promotions overlap"
                );
            }
            previous_end = promotion.end;
        }

        let rebase = |position: usize| -> anyhow::Result<usize> {
            anyhow::ensure!(
                position <= source.len() && source.is_char_boundary(position),
                "legacy edit has an invalid UTF-8 boundary"
            );
            let mut removed = 0usize;
            let mut added = 0usize;
            for promotion in &promotions {
                anyhow::ensure!(
                    position <= promotion.start || position >= promotion.end,
                    "legacy edit boundary lies inside a promoted fragment"
                );
                if promotion.end <= position {
                    removed = removed
                        .checked_add(promotion.end - promotion.start)
                        .ok_or_else(|| anyhow::anyhow!("duration promotion range overflow"))?;
                    added = added
                        .checked_add(promotion.post.len())
                        .ok_or_else(|| anyhow::anyhow!("duration promotion range overflow"))?;
                }
            }
            position
                .checked_sub(removed)
                .and_then(|value| value.checked_add(added))
                .ok_or_else(|| anyhow::anyhow!("duration promotion range overflow"))
        };

        final_edits_on_a
            .into_iter()
            .map(|(start, end, post)| {
                anyhow::ensure!(start <= end, "legacy edit range is reversed");
                Ok((rebase(start)?, rebase(end)?, post))
            })
            .collect()
    })();

    let rebased = match rebased {
        Ok(rebased) => rebased,
        Err(error) => {
            discard_promotions(promotions);
            return Err(error);
        }
    };

    let mut draft = source.to_owned();
    let mut receipts = RenderTransitionReceipts::new();
    for promotion in promotions.into_iter().rev() {
        debug_assert!(draft.is_char_boundary(promotion.start));
        debug_assert!(draft.is_char_boundary(promotion.end));
        draft.replace_range(promotion.start..promotion.end, &promotion.post);
        receipts.push_block_fragment(
            Some(promotion.receipt),
            promotion.start,
            promotion.end,
            &promotion.post,
        );
    }

    Ok((draft, rebased, receipts))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(duration: i32, voice: &str) -> String {
        format!(
            "<note><rest/><duration>{duration}</duration><voice>{voice}</voice><type>16th</type><staff>2</staff></note>"
        )
    }

    #[test]
    fn two_promotions_rebase_real_consumer_edits_without_value_search() {
        let first = note(24, "3");
        let second = note(24, "4");
        let source = format!("α{first}|{second}ω");
        let first_start = source.find(&first).expect("first note");
        let first_end = first_start + first.len();
        let second_start = source.rfind(&second).expect("second note");
        let second_end = second_start + second.len();

        let (first_post, first_receipt) =
            crate::structural_write_authority::set_rest_note_duration_type_authorized(&first, 48);
        let (second_post, second_receipt) =
            crate::structural_write_authority::set_rest_note_duration_type_authorized(&second, 72);
        let first_post = first_post.expect("first #17 post");
        let second_post = second_post.expect("second #17 post");

        let final_first = first_post.replace("<voice>3</voice>", "<voice>1</voice>");
        let final_second = second_post.replace("<voice>4</voice>", "<voice>2</voice>");
        let legacy_edits = vec![
            (first_start, first_end, final_first.clone()),
            (second_start, second_end, final_second.clone()),
        ];

        let (draft, rebased, receipts) = mutate_selected_duration_fragments(
            &source,
            vec![
                DurationPromotion {
                    start: first_start,
                    end: first_end,
                    post: first_post.clone(),
                    receipt: first_receipt.expect("first #17 receipt"),
                },
                DurationPromotion {
                    start: second_start,
                    end: second_end,
                    post: second_post.clone(),
                    receipt: second_receipt.expect("second #17 receipt"),
                },
            ],
            legacy_edits,
        )
        .expect("fragment draft");

        assert_eq!(receipts.len(), 2);
        assert!(draft.contains(&first_post));
        assert!(draft.contains(&second_post));
        assert_eq!(rebased.len(), 2);
        assert_eq!(&draft[rebased[0].0..rebased[0].1], first_post);
        assert_eq!(&draft[rebased[1].0..rebased[1].1], second_post);

        // This unit test owns only the A->B proof. The productive caller adds
        // the real B->C consumer; here the draft proof is a read-only probe.
        receipts.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
    }

    #[test]
    fn boundary_inside_promoted_fragment_fails_and_closes_trial() {
        let fragment = note(24, "3");
        let source = format!("x{fragment}y");
        let start = source.find(&fragment).expect("note");
        let end = start + fragment.len();
        let (post, receipt) =
            crate::structural_write_authority::set_rest_note_duration_type_authorized(&fragment, 48);
        let result = mutate_selected_duration_fragments(
            &source,
            vec![DurationPromotion {
                start,
                end,
                post: post.expect("post"),
                receipt: receipt.expect("receipt"),
            }],
            vec![(start + 1, end, String::new())],
        );
        assert!(result.is_err());
    }
}
