#![feature(target_feature)]

#[doc(hidden)]
pub use self::iter::{Bufchr, Bufchr2, Bufchr3};

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

pub fn get_vector_size() -> usize {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2"){
            return avx::get_vector_size();            
        }
        else if is_x86_feature_detected!("sse2") {
            return sse2::get_vector_size();
        }
    }
    fallback::get_vector_size()
}

pub fn get_batch_byte_size() -> usize {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2"){
            return avx::get_batch_byte_size();            
        }
        else if is_x86_feature_detected!("sse2") {
            return sse2::get_batch_byte_size();
        }
    }
    fallback::get_batch_byte_size()
}
