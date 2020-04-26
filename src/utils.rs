use core::{mem, ptr};

#[cfg(debug_assertions)]
#[inline(always)]
pub fn read_u8(data: &[u8], idx: usize) -> u8 {
    data[idx]
}

#[cfg(not(debug_assertions))]
#[inline(always)]
pub fn read_u8(data: &[u8], idx: usize) -> u8 {
    unsafe {
        *data.get_unchecked(idx)
    }
}

#[inline(always)]
pub fn read_u32(data: &[u8]) -> u32 {
    let mut result = mem::MaybeUninit::<u32>::uninit();
    unsafe {
        ptr::copy_nonoverlapping(data.as_ptr(), &mut result as *mut _ as *mut u8, mem::size_of::<u32>());
        result.assume_init().to_le()
    }
}


#[inline(always)]
pub fn read_part_u32(data: &[u8]) -> u32 {
    ((read_u8(data, 0) as u32) << 16) | (read_u8(data, data.len() >> 1) as u32 ) | read_u8(data, data.len() - 1) as u32
}

#[inline(always)]
pub fn read_u64(data: &[u8], offset: isize) -> u64 {
    let mut result = mem::MaybeUninit::<u64>::uninit();
    unsafe {
        ptr::copy_nonoverlapping(data.as_ptr().offset(offset), &mut result as *mut _ as *mut u8, mem::size_of::<u64>());
        result.assume_init().to_le()
    }
}

#[inline(always)]
pub fn mix(mut left: u64, mut right: u64) -> u64 {
    let mix = u128::from(left) * u128::from(right);
    left = mix as u64;
    right = (mix >> 64) as u64;
    left ^ right
}

