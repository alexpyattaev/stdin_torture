# stdin torture
Torture tests of standard input in various languages

Purpose is to read 100000 lines of "hello world" as quickly as possible, and count their total number. This is much harder than you think.

# Usage 

``` shell
./run_tests.py 
```

will make the input data and run tests on all languages.

# Contributing
Feel free to make a PR if you think my implementations are not idiomatic.
Keep in mind the idea is not necessarily to test the "BEST POSSIBLE" performance achievable with a given language, but rather the performance one can
expect in "real world" using normal, idiomatic code. This means c++ does not get to use SIMD instructions.
