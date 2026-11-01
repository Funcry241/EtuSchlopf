//! A correspondence inside already authenticated owner membership. Intervals
//! compose geometry only; neither this type nor intersections establish identity.
use std::ops::Range;
use anyhow::{Result, ensure, anyhow};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OriginImage {
    pub(super) origin: Range<i64>,
    pub(super) image: Range<i64>,
    grace_point: bool,
}

impl OriginImage {
    pub(super) fn new(origin: Range<i64>, image: Range<i64>) -> Result<Self> {
        let original_len = origin.end.checked_sub(origin.start).ok_or_else(|| anyhow!("Origin length overflow"))?;
        let image_len = image.end.checked_sub(image.start).ok_or_else(|| anyhow!("Image length overflow"))?;
        ensure!(original_len > 0 && original_len == image_len, "Non-bijective original carrier piece");
        Ok(Self { origin, image, grace_point: false })
    }

    // Only the owner with an authenticated source-bound grace carrier may
    // create point support. Ordinary interval construction stays positive.
    pub(super) fn grace_point(origin: i64, image: i64) -> Self {
        Self { origin: origin..origin, image: image..image, grace_point: true }
    }

    pub(super) fn validate(&self) -> Result<()> {
        if self.grace_point {
            ensure!(self.origin.is_empty() && self.origin.start == self.origin.end
                && self.image.is_empty() && self.image.start == self.image.end,
                "Grace carrier is not a point");
            Ok(())
        } else { Self::new(self.origin.clone(), self.image.clone()).map(|_| ()) }
    }

    pub(super) fn translated(image: Range<i64>, offset: i64) -> Result<Self> {
        let original = image.start.checked_add(offset).ok_or_else(|| anyhow!("Origin start overflow"))?
            ..image.end.checked_add(offset).ok_or_else(|| anyhow!("Origin end overflow"))?;
        Self::new(original, image)
    }

    /// `successor` relates a new image to a subdomain of this current image.
    /// The caller has already supplied original operation/emission membership.
    pub(super) fn compose(&self, successor: &Self) -> Result<Option<Self>> {
        self.validate()?;
        successor.validate()?;
        if self.grace_point || successor.grace_point {
            return Ok((self.grace_point && successor.grace_point
                && self.image.start == successor.origin.start)
                .then(|| Self::grace_point(self.origin.start, successor.image.start)));
        }
        let start = self.image.start.max(successor.origin.start);
        let end = self.image.end.min(successor.origin.end);
        if start >= end { return Ok(None); }
        let origin_start = self.origin.start.checked_add(start.checked_sub(self.image.start).ok_or_else(|| anyhow!("Origin intersection overflow"))?)
            .ok_or_else(|| anyhow!("Origin composition overflow"))?;
        let image_start = successor.image.start.checked_add(start.checked_sub(successor.origin.start).ok_or_else(|| anyhow!("Image intersection overflow"))?)
            .ok_or_else(|| anyhow!("Image composition overflow"))?;
        let len = end.checked_sub(start).ok_or_else(|| anyhow!("Intersection length overflow"))?;
        Self::new(origin_start..origin_start.checked_add(len).ok_or_else(|| anyhow!("Origin end overflow"))?,
            image_start..image_start.checked_add(len).ok_or_else(|| anyhow!("Image end overflow"))?).map(Some)
    }
}
