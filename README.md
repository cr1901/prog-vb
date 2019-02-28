# Command Line Virtual Boy Flash Programmer
`prog-vb` is a simple open source utility for programming your
FlashBoy (Plus) development cart from the command line.

This utility is mostly useful for Virtual Boy programmers; the [original
programmer](https://www.planetvb.com/modules/tech/?sec=tools&pid=flashboy)
provided is a (closed sourced!) Windows-only GUI application.
In my experience, I found the GUI broke my concentration when I had to
open the GUI and click around to program my flash cart each time I made
changes to my homebrew (of course I have worse concentration problems that
prematurely "ended" my VB homebrew career :P).

In addition, Mac/Linux users may find this application useful, since I'm
unaware of a Mac/Linux-based solution for FlashBoy. I am looking into hosting
binaries for all OSes using Github releases or perhaps on
[Planet VB](https://www.planetvb.com).

Why did I make this? For fun, mostly. I wanted an excuse to write some Rust.
And I've always wanted an open source version of the programmer :).

## Usage
At present, `prog-vb` takes one mandatory command line argument- the ROM
to flash to the cart. _The ROM must have been padded to 2 Megabytes
ahead of time._ `prog-vb` will automatically detect whether a FlashBoy is
present, so no need to mess with VIDs or PIDs.

Command line invocation is subject to change. The accepted arguments/usage
is display if a user types `prog-vb -h`.

## TODO
* Automatic padding of ROMs.
* Skip erase/erase-only option.
* Pad-only mode.
* Unexpected responses from FlashBoy are handled by failing immediately,
  without a good error message. I've yet to determine what types of
  messages to expect if programming fails.
* Do FlashBoy and FlashBoy Plus have different VID:PIDs?