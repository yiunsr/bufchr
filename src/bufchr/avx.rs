use core::{arch::x86_64::*, cmp, mem::size_of};
use crate::bufchr::fallback;

const VECTOR_SIZE: usize = size_of::<__m256i>();
const LOOP_COUNT: usize = 2;
const BATCH_BYTE_SIZE: usize = VECTOR_SIZE * LOOP_COUNT;

pub fn get_vector_size() -> usize {
    VECTOR_SIZE
}

#[target_feature(enable = "avx")]
pub unsafe fn bufchr(haystack: &[u8], n1: u8, vector_end_ptr: *const u8) -> (Option<usize>, u64) {
    let haystack_len = haystack.len();
    if haystack_len < BATCH_BYTE_SIZE {
        return fallback::bufchr(haystack, n1, vector_end_ptr);
    }
    let start_ptr = haystack.as_ptr();
    let mut ptr = haystack.as_ptr();
    let vn1 = _mm256_set1_epi8(n1 as i8);

    while ptr < vector_end_ptr{
        let chunk1 = _mm256_loadu_si256(ptr as *const __m256i);
        let chunk2 = _mm256_loadu_si256(ptr.add(VECTOR_SIZE) as *const __m256i);
        let mask1 = _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk1, vn1));
        let mask2 = _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk2, vn1));
        if (mask1 | mask2) != 0 {
            let umask = to_u64(mask1, mask2);
            let bit_pos = umask.trailing_zeros() as usize;
            // Reset lowest set bit	
            let cache = umask & (umask - 1);
            return (Some(sub(ptr, start_ptr) + bit_pos), cache);
        }
        ptr = ptr.add(BATCH_BYTE_SIZE);
    }

    let rest_haystack = std::slice::from_raw_parts(
        vector_end_ptr, haystack_len % BATCH_BYTE_SIZE);
        
    match fallback::bufchr_raw(rest_haystack, n1) {
        Some(pos) => {
            (Some(sub(ptr, start_ptr) + pos), 0)
        }
        None => { (None, 0)}
    }
}

#[target_feature(enable = "avx")]
pub unsafe fn bufchr2(haystack: &[u8], n1: u8, n2: u8, vector_end_ptr: *const u8) -> (Option<usize>, u64) {
    let haystack_len = haystack.len();
    if haystack_len < BATCH_BYTE_SIZE {
        return fallback::bufchr2(haystack, n1, n2, vector_end_ptr);
    }
    let start_ptr = haystack.as_ptr();
    let mut ptr = start_ptr;
    let vn1 = _mm256_set1_epi8(n1 as i8);
    let vn2 = _mm256_set1_epi8(n2 as i8);

    while ptr < vector_end_ptr{
        let chunk = _mm256_loadu_si256(ptr as *const __m256i);
        let eq1 = _mm256_cmpeq_epi8(chunk, vn1);
        let eq2 = _mm256_cmpeq_epi8(chunk, vn2);
        let mask1 = _mm256_movemask_epi8(eq1) | _mm256_movemask_epi8(eq2);

        let chunk = _mm256_loadu_si256(ptr.add(VECTOR_SIZE) as *const __m256i);
        let eq1 = _mm256_cmpeq_epi8(chunk, vn1);
        let eq2 = _mm256_cmpeq_epi8(chunk, vn2);
        let mask2 = _mm256_movemask_epi8(eq1) | _mm256_movemask_epi8(eq2);

        if (mask1 | mask2) != 0 {
            let umask = to_u64(mask1, mask2);
            let bit_pos = umask.trailing_zeros() as usize;
            // Reset lowest set bit	
            let cache = umask & (umask - 1);
            return (Some(sub(ptr, start_ptr) + bit_pos), cache);
        }
        ptr = ptr.add(BATCH_BYTE_SIZE);
    }

    let rest_haystack = std::slice::from_raw_parts(
        vector_end_ptr, haystack_len % BATCH_BYTE_SIZE);
        
    match fallback::bufchr2_raw(rest_haystack, n1, n2) {
        Some(pos) => {
            (Some(sub(ptr, start_ptr) + pos), 0)
        }
        None => { (None, 0)}
    }
}

#[target_feature(enable = "avx")]
pub unsafe fn bufchr3(haystack: &[u8], n1: u8, n2: u8, n3: u8, vector_end_ptr: *const u8) -> (Option<usize>, u64) {
    let haystack_len = haystack.len();
    if haystack_len < BATCH_BYTE_SIZE {
        return fallback::bufchr3(haystack, n1, n2, n3, vector_end_ptr);
    }
    let start_ptr = haystack.as_ptr();
    let mut ptr = start_ptr;
    let vn1 = _mm256_set1_epi8(n1 as i8);
    let vn2 = _mm256_set1_epi8(n2 as i8);
    let vn3 = _mm256_set1_epi8(n3 as i8);

    while ptr < vector_end_ptr{
        let chunk = _mm256_loadu_si256(ptr as *const __m256i);
        let eq1 = _mm256_cmpeq_epi8(chunk, vn1);
        let eq2 = _mm256_cmpeq_epi8(chunk, vn2);
        let eq3 = _mm256_cmpeq_epi8(chunk, vn3);
        let mask1 = _mm256_movemask_epi8(eq1) | _mm256_movemask_epi8(eq2) | _mm256_movemask_epi8(eq3);

        let chunk = _mm256_loadu_si256(ptr.add(VECTOR_SIZE) as *const __m256i);
        let eq1 = _mm256_cmpeq_epi8(chunk, vn1);
        let eq2 = _mm256_cmpeq_epi8(chunk, vn2);
        let eq3 = _mm256_cmpeq_epi8(chunk, vn3);
        let mask2 = _mm256_movemask_epi8(eq1) | _mm256_movemask_epi8(eq2) | _mm256_movemask_epi8(eq3);

        if (mask1 | mask2) != 0 {
            let umask = to_u64(mask1, mask2);
            let bit_pos = umask.trailing_zeros() as usize;
            // Reset lowest set bit	
            let cache = umask & (umask - 1);   
            return (Some(sub(ptr, start_ptr) + bit_pos), cache);
        }
        ptr = ptr.add(BATCH_BYTE_SIZE);
    }

    let rest_haystack = std::slice::from_raw_parts(
        vector_end_ptr, haystack_len % BATCH_BYTE_SIZE);
        
    match fallback::bufchr3_raw(rest_haystack, n1, n2, n3) {
        Some(pos) => {
            (Some(sub(ptr, start_ptr) + pos), 0)
        }
        None => { (None, 0)}
    }
}

#[inline]
fn sub(a: *const u8, b: *const u8) -> usize {
    debug_assert!(a >= b);
    (a as usize) - (b as usize)
}

#[inline]
fn to_u64(i1: i32, i2: i32) -> u64 {
    let i1_byte = i1.to_be_bytes(); 
    let u1 = u32::from_be_bytes(i1_byte) as u64;
    let i2_byte = i2.to_be_bytes(); 
    let u2 = u32::from_be_bytes(i2_byte) as u64;
    u2<< 32 | u1
}