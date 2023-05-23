.PHONY:
compiler: target/debug/rvcc-rust

target/debug/rvcc-rust: src/*.rs
	@cargo build --release

.PHONY:
test: compiler
	@./test.sh

.PHONY:
clean:
	rm -f tmp*
	cargo clean



