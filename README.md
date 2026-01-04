# Puzzle More Days

A Libadwaita application to solve daily puzzles in various formats. In the future it may tell you, if you are on the
right track or, whether it is impossible to solve the puzzle with your current approach.

![screenshot](https://til7701.de/assets/images/puzzlemoredays.png)

## Build and Run

You need to have Rust and Cargo installed. Then you can build and run the application with Gnome Builder (recommended)
or install it from the command line (more dependencies may be required):

```bash
meson setup builddir
meson compile -C builddir
meson install -C builddir
```

## License

This project is licensed under the GNU General Public License v3.0. See the COPYING file for details.
