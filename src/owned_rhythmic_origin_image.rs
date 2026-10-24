//! A correspondence inside already authenticated owner membership. Intervals
//! compose geometry only; neither this type nor intersections establish identity.
use std::ops::Range;
use anyhow::{Result, ensure, anyhow};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OriginImage {
    pub(super) origin: Range<i64>,
    pub(super) image: Range<i64>,
}

impl OriginImage {
    pub(super) fn new(origin: Range<i64>, image: Range<i64>) -> Result<Self> {
        let original_len = origin.end.checked_sub(origin.start).ok_or_else(|| anyhow!("Origin length overflow"))?;
        let image_len = image.end.checked_sub(image.start).ok_or_else(|| anyhow!("Image length overflow"))?;
        ensure!(original_len > 0 && original_len == image_len, "Non-bijective original carrier piece");
        Ok(Self { origin, image })
    }

    pub(super) fn translated(image: Range<i64>, offset: i64) -> Result<Self> {
        let original = image.start.checked_add(offset).ok_or_else(|| anyhow!("Origin start overflow"))?
            ..image.end.checked_add(offset).ok_or_else(|| anyhow!("Origin end overflow"))?;
        Self::new(original, image)
    }

    /// `successor` relates a new image to a subdomain of this current image.
    /// The caller has already supplied original operation/emission membership.
    pub(super) fn compose(&self, successor: &Self) -> Result<Option<Self>> {
        Self::new(self.origin.clone(), self.image.clone())?;
        Self::new(successor.origin.clone(), successor.image.clone())?;
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
