#[macro_use]
extern crate bencher;

use bencher::Bencher;

use bufchr;

static CSV_HAYSTACK: &'static [u8] = include_bytes!("../data/gdp.csv");

fn bench_bufchr(bench: &mut Bencher) {
    bench.iter(|| {
        let needle = b',';
        let mut bf = bufchr::Bufchr::new(CSV_HAYSTACK, needle);
        loop {
            let n = bf.next();
            if n == None{break;}
        }
    });
}


fn b(bench: &mut Bencher) {
    const N: usize = 1024;
    bench.iter(|| {
        vec![0u8; N]
    });
 
    bench.bytes = N as u64;
}

benchmark_group!(benches, bench_bufchr, b);
benchmark_main!(benches);