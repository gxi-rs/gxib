run := cargo run --

web:
	$(run) -d ./examples/web web --release --output-dir target/.gxi --target-dir target

desktop:
	$(run) -d ./examples/desktop desktop