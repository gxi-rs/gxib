run := cargo run --

web:
	$(run) -d tests/web web

desktop:
	$(run) -d tests/desktop desktop