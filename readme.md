# bufchr
* Much of this library was referenced from https://github.com/BurntSushi/memchr source.
* x64 simd support byte search in binary.

## bufchr vs memchr
* bufchr specializes in recursively finding separators.
* memchr
  * Effective when needles appear rarely in large haystacks.
* bufchr
  * Effective when needles appear many times in large haystacks.
* benchmark
  * https://gist.github.com/yiunsr/48d7f0f996c1b534ecb842c047ffb815
  * If you're looking for a repeating seperator for a regular csv file, bufchr is definitely faster. It uses half the time than memchr.
* Why is bufchr efficient for csv files
  * memchr and bufchr use _mm256_cmpeq_epi8(or _mm_cmpeq_epi8) instruction.
  bufchr reuse result. bufchr uses the stored result without using _mm256_cmpeq_epi8(or _mm_cmpeq_epi8) again in the already calculated section. So, it is very efficient when there are many needles within 32 bytes (when avx2 is supported). _mm256_cmpeq_epi8 (or _mm_cmpeq_epi8) is fast, but reusing the used result will speed things up because several additional actions are required before and after using this instruction.


## example
* only one type of needle

```
let haystack = b"a11,b11,c111,d1111,e11111";
let needle = b',';
let mut bf = Bufchr::new(haystack, needle);
assert_eq!(bf.next(), Some(3));
assert_eq!(bf.next(), Some(7));
assert_eq!(bf.next(), Some(12));
assert_eq!(bf.next(), Some(18));
assert_eq!(bf.next(), None);
```

* only 2 types of needles

```
let haystack = b"a11,b11,c111,d1111,e11111\n";
let n1 = b',';
let n2 = b'\n';
let mut bf = Bufchr2::new(haystack, n1, n2);
assert_eq!(bf.next(), Some(3));
assert_eq!(bf.next(), Some(7));
assert_eq!(bf.next(), Some(12));
assert_eq!(bf.next(), Some(18));
assert_eq!(bf.next(), Some(25));
assert_eq!(bf.next(), None);
```

* only 3 types of needles

```
let haystack = b"a11,\"b11\",c111,d1111,e11111\n";
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
```

* CSV parsing only function
  * 32 byte memory alignment is required.
  * Only the column delimiter can be passed as an argument.

```
let haystack = b"a11,\"b11\",c111,d1111,e11111\n";
let n1 = b',';
let mut bf = BufchrCSV::new(haystack, n1);
assert_eq!(bf.next(), Some(3));
assert_eq!(bf.next(), Some(4));
assert_eq!(bf.next(), Some(8));
assert_eq!(bf.next(), Some(9));
assert_eq!(bf.next(), Some(14));
assert_eq!(bf.next(), Some(20));
assert_eq!(bf.next(), Some(27));
assert_eq!(bf.next(), None);
```

## Algorithms 
* For CPUs that support avx2, the _mm256_cmpeq_epi8 instruction is supported. Through _mm256_cmpeq_epi8, comparison operation can be performed on 32 bytes at a time. For CPUs that support sse2, the _mm_cmpeq_epi8 instruction is supported and comparison operations are supported for 16 bytes at a time. 


## CSV sample
* gdp.csv : https://data.worldbank.org/indicator/NY.GDP.MKTP.CD
* ISO-3166-1.csv : https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
* comtrade.csv : https://comtrade.un.org/