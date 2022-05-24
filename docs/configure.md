# skyWM - Configuration Documentation

## Autostarting

Add shell commands to autostart by adding them to the `AUTOSTART` array on `main.rs`.
Adjusting the array size accordinly

### E.g.
```
const AUTOSTART: [&str; 3] = [
	"feh --no-fehbg --bg-fill ~/Pictures/Wallpaper.png",
	"picom",
	"discord"
];
```
The `[&str; 3]` means that 3 commands are going to be called. You will need to adjust this to the amount of commands that are being called.
