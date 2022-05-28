CC=gcc
CFLAGS=-std=c99 -Wall -Wextra -Wpedantic -O2
LIBS=-lm
BIN=donut

all: $(BIN)

$(BIN): $(BIN).c
	$(CC) $(CFLAGS) -o $@ $^ $(LIBS)

clean:
	rm $(BIN)
