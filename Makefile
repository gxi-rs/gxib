run := cargo run --

web:
	$(run) -d ./examples/web web

desktop:
	$(run) -d ./examples/desktop desktop