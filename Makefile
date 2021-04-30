run := cargo run --

test_web:
	$(run) -d tests/web web

test_desktop:
	$(run) -d tests/desktop desktop