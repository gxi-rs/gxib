run := cargo run --
args :=

web:
	$(run) -d ./examples/web web $(args) 

desktop:
	$(run) -d ./examples/desktop desktop $(args)

release:
	cargo release $(args)

clean:
	cargo clean
	rm ./Cargo.lock
