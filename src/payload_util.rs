//! Internal helpers for per-message encode/decode bodies. Centralizes the
//! length validations that every payload module would otherwise duplicate.

use crate::error::Error;

/// Reject any payload whose length is not exactly `expected`.
#[inline]
pub(crate) fn require_exact_len(data: &[u8], expected: usize, code: u8) -> Result<(), Error> {
    if data.len() != expected {
        return Err(Error::PayloadLength {
            code,
            expected,
            got: data.len(),
        });
    }
    Ok(())
}

/// Reject any payload shorter than `min` (used when a header has variable-
/// length tail and we need at least `min` bytes to parse the fixed prefix).
#[inline]
pub(crate) fn require_at_least(data: &[u8], min: usize, code: u8) -> Result<(), Error> {
    if data.len() < min {
        return Err(Error::PayloadTooShort {
            code,
            min,
            got: data.len(),
        });
    }
    Ok(())
}

/// Reject empty payloads or those whose length is not a multiple of `block`.
/// Used by record-array commands like `osdp_OUT` and `osdp_LED` where each
/// record is a fixed `block` bytes wide.
#[inline]
pub(crate) fn require_positive_multiple_of(
    data: &[u8],
    block: usize,
    code: u8,
) -> Result<(), Error> {
    if data.is_empty() || data.len() % block != 0 {
        return Err(Error::PayloadNotMultiple {
            code,
            block,
            got: data.len(),
        });
    }
    Ok(())
}

/// Reject payloads whose length is not a multiple of `block`. Empty payloads
/// pass — used by lists like `osdp_PDCAP` where zero records is meaningful.
#[inline]
pub(crate) fn require_multiple_of(data: &[u8], block: usize, code: u8) -> Result<(), Error> {
    if data.len() % block != 0 {
        return Err(Error::PayloadNotMultiple {
            code,
            block,
            got: data.len(),
        });
    }
    Ok(())
}
