use crate::{bufchr};


/// iterator
pub struct Bufchr<'a> {
    haystack: &'a [u8],
    needle0: u8,
    position: usize,
    cache: u32,
    vector_size: usize,
}

impl<'a> Bufchr<'a> {
    #[inline]
    pub fn new(haystack: &[u8], needle0: u8) -> Bufchr<'_> {
        let vector_size = bufchr::get_vector_size();
        Bufchr {haystack: haystack, needle0: needle0,
            position: 0, cache: 0,
            vector_size: vector_size }
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
        let (position, cache) = bufchr::bufchr(new_haystack, self.needle0);
        self.cache = cache;
        self.position = align_pos + position.unwrap() + 1;
        // found_position
        Some(self.position)
    }

}

/// iterator
pub struct Bufchr2<'a> {
    haystack: &'a [u8],
    needle0: u8,
    needle1: u8,
    position: usize,
    cache: u32,
    vector_size: usize,
}


impl<'a> Bufchr2<'a> {
    #[inline]
    pub fn new(haystack: &[u8], needle0: u8, needle1: u8) -> Bufchr2<'_> {
        let vector_size = bufchr::get_vector_size();
        Bufchr2 { haystack: haystack, needle0: needle0, needle1: needle1,
            position: 0, cache: 0,
            vector_size: vector_size }
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
        let (position, cache) = bufchr::bufchr2(new_haystack, self.needle0, self.needle1);
        self.cache = cache;
        self.position = align_pos + position.unwrap() + 1;
        //     found_position
        Some(self.position)
    }

}

fn forward_pos(mask: u32) -> usize {
    mask.trailing_zeros() as usize
}
