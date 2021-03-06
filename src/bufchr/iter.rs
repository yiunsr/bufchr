use crate::bufchr;
use crate::bufchr::CbBufchr;
use crate::bufchr::CbBufchr2;
use crate::bufchr::CbBufchr3;
use crate::bufchr::CbBufchrCSV;

const VECTOR_SIZE: usize = 32;
const BATCH_BYTE_SIZE: usize = 64;
const BATCH_BYTE_SIZE2: usize = 128;

/// struct used when there is only one needle
pub struct Bufchr<'a> {
    haystack: &'a [u8],
    needle0: u8,
    position: usize,
    align_pos: usize,
    cache: u64,
    vector_end_ptr: *const u8,
    cb_bufchr: CbBufchr,
}
impl<'a> Bufchr<'a> {
    /// Needle is what you are trying to find and the location you are looking for is haystack.
    #[inline]
    pub fn new(haystack: &[u8], needle0: u8) -> Bufchr<'_> {
        let cb_bufchr = bufchr::get_cb_bufchr();
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / BATCH_BYTE_SIZE) * BATCH_BYTE_SIZE)
            };

        Bufchr {haystack: haystack, needle0: needle0,
            position: 0, cache: 0, cb_bufchr: cb_bufchr,
            align_pos: 0, vector_end_ptr: vector_end_ptr,
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn new_avx(haystack: &[u8], needle0: u8) -> Bufchr<'_> {
        let cb_bufchr = bufchr::avx::bufchr;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / BATCH_BYTE_SIZE) * BATCH_BYTE_SIZE)
            };
            Bufchr {haystack: haystack, needle0: needle0,
                position: 0, cache: 0, cb_bufchr: cb_bufchr,
                align_pos: 0 ,vector_end_ptr: vector_end_ptr,
            }
    }

    #[doc(hidden)]
    #[inline]
    pub fn new_sse2(haystack: &[u8], needle0: u8) -> Bufchr<'_> {
        let cb_bufchr = bufchr::sse2::bufchr;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / BATCH_BYTE_SIZE) * BATCH_BYTE_SIZE)
            };
        Bufchr {haystack: haystack, needle0: needle0,
            position: 0, cache: 0, cb_bufchr: cb_bufchr,
            align_pos: 0 , vector_end_ptr: vector_end_ptr,
        }
    }
}
impl<'a> Iterator for Bufchr<'a> {
    type Item = usize;

    /// The needle position is returned. If there is no needle, None is returned.
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.cache != 0 {
            let bit_pos = self.cache.trailing_zeros() as usize;
            // Reset lowest set bit	
            self.cache = self.cache & (self.cache - 1);
            let position = self.align_pos + bit_pos;
            self.position = position + 1;
            return Some(position);
        }
        let align_pos;
        if self.position == 0 {
            align_pos = 0;
        }
        else if self.haystack.len() - self.position < BATCH_BYTE_SIZE{
            align_pos = self.position;
        }
        else{
            align_pos = ( (self.position - 1) / BATCH_BYTE_SIZE + 1) * BATCH_BYTE_SIZE;
        }
        let haystack_len = self.haystack.len() - align_pos;
        let new_haystack;
        unsafe{
            let haystack = self.haystack.as_ptr().add(align_pos);
            new_haystack = std::slice::from_raw_parts(haystack, haystack_len);
        }
        let position;
        unsafe{
            let (position_, cache_) = (self.cb_bufchr)(new_haystack, self.needle0, self.vector_end_ptr);
            position = position_;
            self.cache = cache_;
        }
        if let Some(pos) = position {
            let position = align_pos + pos;
            self.position = position + 1;
            if self.cache != 0 {
                self.align_pos = get_align_pos(position);
            }
            return Some(position);
        }
        None
    }

}

/// struct used when there are two needles
pub struct Bufchr2<'a> {
    haystack: &'a [u8],
    needle0: u8,
    needle1: u8,
    position: usize,
    align_pos: usize,
    cache: u64,
    vector_end_ptr: *const u8,
    cb_bufchr2: CbBufchr2,
}
impl<'a> Bufchr2<'a> {
    /// needle0, needle1 are what you are trying to find and the location you are looking for is haystack.
    #[inline]
    pub fn new(haystack: &[u8], needle0: u8, needle1: u8) -> Bufchr2<'_> {
        let cb_bufchr2 = bufchr::get_cb_bufchr2();
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / BATCH_BYTE_SIZE) * BATCH_BYTE_SIZE)
            };
        Bufchr2 {haystack: haystack, needle0: needle0, needle1: needle1,
            position: 0, cache: 0, cb_bufchr2: cb_bufchr2,
            align_pos: 0 , vector_end_ptr: vector_end_ptr,
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn new_avx(haystack: &[u8], needle0: u8, needle1: u8) -> Bufchr2<'_> {
        let cb_bufchr2 = bufchr::avx::bufchr2;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / BATCH_BYTE_SIZE) * BATCH_BYTE_SIZE)
            };
        Bufchr2 {haystack: haystack, needle0: needle0, needle1: needle1,
            position: 0, cache: 0, cb_bufchr2: cb_bufchr2,
            align_pos: 0, vector_end_ptr: vector_end_ptr,
        }
    }

    #[inline]
    pub fn new_sse2(haystack: &[u8], needle0: u8, needle1: u8) -> Bufchr2<'_> {
        let cb_bufchr2 = bufchr::sse2::bufchr2;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / BATCH_BYTE_SIZE) * BATCH_BYTE_SIZE)
            };
        Bufchr2 {haystack: haystack, needle0: needle0, needle1: needle1,
            position: 0, cache: 0, cb_bufchr2: cb_bufchr2,
            align_pos: 0 , vector_end_ptr: vector_end_ptr,
        }
    }
}
impl<'a> Iterator for Bufchr2<'a> {
    type Item = usize;

    /// The needle position is returned. If there is no needle, None is returned.
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.cache != 0 {
            let bit_pos = self.cache.trailing_zeros() as usize;
            // Reset lowest set bit	
            self.cache = self.cache & (self.cache - 1);
            let position = self.align_pos + bit_pos;
            self.position = position + 1;
            return Some(position);
        }
        let align_pos;
        if self.position == 0 {
            align_pos = 0;
        }
        else if self.haystack.len() - self.position < BATCH_BYTE_SIZE{
            align_pos = self.position;
        }
        else{
            align_pos = ( (self.position - 1) / BATCH_BYTE_SIZE + 1) * BATCH_BYTE_SIZE;
        }
        let haystack_len = self.haystack.len() - align_pos;
        let new_haystack;
        unsafe{
            let haystack = self.haystack.as_ptr().add(align_pos);
            new_haystack = std::slice::from_raw_parts(haystack, haystack_len);
        }
        let position;
        unsafe{
            let (position_, cache_) = (self.cb_bufchr2)
                (new_haystack, self.needle0, self.needle1, self.vector_end_ptr);
            position = position_;
            self.cache = cache_;
        }
        if let Some(pos) = position {
            let position = align_pos + pos;
            self.position = position + 1;
            if self.cache != 0 {
                self.align_pos = get_align_pos(position);
            }
            return Some(position);
        }
        None
    }

}

//// struct used when there are three needles
pub struct Bufchr3<'a> {
    haystack: &'a [u8],
    needle0: u8,
    needle1: u8,
    needle2: u8,
    position: usize,
    align_pos: usize,
    cache: u64,
    vector_end_ptr: *const u8,
    cb_bufchr3: CbBufchr3,
}
impl<'a> Bufchr3<'a> {
    /// needle0, needle1, needle2 are what you are trying to find and the location you are looking for is haystack.
    #[inline]
    pub fn new(haystack: &[u8], needle0: u8, needle1: u8, needle2: u8) -> Bufchr3<'_> {
        let cb_bufchr3 = bufchr::get_cb_bufchr3();
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / BATCH_BYTE_SIZE) * BATCH_BYTE_SIZE)
            };
        Bufchr3 {haystack: haystack, needle0: needle0, needle1: needle1, needle2: needle2,
            position: 0, cache: 0, cb_bufchr3: cb_bufchr3,
            align_pos: 0, vector_end_ptr: vector_end_ptr,
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn new_avx(haystack: &[u8], needle0: u8, needle1: u8, needle2: u8) -> Bufchr3<'_> {
        let cb_bufchr3 = bufchr::avx::bufchr3;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / BATCH_BYTE_SIZE) * BATCH_BYTE_SIZE)
            };
        Bufchr3 {haystack: haystack, needle0: needle0, needle1: needle1, needle2: needle2,
            position: 0, cache: 0, cb_bufchr3: cb_bufchr3,
            align_pos: 0, vector_end_ptr: vector_end_ptr,
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn new_sse2(haystack: &[u8], needle0: u8, needle1: u8, needle2: u8) -> Bufchr3<'_> {
        let cb_bufchr3 = bufchr::sse2::bufchr3;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / BATCH_BYTE_SIZE) * BATCH_BYTE_SIZE)
            };
        Bufchr3 {haystack: haystack, needle0: needle0, needle1: needle1, needle2: needle2,
            position: 0, cache: 0, cb_bufchr3: cb_bufchr3,
            align_pos: 0, vector_end_ptr: vector_end_ptr,
        }
    }
}
impl<'a> Iterator for Bufchr3<'a> {
    type Item = usize;

    /// The needle position is returned. If there is no needle, None is returned.
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.cache != 0 {
            let bit_pos = self.cache.trailing_zeros() as usize;
            // Reset lowest set bit	
            self.cache = self.cache & (self.cache - 1);
            let position = self.align_pos + bit_pos;
            self.position = position + 1;
            return Some(position);
        }
        let align_pos;
        if self.position == 0 {
            align_pos = 0;
        }
        else if self.haystack.len() - self.position < BATCH_BYTE_SIZE{
            align_pos = self.position;
        }
        else{
            align_pos = ( (self.position - 1) / BATCH_BYTE_SIZE + 1) * BATCH_BYTE_SIZE;
        }
        let haystack_len = self.haystack.len() - align_pos;
        let new_haystack;
        unsafe{
            let haystack = self.haystack.as_ptr().add(align_pos);
            new_haystack = std::slice::from_raw_parts(haystack, haystack_len);
        }
        let position;
        unsafe{
            let (position_, cache_) = (self.cb_bufchr3)(
                new_haystack, self.needle0, self.needle1, self.needle2, self.vector_end_ptr);
            position = position_;
            self.cache = cache_;
        }
        if let Some(pos) = position {
            let position = align_pos + pos;
            self.position = position + 1;
            if self.cache != 0 {
                self.align_pos = get_align_pos(position);
            }
            return Some(position);
        }
        None
    }

}

pub struct BufchrCSV<'a> {
    haystack: &'a [u8],
    needle0: u8,
    position: usize,
    align_pos: usize,
    cache1: u64,
    cache2: u64,
    vector_end_ptr: *const u8,
    cb_bufchr_csv: CbBufchrCSV,
}
impl<'a> BufchrCSV<'a> {
    /// needle0, needle1, needle2 are what you are trying to find and the location you are looking for is haystack.
    #[inline]
    pub fn new(haystack: &[u8], needle0: u8) -> BufchrCSV<'_> {
        let cb_bufchr_csv = bufchr::get_cb_BufchrCSV();
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / BATCH_BYTE_SIZE2) * BATCH_BYTE_SIZE2)
            };
        BufchrCSV {haystack: haystack, needle0: needle0,
            position: 0, cache1: 0, cache2: 0, cb_bufchr_csv: cb_bufchr_csv,
            align_pos: 0, vector_end_ptr: vector_end_ptr,
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn new_avx(haystack: &[u8], needle0: u8) -> BufchrCSV<'_> {
        let cb_bufchr_csv = bufchr::avx::bufchr_csv;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / BATCH_BYTE_SIZE) * BATCH_BYTE_SIZE)
            };
        BufchrCSV {haystack: haystack, needle0: needle0,
            position: 0, cache1: 0, cache2: 0, cb_bufchr_csv: cb_bufchr_csv,
            align_pos: 0, vector_end_ptr: vector_end_ptr,
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn new_sse2(haystack: &[u8], needle0: u8) -> BufchrCSV<'_> {
        let cb_bufchr_csv = bufchr::sse2::bufchr_csv;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / BATCH_BYTE_SIZE) * BATCH_BYTE_SIZE)
            };
        BufchrCSV {haystack: haystack, needle0: needle0,
            position: 0, cache1: 0, cache2: 0, cb_bufchr_csv: cb_bufchr_csv,
            align_pos: 0, vector_end_ptr: vector_end_ptr,
        }
    }
}
impl<'a> Iterator for BufchrCSV<'a> {
    type Item = usize;

    /// The needle position is returned. If there is no needle, None is returned.
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.cache1 != 0 {
            let bit_pos = self.cache1.trailing_zeros() as usize;
            // Reset lowest set bit	
            self.cache1 = self.cache1 & (self.cache1 - 1);
            let position = self.align_pos + bit_pos;
            self.position = position + 1;
            return Some(position);
        }
        else if self.cache2 != 0 {
            let bit_pos = self.cache2.trailing_zeros() as usize;
            // Reset lowest set bit	
            self.cache2 = self.cache2 & (self.cache2 - 1);
            let position = self.align_pos + bit_pos + BATCH_BYTE_SIZE;
            self.position = position + 1;
            return Some(position);
        }
        let align_pos;
        if self.position == 0 {
            align_pos = 0;
        }
        else if self.haystack.len() - self.position < BATCH_BYTE_SIZE2{
            align_pos = self.position;
        }
        else{
            align_pos = ( (self.position - 1) / BATCH_BYTE_SIZE2 + 1) * BATCH_BYTE_SIZE2;
        }
        let haystack_len = self.haystack.len() - align_pos;
        let new_haystack;
        unsafe{
            let haystack = self.haystack.as_ptr().add(align_pos);
            new_haystack = std::slice::from_raw_parts(haystack, haystack_len);
        }
        let position;
        unsafe{
            let (position_, cache1, cache2) = (self.cb_bufchr_csv)(
                new_haystack, self.needle0, self.vector_end_ptr);
            position = position_;
            self.cache1 = cache1;
            self.cache2 = cache2;
        }
        if let Some(pos) = position {
            let position = align_pos + pos;
            self.position = position + 1;
            if self.cache1 != 0 {
                self.align_pos = get_align_pos_fast(position);
            }
            else if self.cache2 != 0 {
                self.align_pos = get_align_pos_fast(position);
            }
            return Some(position);
        }
        None
    }

}

#[inline(always)]
fn get_align_pos(position: usize) -> usize {
    (position / BATCH_BYTE_SIZE) * BATCH_BYTE_SIZE
}
#[inline(always)]
fn get_align_pos_fast(position: usize) -> usize {
    (position / BATCH_BYTE_SIZE2) * BATCH_BYTE_SIZE2
}

