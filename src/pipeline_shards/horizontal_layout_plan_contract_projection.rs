// Read-only horizontal-layout plan contract projection.
//
// The projection remains snapshot-bound and writer-free. Take 4 reuses the
// shared structural directive model introduced by the decision owner so the
// immutable planning result and the final readback compare the same fields.

#[derive(Debug, Clone)]
struct HorizontalLayoutPlanContractProjection {
    source_fingerprint: u64,
    slots: Vec<HorizontalLayoutDirectiveState>,
}

impl HorizontalLayoutPlanContractProjection {
    fn capture(xml: &str, source_fingerprint: u64) -> anyhow::Result<Self> {
        Ok(Self {
            source_fingerprint,
            slots: capture_horizontal_layout_directives(xml)?,
        })
    }

    fn matches_snapshot_bound_source(
        &self,
        xml: &str,
        source_fingerprint: u64,
    ) -> anyhow::Result<bool> {
        if self.source_fingerprint != source_fingerprint {
            return Ok(false);
        }
        let current = Self::capture(xml, source_fingerprint)?;
        if self.slots.len() != current.slots.len() {
            return Ok(false);
        }
        Ok(self
            .slots
            .iter()
            .zip(current.slots.iter())
            .all(|(left, right)| left.same_directives(right)))
    }

    fn matches_decision_plan(&self, plan: &HorizontalLayoutPlan) -> bool {
        self.slots.len() == plan.slots.len()
            && self
                .slots
                .iter()
                .zip(plan.slots.iter())
                .all(|(projected, planned)| {
                    projected.same_directives(&planned.directives)
                })
    }
}

#[cfg(test)]
mod horizontal_layout_plan_contract_projection_tests {
    use super::{
        HorizontalLayoutPlanContractProjection, horizontal_layout_opening_tag_attribute,
    };

    #[test]
    fn projection_uses_structural_sequence_without_origin_identity() {
        let xml = r#"<score><part><measure number="alpha" width="1200"><print new-system="yes"/></measure><measure number="beta"><print new-page="yes"/></measure></part></score>"#;
        let plan = HorizontalLayoutPlanContractProjection::capture(xml, 7)
            .expect("structural projection");

        assert!(plan
            .matches_snapshot_bound_source(xml, 7)
            .expect("exact projection"));
        assert!(!plan
            .matches_snapshot_bound_source(xml, 8)
            .expect("fingerprint mismatch"));
    }

    #[test]
    fn projection_rejects_horizontal_directive_delta() {
        let before = r#"<score><part><measure number="x" width="900"></measure></part></score>"#;
        let after = r#"<score><part><measure number="x" width="901"></measure></part></score>"#;
        let plan = HorizontalLayoutPlanContractProjection::capture(before, 11)
            .expect("structural projection");

        assert!(!plan
            .matches_snapshot_bound_source(after, 11)
            .expect("directive comparison"));
    }

    #[test]
    fn attribute_reader_supports_both_quote_styles() {
        assert_eq!(
            horizontal_layout_opening_tag_attribute("<measure width=\"123\">", "width")
                .as_deref(),
            Some("123")
        );
        assert_eq!(
            horizontal_layout_opening_tag_attribute("<measure width='456'>", "width")
                .as_deref(),
            Some("456")
        );
    }
}
