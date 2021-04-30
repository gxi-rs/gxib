run := cargo run --

web:
	$(run) -d ../gxi/examples/web web

desktop:
	$(run) -d ../gxi/examples/desktop desktop