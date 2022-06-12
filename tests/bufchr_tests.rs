
#[cfg(test)]
mod tests {
    use std::mem;
    use bufchr::{Bufchr, Bufchr2, Bufchr3, BufchrFast3};
    use super::*;

    static HAYSTACK_ISO_3166: &'static [u8] = include_bytes!("../data/test/ISO-3166-1.csv");
    static TEST_01: &'static [u8] = include_bytes!("../data/test/test01.txt");

    #[repr(align(32))]
    struct AlignTo32Short{
        pub data:[u8;31]
    }

    #[repr(align(32))]
    struct AlignTo32{
        pub data:[u8;512]
    }

    #[repr(align(32))]
    struct AlignTo32Long{
        pub data:[u8;1024]
    }

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
    fn test_0003_04_checkshort() {
        let haystack = b"a1,\"b11\",c111,d1111,e11111\n";

        let mut a32 = AlignTo32Short{data:[0;31]};
        let mut idx = 0;
        for u in haystack.iter(){
            a32.data[idx] = *u;
            idx +=1;
        }
        
        // let AlignData
        let n1 = b',';
        let n2 = b'\n';
        let n3 = b'"';
        let mut bf = BufchrFast3::new(&a32.data, n1, n2, n3);
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
    fn test_0003_05_checkshort() {
        let haystack = b"a1,\"b11\",c111,d1111,e11111\n";

        let mut a32 = AlignTo32Short{data:[0;31]};
        let mut idx = 0;
        for u in haystack.iter(){
            a32.data[idx] = *u;
            idx +=1;
        }
        
        // let AlignData
        let n1 = b',';
        let n2 = b'\n';
        let n3 = b'"';
        let mut bf = BufchrFast3::new_sse2(&a32.data, n1, n2, n3);
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
    fn test_0003_06_checkshort() {
        let haystack = b"a1,\"b11\",c111,d1111,e11111\n";

        let mut a32 = AlignTo32Short{data:[0;31]};
        let mut idx = 0;
        for u in haystack.iter(){
            a32.data[idx] = *u;
            idx +=1;
        }
        
        // let AlignData
        let n1 = b',';
        let n2 = b'\n';
        let n3 = b'"';
        let mut bf = BufchrFast3::new_avx(&a32.data, n1, n2, n3);
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
        let haystack = b"0123456789012345678901234567890123,567,8\"0\n123456789012345678901234567890";
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

        let byte64 = b"012345678901234567890123456789012345678901234567890123456789zzZZ";
        let mut a32 = AlignTo32{data:[0;512]};
        let mut idx = 0;
        for u in byte64.iter(){
            a32.data[idx] = *u;idx +=1;
        }
        for u in haystack.iter(){
            a32.data[idx] = *u;idx +=1;
        }
        let mut bf = BufchrFast3::new(&a32.data, n1, n2, n3);
        assert_eq!(bf.next(), Some(64 + 34));
        assert_eq!(bf.next(), Some(64 + 38));
        assert_eq!(bf.next(), Some(64 + 40));
        assert_eq!(bf.next(), Some(64 + 42));
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
        let mut bf = Bufchr2::new_avx(haystack, n1, n2);
        assert_eq!(bf.next(), Some(34));
        assert_eq!(bf.next(), Some(38));
        assert_eq!(bf.next(), Some(40));
        assert_eq!(bf.next(), None);

        let n3 = b'\n';
        let mut bf = Bufchr3::new_avx(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(34));
        assert_eq!(bf.next(), Some(38));
        assert_eq!(bf.next(), Some(40));
        assert_eq!(bf.next(), Some(42));
        assert_eq!(bf.next(), None);

        let byte64 = b"012345678901234567890123456789012345678901234567890123456789zzZZ";
        let mut a32 = AlignTo32{data:[0;512]};
        let mut idx = 0;
        for u in byte64.iter(){
            a32.data[idx] = *u;idx +=1;
        }
        for u in haystack.iter(){
            a32.data[idx] = *u;idx +=1;
        }
        let mut bf = BufchrFast3::new_avx(&a32.data, n1, n2, n3);
        assert_eq!(bf.next(), Some(64 + 34));
        assert_eq!(bf.next(), Some(64 + 38));
        assert_eq!(bf.next(), Some(64 + 40));
        assert_eq!(bf.next(), Some(64 + 42));
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
        let mut bf = Bufchr2::new_sse2(haystack, n1, n2);
        assert_eq!(bf.next(), Some(34));
        assert_eq!(bf.next(), Some(38));
        assert_eq!(bf.next(), Some(40));
        assert_eq!(bf.next(), None);

        let n3 = b'\n';
        let mut bf = Bufchr3::new_sse2(haystack, n1, n2, n3);
        assert_eq!(bf.next(), Some(34));
        assert_eq!(bf.next(), Some(38));
        assert_eq!(bf.next(), Some(40));
        assert_eq!(bf.next(), Some(42));
        assert_eq!(bf.next(), None);

        let byte64 = b"012345678901234567890123456789012345678901234567890123456789zzZZ";
        let mut a32 = AlignTo32{data:[0;512]};
        let mut idx = 0;
        for u in byte64.iter(){
            a32.data[idx] = *u;idx +=1;
        }

        for u in haystack.iter(){
            a32.data[idx] = *u;idx +=1;
        }
        let mut bf = BufchrFast3::new_sse2(&a32.data, n1, n2, n3);
        assert_eq!(bf.next(), Some(64 + 34));
        assert_eq!(bf.next(), Some(64 + 38));
        assert_eq!(bf.next(), Some(64 + 40));
        assert_eq!(bf.next(), Some(64 + 42));
        assert_eq!(bf.next(), None);
        
    }

    #[test]
    fn test_0004_04_no_needle_in_first_vector() {
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

        let byte64 = b"012345678901234567890123456789012345678901234567890123456789zzZZ";
        let mut a32 = AlignTo32{data:[0;512]};
        let mut idx = 0;
        for u in byte64.iter(){
            a32.data[idx] = *u;idx +=1;
        }
        for u in byte64.iter(){
            a32.data[idx] = *u;idx +=1;
        }
        for u in haystack.iter(){
            a32.data[idx] = *u;idx +=1;
        }
        let mut bf = BufchrFast3::new_sse2(&a32.data, n1, n2, n3);
        assert_eq!(bf.next(), Some(128 + 34));
        assert_eq!(bf.next(), Some(128 + 38));
        assert_eq!(bf.next(), Some(128 + 40));
        assert_eq!(bf.next(), Some(128 + 42));
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

        let byte64 = b"012345678901234567890123456789012345678901234567890123456789zzZZ";
        let mut a32 = AlignTo32{data:[0;512]};
        let mut idx = 0;
        for u in byte64.iter(){
            a32.data[idx] = *u;idx +=1;
        }
        for u in byte64.iter(){
            a32.data[idx] = *u;idx +=1;
        }
        for u in haystack.iter(){
            a32.data[idx] = *u;idx +=1;
        }
        let mut bf = BufchrFast3::new(&a32.data, n1, n2, n3);
        assert_eq!(bf.next(), Some(128 + 64));
        assert_eq!(bf.next(), Some(128 + 66));
        assert_eq!(bf.next(), Some(128 + 67));
        assert_eq!(bf.next(), Some(128 + 68));
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

        let mut bf = BufchrFast3::new(TEST_01, n1, n2, n3);
        assert_eq!(bf.next(), Some(128));
        assert_eq!(bf.next(), Some(129));
        assert_eq!(bf.next(), Some(131));
        assert_eq!(bf.next(), None);

        let mut bf = BufchrFast3::new(TEST_01, n1, n2, n3);
        assert_eq!(bf.next(), Some(128));
        assert_eq!(bf.next(), Some(129));
        assert_eq!(bf.next(), Some(131));
        assert_eq!(bf.next(), None);

        let byte64 = b"012345678901234567890123456789012345678901234567890123456789zzZZ";
        let mut a32 = AlignTo32{data:[0;512]};
        let mut idx = 0;
        for u in byte64.iter(){
            a32.data[idx] = *u;idx +=1;
        }
        for u in byte64.iter(){
            a32.data[idx] = *u;idx +=1;
        }
        for u in TEST_01.iter(){
            a32.data[idx] = *u;idx +=1;
        }

        let mut bf = BufchrFast3::new(&a32.data, n1, n2, n3);
        assert_eq!(bf.next(), Some(128 + 128));
        assert_eq!(bf.next(), Some(128 + 129));
        assert_eq!(bf.next(), Some(128 + 131));
        assert_eq!(bf.next(), None);
    }

    #[test]
    fn test_1001_iso_3166() {
        //let needle = b',';
        let n1 = b'\n';
        let mut bf = Bufchr::new_avx(HAYSTACK_ISO_3166, n1);
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

    #[test]
    fn test_1002_iso_3166() {
        let n1 = b',';
        let n2 = b'\n';
        let mut bf = Bufchr2::new_avx(HAYSTACK_ISO_3166, n1, n2);
        assert_eq!(bf.next(), Some(4));
        assert_eq!(bf.next(), Some(18));
        let mut last_pos;
        while let position = bf.next() {
            if position == None{
                break;
            }
            last_pos = position.unwrap();
        }
        println!("last");
    }

    #[test]
    fn test_1003_iso_3166() {
        let n1 = b',';
        let n2 = b' ';
        let n3 = b'\n';
        let mut bf = Bufchr3::new_avx(HAYSTACK_ISO_3166, n1, n2, n3);
        assert_eq!(bf.next(), Some(4));
        assert_eq!(bf.next(), Some(12));
        assert_eq!(bf.next(), Some(18));
        let mut last_pos;
        while let position = bf.next() {
            if position == None{
                break;
            }
            last_pos = position.unwrap();
        }
        println!("last");
    }

    #[test]
    fn test_1004_iso_3166() {
        let n1 = b',';
        let n2 = b' ';
        let n3 = b'\n';
        let mut bf = BufchrFast3::new_avx(HAYSTACK_ISO_3166, n1, n2, n3);
        assert_eq!(bf.next(), Some(4));
        assert_eq!(bf.next(), Some(12));
        assert_eq!(bf.next(), Some(18));
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
