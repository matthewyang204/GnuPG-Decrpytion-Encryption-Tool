RC = rustc
RFLAGS = -C opt-level=3
BIN = gpgde
SRC = src/main.rs

all: build

build: $(SRC) src/decrypter.rs src/encrypter.rs
	$(RC) $(RFLAGS) $(SRC) -o $(BIN)
	
clean:
	rm -f $(BIN)
	
.PHONY: all build clean
