#[cfg(test)]
mod tests {
    use bufchr::Bufchr;
    use super::*;

    #[test]
    fn test_0001_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111";
        let needle = b',';
        let mut bf = Bufchr::new(haystack, needle);
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(7));
    }

    fn test_0002_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111";
        let needle = b',';
        let mut bf = Bufchr::new(haystack, needle);
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(7));
    }
}