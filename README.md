`pathmatch`: a better `fnmatch`
====

This is an experiment to develop an improved alternative to [POSIX `fnmatch` function](http://pubs.opengroup.org/onlinepubs/9699919799/functions/fnmatch.html). I'm currently using Rust 0.11.

A quick taste of the idea
----

`*.txt` matches all plain text files in the current directory. `*` does not match path separators:

    one.txt
    two.txt

`**.txt` matches all plain text files recursively starting from the current directory:

    one.txt
    two.txt
    foo/3.txt
    foo/bar/4.txt

`**/build/**` would match any path containing given component, including just bare string `build` itself (without any slashes).

`**/build/{Debug,Release}` matches any path ending with `build/Debug` or `build/Release` (including exactly those):

    build/Debug
    build/Release
    subproject1/build/Debug
    subproject1/build/Release

To exclude current folder from search, just require at least one path compoment: `*/**/build/{Debug,Release}`. In other words, `**/` may match start of the path string, `/**` may match end of the path string, and `/**/` may match single path separator (`/`).

The `?` wildcard is also supported, matching exactly one character excluding path separator.

