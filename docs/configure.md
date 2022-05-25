# skyWM - Configuration Documentation

## Automatically Starting Commands on launch

Add shell commands to autostart by adding them to the `AUTOSTART` array on `main.rs`.

### E.g.
```
const AUTOSTART: [&str; 3] = [
	"feh --no-fehbg --bg-fill ~/Pictures/Wallpaper.png",
	"picom",
	"discord"
];
```
The `[&str; 3]` section in the array means that 3 commands are going to be called. You will need to adjust this to the amount of commands that are being called.

## Adding custom keybindings:

After line 20 in main.rs, you will need to add the program you wish to launch. Here is an example:

```rs
const DISCORD: &str = "discord";
```

This will create a constant that can be used to launch discord, next you will need to create the keybinding after line 63. Here is an example:

```rs
"M-h" => run_external!(DISCORD);
```

This sets the Window Manager to launch Discord. The capital "M" stands for MOD, which is the SUPER key by default. The lower-case "h" is for the H key on your keyboard.


## Utilising your changes

Then to use this, you will need to recompile skyWM with the additional changes, [See installation documentation](https://github.com/MrBeeBenson/skyWM/blob/main/docs/INSTALL.md) for how to build from source.