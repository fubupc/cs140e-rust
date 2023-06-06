/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if !align.is_power_of_two() {
        panic!("align_down: align {} is not a power of 2", align);
    }

    addr & !(align - 1)
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// The returned `usize` is always >= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    // NOTE: If addr is big enough to close to usize::MAX, addr + align will
    // panic with overflow.
    align_down(addr + (align - 1), align)
}
