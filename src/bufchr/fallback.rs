
pub fn bufchr(haystack: &[u8], n1: u8, vector_end_ptr: *const u8) -> (Option<usize>, u64) {
    (haystack.iter().position(|&b| b == n1), 0)
}
pub fn bufchr_raw(haystack: &[u8], n1: u8) -> Option<usize>{
    haystack.iter().position(|&b| b == n1)
}

pub fn bufchr2(haystack: &[u8], n1: u8, n2: u8, vector_end_ptr: *const u8) -> (Option<usize>, u64) {
    (haystack.iter().position(|&b| b == n1 || b == n2), 0)
}
pub fn bufchr2_raw(haystack: &[u8], n1: u8, n2: u8) -> Option<usize> {
    haystack.iter().position(|&b| b == n1 || b == n2)
}

pub fn bufchr3(haystack: &[u8], n1: u8, n2: u8, n3: u8, vector_end_ptr: *const u8) -> (Option<usize>, u64) {
    (haystack.iter().position(|&b| b == n1 || b == n2 || b == n3), 0)
}
pub fn bufchr3_raw(haystack: &[u8], n1: u8, n2: u8, n3: u8) -> Option<usize> {
    haystack.iter().position(|&b| b == n1 || b == n2 || b == n3)
}

pub fn get_vector_size() -> usize{
    1
}

pub fn get_batch_byte_size() -> usize{
    1
}