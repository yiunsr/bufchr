#![feature(target_feature)]

#[doc(hidden)]
pub use self::iter::{Bufchr, Bufchr2, Bufchr3, BufchrFast3};

#[doc(hidden)]
pub mod iter;
#[doc(hidden)]
pub mod avx;
#[doc(hidden)]
pub mod sse2;
#[doc(hidden)]
pub mod fallback;

#[doc(hidden)]
pub type CbBufchr = unsafe fn(haystack: &[u8], n1: u8,  *const u8) -> (Option<usize>, u64);
#[doc(hidden)]
pub type CbBufchr2 = unsafe fn(haystack: &[u8], n1: u8, n2: u8, *const u8) -> (Option<usize>, u64);
#[doc(hidden)]
pub type CbBufchr3 = unsafe fn(haystack: &[u8], n1: u8, n2: u8, n3:u8, *const u8) -> (Option<usize>, u64);
#[doc(hidden)]
pub type CbBufchrFast3 = unsafe fn(haystack: &[u8], n1: u8, n2: u8, n3:u8, *const u8) -> (Option<usize>, u64, u64);


#[doc(hidden)]
pub fn get_cb_bufchr() -> CbBufchr{
    if is_x86_feature_detected!("avx2"){
        unsafe{
            return avx::bufchr;
        }
    }
    else if is_x86_feature_detected!("sse2") {
        unsafe{
            return sse2::bufchr;
        }
    }
    fallback::bufchr
}

#[doc(hidden)]
pub fn get_cb_bufchr2() -> CbBufchr2{
    if is_x86_feature_detected!("avx2"){
        unsafe{
            return avx::bufchr2;
        }
    }
    else if is_x86_feature_detected!("sse2") {
        unsafe{
            return sse2::bufchr2;
        }
    }
    fallback::bufchr2
}

pub fn get_cb_bufchr3() -> CbBufchr3{
    if is_x86_feature_detected!("avx2"){
        unsafe{
            return avx::bufchr3;
        }
    }
    else if is_x86_feature_detected!("sse2") {
        unsafe{
            return sse2::bufchr3;
        }
    }
    fallback::bufchr3
}

pub fn get_cb_bufchrfast3() -> CbBufchrFast3{
    if is_x86_feature_detected!("avx2"){
        unsafe{
            return avx::bufchrfast3;
        }
    }
    else if is_x86_feature_detected!("sse2") {
        unsafe{
            return sse2::bufchrfast3;
        }
    }
    fallback::bufchrfast3
}
