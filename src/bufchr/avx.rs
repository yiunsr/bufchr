use core::{arch::x86_64::*, cmp, mem::size_of};
use crate::bufchr::fallback;

const VECTOR_SIZE: usize = size_of::<__m256i>();
const VECTOR_SIZE_M4: usize = VECTOR_SIZE * 4;
const WINDOW_SIZE: usize = VECTOR_SIZE;

pub fn get_vector_size() -> usize {
    return VECTOR_SIZE;
}

#[target_feature(enable = "avx")]
pub unsafe fn bufchr(haystack: &[u8], n1: u8, vector_end_ptr: *const u8) -> (Option<usize>, u32) {
    let haystack_len = haystack.len();
    if haystack_len < VECTOR_SIZE {
        return fallback::bufchr(haystack, n1, vector_end_ptr);
    }
    let start_ptr = haystack.as_ptr();
    let mut ptr = haystack.as_ptr();
    let vn1 = _mm256_set1_epi8(n1 as i8);

    while ptr < vector_end_ptr{
        let chunk = _mm256_loadu_si256(ptr as *const __m256i);
        let mask = _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, vn1));
        if mask != 0 {
            let umask = to_u32(mask);
            let bit_pos = forward_pos(umask);
            let cache = umask & !(1 << bit_pos);
            return (Some(sub(ptr, start_ptr) + bit_pos), cache);
        }
        ptr = ptr.add(VECTOR_SIZE);
    }

    let rest_haystack = std::slice::from_raw_parts(
        vector_end_ptr, haystack_len % VECTOR_SIZE);
    fallback::bufchr(rest_haystack, n1, vector_end_ptr)
}

#[target_feature(enable = "avx")]
pub unsafe fn bufchr2(haystack: &[u8], n1: u8, n2: u8) -> (Option<usize>, u32) {
    let haystack_len = haystack.len();
    if haystack_len < VECTOR_SIZE {
        return fallback::bufchr2(haystack, n1, n2);
    }
    let start_ptr = haystack.as_ptr();
    let mut ptr = start_ptr;
    let vector_end_ptr = start_ptr.add((haystack_len / VECTOR_SIZE) * VECTOR_SIZE);
    let vn1 = _mm256_set1_epi8(n1 as i8);
    let vn2 = _mm256_set1_epi8(n2 as i8);

    while ptr < vector_end_ptr{
        let chunk = _mm256_loadu_si256(ptr as *const __m256i);
        let eq1 = _mm256_cmpeq_epi8(chunk, vn1);
        let eq2 = _mm256_cmpeq_epi8(chunk, vn2);
        let mask1 = _mm256_movemask_epi8(eq1);
        let mask2 = _mm256_movemask_epi8(eq2);
        let mask = mask1 | mask2;
        if mask != 0 {
            let umask = to_u32(mask);
            let bit_pos = forward_pos(umask);
            let cache = umask & !(1 << bit_pos);
            return (Some(sub(ptr, start_ptr) + bit_pos), cache);
        }
        ptr = ptr.add(VECTOR_SIZE);
    }

    let rest_haystack = std::slice::from_raw_parts(
        vector_end_ptr, haystack_len % VECTOR_SIZE);
    fallback::bufchr2(rest_haystack, n1, n2)
}

#[target_feature(enable = "avx")]
pub unsafe fn bufchr3(haystack: &[u8], n1: u8, n2: u8, n3: u8) -> (Option<usize>, u32) {
    let haystack_len = haystack.len();
    if haystack_len < VECTOR_SIZE {
        return fallback::bufchr3(haystack, n1, n2, n3);
    }
    let start_ptr = haystack.as_ptr();
    let mut ptr = start_ptr;
    let vector_end_ptr = start_ptr.add((haystack_len / VECTOR_SIZE) * VECTOR_SIZE);
    let vn1 = _mm256_set1_epi8(n1 as i8);
    let vn2 = _mm256_set1_epi8(n2 as i8);
    let vn3 = _mm256_set1_epi8(n3 as i8);

    while ptr < vector_end_ptr{
        let chunk = _mm256_loadu_si256(ptr as *const __m256i);
        let eq1 = _mm256_cmpeq_epi8(chunk, vn1);
        let eq2 = _mm256_cmpeq_epi8(chunk, vn2);
        let eq3 = _mm256_cmpeq_epi8(chunk, vn3);
        let mask1 = _mm256_movemask_epi8(eq1);
        let mask2 = _mm256_movemask_epi8(eq2);
        let mask3 = _mm256_movemask_epi8(eq3);
        let mask = mask1 | mask2 | mask3;
        if mask != 0 {
            let umask = to_u32(mask);
            let bit_pos = forward_pos(umask);
            let cache = umask & !(1 << bit_pos);
            return (Some(sub(ptr, start_ptr) + bit_pos), cache);
        }
        ptr = ptr.add(VECTOR_SIZE);
    }

    let rest_haystack = std::slice::from_raw_parts(
        vector_end_ptr, haystack_len % VECTOR_SIZE);
    fallback::bufchr2(rest_haystack, n1, n2)
}

#[inline]
fn forward_pos(mask: u32) -> usize {
    unsafe{
        _tzcnt_u32(mask) as usize
     }
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