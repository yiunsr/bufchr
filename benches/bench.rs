#[macro_use]
extern crate bencher;

use bencher::Bencher;

use bufchr;

static CSV_HAYSTACK: &'static [u8] = include_bytes!("../data/gdp.csv");

fn read_gdp_csv(bench: &mut Bencher) {
    bench.iter(|| {
        let needle = b',';
        let mut bf = bufchr::Bufchr::new(CSV_HAYSTACK, needle);
        loop {
            let n = bf.next();
            if n == None{break;}
        }
    });
}

fn read_gdp_csv2(bench: &mut Bencher) {
    bench.iter(|| {
        let n1 = b',';
        let n2 = b'"';
        let mut bf = bufchr::Bufchr2::new(CSV_HAYSTACK, n1, n2);
        loop {
            let n = bf.next();
            if n == None{break;}
        }
    });
}

fn read_gdp_csv3(bench: &mut Bencher) {
    bench.iter(|| {
        let n1 = b',';
        let n2 = b'"';
        let n3 = b'\n';
        let mut bf = bufchr::Bufchr3::new(CSV_HAYSTACK, n1, n2, n3);
        loop {
            let n = bf.next();
            if n == None{break;}
        }
    });
}

fn read_gdp_csv3_fast(bench: &mut Bencher) {
    bench.iter(|| {
        let n1 = b',';
        let n2 = b'"';
        let n3 = b'\n';
        let mut bf = bufchr::BufchrFast3::new(CSV_HAYSTACK, n1, n2, n3);
        loop {
            let n = bf.next();
            if n == None{break;}
        }
    });
}

benchmark_group!(benches, read_gdp_csv, read_gdp_csv2, read_gdp_csv3, read_gdp_csv3_fast);
benchmark_main!(benches);