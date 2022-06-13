use core::{arch::x86_64::*, cmp, mem::size_of};
use crate::bufchr::fallback;

const VECTOR_SIZE: usize = size_of::<__m256i>();
const LOOP_COUNT: usize = 2;
const BATCH_BYTE_SIZE: usize = VECTOR_SIZE * LOOP_COUNT;
const BATCH_BYTE_SIZE2: usize = VECTOR_SIZE * LOOP_COUNT * 2;

const LINE_FEED_M256I:__m256i = unsafe {
    std::mem::transmute::<[u32;8], __m256i>(
        [0x0A0A0A0A, 0x0A0A0A0A, 0x0A0A0A0A, 0x0A0A0A0A, 0x0A0A0A0A, 0x0A0A0A0A, 0x0A0A0A0A, 0x0A0A0A0A])
};

const DOUBLE_QUOTATION_M256I:__m256i = unsafe {
    std::mem::transmute::<[u32;8], __m256i>(
        [0x22222222, 0x22222222, 0x22222222, 0x22222222, 0x22222222, 0x22222222, 0x22222222, 0x22222222])
};

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


#[target_feature(enable = "avx")]
pub unsafe fn bufchr_csv(haystack: &[u8], n1: u8, vector_end_ptr: *const u8) -> (Option<usize>, u64, u64) {
    let haystack_len = haystack.len();
    if haystack_len < BATCH_BYTE_SIZE2 {
        return fallback::bufchr_csv(haystack, n1, vector_end_ptr);
    }
    let start_ptr = haystack.as_ptr();
    let mut ptr = start_ptr;
    let vn1 = _mm256_set1_epi8(n1 as i8);
    let &vn2 = &LINE_FEED_M256I;
    let &vn3 = &DOUBLE_QUOTATION_M256I;

    while ptr < vector_end_ptr{
        let chunk = _mm256_load_si256(ptr as *const __m256i);
        let eq1 = _mm256_cmpeq_epi8(chunk, vn1);
        let eq2 = _mm256_cmpeq_epi8(chunk, vn2);
        let eq3 = _mm256_cmpeq_epi8(chunk, vn3);
        let mask1 = _mm256_movemask_epi8(eq1) | _mm256_movemask_epi8(eq2) | _mm256_movemask_epi8(eq3);

        let chunk = _mm256_load_si256(ptr.add(VECTOR_SIZE) as *const __m256i);
        let eq1 = _mm256_cmpeq_epi8(chunk, vn1);
        let eq2 = _mm256_cmpeq_epi8(chunk, vn2);
        let eq3 = _mm256_cmpeq_epi8(chunk, vn3);
        let mask2 = _mm256_movemask_epi8(eq1) | _mm256_movemask_epi8(eq2) | _mm256_movemask_epi8(eq3);

        let chunk = _mm256_load_si256(ptr.add(VECTOR_SIZE * 2) as *const __m256i);
        let eq1 = _mm256_cmpeq_epi8(chunk, vn1);
        let eq2 = _mm256_cmpeq_epi8(chunk, vn2);
        let eq3 = _mm256_cmpeq_epi8(chunk, vn3);
        let mask3 = _mm256_movemask_epi8(eq1) | _mm256_movemask_epi8(eq2) | _mm256_movemask_epi8(eq3);

        let chunk = _mm256_load_si256(ptr.add(VECTOR_SIZE * 3) as *const __m256i);
        let eq1 = _mm256_cmpeq_epi8(chunk, vn1);
        let eq2 = _mm256_cmpeq_epi8(chunk, vn2);
        let eq3 = _mm256_cmpeq_epi8(chunk, vn3);
        let mask4 = _mm256_movemask_epi8(eq1) | _mm256_movemask_epi8(eq2) | _mm256_movemask_epi8(eq3);

        if (mask1 | mask2 ) != 0 {
            let umask1 = to_u64(mask1, mask2);
            let umask2 = to_u64(mask3, mask4);
            let bit_pos = umask1.trailing_zeros() as usize;
            // Reset lowest set bit	
            let cache = umask1 & (umask1 - 1);   
            return (Some(sub(ptr, start_ptr) + bit_pos), cache, umask2);
        }
        else if(mask3 | mask4 ) != 0 {
            let umask2 = to_u64(mask3, mask4);
            let bit_pos = umask2.trailing_zeros() as usize;
            // Reset lowest set bit	
            let cache = umask2 & (umask2 - 1);   
            return (Some(sub(ptr, start_ptr) + bit_pos + BATCH_BYTE_SIZE), 0, cache);
        }
        ptr = ptr.add(BATCH_BYTE_SIZE2);
    }

    let rest_haystack = std::slice::from_raw_parts(
        vector_end_ptr, haystack_len % BATCH_BYTE_SIZE2);
        
    match fallback::bufchr_csv_raw(rest_haystack, n1) {
        Some(pos) => {
            (Some(sub(ptr, start_ptr) + pos), 0, 0)
        }
        None => { (None, 0, 0)}
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
    u2 << 32 | u1
}

