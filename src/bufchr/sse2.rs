use core::{arch::x86_64::*, cmp, mem::size_of};
use crate::bufchr::fallback;

const VECTOR_SIZE: usize = size_of::<__m128i>();
const LOOP_COUNT: usize = 4;
const BATCH_BYTE_SIZE: usize = VECTOR_SIZE * LOOP_COUNT;

pub fn get_vector_size() -> usize {
    VECTOR_SIZE
}

pub fn get_batch_byte_size() -> usize {
    return VECTOR_SIZE * 4;
}

pub unsafe fn bufchr(haystack: &[u8], n1: u8, vector_end_ptr: *const u8) -> (Option<usize>, u64) {
    let haystack_len = haystack.len();
    if haystack_len < BATCH_BYTE_SIZE {
        return fallback::bufchr(haystack, n1, vector_end_ptr);
    }
    let start_ptr = haystack.as_ptr();
    let mut ptr = haystack.as_ptr();
    let vn1 = _mm_set1_epi8(n1 as i8);

    while ptr < vector_end_ptr{
        // https://stackoverflow.com/a/15964428/6652082
        // if memory alignment work, use _mm_load_si128
        let chunk1 = _mm_loadu_si128(ptr as *const __m128i);
        let chunk2 = _mm_loadu_si128(ptr.add(VECTOR_SIZE) as *const __m128i);
        let chunk3 = _mm_loadu_si128(ptr.add(VECTOR_SIZE*2) as *const __m128i);
        let chunk4 = _mm_loadu_si128(ptr.add(VECTOR_SIZE*3) as *const __m128i);
        let mask1 = _mm_movemask_epi8(_mm_cmpeq_epi8(vn1, chunk1));
        let mask2 = _mm_movemask_epi8(_mm_cmpeq_epi8(vn1, chunk2));
        let mask3 = _mm_movemask_epi8(_mm_cmpeq_epi8(vn1, chunk3));
        let mask4 = _mm_movemask_epi8(_mm_cmpeq_epi8(vn1, chunk4));
        if (mask1 | mask2 | mask3 | mask4) != 0 {
            let umask = to_u64(mask1, mask2, mask3, mask4);
            let bit_pos = umask.trailing_zeros() as usize;
            // Reset lowest set bit	
            let cache = umask & (umask - 1);   
            return (Some(sub(ptr, start_ptr) + bit_pos), cache);
        }
        ptr = ptr.add(BATCH_BYTE_SIZE);
    }

    let rest_haystack = std::slice::from_raw_parts(
        vector_end_ptr, haystack_len % VECTOR_SIZE);
    fallback::bufchr(rest_haystack, n1, vector_end_ptr)
}

#[target_feature(enable = "sse2")]
pub unsafe fn bufchr2(haystack: &[u8], n1: u8, n2: u8, vector_end_ptr: *const u8) -> (Option<usize>, u64) {
    let haystack_len = haystack.len();
    if haystack_len < BATCH_BYTE_SIZE {
        return fallback::bufchr2(haystack, n1, n2, vector_end_ptr);
    }
    let start_ptr = haystack.as_ptr();
    let mut ptr = start_ptr;
    let align_end_ptr = start_ptr.add((haystack_len / VECTOR_SIZE) * VECTOR_SIZE);
    let vn1 = _mm_set1_epi8(n1 as i8);
    let vn2 = _mm_set1_epi8(n2 as i8);

    while ptr < vector_end_ptr{
        // https://stackoverflow.com/a/15964428/6652082
        // if memory alignment work, use _mm_load_si128
        let chunk = _mm_loadu_si128(ptr as *const __m128i);
        let eq1 = _mm_cmpeq_epi8(vn1, chunk);
        let eq2 = _mm_cmpeq_epi8(vn2, chunk);
        let mask1 = _mm_movemask_epi8(eq1) | _mm_movemask_epi8(eq2);

        let chunk = _mm_loadu_si128(ptr.add(VECTOR_SIZE) as *const __m128i);
        let eq1 = _mm_cmpeq_epi8(vn1, chunk);
        let eq2 = _mm_cmpeq_epi8(vn2, chunk);
        let mask2 = _mm_movemask_epi8(eq1) | _mm_movemask_epi8(eq2);

        let chunk = _mm_loadu_si128(ptr.add(VECTOR_SIZE * 2) as *const __m128i);
        let eq1 = _mm_cmpeq_epi8(vn1, chunk);
        let eq2 = _mm_cmpeq_epi8(vn2, chunk);
        let mask3 = _mm_movemask_epi8(eq1) | _mm_movemask_epi8(eq2);

        let chunk = _mm_loadu_si128(ptr.add(VECTOR_SIZE * 3) as *const __m128i);
        let eq1 = _mm_cmpeq_epi8(vn1, chunk);
        let eq2 = _mm_cmpeq_epi8(vn2, chunk);
        let mask4 = _mm_movemask_epi8(eq1) | _mm_movemask_epi8(eq2);
        
        if (mask1 | mask2 | mask3 | mask4) != 0 {
            let umask = to_u64(mask1, mask2, mask3, mask4);
            let bit_pos = umask.trailing_zeros() as usize;
            // Reset lowest set bit	
            let cache = umask & (umask - 1);   
            return (Some(sub(ptr, start_ptr) + bit_pos), cache);
        }
        ptr = ptr.add(BATCH_BYTE_SIZE);
    }

    let rest_haystack = std::slice::from_raw_parts(
        align_end_ptr, haystack_len % VECTOR_SIZE);
    fallback::bufchr2(rest_haystack, n1, n2, vector_end_ptr)
}

#[target_feature(enable = "sse2")]
pub unsafe fn bufchr3(haystack: &[u8], n1: u8, n2: u8, n3: u8, vector_end_ptr: *const u8) -> (Option<usize>, u64) {
    let haystack_len = haystack.len();
    if haystack_len < BATCH_BYTE_SIZE {
        return fallback::bufchr3(haystack, n1, n2, n3, vector_end_ptr);
    }
    let start_ptr = haystack.as_ptr();
    let mut ptr = start_ptr;
    let end_ptr = start_ptr.add(haystack_len);
    let align_end_ptr = start_ptr.add((haystack_len / VECTOR_SIZE) * VECTOR_SIZE);
    let vn1 = _mm_set1_epi8(n1 as i8);
    let vn2 = _mm_set1_epi8(n2 as i8);
    let vn3 = _mm_set1_epi8(n3 as i8);

    while ptr < end_ptr{
        // https://stackoverflow.com/a/15964428/6652082
        // if memory alignment work, use _mm_load_si128
        let chunk = _mm_loadu_si128(ptr as *const __m128i);
        let eq1 = _mm_cmpeq_epi8(vn1, chunk);
        let eq2 = _mm_cmpeq_epi8(vn2, chunk);
        let eq3 = _mm_cmpeq_epi8(vn3, chunk);
        let mask1 = _mm_movemask_epi8(eq1) | _mm_movemask_epi8(eq2) | _mm_movemask_epi8(eq3);

        let chunk = _mm_loadu_si128(ptr.add(VECTOR_SIZE) as *const __m128i);
        let eq1 = _mm_cmpeq_epi8(vn1, chunk);
        let eq2 = _mm_cmpeq_epi8(vn2, chunk);
        let eq3 = _mm_cmpeq_epi8(vn3, chunk);
        let mask2 = _mm_movemask_epi8(eq1) | _mm_movemask_epi8(eq2) | _mm_movemask_epi8(eq3);

        let chunk = _mm_loadu_si128(ptr.add(VECTOR_SIZE * 2) as *const __m128i);
        let eq1 = _mm_cmpeq_epi8(vn1, chunk);
        let eq2 = _mm_cmpeq_epi8(vn2, chunk);
        let eq3 = _mm_cmpeq_epi8(vn3, chunk);
        let mask3 = _mm_movemask_epi8(eq1) | _mm_movemask_epi8(eq2) | _mm_movemask_epi8(eq3);

        let chunk = _mm_loadu_si128(ptr.add(VECTOR_SIZE * 3) as *const __m128i);
        let eq1 = _mm_cmpeq_epi8(vn1, chunk);
        let eq2 = _mm_cmpeq_epi8(vn2, chunk);
        let eq3 = _mm_cmpeq_epi8(vn3, chunk);
        let mask4 = _mm_movemask_epi8(eq1) | _mm_movemask_epi8(eq2) | _mm_movemask_epi8(eq3);

        if (mask1 | mask2 | mask3 | mask4) != 0 {
            let umask = to_u64(mask1, mask2, mask3, mask4);
            let bit_pos = umask.trailing_zeros() as usize;
            // Reset lowest set bit	
            let cache = umask & (umask - 1);   
            return (Some(sub(ptr, start_ptr) + bit_pos), cache);
        }
        ptr = ptr.add(BATCH_BYTE_SIZE);
    }

    let rest_haystack = std::slice::from_raw_parts(
        align_end_ptr, haystack_len % VECTOR_SIZE);
    return fallback::bufchr3(haystack, n1, n2, n3, vector_end_ptr);
}

#[inline]
fn forward_pos(mask: u32) -> usize {
    mask.trailing_zeros() as usize
}

#[inline]
fn sub(a: *const u8, b: *const u8) -> usize {
    debug_assert!(a >= b);
    (a as usize) - (b as usize)
}

#[inline]
fn to_u32(i: i32) -> u32 {
    let x_bytes = i.to_be_bytes();   
    u32::from_be_bytes(x_bytes)
}

#[inline]
fn to_u64(i1: i32, i2: i32, i3: i32, i4: i32) -> u64 {
    (i4 as u64) << 48 | (i3 as u64) <<32 | (i2  as u64) << 16 | (i1 as u64)
}