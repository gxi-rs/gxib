run := cargo run --
args :=

web:
	$(run) -d ./examples/web web --release

desktop:
	$(run) -d ./examples/desktop desktop

release:
	cargo release $(args)
