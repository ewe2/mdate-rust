# Rust APIs and Mdate

In this rewrite, I've tried to go native with rust, using the API's commonly used by the community.

Running list of API's, explicit or implicit
--------------------------------------------

* std: used for most things, obviously
* nix: general OS stuff.
* libc: implements most stuff we need for time functions: gettimeofday, struct tm, localtime, time, etc.
* libm: for math functions we need.
