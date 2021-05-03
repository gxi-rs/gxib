run := cargo run --

web:
	$(run) -d ./examples/web web --release

desktop:
	$(run) -d ./examples/desktop desktop