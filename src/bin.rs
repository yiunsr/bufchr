use std::time::Instant;

use bufchr;

static CSV_HAYSTACK: &'static [u8] = include_bytes!("../data/gdp_org.csv");
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

pub fn main() {
    boot_haystack();
    test_bufchr();
    // bufchr::test_bufchr();
    // bufchr::test_bufchr2();
    // bufchr::test_bufchr3();
}