#[cfg(test)]
mod tests {
    use bufchr::{Bufchr, Bufchr2, Bufchr3};
    use super::*;

    static HAYSTACK_ISO_3166: &'static [u8] = include_bytes!("../data/test/ISO-3166-1.csv");

    #[test]
    fn test_0001_01_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111";
        let needle = b',';
        let mut bf = Bufchr::new(haystack, needle);
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(7));
        assert_eq!(bf.next(), Some(12));
        assert_eq!(bf.next(), Some(18));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0001_02_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111";
        let needle = b',';
        let mut bf = Bufchr::new_avx(haystack, needle);
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(7));
        assert_eq!(bf.next(), Some(12));
        assert_eq!(bf.next(), Some(18));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0001_03_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111";
        let needle = b',';
        let mut bf = Bufchr::new_sse2(haystack, needle);
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(7));
        assert_eq!(bf.next(), Some(12));
        assert_eq!(bf.next(), Some(18));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0002_01_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111\n";
        let n1 = b',';
        let n2 = b'\n';
        let mut bf = Bufchr2::new(haystack, n1, n2);
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(7));
        assert_eq!(bf.next(), Some(12));
        assert_eq!(bf.next(), Some(18));
        assert_eq!(bf.next(), Some(25));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0002_02_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111\n";
        let n1 = b',';
        let n2 = b'\n';
        let mut bf = Bufchr2::new_avx(haystack, n1, n2);
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(7));
        assert_eq!(bf.next(), Some(12));
        assert_eq!(bf.next(), Some(18));
        assert_eq!(bf.next(), Some(25));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0002_03_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111\n";
        let n1 = b',';
        let n2 = b'\n';
        let mut bf = Bufchr2::new_sse2(haystack, n1, n2);
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(7));
        assert_eq!(bf.next(), Some(12));
        assert_eq!(bf.next(), Some(18));
        assert_eq!(bf.next(), Some(25));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0003_01_checkshort() {
        let haystack = b"a1,\"b11\",c111,d1111,e11111\n";
        let n1 = b',';
        let n2 = b'\n';
        let n3 = b'"';
        let mut bf = Bufchr3::new(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(4));
        assert_eq!(bf.next(), Some(8));
        assert_eq!(bf.next(), Some(9));
        assert_eq!(bf.next(), Some(14));
        assert_eq!(bf.next(), Some(20));
        assert_eq!(bf.next(), Some(27));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0003_02_checkshort() {
        let haystack = b"a1,\"b11\",c111,d1111,e11111\n";
        let n1 = b',';
        let n2 = b'\n';
        let n3 = b'"';
        let mut bf = Bufchr3::new_avx(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(4));
        assert_eq!(bf.next(), Some(8));
        assert_eq!(bf.next(), Some(9));
        assert_eq!(bf.next(), Some(14));
        assert_eq!(bf.next(), Some(20));
        assert_eq!(bf.next(), Some(27));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0003_03_checkshort() {
        let haystack = b"a1,\"b11\",c111,d1111,e11111\n";
        let n1 = b',';
        let n2 = b'\n';
        let n3 = b'"';
        let mut bf = Bufchr3::new_sse2(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(4));
        assert_eq!(bf.next(), Some(8));
        assert_eq!(bf.next(), Some(9));
        assert_eq!(bf.next(), Some(14));
        assert_eq!(bf.next(), Some(20));
        assert_eq!(bf.next(), Some(27));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0004_01_no_needle_in_first_vector() {
        let haystack = b"0123456789012345678901234567890123,567,8\"0\n23456789012345678901234567890";
        let n1 = b',';
        let mut bf = Bufchr::new(haystack, n1);
        assert_eq!(bf.next(), Some(35));
        assert_eq!(bf.next(), Some(39));
        assert_eq!(bf.next(), None);

        let n2 = b'"';
        let mut bf = Bufchr2::new(haystack, n1, n2);
        assert_eq!(bf.next(), Some(35));
        assert_eq!(bf.next(), Some(39));
        assert_eq!(bf.next(), Some(41));
        assert_eq!(bf.next(), None);

        let n3 = b'\n';
        let mut bf = Bufchr3::new(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(35));
        assert_eq!(bf.next(), Some(39));
        assert_eq!(bf.next(), Some(41));
        assert_eq!(bf.next(), Some(43));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0004_02_no_needle_in_first_vector() {
        let haystack = b"0123456789012345678901234567890123,567,8\"0\n123456789012345678901234567890";
        let n1 = b',';
        let mut bf = Bufchr::new_avx(haystack, n1);
        assert_eq!(bf.next(), Some(35));
        assert_eq!(bf.next(), Some(39));
        assert_eq!(bf.next(), None);

        let n2 = b'"';
        let mut bf = Bufchr2::new(haystack, n1, n2);
        assert_eq!(bf.next(), Some(35));
        assert_eq!(bf.next(), Some(39));
        assert_eq!(bf.next(), Some(41));
        assert_eq!(bf.next(), None);

        let n3 = b'\n';
        let mut bf = Bufchr3::new(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(35));
        assert_eq!(bf.next(), Some(39));
        assert_eq!(bf.next(), Some(41));
        assert_eq!(bf.next(), Some(43));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0004_03_no_needle_in_first_vector() {
        let haystack = b"0123456789012345678901234567890123,567,8\"0\n123456789012345678901234567890";
        let n1 = b',';
        let mut bf = Bufchr::new_sse2(haystack, n1);
        assert_eq!(bf.next(), Some(35));
        assert_eq!(bf.next(), Some(39));
        assert_eq!(bf.next(), None);

        let n2 = b'"';
        let mut bf = Bufchr2::new(haystack, n1, n2);
        assert_eq!(bf.next(), Some(35));
        assert_eq!(bf.next(), Some(39));
        assert_eq!(bf.next(), Some(41));
        assert_eq!(bf.next(), None);

        let n3 = b'\n';
        let mut bf = Bufchr3::new(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(35));
        assert_eq!(bf.next(), Some(39));
        assert_eq!(bf.next(), Some(41));
        assert_eq!(bf.next(), Some(43));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_1000_iso_3166() {
        //let needle = b',';
        let needle = b'\n';
        let mut bf = Bufchr::new_avx(HAYSTACK_ISO_3166, needle);
        assert_eq!(bf.next(), Some(19));
        assert_eq!(bf.next(), Some(31));
        let mut last_pos;
        while let position = bf.next() {
            if position == None{
                break;
            }
            last_pos = position.unwrap();
        }
        println!("last");
    }
}
