# bufchr
* Much of this library was referenced from https://github.com/BurntSushi/memchr source.
* x64 simd support byte search in binary.

## bufchr vs memchr
* memchr
  * Effective when needles appear rarely in large haystacks.
* bufchr
  * Effective when needles appear many times in large haystacks.