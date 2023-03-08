.PHONY: all clean

TARGET = week7_rust_ds581

.PHONY: all clean

all: $(TARGET)

$(TARGET):
	cargo build

clean:
	cargo clean

run: $(TARGET)
	./target/debug/$(TARGET)

test:
	cargo test
