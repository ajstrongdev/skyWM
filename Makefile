PREFIX=/usr
EXECUTABLE=skyWM

build:
	cargo build --release

install:
	cp target/release/$(EXECUTABLE) $(PREFIX)/bin/$(PREFIX)
	cp extra/skywm.desktop $(PREFIX)/share/xsessions/

uninstall:
	rm $(PREFIX)/bin/skyWM
	rm $(PREFIX)/share/xsessions/skywm.desktop

clean:
	rm -r target/
