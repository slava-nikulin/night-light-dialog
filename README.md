# Night Light Dialog Tool

A small Rust tray application for switching Redshift color-temperature presets.

## Features

- Native StatusNotifierItem integration for Linux desktop panels.
- The same preset menu opens from both left-click and right-click on the tray icon.
- Radio-style temperature presets loaded from `~/.config/nighlight/presets.toml`.
- The selected preset is synchronized from the current Redshift configuration when the menu opens.
- No standalone GTK window or manual window positioning.

## Dependencies

- Rust
- Redshift
- A desktop environment / panel with StatusNotifierItem support (for example XFCE Status Tray)

## Build

```bash
cargo build --release
```

Run the tray service directly:

```bash
./target/release/night-light-dialog
```

The tray icon uses the symbolic `weather-clear-night-symbolic` icon from the current desktop icon theme.

## Presets

The presets configuration file is located at:

```text
~/.config/nighlight/presets.toml
```

If it does not exist or contains no presets, the application uses these defaults:

- 2K = 2000 K
- 2.5K = 2500 K
- 3K = 3000 K
- 3.5K = 3500 K
- 4K = 4000 K
- 4.5K = 4500 K
- 5K = 5000 K
- 5.5K = 5500 K
- 6K = 6000 K
- 6.5K = 6500 K

See `examples/presets.toml` for an example configuration.

## Redshift configuration

The selected temperature is stored in:

```text
~/.config/redshift/config
```

Selecting a preset updates the config and reapplies Redshift.

## Migration note

The previous GTK window launcher/wrapper is no longer required by the application itself. Keep the existing local wrapper and XFCE launcher in place until the tray implementation has been tested successfully on the target desktop. After validation, remove the old wrapper/launcher and configure the tray binary to start with the XFCE session.
