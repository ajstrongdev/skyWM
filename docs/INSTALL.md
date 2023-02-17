# skyWM Installation Guide

* While this installation guide will work cross-distribution, we have only documented the dependencies for a few distributions at the moment. Feel free to contribute by creating a Pull Request

## Dependencies:

Install dependencies:

* Installation assumes that you already have installed the latest rust toolchain. If not please install it from here: https://rust-lang.org.

**Debian/Ubuntu:**

```sh
sudo apt-get install build-essential libglib2.0-dev cmake pkg-config libxcb-randr0-dev libxcb-xtest0-dev libxcb-xinerama0-dev libxcb-shape0-dev libxcb-xkb-dev libx11-dev libgtk-3-dev dmenu terminator
```

**Arch:**
```sh
sudo pacman -S base-devel glib2 cmake pkg-config libxcb libx11 gtk3 dmenu terminator
```

**Gentoo:**
```sh
doas emerge -aq base-devel glib2 cmake pkg-config libxcb libx11 gtk3 dmenu terminator
```


* dmenu and terminator not required for the Window Manager to function, however you will need to edit main.rs to utilise other applications in-place of them. You can edit from line 19-23 to modify these.

* To customise the Window Manager please directly edit main.rs to add the functions you require. There is no specific configuration file and all configurations are edited in main.rs

Alternatively, you can use/alter the commands below to your use case:

```sh
git clone https://github.com/httpllamaz/skyWM.git
cd skyWM/
cargo build --release
sudo cp target/release/skyWM /usr/bin/skyWM
sudo cp extra/skywm.desktop /usr/share/xsessions
```
Then add exec skyWM to the bottom of your ~/.xinitrc file.

To learn the keybinds, visit our [Keybinds Documentation](https://github.com/MrBeeBenson/skyWM/blob/main/docs/keybinds.md).

To learn how to do more configuration, such as custom keybinds or automatically starting commands, please read our [Configuration Documentation](https://github.com/MrBeeBenson/skyWM/blob/main/docs/configure.md).
