NAME = sinkgui

all: $(NAME)

$(NAME): main.c Makefile
	$(CC) main.c -Wall -o $(NAME) -lX11

install:
	mkdir -p $(DESTDIR)/bin
	cp -f $(NAME) $(DESTDIR)/bin
	chmod 755 $(DESTDIR)/bin/$(NAME)

clean:
	rm -f $(NAME)

