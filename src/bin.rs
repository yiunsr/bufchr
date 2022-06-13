use std::time::Instant;
use bufchr;

// https://jack.wrenn.fyi/blog/include-transmute/
macro_rules! include_bytes_align_as {
    ($align_ty:ty, $path:literal) => {{
        #[repr(C)]
        pub struct AlignedAs<Align, Bytes: ?Sized> {
            pub _align: [Align; 0],
            pub bytes: Bytes,
        }

        static ALIGNED: &AlignedAs::<$align_ty, [u8]> = &AlignedAs {
            _align: [],
            bytes: *include_bytes!($path),
        };

        &ALIGNED.bytes
    }};
}

#[repr(align(32))]
struct Align32;

//static CSV_HAYSTACK: &'static [u8] = include_bytes!("../data/gdp_org.csv");
//static CSV_HAYSTACK: &'static [u8] = include_bytes!("../data/comtrade.csv");
static CSV_HAYSTACK: &'static [u8] = include_bytes_align_as!(Align32, "../data/comtrade.csv"); // alignment of 32

fn boot_haystack(){
    let before = Instant::now();
    let mut count = 0;
    let needle = b',';
    for ch in CSV_HAYSTACK{
        if *ch == needle{
            count += 1;
        }
    }
    println!("boot_haystack\t time:{:.3?} \t count: {}", before.elapsed(), count);
}

fn test_bufchr(){
    let before = Instant::now();
    let mut count = 0;
    let needle = b',';
    let mut bf = bufchr::Bufchr::new(CSV_HAYSTACK, needle);
    loop {
        let n = bf.next();
        if n == None{break;}
        count +=1;
    }
    println!("test_bufchr\t time:{:.3?} \t count: {}", before.elapsed(), count);
}

fn test_bufchr3() {
    let before = Instant::now();
    let n1 = b',';
    let n2 = b'"';
    let n3 = b'\n';
    let mut bf = bufchr::Bufchr3::new(CSV_HAYSTACK, n1, n2, n3);
    let mut count = 0;
    loop {
        let n = bf.next();
        if n == None{break;}
        count +=1;
    }
    println!("test_bufchr3\t time:{:.3?} \t count: {}", before.elapsed(), count);
}

fn test_BufchrCSV() {
    let before = Instant::now();
    let n1 = b',';
    let n2 = b'\n';
    let n3 = b'"';
    let mut bf = bufchr::BufchrCSV::new(CSV_HAYSTACK,  b',');
    let mut count = 0;
    loop {
        let n = bf.next();
        if n == None{break;}
        count +=1;
    }
    println!("test_BufchrCSV\t time:{:.3?} \t count: {}", before.elapsed(), count);
}

fn test_bufchr3_simple() {
    let before = Instant::now();
    let n1 = b',';
    let n2 = b'\n';
    let n3 = b'"';
    let mut count = 0;
    for ch in CSV_HAYSTACK{
        if *ch == n1 || *ch == n2 || *ch==n3{
            count += 1;
        }
    }
    println!("test_bufchr3_simple\t time:{:.3?} \t count: {}", before.elapsed(), count);
}

pub fn main() {
    boot_haystack();
    test_bufchr();
    test_bufchr3();
    test_BufchrCSV();
    test_bufchr3_simple();
}