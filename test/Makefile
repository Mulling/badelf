all: lib.a lib.so lib.o exe

lib.o: src/lib.c
	${CC} -c ${CFLAGS} $^ -o $@

lib.so: src/lib.c
	${CC}  -shared -fpie ${CFLAGS} $^ -o $@

lib.a: lib.o
	ar rcs lib.a lib.o

exe: lib.o src/exe.c
	${CC} $^ ${CFLAGS} -o $@

clean:
	rm -f lib.a lib.o lib.so exe

.PHONY: clean
