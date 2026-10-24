// Central first-class horizontal-layout decision model.
//
// The structural seed produced by the base paginator enters here
// without origin identity. Every later horizontal decision is recorded in this
// one builder. Take 6 closes the immutable plan before one source-bound
// StructuralWriteAuthority materializer emits horizontal geometry exactly once.

#[derive(Debug, Clone, PartialEq, Eq)]
struct HorizontalLayoutDirectiveState {
    explicit_width: Option<String>,
    starts_new_system: bool,
    starts_new_page: bool,
}

impl HorizontalLayoutDirectiveState {
    fn same_directives(&self, other: &Self) -> bool {
        self == other
    }
}

fn capture_horizontal_layout_directives(
    xml: &str,
) -> anyhow::Result<Vec<HorizontalLayoutDirectiveState>> {
    let mut slots = Vec::new();
    let mut scan = 0usize;

    while let Some(rel) = xml[scan..].find("<measure") {
        let start = scan + rel;
        if !horizontal_layout_is_measure_start(xml, start) {
            scan = start + "<measure".len();
            continue;
        }
        let Some(open_end_rel) = xml[start..].find('>') else {
            anyhow::bail!("HorizontalLayoutDecisionPlan: unvollstaendiges measure-Starttag");
        };
        let open_end = start + open_end_rel + 1;
        let opening_tag = &xml[start..open_end];
        let Some(close_rel) = xml[open_end..].find("</measure>") else {
            anyhow::bail!("HorizontalLayoutDecisionPlan: unvollstaendiger measure-Block");
        };
        let close_start = open_end + close_rel;
        let body = &xml[open_end..close_start];

        slots.push(HorizontalLayoutDirectiveState {
            explicit_width: horizontal_layout_opening_tag_attribute(opening_tag, "width"),
            starts_new_system: body.contains("new-system=\"yes\"")
                || body.contains("new-system='yes'"),
            starts_new_page: body.contains("new-page=\"yes\"")
                || body.contains("new-page='yes'"),
        });

        scan = close_start + "</measure>".len();
    }

    if slots.is_empty() {
        anyhow::bail!("HorizontalLayoutDecisionPlan: keine strukturellen measure-Slots gefunden");
    }
    Ok(slots)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HorizontalLayoutDecisionOwner {
    Seed,
    VisualLayoutPressure,
    FinalShortSystemBalance,
    GlobalPageSystemReflow,
    RendererFeasibility,
}

#[derive(Debug, Clone)]
struct HorizontalLayoutPlannedSlot {
    directives: HorizontalLayoutDirectiveState,
    width_owner: Option<HorizontalLayoutDecisionOwner>,
    system_owner: Option<HorizontalLayoutDecisionOwner>,
    page_owner: Option<HorizontalLayoutDecisionOwner>,
}

impl HorizontalLayoutPlannedSlot {
    fn from_seed(seed: &HorizontalLayoutSeedSlot) -> Self {
        Self {
            directives: HorizontalLayoutDirectiveState {
                explicit_width: None,
                starts_new_system: seed.starts_new_system,
                starts_new_page: seed.starts_new_page,
            },
            width_owner: None,
            system_owner: seed
                .starts_new_system
                .then_some(HorizontalLayoutDecisionOwner::Seed),
            page_owner: seed
                .starts_new_page
                .then_some(HorizontalLayoutDecisionOwner::Seed),
        }
    }

    fn from_directives_as_seed(directives: HorizontalLayoutDirectiveState) -> Self {
        let width_owner = directives
            .explicit_width
            .as_ref()
            .map(|_| HorizontalLayoutDecisionOwner::Seed);
        let system_owner = directives
            .starts_new_system
            .then_some(HorizontalLayoutDecisionOwner::Seed);
        let page_owner = directives
            .starts_new_page
            .then_some(HorizontalLayoutDecisionOwner::Seed);
        Self {
            directives,
            width_owner,
            system_owner,
            page_owner,
        }
    }

    fn provenance_complete(&self) -> bool {
        (self.directives.explicit_width.is_none() || self.width_owner.is_some())
            && (!self.directives.starts_new_system || self.system_owner.is_some())
            && (!self.directives.starts_new_page || self.page_owner.is_some())
    }
}

#[derive(Debug, Clone)]
struct HorizontalLayoutPlan {
    slots: Vec<HorizontalLayoutPlannedSlot>,
}

impl HorizontalLayoutPlan {
    fn provenance_complete(&self) -> bool {
        !self.slots.is_empty()
            && self
                .slots
                .iter()
                .all(HorizontalLayoutPlannedSlot::provenance_complete)
    }

    fn matches_materialized(&self, xml: &str) -> anyhow::Result<bool> {
        let current = capture_horizontal_layout_directives(xml)?;
        if self.slots.len() != current.len() {
            return Ok(false);
        }
        Ok(self
            .slots
            .iter()
            .zip(current.iter())
            .all(|(planned, actual)| planned.directives.same_directives(actual)))
    }

    fn materialization_directives(
        &self,
    ) -> Vec<structural_write_authority::HorizontalLayoutMaterializationDirective> {
        self.slots
            .iter()
            .map(|slot| {
                let system_break = if slot.directives.starts_new_page {
                    structural_write_authority::HorizontalSystemBreakMaterialization::None
                } else if slot.directives.starts_new_system {
                    structural_write_authority::HorizontalSystemBreakMaterialization::Enabled
                } else if slot.system_owner
                    == Some(HorizontalLayoutDecisionOwner::FinalShortSystemBalance)
                {
                    structural_write_authority::HorizontalSystemBreakMaterialization::ExplicitlyDisabled
                } else {
                    structural_write_authority::HorizontalSystemBreakMaterialization::None
                };
                structural_write_authority::HorizontalLayoutMaterializationDirective {
                    explicit_width: slot.directives.explicit_width.clone(),
                    system_break,
                    starts_new_page: slot.directives.starts_new_page,
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct HorizontalLayoutPlanDecisionDriftAudit {
    slots_compared: usize,
    changed_slots: usize,
    width_changes: usize,
    system_break_changes: usize,
    page_break_changes: usize,
    provenance_changes: usize,
}

impl HorizontalLayoutPlan {
    fn compare_decisions(
        &self,
        other: &Self,
    ) -> anyhow::Result<HorizontalLayoutPlanDecisionDriftAudit> {
        if self.slots.len() != other.slots.len() {
            anyhow::bail!(
                "HorizontalLayoutFinalNotationPlanAudit: Slotanzahl driftete von {} auf {}",
                self.slots.len(),
                other.slots.len()
            );
        }

        let mut audit = HorizontalLayoutPlanDecisionDriftAudit {
            slots_compared: self.slots.len(),
            ..HorizontalLayoutPlanDecisionDriftAudit::default()
        };
        for (before, after) in self.slots.iter().zip(other.slots.iter()) {
            let mut slot_changed = false;
            if before.directives.explicit_width != after.directives.explicit_width {
                audit.width_changes += 1;
                slot_changed = true;
            }
            if before.directives.starts_new_system != after.directives.starts_new_system {
                audit.system_break_changes += 1;
                slot_changed = true;
            }
            if before.directives.starts_new_page != after.directives.starts_new_page {
                audit.page_break_changes += 1;
                slot_changed = true;
            }
            if before.width_owner != after.width_owner
                || before.system_owner != after.system_owner
                || before.page_owner != after.page_owner
            {
                audit.provenance_changes += 1;
                slot_changed = true;
            }
            if slot_changed {
                audit.changed_slots += 1;
            }
        }
        Ok(audit)
    }
}

struct HorizontalLayoutPlanBuilder {
    slots: Vec<HorizontalLayoutPlannedSlot>,
}

impl HorizontalLayoutPlanBuilder {
    fn from_seed(
        seed: &HorizontalLayoutSeedPlan,
        materialized_xml: &str,
    ) -> anyhow::Result<Self> {
        let slots = if seed.slots.is_empty() {
            capture_horizontal_layout_directives(materialized_xml)?
                .into_iter()
                .map(HorizontalLayoutPlannedSlot::from_directives_as_seed)
                .collect::<Vec<_>>()
        } else {
            seed.slots
                .iter()
                .map(HorizontalLayoutPlannedSlot::from_seed)
                .collect::<Vec<_>>()
        };
        let builder = Self { slots };
        if !builder.working_xml_is_compatible(materialized_xml)? {
            anyhow::bail!(
                "HorizontalLayoutPlanMaterializationClosure: struktureller Layout-Seed und noch unmaterialisiertes Arbeits-XML sind nicht kompatibel"
            );
        }
        Ok(builder)
    }

    fn from_seed_for_final_notation(
        seed: &HorizontalLayoutSeedPlan,
        final_notation_xml: &str,
    ) -> anyhow::Result<Self> {
        if seed.slots.is_empty() {
            anyhow::bail!(
                "HorizontalLayoutFinalNotationPlanAudit: struktureller Layout-Seed fehlt"
            );
        }
        let final_slot_count = capture_horizontal_layout_directives(final_notation_xml)?.len();
        if seed.slots.len() != final_slot_count {
            anyhow::bail!(
                "HorizontalLayoutFinalNotationPlanAudit: Layout-Seed-Slots {} stimmen nicht mit finalen Notationsslots {} ueberein",
                seed.slots.len(),
                final_slot_count
            );
        }
        Ok(Self {
            slots: seed
                .slots
                .iter()
                .map(HorizontalLayoutPlannedSlot::from_seed)
                .collect(),
        })
    }

    fn plan_new_system(
        &mut self,
        slot_index: usize,
        owner: HorizontalLayoutDecisionOwner,
    ) {
        let slot = self
            .slots
            .get_mut(slot_index)
            .expect("horizontal-layout slot comes from the same structural scan");
        if !slot.directives.starts_new_page && !slot.directives.starts_new_system {
            slot.directives.starts_new_system = true;
            slot.system_owner = Some(owner);
        }
    }

    fn plan_new_page(
        &mut self,
        slot_index: usize,
        owner: HorizontalLayoutDecisionOwner,
    ) {
        let slot = self
            .slots
            .get_mut(slot_index)
            .expect("horizontal-layout slot comes from the same structural scan");
        slot.directives.starts_new_page = true;
        slot.directives.starts_new_system = false;
        slot.page_owner = Some(owner);
        slot.system_owner = Some(owner);
    }

    fn plan_system_state(
        &mut self,
        slot_index: usize,
        starts_new_system: bool,
        owner: HorizontalLayoutDecisionOwner,
    ) {
        let slot = self
            .slots
            .get_mut(slot_index)
            .expect("horizontal-layout slot comes from the same structural scan");
        slot.directives.starts_new_system = starts_new_system;
        slot.system_owner = Some(owner);
    }

    fn working_xml_is_compatible(&self, xml: &str) -> anyhow::Result<bool> {
        let current = capture_horizontal_layout_directives(xml)?;
        if self.slots.len() != current.len() {
            return Ok(false);
        }
        Ok(self
            .slots
            .iter()
            .zip(current.iter())
            .all(|(planned, actual)| {
                let actual_width = actual
                    .explicit_width
                    .as_deref()
                    .and_then(|value| value.trim().parse::<usize>().ok());
                let planned_width = planned
                    .directives
                    .explicit_width
                    .as_deref()
                    .and_then(|value| value.trim().parse::<usize>().ok());
                let width_compatible = match (actual_width, planned_width) {
                    (None, _) => actual.explicit_width.is_none(),
                    (Some(actual), Some(planned)) => actual <= planned,
                    _ => false,
                };
                let system_compatible = !actual.starts_new_system
                    || planned.directives.starts_new_system
                    || planned.directives.starts_new_page
                    || planned.system_owner
                        == Some(HorizontalLayoutDecisionOwner::FinalShortSystemBalance);
                let page_compatible =
                    !actual.starts_new_page || planned.directives.starts_new_page;
                width_compatible && system_compatible && page_compatible
            }))
    }

    fn slot_count(&self) -> usize {
        self.slots.len()
    }

    fn planned_starts_new_system(&self, slot_index: usize) -> Option<bool> {
        self.slots
            .get(slot_index)
            .map(|slot| slot.directives.starts_new_system)
    }

    fn planned_starts_new_page(&self, slot_index: usize) -> Option<bool> {
        self.slots
            .get(slot_index)
            .map(|slot| slot.directives.starts_new_page)
    }

    fn planned_system_start_slots(&self) -> Vec<usize> {
        if self.slots.is_empty() {
            return Vec::new();
        }
        let mut starts = vec![0usize];
        starts.extend(
            self.slots
                .iter()
                .enumerate()
                .skip(1)
                .filter_map(|(slot_index, slot)| {
                    (slot.directives.starts_new_system || slot.directives.starts_new_page)
                        .then_some(slot_index)
                }),
        );
        starts
    }

    fn close(self) -> anyhow::Result<HorizontalLayoutPlan> {
        let plan = HorizontalLayoutPlan {
            slots: self.slots,
        };
        if !plan.provenance_complete() {
            anyhow::bail!(
                "HorizontalLayoutDecisionOwnership: geschlossener Plan besitzt unvollstaendige Decision-Provenienz"
            );
        }
        Ok(plan)
    }
}

fn horizontal_layout_slot_index_for_locator(
    xml: &str,
    target_locator: usize,
) -> Option<usize> {
    let mut scan = 0usize;
    let mut slot_index = 0usize;
    while let Some((_start, end, locator)) = find_next_measure_bounds(xml, scan) {
        if locator == target_locator {
            return Some(slot_index);
        }
        slot_index += 1;
        scan = end;
    }
    None
}

fn horizontal_layout_is_measure_start(xml: &str, start: usize) -> bool {
    matches!(
        xml.as_bytes().get(start + "<measure".len()).copied(),
        Some(b' ' | b'\t' | b'\r' | b'\n' | b'>')
    )
}

fn horizontal_layout_opening_tag_attribute(opening_tag: &str, key: &str) -> Option<String> {
    for quote in ['\"', '\''] {
        let needle = format!("{key}={quote}");
        let Some(start) = opening_tag.find(&needle) else {
            continue;
        };
        let value_start = start + needle.len();
        let value_end = opening_tag[value_start..].find(quote)? + value_start;
        return Some(opening_tag[value_start..value_end].to_string());
    }
    None
}

#[cfg(test)]
mod horizontal_layout_decision_plan_tests {
    use super::{
        HorizontalLayoutDecisionOwner, HorizontalLayoutPlanBuilder,
        HorizontalLayoutSeedPlan, HorizontalLayoutSeedSlot,
    };

    #[test]
    fn decision_builder_preserves_renderer_auto_width_seed_boundary() {
        let xml = r#"<score><part><measure number="alpha"></measure><measure number="beta"><print new-system="yes"/></measure></part></score>"#;
        let seed = HorizontalLayoutSeedPlan {
            slots: vec![
                HorizontalLayoutSeedSlot {
                    starts_new_system: false,
                    starts_new_page: false,
                },
                HorizontalLayoutSeedSlot {
                    starts_new_system: true,
                    starts_new_page: false,
                },
            ],
        };
        let builder =
            HorizontalLayoutPlanBuilder::from_seed(&seed, xml).expect("seed parity");
        let plan = builder.close().expect("plan close");

        assert!(plan.provenance_complete());
        assert!(plan
            .slots
            .iter()
            .all(|slot| slot.directives.explicit_width.is_none() && slot.width_owner.is_none()));
        assert!(plan.matches_materialized(xml).expect("renderer-auto seed parity"));
    }

    #[test]
    fn page_replacement_owns_system_false_state() {
        let xml = r#"<score><part><measure number="x"><print new-system="yes"/></measure></part></score>"#;
        let seed = HorizontalLayoutSeedPlan {
            slots: vec![HorizontalLayoutSeedSlot {
                starts_new_system: true,
                starts_new_page: false,
            }],
        };
        let mut builder =
            HorizontalLayoutPlanBuilder::from_seed(&seed, xml).expect("seed parity");
        builder.plan_new_page(0, HorizontalLayoutDecisionOwner::VisualLayoutPressure);
        let plan = builder.close().expect("plan close");

        assert!(plan.slots[0].directives.starts_new_page);
        assert!(!plan.slots[0].directives.starts_new_system);
    }
    #[test]
    fn planned_system_state_is_structural_and_identity_free() {
        let xml = r#"<score><part><measure number="alpha"></measure><measure number="beta"><print new-system="yes"/></measure><measure number="gamma"><print new-page="yes"/></measure></part></score>"#;
        let seed = HorizontalLayoutSeedPlan {
            slots: vec![
                HorizontalLayoutSeedSlot::default(),
                HorizontalLayoutSeedSlot {
                    starts_new_system: true,
                    starts_new_page: false,
                },
                HorizontalLayoutSeedSlot {
                    starts_new_system: false,
                    starts_new_page: true,
                },
            ],
        };
        let builder =
            HorizontalLayoutPlanBuilder::from_seed(&seed, xml).expect("seed parity");

        assert_eq!(builder.slot_count(), 3);
        assert_eq!(builder.planned_system_start_slots(), vec![0, 1, 2]);
        assert_eq!(builder.planned_starts_new_system(1), Some(true));
        assert_eq!(builder.planned_starts_new_page(2), Some(true));
        assert_eq!(builder.planned_starts_new_system(99), None);
    }

}
