CFLAGS=-std=c11 -g -static

mycc: mycc.c

test: mycc
	./test.sh

clean:
	rm -f tmp* *.out

.PHONY: test clean
