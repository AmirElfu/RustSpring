Nostalgia bucklespring keyboard sound
=====================================

Copyright 2016 Ico Doornekamp

This project emulates the sound of my old faithful IBM Model-M space saver
bucklespring keyboard while typing on my notebook, mainly for the purpose of
annoying the hell out of my coworkers.

![Model M](img/model-m.jpg)
![Buckle](img/buckle.gif)

Bucklespring runs as a background process and plays back the sound of each key
pressed and released on your keyboard, just as if you were using an IBM
Model-M. The sound of each key has carefully been sampled, and is played back
while simulating the proper distance and direction for a realistic 3D sound
palette of pure nostalgic bliss.

To temporarily silence bucklespring, for example to enter secrets, press
ScrollLock twice (but be aware that those ScrollLock events _are_ delivered to
the application); same to unmute. The keycode for muting can be changed with
the `-m` option. Use keycode 0 to disable the mute function.

Installation
------------


### Debian/Ubuntu

Download last .deb file from release pages then :

```
$ sudo apt install ./PATH/OF/RUSTSPRING.deb
```

### Linux, building from source

To compile on debian-based linux distributions, first make sure the require
libraries and header files are installed, then simply run `make`:

#### Dependencies on Debian/Ubuntu
```
$ sudo apt install libopenal-dev libalure-dev libxtst-dev
```

#### Dependencies on Arch Linux
```
$ sudo pacman -Sy openal alure libxtst
```

#### Dependencies on Fedora Linux
```
$ sudo dnf install openal-soft-devel alure-devel libX11-devel libXtst-devel
```

#### Building
```
$ cargo build --release
$ ./target/release/rustspring
```

The default Linux build requires X11 for grabbing events. If you want to use
Bucklespring on the linux console or Wayland display server, you can SHOULD WAIT.

### MacOS

SOON..


### Windows

NOBODY USES WINDOWS.
