# skyWM - Configuration Documentation

## Autostarting

Add shell commands to autostart by adding them to the `AUTOSTART` array on `main.rs`.
Ajusting the array size accordinly

### E.g.
```
const AUTOSTART: [&str; 3] = [
	"feh --no-fehbg --bg-fill ~/Pictures/Wallpaper.png",
	"picom",
	"discord"
];
```
