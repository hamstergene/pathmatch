all :
	false

test : ./test-pathmatch
	./test-pathmatch

./test-pathmatch : pathmatch.rs Makefile
	rustc --test -o "$@" "$<"


