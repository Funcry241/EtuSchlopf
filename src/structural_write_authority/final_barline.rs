//! Central, source-bound insertion of the final right light-heavy barline.
//!
//! The notation pass provides no project identity, measure number, pitch, staff
//! or voice identity. Authorization is derived only from the complete XML
//! source and the structural boundary immediately before the final
//! `</measure>`. The writer performs exactly one `insert_str` mutation and
//! never writes a file.

use super::authority_primitives::fingerprint as source_fingerprint;
use crate::master_state_identity::{AdditiveTransformationReceipt, ExactEdit};
use anyhow::{anyhow, bail, Result};

/// Sealed mint capability of `authorized_final_barline_writer` (ADDITIVE E004).
pub(crate) struct FinalBarlineAdditiveMint {
    _sealed: (),
}

const FINAL_BARLINE_FRAGMENT: &str =
    "\n      <barline location=\"right\">\n        <bar-style>light-heavy</bar-style>\n      </barline>";

#[derive(Debug, PartialEq, Eq)]
pub enum FinalBarlineAuthorizationOutcome {
    AlreadyPresent,
    MissingFinalMeasure,
    Inserted,
}

#[derive(Debug)]
struct AuthorizedFinalBarlineInsertion {
    source_fingerprint: u64,
    insertion_offset: usize,
    insertion_fingerprint: u64,
}

impl AuthorizedFinalBarlineInsertion {
    fn issue(source_xml: &str) -> Result<Option<Self>> {
        if source_xml.contains("<bar-style>light-heavy</bar-style>") {
            return Ok(None);
        }
        let Some(insertion_offset) = source_xml.rfind("</measure>") else {
            return Ok(None);
        };
        verify_final_measure_boundary(source_xml, insertion_offset)?;
        Ok(Some(Self {
            source_fingerprint: source_fingerprint(source_xml),
            insertion_offset,
            insertion_fingerprint: source_fingerprint(FINAL_BARLINE_FRAGMENT),
        }))
    }
}

/// Ensures the final light-heavy barline. Returns the authorization outcome and
/// the ADDITIVE E004 receipt of the insertion, if one happened.
pub fn ensure_final_barline_authorized(
    xml: &mut String,
) -> Result<(FinalBarlineAuthorizationOutcome, Option<AdditiveTransformationReceipt>)> {
    if xml.contains("<bar-style>light-heavy</bar-style>") {
        return Ok((FinalBarlineAuthorizationOutcome::AlreadyPresent, None));
    }
    if !xml.contains("</measure>") {
        return Ok((FinalBarlineAuthorizationOutcome::MissingFinalMeasure, None));
    }

    let token = AuthorizedFinalBarlineInsertion::issue(xml)?
        .ok_or_else(|| anyhow!("final barline authorization unexpectedly produced no token"))?;
    let (written, receipt) = authorized_final_barline_writer(xml, token);
    written?;
    Ok((FinalBarlineAuthorizationOutcome::Inserted, receipt))
}

fn authorized_final_barline_writer(
    xml: &mut String,
    token: AuthorizedFinalBarlineInsertion,
) -> (Result<usize>, Option<AdditiveTransformationReceipt>) {
    if let Err(error) = verify_final_barline_token(xml, &token) {
        return (Err(error), None);
    }
    let pre_xml = xml.clone();
    xml.insert_str(token.insertion_offset, FINAL_BARLINE_FRAGMENT);
    // ADDITIVE receipt E004, minted by this owner from its bound insertion.
    let receipt = AdditiveTransformationReceipt::mint_additive_receipt(
        FinalBarlineAdditiveMint { _sealed: () },
        &pre_xml,
        xml,
        vec![ExactEdit::bound(
            token.insertion_offset,
            token.insertion_offset,
            FINAL_BARLINE_FRAGMENT.to_string(),
        )],
    );
    crate::master_state_identity::capture_additive_origin(FinalBarlineAdditiveMint { _sealed: () }, &receipt, &pre_xml, xml);
    (Ok(1), receipt)
}

fn verify_final_barline_token(xml: &str, token: &AuthorizedFinalBarlineInsertion) -> Result<()> {
    if source_fingerprint(xml) != token.source_fingerprint {
        bail!("source changed after final barline authorization");
    }
    verify_final_measure_boundary(xml, token.insertion_offset)?;
    let Some(expected_offset) = xml.rfind("</measure>") else {
        bail!("final measure disappeared after final barline authorization");
    };
    if expected_offset != token.insertion_offset {
        bail!("final measure boundary changed after final barline authorization");
    }
    if xml.contains("<bar-style>light-heavy</bar-style>") {
        bail!("final barline appeared after authorization");
    }
    if source_fingerprint(FINAL_BARLINE_FRAGMENT) != token.insertion_fingerprint {
        bail!("final barline fragment changed after authorization");
    }
    Ok(())
}

fn verify_final_measure_boundary(xml: &str, offset: usize) -> Result<()> {
    if offset > xml.len() || !xml.is_char_boundary(offset) {
        bail!("final barline insertion offset is invalid");
    }
    if !xml[offset..].starts_with("</measure>") {
        bail!("final barline insertion is not bound to a measure closing boundary");
    }
    if xml.rfind("</measure>") != Some(offset) {
        bail!("final barline insertion is not bound to the final measure");
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::{
        authorized_final_barline_writer,
        ensure_final_barline_authorized,
        AuthorizedFinalBarlineInsertion,
        FinalBarlineAuthorizationOutcome,
        FINAL_BARLINE_FRAGMENT,
    };

    fn legacy(xml: &mut String) -> FinalBarlineAuthorizationOutcome {
        if xml.contains("<bar-style>light-heavy</bar-style>") {
            return FinalBarlineAuthorizationOutcome::AlreadyPresent;
        }
        let Some(close_measure) = xml.rfind("</measure>") else {
            return FinalBarlineAuthorizationOutcome::MissingFinalMeasure;
        };
        xml.insert_str(close_measure, FINAL_BARLINE_FRAGMENT);
        FinalBarlineAuthorizationOutcome::Inserted
    }

    #[test]
    fn insertion_matches_legacy_bytes_at_final_measure() {
        let source = "<score-partwise><part><measure number=\"\
1\"></measure><measure number=\"\
2\"><note/></measure></part></score-partwise>";
        let mut expected = source.to_string();
        let mut actual = source.to_string();
        assert_eq!(legacy(&mut expected), FinalBarlineAuthorizationOutcome::Inserted);
        let (outcome, receipt) =
            ensure_final_barline_authorized(&mut actual).expect("authorized final barline");
        assert_eq!(outcome, FinalBarlineAuthorizationOutcome::Inserted);
        assert!(receipt.is_some(), "a final barline insertion must carry its ADDITIVE receipt");
        assert_eq!(actual, expected);
        assert_eq!(actual.matches("<bar-style>light-heavy</bar-style>").count(), 1);
        let insertion = actual.find(FINAL_BARLINE_FRAGMENT).expect("inserted fragment");
        let first_close = actual.find("</measure>").expect("first measure close");
        assert!(insertion > first_close);
    }

    #[test]
    fn existing_final_barline_and_missing_measure_remain_no_ops() {
        let mut existing = format!(
            "<score-partwise><measure>{FINAL_BARLINE_FRAGMENT}</measure></score-partwise>"
        );
        let existing_before = existing.clone();
        assert_eq!(
            ensure_final_barline_authorized(&mut existing).expect("existing final barline").0,
            FinalBarlineAuthorizationOutcome::AlreadyPresent
        );
        assert_eq!(existing, existing_before);

        let mut incomplete = "<score-partwise/>".to_string();
        let incomplete_before = incomplete.clone();
        assert_eq!(
            ensure_final_barline_authorized(&mut incomplete).expect("missing measure").0,
            FinalBarlineAuthorizationOutcome::MissingFinalMeasure
        );
        assert_eq!(incomplete, incomplete_before);
    }

    #[test]
    fn changed_source_is_rejected_after_authorization() {
        let source = "<score-partwise><measure number=\"\
1\"></measure></score-partwise>";
        let token = AuthorizedFinalBarlineInsertion::issue(source)
            .expect("authorization")
            .expect("write token");
        let mut changed = source.replace(
            "<score-partwise>",
            "<score-partwise version=\"4.0\">",
        );
        assert!(authorized_final_barline_writer(&mut changed, token).0.is_err());
    }

    #[test]
    fn non_final_boundary_is_rejected() {
        let source = "<score-partwise><measure></measure><measure></measure></score-partwise>";
        let token = AuthorizedFinalBarlineInsertion {
            source_fingerprint: super::source_fingerprint(source),
            insertion_offset: source.find("</measure>").expect("first close"),
            insertion_fingerprint: super::source_fingerprint(FINAL_BARLINE_FRAGMENT),
        };
        let mut xml = source.to_string();
        assert!(authorized_final_barline_writer(&mut xml, token).0.is_err());
    }

    #[test]
    fn token_is_private_move_only_and_writer_is_token_only() {
        let source = include_str!("final_barline.rs");
        let production = source.split("#[cfg(test)]").next().unwrap_or(source);
        assert!(production.contains("struct AuthorizedFinalBarlineInsertion"));
        assert!(!production.contains("#[derive(Clone"));
        assert!(!production.contains("#[derive(Copy"));
        assert_eq!(production.matches("xml.insert_str(").count(), 1);
        assert_eq!(production.matches("std::fs::write").count(), 0);
        assert_eq!(production.matches("fs::write").count(), 0);
    }
}
