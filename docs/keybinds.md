# skyWM - Keybinds Documentation

## Default keybindings

* The SUPER key refers to the Windows key.

### Application launchers:

**Launch dmenu:**

- Super + D

**Launch terminal emulator:**

- Super + Return

### Move between windows:

**Switch forwards to window:**

- SUPER + Left Arrow

**Switch backwards to window:**
- SUPER + Right Arrow

### Move windows around:

**Move window forwards:**

- SUPER + Shift + Left Arrow

**Move window backwards:**

- SUPER + Shift + Right Arrow

**Toggle fullscreen:**

- SUPER + Shift + F

**Close window:**

- SUPER + Shift + Q

### Workspace Management

**Switch workspaces:**

- SUPER + 1-9

**Toggle workspaces:**

- SUPER + Tab

**Cycle workspaces forward:**

- SUPER + Alt + Period

**Cycle workspaces backwards:**

- SUPER + Alt + Comma

### Window resizing: 

**Make windows larger (vertically):**

- SUPER + Alt + Up Arrow

**Make windows smaller (vertically):** 

- SUPER + Alt + Down Arrow

**Make windows larger (Horizontally):** 

- SUPER + Alt + Right Arrow

**Make windows smaller (Horzontally):** 

- SUPER + Alt + Left Arrow

### Exiting skyWM:

- SUPER + Shift + E

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

Then to use this, you will need to recompile skyWM with the additional changes, [See installation documentation](https://github.com/MrBeeBenson/skyWM/blob/main/docs/INSTALL.md) and move it to /usr/bin/skyWM.