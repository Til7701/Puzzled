# Puzzled <img src="data/icons/hicolor/scalable/apps/de.til7701.Puzzled.svg" alt="icon" width="50" align="right"/>

An Adwaita application to solve daily puzzles in various formats. It can tell you, if you are on the
right track or, whether it is impossible to solve the puzzle with your current approach.

![Screenshort of the start screen](data/screenshot-start-dark.png)
![Screenshot of the puzzle area while solving the puzzle](data/screenshot-year-dark.png)

## Installation

<a href='https://flathub.org/apps/de.til7701.Puzzled'>
    <img width='240' alt='Get it on Flathub' src='https://flathub.org/api/badge?locale=en'/>
</a>

Puzzled is available on Flathub. Alternatively, you can install it manually by downloading the latest release from the
[releases page](https://github.com/Til7701/Puzzled/releases). After downloading the Flatpak bundle, you can install it
using the following command:

```bash
flatpak install --user path/to/de.til7701.Puzzled_VERSION_ARCH.flatpak
```

## Build and Run

### Gnome Builder (recommended)

When you clone and open the project in Gnome Builder, it will automatically tell you to download the required
dependencies. After that, you can build and run the project from within Gnome Builder.

### Build Manually

You need the following dependencies to build the project:

- Rust and Cargo
- Meson
- GTK4 and Libadwaita dependencies

Then you can install it from the command line (more dependencies may be required):

```bash
meson setup build
meson compile -C build
meson install -C build
```

## License

This project is licensed under the GNU General Public License v3.0. See the COPYING file for details.
