# stdin torture
Torture tests of standard input in various languages

Purpose is to read 10_000_000 lines of "hello world" as quickly as possible, and count their total number. This is much harder than you think.

# Usage 

``` shell
./run_tests.py 
```

will make the input data and run tests on all languages for which there is an implementation.

# Runtimes/compilers

 - c: GCC
 - c++: G++
 - Java: openJDK
 - Python: CPython 3.8 +
 - Rust: rustc 1.65 +
 - JavaScript: bun

# Contributing

Feel free to make a PR if you think my implementations are not idiomatic.

Also feel free to add PRs for more languages and/or compiler options. Esoteric languages are not welcome.

Keep in mind the idea is not necessarily to test the "best possible" performance achievable with a given language, but rather the performance one can
expect in "real world" using normal, idiomatic code. This means, for example, that c does not get to use inline SIMD instructions (as that would imply assembly as the language).
