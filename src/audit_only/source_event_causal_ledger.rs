//! Read-only SourceEvent causal-ledger report projection.
//!
//! The projection proves raw MIDI pairing and exact canonicalization. It grants no selection,
//! ranking, weighting, hand/staff/voice, mutation or writer authority.

use crate::source_event::{
    CanonicalSourceEventProjection, SourceEventCausalLedgerProjection,
};
use std::fmt::Write as _;

pub(crate) fn append_source_event_causal_ledger_foundation_body(
    out: &mut String,
    canonical: &CanonicalSourceEventProjection,
    causal: &SourceEventCausalLedgerProjection,
) {
    writeln!(out, "SchemaVersion: 1").expect("String write cannot fail");
    writeln!(
        out,
        "PairingContract: global_tick_track_event_order_then_fifo_per_channel_key"
    )
    .expect("String write cannot fail");
    writeln!(out, "ReplayVerified: ja").expect("String write cannot fail");
    writeln!(
        out,
        "RawNoteEventsCovered: {}",
        causal.raw_note_event_count
    )
    .expect("String write cannot fail");
    writeln!(out, "LedgerEntries: {}", causal.entry_count)
        .expect("String write cannot fail");
    writeln!(
        out,
        "CanonicalSourceEvents: {}",
        canonical.event_count
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "DuplicatePitchOnsetEventsPreserved: {}",
        canonical.duplicate_pitch_onset_events
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "CrossTrackPairs: {}",
        causal.cross_track_pair_count
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "CanonicalTerminalExtentTicks: {}",
        canonical.terminal_extent_ticks
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "HistoricalCanonicalEventFingerprintSHA256: {}",
        canonical.event_fingerprint_sha256
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "RawPairLedgerFingerprintSHA256: {}",
        causal.raw_pair_fingerprint_sha256
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "CausalLedgerFingerprintSHA256: {}",
        causal.causal_fingerprint_sha256
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "CanonicalIdentityContract: unchanged_28_bytes_ordinal_sensitive"
    )
    .expect("String write cannot fail");
    writeln!(out, "SelectionSemantics: none").expect("String write cannot fail");
    writeln!(out, "Ranking: none").expect("String write cannot fail");
    writeln!(out, "Weighting: none").expect("String write cannot fail");
    writeln!(out, "HandStaffVoiceAuthority: none").expect("String write cannot fail");
    writeln!(out, "WriterAuthority: none").expect("String write cannot fail");
    writeln!(out, "MusicMutation: nein").expect("String write cannot fail");
}
