#![feature(target_feature)]

use crate::bufchr::iter::Bufchr;

pub mod iter;
pub mod avx;
pub mod sse2;
pub mod fallback;

#[inline]
pub fn bufchr(haystack: &[u8], needle: u8) 
        -> (Option<usize>, u32) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if _is_x86_feature_avx2() {
            unsafe{
                return avx::bufchr(haystack, needle);
            }
        }
        else if _is_x86_feature_sse2() {
            unsafe{
                return sse2::bufchr(haystack, needle);
            }
        }
    }
    fallback::bufchr(haystack, needle)
}

#[inline]
pub fn bufchr2(haystack: &[u8], needle0: u8, needle1: u8) 
        -> (Option<usize>, u32) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if _is_x86_feature_avx2() {
            unsafe{
                return avx::bufchr2(haystack, needle0, needle1);
            }
        }
        else if _is_x86_feature_sse2() {
            unsafe{
                return sse2::bufchr2(haystack, needle0, needle1);
            }
        }
    }
    fallback::bufchr(haystack, needle0)
}


#[inline]
pub fn bufchr3(haystack: &[u8], needle0: u8, needle1: u8, needle2: u8) 
        -> (Option<usize>, u32) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if _is_x86_feature_avx2() {
            unsafe{
                return avx::bufchr3(haystack, needle0, needle1, needle2);
            }
        }
        else if _is_x86_feature_sse2() {
            unsafe{
                return sse2::bufchr3(haystack, needle0, needle1, needle2);
            }
        }
    }
    fallback::bufchr(haystack, needle0)
}

pub fn get_vector_size() -> usize {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if _is_x86_feature_avx2() {
            return avx::get_vector_size();            
        }
        else if _is_x86_feature_sse2() {
            return sse2::get_vector_size();
        }
    }
    fallback::get_vector_size()
}

fn _is_x86_feature_avx2() -> bool {
    is_x86_feature_detected!("avx2")
}

fn _is_x86_feature_sse2() -> bool {
    is_x86_feature_detected!("sse2")
}