/*!
This library uses simd instuction to quickly find one byte in a string repeatedly.

# Overview
 The bufchr was created by referring to the code of [memchr](https://github.com/BurntSushi/memchr).
It is faster than memchr in very special cases.
It works efficiently when needle is repeatedly appeared in a smaller place than the vector size (avx is 32 bytes, sse2 is 16 bytes).
When using simd operation, it is possible to calculate whether there is a specific byte in a vector at once.
In the case of memchr, the calculated result is not reused, but in the case of bufchr, the calculated result is used when searching again next time.
<br><br>
 The bufchr is a library developed to make simd csv parser [ss-csv](https://github.com/yiunsr/ss-csv). 
Like a csv file, it is necessary to repeatedly find specific characters (comma, quotation marks, line feed), and it works efficiently when there are several needles within 16 bytes or 32 bytes.
<br><br>
 Currently, only the simd operation of Intel CPU is supported. Only when supporting sse2 or avx can you expect a speedup. In other systems, it operates as a simple search.

 # Example: bufchr

 ```
let haystack = b"a1,b11,c111";
let needle = b',';
let mut bf = Bufchr::new(haystack, needle);
assert_eq!(bf.next(), Some(3));
assert_eq!(bf.next(), Some(7));
assert_eq!(bf.next(), None);

 ```

*/

#[doc(hidden)]
pub use crate::bufchr::{bufchr, bufchr2, bufchr3, CbBufchr, CbBufchr2, CbBufchr3};

pub use crate::bufchr::{
    Bufchr, Bufchr2, Bufchr3,
};

pub mod bufchr;

#[doc(hidden)]
pub fn test_bufchr() {
    println!("======== Start test_bufchr ========");
    let haystack = b"a1,b11,c111,d1111,e11111\n\
        a2,b22,c222,d2222,e22222\n\
        a3,b33,c333,d3333,e33333\n\
        a4,b4,,,";
    let needle = b',';
    let mut bf = bufchr::iter::Bufchr::new(haystack, needle);

    // line 1
    let no_0 = bf.next();
    println!("{}", no_0.unwrap() == 3);
    let no_1 = bf.next();
    println!("{}", no_1.unwrap() == 7);
    let no_2  = bf.next();
    println!("{}", no_2.unwrap() == 12);
    let no_3  = bf.next();
    println!("{}", no_3.unwrap() == 18);

    // line 2
    let no_4  = bf.next();
    println!("{}", no_4.unwrap() == 28);
    let no_5  = bf.next();
    println!("{}", no_5.unwrap() == 32);
    let no_6  = bf.next();
    println!("{}", no_6.unwrap() == 37);
    let no_7  = bf.next();
    println!("{}", no_7.unwrap() == 43);

    // line 3
    let no_8  = bf.next();
    println!("{}", no_8.unwrap() == 53);
    let no_9  = bf.next();
    println!("{}", no_9.unwrap() == 57);
    let no_10  = bf.next();
    println!("{}", no_10.unwrap() == 62);
    let no_11  = bf.next();
    println!("no_11 : {}", no_11.unwrap());
    println!("{}", no_11.unwrap() == 68);

    // line 4
    let no_12 = bf.next();
    println!("no_12 : {}", no_12.unwrap());
    println!("{}", no_12.unwrap() == 78);
    let no_13 = bf.next();
    println!("no_13 : {}", no_13.unwrap());
    println!("{}", no_13.unwrap() == 81);
    println!("======== End ========");
}

#[doc(hidden)]
pub fn test_bufchr2() {
    println!("======== Start test_bufchr2 ========");
    let haystack = b"a1,b11,c111,d1111,e11111\n\
        a2,b22,c222,d2222,e22222\n\
        a3,b33,c333,d3333,e33333\n\
        a4,b4,,,";
    let n0 = b',';
    let n1 = b'\n';
    let mut bf = bufchr::iter::Bufchr2::new(haystack, n0, n1);

    // line 1
    let no_0 = bf.next();
    println!("{}", no_0.unwrap() == 3);
    let no_1 = bf.next();
    println!("{}", no_1.unwrap() == 7);
    let no_2  = bf.next();
    println!("{}", no_2.unwrap() == 12);
    let no_3  = bf.next();
    println!("{}", no_3.unwrap() == 18);
    let no_4  = bf.next();
    println!("no_4 : {}", no_4.unwrap());
    println!("{}", no_4.unwrap() == 25);

    // line 2
    let no_5  = bf.next();
    println!("{}", no_5.unwrap() == 28);
    let no_6  = bf.next();
    println!("{}", no_6.unwrap() == 32);
    let no_7  = bf.next();
    println!("{}", no_7.unwrap() == 37);
    let no_8  = bf.next();
    println!("{}", no_8.unwrap() == 43);
    let no_9  = bf.next();
    println!("no_9 : {}", no_4.unwrap());
    println!("{}", no_9.unwrap() == 50);

    // line 3
    let no_10  = bf.next();
    println!("{}", no_10.unwrap() == 53);
    let no_11  = bf.next();
    println!("{}", no_11.unwrap() == 57);
    let no_12  = bf.next();
    println!("{}", no_12.unwrap() == 62);
    let no_13  = bf.next();
    println!("{}", no_13.unwrap() == 68);
    let no_14 = bf.next();
    println!("no_14 : {}", no_14.unwrap());
    println!("{}", no_14.unwrap() == 75);

    // line 4
    let no_15 = bf.next();
    println!("no_15 : {}", no_15.unwrap());
    println!("{}", no_15.unwrap() == 78);
    let no_16 = bf.next();
    println!("no_16 : {}", no_16.unwrap());
    println!("{}", no_16.unwrap() == 81);
    println!("======== End ========");
}

#[doc(hidden)]
pub fn test_bufchr3() {
    println!("======== Start test_bufchr2 ========");
    let haystack = b"a1,b11,\"c1\",d1111,e11111\n\
        a2,b22,c222,d2222,e22222\n\
        a3,b33,\"c3\",d3333,e33333\n\
        a4,b4,,,";
    let n0 = b',';
    let n1 = b'\n';
    let n2 = b'"';
    let mut bf = bufchr::iter::Bufchr3::new(haystack, n0, n1, n2);

    // line 1
    let no_0 = bf.next();
    println!("{}", no_0.unwrap() == 3);
    let no_1 = bf.next();
    println!("{}", no_1.unwrap() == 7);
    let no_2  = bf.next();
    println!("{}", no_2.unwrap() == 8);
    let no_3  = bf.next();
    println!("{}", no_3.unwrap() == 11);
    let no_4  = bf.next();
    println!("{}", no_4.unwrap() == 12);
    let no_5  = bf.next();
    println!("{}", no_5.unwrap() == 18);
    let no_6  = bf.next();
    println!("no_6 : {}", no_6.unwrap());
    println!("{}", no_6.unwrap() == 25);

    // line 2
    let no_7  = bf.next();
    println!("{}", no_7.unwrap() == 28);
    let no_8  = bf.next();
    println!("{}", no_8.unwrap() == 32);
    let no_9  = bf.next();
    println!("{}", no_9.unwrap() == 37);
    let no_10  = bf.next();
    println!("{}", no_10.unwrap() == 43);
    let no_11  = bf.next();
    println!("no_11 : {}", no_11.unwrap());
    println!("{}", no_11.unwrap() == 50);

    // line 3
    let no_12  = bf.next();
    println!("{}", no_12.unwrap() == 53);
    let no_13  = bf.next();
    println!("{}", no_13.unwrap() == 57);
    let no_14  = bf.next();
    println!("{}", no_14.unwrap() == 58);
    let no_15  = bf.next();
    println!("{}", no_15.unwrap() == 61);
    let no_16  = bf.next();
    println!("{}", no_16.unwrap() == 62);
    let no_17  = bf.next();
    println!("{}", no_17.unwrap() == 68);
    let no_18 = bf.next();
    println!("no_18 : {}", no_18.unwrap());
    println!("{}", no_18.unwrap() == 75);

    // line 4
    let no_19 = bf.next();
    println!("no_19 : {}", no_19.unwrap());
    println!("{}", no_19.unwrap() == 78);
    let no_20 = bf.next();
    println!("no_20 : {}", no_20.unwrap());
    println!("{}", no_20.unwrap() == 81);
    println!("======== End ========");
}