# Better Hotkeys

A small Rust program for Windows that adds custom global hotkeys for managing
windows. This project was made for learning, so configuration is currently done
through a TOML file.

## What it can do

- make the active window fullscreen;
- quickly close the active window;
- move the active window to another virtual desktop;
- move a window to a desktop and switch to it immediately;
- drag a window with the mouse.

The program runs in the background and listens for global key presses, so the
hotkeys work even when the program window is not active.

## Building

You need to install [Rust](https://www.rust-lang.org/tools/install) first.

Run this in the project folder:

```powershell
cargo build --release
```

The executable will appear here:

```text
target\release\better_hotkeys.exe
```

To run it in development mode:

```powershell
cargo run
```

## Configuration

By default, the program looks for the configuration here:

```text
%USERPROFILE%\.lixify\better_hotkeys\config.toml
```

You can provide another folder containing `config.toml` or a path to the file:

```powershell
better_hotkeys.exe --config-path "C:\path\to\config.toml"
```

Example configuration:

```toml
[window_features.set_window_fullscreen]
active = true
key_bindings = ["WinLeft", "KeyF"]

[window_features.quick_close_window]
active = true
key_bindings = ["WinLeft", "KeyQ"]

[window_features.slide_to_desktop]
active = true
key_bindings = ["WinLeft"]

[window_features.move_window_to_desktop]
active = true
key_bindings = ["ShiftLeft", "WinLeft"]
additional_flags = ["auto_slide"]

[window_features.drag_window]
active = true
key_bindings = ["WinLeft"]
additional_flags = ["mouse_wheel_only"]
```

Every feature has an `active` option. If it is set to `false`, the feature will
not run. Key names must be written in the same format as in the example:
`WinLeft`, `ShiftLeft`, `KeyF`, `KeyQ`, etc.

## Hotkeys from the example

| Shortcut | Action |
|---|---|
| `WinLeft + F` | Make the window fullscreen |
| `WinLeft + Q` | Close the window |
| `WinLeft + 1..9` | Switch to a desktop |
| `ShiftLeft + WinLeft + 1..9` | Move the window to a desktop |
| Middle mouse button | Drag the window |

For `slide_to_desktop` and `move_window_to_desktop`, a number from `1` to `9`
is added to the configured shortcut. With the `auto_slide` flag, the window is
moved and the selected desktop becomes active immediately.

## Command-line arguments

Main options:

```text
--config-path <PATH>  path to the configuration
--no-window            do not show the console window
--kill                 stop an already running instance
--help                 show help
--version              show the version
```

For example, to run it in the background:

```powershell
better_hotkeys.exe --no-window
```

## Important

The program uses global keyboard input and Windows window management. It is
better to test the configuration by running it normally from a console first.
If the program is already running, starting it again will not create a second
instance.

You can stop the running program with:

```powershell
better_hotkeys.exe --kill
```
