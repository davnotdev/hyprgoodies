#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <unistd.h>

void run() {
	Display *dpy = XOpenDisplay(NULL);
	Window root = XDefaultRootWindow(dpy);

	Window window = XCreateSimpleWindow(dpy, root, 0, 0, 500, 500, 0, 0, 0x222228);
	XSelectInput(dpy, window, KeyPressMask | ButtonPress);
	XMapWindow(dpy, window);

	Atom WM_DELETE_WINDOW = XInternAtom(dpy, "WM_DELETE_WINDOW", False);
	XSetWMProtocols(dpy, window, &WM_DELETE_WINDOW, 1);

	XEvent ev;
	while (1) {
		XNextEvent(dpy, &ev);
		if (ev.type == KeyPress || ev.type == ButtonPress) {
			XDestroyWindow(dpy, window);
			break;
		}

		if (ev.type == ClientMessage) {
			XClientMessageEvent *Event = (XClientMessageEvent *)&ev;
			if ((Atom)Event->data.l[0] == WM_DELETE_WINDOW) {
				XDestroyWindow(dpy, window);
				break;
			}
		}
	}
}

int main() {
	run();
}
