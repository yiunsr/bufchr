

#[cfg(test)]
mod tests {
    use bufchr::{Bufchr, Bufchr2, Bufchr3};
    use super::*;

    static HAYSTACK_ISO_3166: &'static [u8] = include_bytes!("../data/test/ISO-3166-1.csv");
    static TEST_01: &'static [u8] = include_bytes!("../data/test/test01.txt");

    #[test]
    fn test_0001_01_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111";
        let needle = b',';
        let mut bf = Bufchr::new(haystack, needle);
        assert_eq!(bf.next(), Some(2));
        assert_eq!(bf.next(), Some(6));
        assert_eq!(bf.next(), Some(11));
        assert_eq!(bf.next(), Some(17));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0001_02_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111";
        let needle = b',';
        let mut bf = Bufchr::new_avx(haystack, needle);
        assert_eq!(bf.next(), Some(2));
        assert_eq!(bf.next(), Some(6));
        assert_eq!(bf.next(), Some(11));
        assert_eq!(bf.next(), Some(17));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0001_03_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111";
        let needle = b',';
        let mut bf = Bufchr::new_sse2(haystack, needle);
        assert_eq!(bf.next(), Some(2));
        assert_eq!(bf.next(), Some(6));
        assert_eq!(bf.next(), Some(11));
        assert_eq!(bf.next(), Some(17));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0002_01_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111\n";
        let n1 = b',';
        let n2 = b'\n';
        let mut bf = Bufchr2::new(haystack, n1, n2);
        assert_eq!(bf.next(), Some(2));
        assert_eq!(bf.next(), Some(6));
        assert_eq!(bf.next(), Some(11));
        assert_eq!(bf.next(), Some(17));
        assert_eq!(bf.next(), Some(24));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0002_02_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111\n";
        let n1 = b',';
        let n2 = b'\n';
        let mut bf = Bufchr2::new_avx(haystack, n1, n2);
        assert_eq!(bf.next(), Some(2));
        assert_eq!(bf.next(), Some(6));
        assert_eq!(bf.next(), Some(11));
        assert_eq!(bf.next(), Some(17));
        assert_eq!(bf.next(), Some(24));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0002_03_checkshort() {
        let haystack = b"a1,b11,c111,d1111,e11111\n";
        let n1 = b',';
        let n2 = b'\n';
        let mut bf = Bufchr2::new_sse2(haystack, n1, n2);
        assert_eq!(bf.next(), Some(2));
        assert_eq!(bf.next(), Some(6));
        assert_eq!(bf.next(), Some(11));
        assert_eq!(bf.next(), Some(17));
        assert_eq!(bf.next(), Some(24));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0003_01_checkshort() {
        let haystack = b"a1,\"b11\",c111,d1111,e11111\n";
        let n1 = b',';
        let n2 = b'\n';
        let n3 = b'"';
        let mut bf = Bufchr3::new(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(2));
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(7));
        assert_eq!(bf.next(), Some(8));
        assert_eq!(bf.next(), Some(13));
        assert_eq!(bf.next(), Some(19));
        assert_eq!(bf.next(), Some(26));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0003_02_checkshort() {
        let haystack = b"a1,\"b11\",c111,d1111,e11111\n";
        let n1 = b',';
        let n2 = b'\n';
        let n3 = b'"';
        let mut bf = Bufchr3::new_avx(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(2));
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(7));
        assert_eq!(bf.next(), Some(8));
        assert_eq!(bf.next(), Some(13));
        assert_eq!(bf.next(), Some(19));
        assert_eq!(bf.next(), Some(26));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0003_03_checkshort() {
        let haystack = b"a1,\"b11\",c111,d1111,e11111\n";
        let n1 = b',';
        let n2 = b'\n';
        let n3 = b'"';
        let mut bf = Bufchr3::new_sse2(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(2));
        assert_eq!(bf.next(), Some(3));
        assert_eq!(bf.next(), Some(7));
        assert_eq!(bf.next(), Some(8));
        assert_eq!(bf.next(), Some(13));
        assert_eq!(bf.next(), Some(19));
        assert_eq!(bf.next(), Some(26));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0004_01_no_needle_in_first_vector() {
        let haystack = b"0123456789012345678901234567890123,567,8\"0\n23456789012345678901234567890";
        let n1 = b',';
        let mut bf = Bufchr::new(haystack, n1);
        assert_eq!(bf.next(), Some(34));
        assert_eq!(bf.next(), Some(38));
        assert_eq!(bf.next(), None);

        let n2 = b'"';
        let mut bf = Bufchr2::new(haystack, n1, n2);
        assert_eq!(bf.next(), Some(34));
        assert_eq!(bf.next(), Some(38));
        assert_eq!(bf.next(), Some(40));
        assert_eq!(bf.next(), None);

        let n3 = b'\n';
        let mut bf = Bufchr3::new(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(34));
        assert_eq!(bf.next(), Some(38));
        assert_eq!(bf.next(), Some(40));
        assert_eq!(bf.next(), Some(42));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0004_02_no_needle_in_first_vector() {
        let haystack = b"0123456789012345678901234567890123,567,8\"0\n123456789012345678901234567890";
        let n1 = b',';
        let mut bf = Bufchr::new_avx(haystack, n1);
        assert_eq!(bf.next(), Some(34));
        assert_eq!(bf.next(), Some(38));
        assert_eq!(bf.next(), None);

        let n2 = b'"';
        let mut bf = Bufchr2::new(haystack, n1, n2);
        assert_eq!(bf.next(), Some(34));
        assert_eq!(bf.next(), Some(38));
        assert_eq!(bf.next(), Some(40));
        assert_eq!(bf.next(), None);

        let n3 = b'\n';
        let mut bf = Bufchr3::new(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(34));
        assert_eq!(bf.next(), Some(38));
        assert_eq!(bf.next(), Some(40));
        assert_eq!(bf.next(), Some(42));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0004_03_no_needle_in_first_vector() {
        let haystack = b"0123456789012345678901234567890123,567,8\"0\n123456789012345678901234567890";
        let n1 = b',';
        let mut bf = Bufchr::new_sse2(haystack, n1);
        assert_eq!(bf.next(), Some(34));
        assert_eq!(bf.next(), Some(38));
        assert_eq!(bf.next(), None);

        let n2 = b'"';
        let mut bf = Bufchr2::new(haystack, n1, n2);
        assert_eq!(bf.next(), Some(34));
        assert_eq!(bf.next(), Some(38));
        assert_eq!(bf.next(), Some(40));
        assert_eq!(bf.next(), None);

        let n3 = b'\n';
        let mut bf = Bufchr3::new(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(34));
        assert_eq!(bf.next(), Some(38));
        assert_eq!(bf.next(), Some(40));
        assert_eq!(bf.next(), Some(42));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0004_01_no_needle_in_first_batch() {
        let haystack = 
            b"0123456789012345678901234567890123456789012345678901234567890123,5,\"\n9";
        let n1 = b',';
        let mut bf = Bufchr::new(haystack, n1);
        assert_eq!(bf.next(), Some(64));
        assert_eq!(bf.next(), Some(66));
        assert_eq!(bf.next(), None);

        let n2 = b'"';
        let mut bf = Bufchr2::new(haystack, n1, n2);
        assert_eq!(bf.next(), Some(64));
        assert_eq!(bf.next(), Some(66));
        assert_eq!(bf.next(), Some(67));
        assert_eq!(bf.next(), None);

        let n3 = b'\n';
        let mut bf = Bufchr3::new(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(64));
        assert_eq!(bf.next(), Some(66));
        assert_eq!(bf.next(), Some(67));
        assert_eq!(bf.next(), Some(68));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_0004_02_no_needle_in_first_batch() {
        let n1 = b',';
        let mut bf = Bufchr::new(TEST_01, n1);
        assert_eq!(bf.next(), Some(128));
        assert_eq!(bf.next(), None);

        let n2 = b'"';
        let mut bf = Bufchr2::new(TEST_01, n1, n2);
        assert_eq!(bf.next(), Some(128));
        assert_eq!(bf.next(), Some(129));
        assert_eq!(bf.next(), None);

        let n3 = b'\n';
        let mut bf = Bufchr3::new(TEST_01, n1, n2, n3);
        assert_eq!(bf.next(), Some(128));
        assert_eq!(bf.next(), Some(129));
        assert_eq!(bf.next(), Some(131));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_1000_iso_3166() {
        //let needle = b',';
        let needle = b'\n';
        let mut bf = Bufchr::new_avx(HAYSTACK_ISO_3166, needle);
        assert_eq!(bf.next(), Some(18));
        assert_eq!(bf.next(), Some(30));
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
