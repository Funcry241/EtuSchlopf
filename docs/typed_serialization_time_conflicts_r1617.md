# r1617 Typed serialization time conflicts

The overlap queries accept separate WrittenInterval and SoundingInterval values.
The greedy allocator, overflow search and cross-role sharing retain the existing
logical union of both typed conflict sets. The rendered scores are expected to
remain byte-identical in both Master and Render MusicXML for all ten projects.

## Existing tests that depend on SoundingDifferent

These eleven tests passed with the packaged implementation and failed in a
separate experiment that omitted SoundingDifferent only from the shared channel
conflict query. Names are relative to structural_write_authority::serialization_plan.
The failures indicate policy dependence. They do not establish that removing the
edge alone would be musically safe.

- `actual_source_beam_binding::tests::an_actual_rest_on_the_guest_label_keeps_that_channel_protected`
- `actual_source_beam_binding::tests::complete_channel_search_uses_the_original_staff_split_container`
- `actual_source_beam_binding::tests::complete_measure_components_can_evacuate_one_channel_to_distinct_hosts`
- `actual_source_beam_binding::tests::distributed_channel_search_refuses_a_component_with_no_conflict_free_host`
- `actual_source_beam_binding::tests::distributed_guest_majorities_preserve_productive_host_labels`
- `actual_source_beam_binding::tests::lower_primary_reader_predicate_remains_a_binding_obligation`
- `actual_source_beam_binding::tests::many_independent_components_use_one_linear_host_selection_pass`
- `actual_source_beam_binding::tests::unknown_context_with_new_same_measure_partners_is_refused`
- `channel_consumer_allocation_tests::greedy_overflow_is_repaired_by_search_under_unchanged_edges_and_roles`
- `channel_consumer_allocation_tests::independent_temporal_copies_preserve_the_recoloured_local_partitions`
- `cross_role_channel_sharing::tests::a_refused_complete_distribution_can_use_another_host_without_relaxing_beams`

The additional test sounding_overlap_without_written_overlap_still_separates_channels
checks both time predicates and expects two channels under the current rule.
It was the twelfth failure in the experiment.
