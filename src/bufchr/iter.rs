use crate::bufchr;
use crate::bufchr::CbBufchr;
use crate::bufchr::CbBufchr2;
use crate::bufchr::CbBufchr3;


/// struct used when there is only one needle
pub struct Bufchr<'a> {
    haystack: &'a [u8],
    needle0: u8,
    position: usize,
    cache: u64,
    vector_size: usize,
    vector_end_ptr: *const u8,
    cb_bufchr: CbBufchr,
}
impl<'a> Bufchr<'a> {
    /// Needle is what you are trying to find and the location you are looking for is haystack.
    #[inline]
    pub fn new(haystack: &[u8], needle0: u8) -> Bufchr<'_> {
        let vector_size = bufchr::get_batch_byte_size();
        let cb_bufchr = bufchr::get_cb_bufchr();
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / vector_size) * vector_size)
            };

        Bufchr {haystack: haystack, needle0: needle0,
            position: 0, cache: 0,
            vector_size: vector_size, vector_end_ptr: vector_end_ptr, cb_bufchr: cb_bufchr}
    }

    #[doc(hidden)]
    #[inline]
    pub fn new_avx(haystack: &[u8], needle0: u8) -> Bufchr<'_> {
        let vector_size = bufchr::avx::get_batch_byte_size();
        let cb_bufchr = bufchr::avx::bufchr;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / vector_size) * vector_size)
            };
        Bufchr {haystack: haystack, needle0: needle0,
            position: 0, cache: 0,
            vector_size: vector_size, vector_end_ptr: vector_end_ptr, cb_bufchr: cb_bufchr}
    }

    #[doc(hidden)]
    #[inline]
    pub fn new_sse2(haystack: &[u8], needle0: u8) -> Bufchr<'_> {
        let vector_size = bufchr::sse2::get_batch_byte_size();
        let cb_bufchr = bufchr::sse2::bufchr;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / vector_size) * vector_size)
            };
        Bufchr {haystack: haystack, needle0: needle0,
            position: 0, cache: 0,
            vector_size: vector_size, vector_end_ptr: vector_end_ptr, cb_bufchr: cb_bufchr}
    }
}
impl<'a> Iterator for Bufchr<'a> {
    type Item = usize;
    

    /// The needle position is returned. If there is no needle, None is returned.
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.cache != 0 {
            let start_align_pos = (self.position / self.vector_size) * self.vector_size;
            let bit_pos = self.cache.trailing_zeros() as usize;
            self.cache = self.cache & !(1 << bit_pos);
            self.position = start_align_pos + bit_pos + 1;
            return Some(self.position);
        }
        let align_pos;
        if self.position == 0 {
            align_pos = 0;
        }
        else if self.haystack.len() - self.position < self.vector_size{
            align_pos = self.position;
        }
        else{
            align_pos = ( (self.position - 1) / self.vector_size + 1) * self.vector_size;
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
        match position {
            Some(pos) => {
                self.position = align_pos + pos + 1;
            }
            None =>{
                return None;
            }
        }
        Some(self.position)
    }

}

/// struct used when there are two needles
pub struct Bufchr2<'a> {
    haystack: &'a [u8],
    needle0: u8,
    needle1: u8,
    position: usize,
    cache: u64,
    vector_size: usize,
    vector_end_ptr: *const u8,
    cb_bufchr2: CbBufchr2,
}
impl<'a> Bufchr2<'a> {
    /// needle0, needle1 are what you are trying to find and the location you are looking for is haystack.
    #[inline]
    pub fn new(haystack: &[u8], needle0: u8, needle1: u8) -> Bufchr2<'_> {
        let vector_size = bufchr::get_batch_byte_size();
        let cb_bufchr2 = bufchr::get_cb_bufchr2();
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / vector_size) * vector_size)
            };
        Bufchr2 { haystack: haystack, needle0: needle0, needle1: needle1,
            position: 0, cache: 0,
            vector_size: vector_size, vector_end_ptr: vector_end_ptr, cb_bufchr2: cb_bufchr2 }
    }

    #[doc(hidden)]
    #[inline]
    pub fn new_avx(haystack: &[u8], needle0: u8, needle1: u8) -> Bufchr2<'_> {
        let vector_size = bufchr::avx::get_batch_byte_size();
        let cb_bufchr2 = bufchr::avx::bufchr2;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / vector_size) * vector_size)
            };
        Bufchr2 { haystack: haystack, needle0: needle0, needle1: needle1,
            position: 0, cache: 0,
            vector_size: vector_size, vector_end_ptr: vector_end_ptr, cb_bufchr2: cb_bufchr2 }
    }

    #[inline]
    pub fn new_sse2(haystack: &[u8], needle0: u8, needle1: u8) -> Bufchr2<'_> {
        let vector_size = bufchr::sse2::get_batch_byte_size();
        let cb_bufchr2 = bufchr::sse2::bufchr2;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / vector_size) * vector_size)
            };
        Bufchr2 { haystack: haystack, needle0: needle0, needle1: needle1,
            position: 0, cache: 0,
            vector_size: vector_size, vector_end_ptr: vector_end_ptr, cb_bufchr2: cb_bufchr2 }
    }
}
impl<'a> Iterator for Bufchr2<'a> {
    type Item = usize;

    /// The needle position is returned. If there is no needle, None is returned.
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.cache != 0 {
            let start_align_pos = (self.position / self.vector_size) * self.vector_size;
            let bit_pos = self.cache.trailing_zeros() as usize;
            self.cache = self.cache & !(1 << bit_pos);
            self.position = start_align_pos + bit_pos + 1;
            return Some(self.position);
        }
        let align_pos;
        if self.position == 0 {
            align_pos = 0;
        }
        else if self.haystack.len() - self.position < self.vector_size{
            align_pos = self.position;
        }
        else{
            align_pos = ( (self.position - 1) / self.vector_size + 1) * self.vector_size;
        }
        let haystack_len = self.haystack.len() - align_pos;
        let new_haystack;
        unsafe{
            let haystack = self.haystack.as_ptr().add(align_pos);
            new_haystack = std::slice::from_raw_parts(haystack, haystack_len);
        }
        let position;
        let cache;
        unsafe{
            let (position_, cache_) = (self.cb_bufchr2)
                (new_haystack, self.needle0, self.needle1, self.vector_end_ptr);
            position = position_;
            cache = cache_;
        }
        self.cache = cache;
        match position {
            Some(pos) => {
                self.position = align_pos + pos + 1;
            }
            None =>{
                return None;
            }
        }
        Some(self.position)
    }

}

//// struct used when there are three needles
pub struct Bufchr3<'a> {
    haystack: &'a [u8],
    needle0: u8,
    needle1: u8,
    needle2: u8,
    position: usize,
    cache: u64,
    vector_size: usize,
    vector_end_ptr: *const u8,
    cb_bufchr3: CbBufchr3,
}
impl<'a> Bufchr3<'a> {
    /// needle0, needle1, needle2 are what you are trying to find and the location you are looking for is haystack.
    #[inline]
    pub fn new(haystack: &[u8], needle0: u8, needle1: u8, needle2: u8) -> Bufchr3<'_> {
        let vector_size = bufchr::get_batch_byte_size();
        let cb_bufchr3 = bufchr::get_cb_bufchr3();
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / vector_size) * vector_size)
            };
        Bufchr3 { haystack: haystack, needle0: needle0, needle1: needle1,
            needle2: needle2, position: 0, cache: 0,
            vector_size: vector_size, vector_end_ptr: vector_end_ptr, cb_bufchr3: cb_bufchr3 }
    }

    #[doc(hidden)]
    #[inline]
    pub fn new_avx(haystack: &[u8], needle0: u8, needle1: u8, needle2: u8) -> Bufchr3<'_> {
        let vector_size = bufchr::avx::get_batch_byte_size();
        let cb_bufchr3 = bufchr::avx::bufchr3;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / vector_size) * vector_size)
            };
        Bufchr3 { haystack: haystack, needle0: needle0, needle1: needle1,
            needle2: needle2, position: 0, cache: 0,
            vector_size: vector_size, vector_end_ptr: vector_end_ptr, cb_bufchr3: cb_bufchr3 }
    }

    #[doc(hidden)]
    #[inline]
    pub fn new_sse2(haystack: &[u8], needle0: u8, needle1: u8, needle2: u8) -> Bufchr3<'_> {
        let vector_size = bufchr::sse2::get_batch_byte_size();
        let cb_bufchr3 = bufchr::sse2::bufchr3;
        let haystack_len = haystack.len();
        let start_ptr = haystack.as_ptr();
        let vector_end_ptr = 
            unsafe{
                start_ptr.add((haystack_len / vector_size) * vector_size)
            };
        Bufchr3 { haystack: haystack, needle0: needle0, needle1: needle1,
            needle2: needle2, position: 0, cache: 0,
            vector_size: vector_size, vector_end_ptr: vector_end_ptr, cb_bufchr3: cb_bufchr3 }
    }
}
impl<'a> Iterator for Bufchr3<'a> {
    type Item = usize;

    /// The needle position is returned. If there is no needle, None is returned.
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.cache != 0 {
            let start_align_pos = (self.position / self.vector_size) * self.vector_size;
            let bit_pos = self.cache.trailing_zeros() as usize;
            self.cache = self.cache & !(1 << bit_pos);
            self.position = start_align_pos + bit_pos + 1;
            return Some(self.position);
        }
        let align_pos;
        if self.position == 0 {
            align_pos = 0;
        }
        else if self.haystack.len() - self.position < self.vector_size{
            align_pos = self.position;
        }
        else{
            align_pos = ( (self.position - 1) / self.vector_size + 1) * self.vector_size;
        }
        let haystack_len = self.haystack.len() - align_pos;
        let new_haystack;
        unsafe{
            let haystack = self.haystack.as_ptr().add(align_pos);
            new_haystack = std::slice::from_raw_parts(haystack, haystack_len);
        }
        let position;
        let cache;
        unsafe{
            let (position_, cache_) = (self.cb_bufchr3)(
                new_haystack, self.needle0, self.needle1, self.needle2, self.vector_end_ptr);
            position = position_;
            cache = cache_;
        }
        self.cache = cache;
        match position {
            Some(pos) => {
                self.position = align_pos + pos + 1;
            }
            None =>{
                return None;
            }
        }
        Some(self.position)
    }

}

#[inline]
fn forward_pos(mask: u32) -> usize {
    mask.trailing_zeros() as usize
}
