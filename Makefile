.PHONY : all test clean

all : pmfind test

pmfind : pmfind.rs pathmatch.rs Makefile
	rustc -g -o "$@" "$<"

test : test-pathmatch
	./test-pathmatch

test-pathmatch : pathmatch.rs Makefile
	rustc -g --test -o "$@" "$<"

clean :
	rm -rf test-pathmatch pmfind test-pathmatch.dSYM pmfind.dSYM

