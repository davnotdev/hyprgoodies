all: build

build:
	$(MAKE) -C sinkgui
	cargo b --release

install:
	mkdir -p $(DESTDIR)/bin
	$(MAKE) DESTDIR=$(DESTDIR) -C sinkgui install
	cp -f $(CURDIR)/target/release/hyprstash $(DESTDIR)/bin
	cp -f $(CURDIR)/target/release/hyprfill $(DESTDIR)/bin
	chmod 755 $(DESTDIR)/bin/hyprstash
	chmod 755 $(DESTDIR)/bin/hyprfill

clean:
	rm -f $(NAME)

