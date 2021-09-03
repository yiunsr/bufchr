use crate::bufchr;
use crate::bufchr::CbBufchr;
use crate::bufchr::CbBufchr2;
use crate::bufchr::CbBufchr3;

pub struct Bufchr<'a> {
    haystack: &'a [u8],
    needle0: u8,
    position: usize,
    cache: u32,
    vector_size: usize,
    cb_bufchr: CbBufchr,
}
impl<'a> Bufchr<'a> {
    #[inline]
    pub fn new(haystack: &[u8], needle0: u8) -> Bufchr<'_> {
        let vector_size = bufchr::get_vector_size();
        let cb_bufchr = bufchr::get_cb_bufchr();
        Bufchr {haystack: haystack, needle0: needle0,
            position: 0, cache: 0,
            vector_size: vector_size, cb_bufchr: cb_bufchr}
    }

    #[inline]
    pub fn new_avx(haystack: &[u8], needle0: u8) -> Bufchr<'_> {
        let vector_size = bufchr::avx::get_vector_size();
        let cb_bufchr = bufchr::avx::bufchr;
        Bufchr {haystack: haystack, needle0: needle0,
            position: 0, cache: 0,
            vector_size: vector_size, cb_bufchr: cb_bufchr}
    }

    #[inline]
    pub fn new_sse2(haystack: &[u8], needle0: u8) -> Bufchr<'_> {
        let vector_size = bufchr::sse2::get_vector_size();
        let cb_bufchr = bufchr::sse2::bufchr;
        Bufchr {haystack: haystack, needle0: needle0,
            position: 0, cache: 0,
            vector_size: vector_size, cb_bufchr: cb_bufchr}
    }
}
impl<'a> Iterator for Bufchr<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.cache != 0 {
            let start_align_pos = (self.position / self.vector_size) * self.vector_size;
            let bit_pos = forward_pos(self.cache);
            let cache = self.cache & !(1 << bit_pos);
            self.cache = cache;
            let new_position = start_align_pos + bit_pos + 1;
            self.position = new_position;
            return Some(new_position);
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
            let (position_, cache_) = (self.cb_bufchr)(new_haystack, self.needle0);
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


pub struct Bufchr2<'a> {
    haystack: &'a [u8],
    needle0: u8,
    needle1: u8,
    position: usize,
    cache: u32,
    vector_size: usize,
    cb_bufchr2: CbBufchr2,
}
impl<'a> Bufchr2<'a> {
    #[inline]
    pub fn new(haystack: &[u8], needle0: u8, needle1: u8) -> Bufchr2<'_> {
        let vector_size = bufchr::get_vector_size();
        let cb_bufchr2 = bufchr::get_cb_bufchr2();
        Bufchr2 { haystack: haystack, needle0: needle0, needle1: needle1,
            position: 0, cache: 0,
            vector_size: vector_size, cb_bufchr2: cb_bufchr2 }
    }

    #[inline]
    pub fn new_avx(haystack: &[u8], needle0: u8, needle1: u8) -> Bufchr2<'_> {
        let vector_size = bufchr::avx::get_vector_size();
        let cb_bufchr2 = bufchr::avx::bufchr2;
        Bufchr2 { haystack: haystack, needle0: needle0, needle1: needle1,
            position: 0, cache: 0,
            vector_size: vector_size, cb_bufchr2: cb_bufchr2 }
    }

    #[inline]
    pub fn new_sse2(haystack: &[u8], needle0: u8, needle1: u8) -> Bufchr2<'_> {
        let vector_size = bufchr::sse2::get_vector_size();
        let cb_bufchr2 = bufchr::sse2::bufchr2;
        Bufchr2 { haystack: haystack, needle0: needle0, needle1: needle1,
            position: 0, cache: 0,
            vector_size: vector_size, cb_bufchr2: cb_bufchr2 }
    }
}
impl<'a> Iterator for Bufchr2<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.cache != 0 {
            let start_align_pos = (self.position / self.vector_size) * self.vector_size;
            let bit_pos = forward_pos(self.cache);
            let cache = self.cache & !(1 << bit_pos);
            self.cache = cache;
            let new_position = start_align_pos + bit_pos + 1;
            self.position = new_position;
            return Some(new_position);
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
            let (position_, cache_) = (self.cb_bufchr2)(new_haystack, self.needle0, self.needle1);
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

pub struct Bufchr3<'a> {
    haystack: &'a [u8],
    needle0: u8,
    needle1: u8,
    needle2: u8,
    position: usize,
    cache: u32,
    vector_size: usize,
    cb_bufchr3: CbBufchr3,
}
impl<'a> Bufchr3<'a> {
    #[inline]
    pub fn new(haystack: &[u8], needle0: u8, needle1: u8, needle2: u8) -> Bufchr3<'_> {
        let vector_size = bufchr::get_vector_size();
        let cb_bufchr3 = bufchr::get_cb_bufchr3();
        Bufchr3 { haystack: haystack, needle0: needle0, needle1: needle1,
            needle2: needle2, position: 0, cache: 0,
            vector_size: vector_size, cb_bufchr3: cb_bufchr3 }
    }

    #[inline]
    pub fn new_avx(haystack: &[u8], needle0: u8, needle1: u8, needle2: u8) -> Bufchr3<'_> {
        let vector_size = bufchr::avx::get_vector_size();
        let cb_bufchr3 = bufchr::avx::bufchr3;
        Bufchr3 { haystack: haystack, needle0: needle0, needle1: needle1,
            needle2: needle2, position: 0, cache: 0,
            vector_size: vector_size, cb_bufchr3: cb_bufchr3 }
    }

    #[inline]
    pub fn new_sse2(haystack: &[u8], needle0: u8, needle1: u8, needle2: u8) -> Bufchr3<'_> {
        let vector_size = bufchr::sse2::get_vector_size();
        let cb_bufchr3 = bufchr::sse2::bufchr3;
        Bufchr3 { haystack: haystack, needle0: needle0, needle1: needle1,
            needle2: needle2, position: 0, cache: 0,
            vector_size: vector_size, cb_bufchr3: cb_bufchr3 }
    }
}
impl<'a> Iterator for Bufchr3<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.cache != 0 {
            let start_align_pos = (self.position / self.vector_size) * self.vector_size;
            let bit_pos = forward_pos(self.cache);
            let cache = self.cache & !(1 << bit_pos);
            self.cache = cache;
            let new_position = start_align_pos + bit_pos + 1;
            self.position = new_position;
            return Some(new_position);
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
                new_haystack, self.needle0, self.needle1, self.needle2);
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

fn forward_pos(mask: u32) -> usize {
    mask.trailing_zeros() as usize
}
