## Rusty Snake ##

A translation of my [other Snake program](https://github.com/PtravisCS/snake) originally programmed in C.
The goal of this project was to simply try and recreate the program using Rust so as to gain more experience with the language.

This likely isn't idomatic Rust and could probably be better designed.
This project was intended to be as similar to the original C program as possible to allow for some comparison between the two.

This program relies on the [_ncurses_ crate](https://crates.io/crates/ncurses), which in turn requires that the C ncurses library be installed on your PC.
The ncurses crate is ***Not Safe*** due to it not providing any protection against feeding C style format strings to it's functions among other issues.
More on this problem can be read [here](https://github.com/RustSec/advisory-db/issues/106).

In a future revision of this program I may replace my TUI library to help resolve these security vulnerabilities, the code may also be refactored to be more idiomatic Rust as I learn more going forward.
