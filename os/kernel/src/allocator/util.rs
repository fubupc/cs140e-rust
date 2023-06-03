/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align == 0 {
        panic!("align_down: align 0 is not a power of 2");
    }
    let mut align = align;
    let mut power: usize = 0;
    while align != 1 {
        if align & 0x1 == 1 {
            panic!("align_down: align {} is not a power of 2", align);
        }
        align = align >> 1;
        power += 1;
    }

    addr >> power << power
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// The returned `usize` is always >= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    let down = align_down(addr, align);
    if addr == down {
        down
    } else {
        down + align
    }
}
