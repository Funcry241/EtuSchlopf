#![forbid(unsafe_code)]

/// Pure finite reading-course model shared by the owner adapter and callers.
/// It contains no material identity, parser, writer, or adoption capability.
#[path = "pipeline_shards/joint_reading_c3_course.rs"]
pub mod joint_reading_course;

pub mod console_presentation_contract;
pub mod console_status_record;
pub mod corpus_run_manifest;
#[path = "build_gate/formal_green_contract.rs"]
pub mod formal_green_contract;

pub mod observation;

pub mod observation_protocol;

pub mod observation_snapshot_projection;

pub mod observation_protocol_encoder;

pub mod observation_transport;

pub mod observation_receiver;

pub mod observation_session_aggregator;
