# skyWM Installation Guide

* While this installation guide will work cross-distribution, we have only documented the dependencies for Ubuntu and Ubuntu-based systems for now. Please feel free to contribute by creating a pull request.

## Dependencies:

**Ubuntu:**

Install dependencies:

```sh
sudo apt-get install build-essential libglib2.0-dev cmake pkg-config libxcb-randr0-dev libxcb-xtest0-dev libxcb-xinerama0-dev libxcb-shape0-dev libxcb-xkb-dev libx11-dev libgtk-3-dev dmenu terminator
```

* dmenu and terminator not required for the Window Manager to function, however you will need to edit main.rs to utilise other applications in-place of them. You can edit from line 19-23 to modify these.

* To customise the Window Manager please directly edit main.rs to add the functions you require. There is no specific configuration file and all configurations are edited in main.rs

## Build from source:

```sh
git clone https://github.com/MrBeeBenson/skyWM.git
cd skyWM/
cargo build --release
sudo cp target/release/skyWM /usr/bin/skyWM
```

Then add `exec skyWM` to the bottom of your `~/.xinitrc` file.

To learn the keybinds, or to add custom keybindings, visit our [Keybinds Documentation]().