use core::{arch::x86_64::*, cmp, mem::size_of};
use crate::bufchr::fallback;

const VECTOR_SIZE: usize = size_of::<__m128i>();

pub fn get_vector_size() -> usize {
    return VECTOR_SIZE;
}

#[target_feature(enable = "sse2")]
pub unsafe fn bufchr(haystack: &[u8], n1: u8) -> (Option<usize>, u32) {
    let haystack_len = haystack.len();
    if haystack_len < VECTOR_SIZE {
        return fallback::bufchr(haystack, n1);
    }
    let start_ptr = haystack.as_ptr();
    let ptr = start_ptr;
    let end_ptr = start_ptr.add(haystack_len);
    let align_end_ptr = start_ptr.add((haystack_len / VECTOR_SIZE) * VECTOR_SIZE);
    let vn1 = _mm_set1_epi8(n1 as i8);

    while ptr < end_ptr{
        // https://stackoverflow.com/a/15964428/6652082
        // if memory alignment work, use _mm_load_si128
        let chunk = _mm_loadu_si128(ptr as *const __m128i);
        let mask = _mm_movemask_epi8(_mm_cmpeq_epi8(vn1, chunk));
        if mask != 0 {
            let umask = to_u32(mask);
            let bit_pos = forward_pos(umask);
            let cache = umask & !(1 << bit_pos);
            return (Some(sub(ptr, start_ptr) + bit_pos), cache);
        }
        ptr.add(VECTOR_SIZE);
    }

    let rest_haystack = std::slice::from_raw_parts(
        align_end_ptr, haystack_len % VECTOR_SIZE);
    fallback::bufchr(rest_haystack, n1)
}

#[target_feature(enable = "sse2")]
pub unsafe fn bufchr2(haystack: &[u8], n1: u8, n2: u8) -> (Option<usize>, u32) {
    let haystack_len = haystack.len();
    if haystack_len < VECTOR_SIZE {
        return fallback::bufchr(haystack, n1);
    }
    let start_ptr = haystack.as_ptr();
    let ptr = start_ptr;
    let end_ptr = start_ptr.add(haystack_len);
    let align_end_ptr = start_ptr.add((haystack_len / VECTOR_SIZE) * VECTOR_SIZE);
    let vn1 = _mm_set1_epi8(n1 as i8);
    let vn2 = _mm_set1_epi8(n2 as i8);

    while ptr < end_ptr{
        // https://stackoverflow.com/a/15964428/6652082
        // if memory alignment work, use _mm_load_si128
        let chunk = _mm_loadu_si128(ptr as *const __m128i);
        let mask1 = _mm_movemask_epi8(_mm_cmpeq_epi8(vn1, chunk));
        let mask2 = _mm_movemask_epi8(_mm_cmpeq_epi8(vn2, chunk));
        let mask = mask1 | mask2;
        if mask != 0 {
            let umask = to_u32(mask);
            let bit_pos = forward_pos(umask);
            let cache = umask & !(1 << bit_pos);
            return (Some(sub(ptr, start_ptr) + bit_pos), cache);
        }
        ptr.add(VECTOR_SIZE);
    }

    let rest_haystack = std::slice::from_raw_parts(
        align_end_ptr, haystack_len % VECTOR_SIZE);
    fallback::bufchr(rest_haystack, n1)
}

#[target_feature(enable = "sse2")]
pub unsafe fn bufchr3(haystack: &[u8], n1: u8, n2: u8, n3: u8) -> (Option<usize>, u32) {
    let haystack_len = haystack.len();
    if haystack_len < VECTOR_SIZE {
        return fallback::bufchr(haystack, n1);
    }
    let start_ptr = haystack.as_ptr();
    let ptr = start_ptr;
    let end_ptr = start_ptr.add(haystack_len);
    let align_end_ptr = start_ptr.add((haystack_len / VECTOR_SIZE) * VECTOR_SIZE);
    let vn1 = _mm_set1_epi8(n1 as i8);
    let vn2 = _mm_set1_epi8(n2 as i8);
    let vn3 = _mm_set1_epi8(n3 as i8);

    while ptr < end_ptr{
        // https://stackoverflow.com/a/15964428/6652082
        // if memory alignment work, use _mm_load_si128
        let chunk = _mm_loadu_si128(ptr as *const __m128i);
        let mask1 = _mm_movemask_epi8(_mm_cmpeq_epi8(vn1, chunk));
        let mask2 = _mm_movemask_epi8(_mm_cmpeq_epi8(vn2, chunk));
        let mask3 = _mm_movemask_epi8(_mm_cmpeq_epi8(vn3, chunk));
        let mask = mask1 | mask2 | mask3;
        if mask != 0 {
            let umask = to_u32(mask);
            let bit_pos = forward_pos(umask);
            let cache = umask & !(1 << bit_pos);
            return (Some(sub(ptr, start_ptr) + bit_pos), cache);
        }
        ptr.add(VECTOR_SIZE);
    }

    let rest_haystack = std::slice::from_raw_parts(
        align_end_ptr, haystack_len % VECTOR_SIZE);
    fallback::bufchr(rest_haystack, n1)
}

fn forward_pos(mask: u32) -> usize {
    mask.trailing_zeros() as usize
}

fn sub(a: *const u8, b: *const u8) -> usize {
    debug_assert!(a >= b);
    (a as usize) - (b as usize)
}

fn to_u32(i: i32) -> u32 {
    let x_bytes = i.to_be_bytes();   
    u32::from_be_bytes(x_bytes)
}