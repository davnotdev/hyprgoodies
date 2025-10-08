# `hypr` Goodies

Hyprland is a fun, bleeding edge Wayland compositor that is surprisingly stable for daily use. 
This repository contains a few tools I personally use to manage my Hyprland desktop.

Currently, these tools mainly exist for the sake of workplace and monitor management.
As a college student, I created these tools to tackle the following environment:
- external display gets plugged in and out multiple times a day
- context switching between projects and classes constantly.

## Installation / Setup

You can build everything using `cargo`.
`sinkgui` uses `make` and requires the `X11` headers to be installed.

## `hyprstash`

`hyprstash` allows you to stash workspaces, monitors, or entire sessions for later use.

```
hyprstash stash-workspace <NAME>
    --workspace [OPTIONAL WORKSPACE ID]

hyprstash stash-monitor <NAME>
    --monitor [OPTIONAL MONITOR ID]

hyprstash stash-everything <NAME>

# ---

hyprstash pop-workspace <NAME>
    --target [OPTIONAL TARGET]

hyprstash pop-monitor <NAME>
    --target [OPTIONAL TARGET]
    --relative
        absolute attempts to preserve the ordering of workspaces at stash time
        relative places windows into existing workspaces regardless of where they lie 

hyprstash pop-everything <NAME>
    --relative
        absolute moves workspaces to where they were at stash time
        relative places windows into existing workspaces regardless of where they lie
    --no-missing-monitors
        throw an error if one or more monitors are missing

# ---

hyprstash list
hyprstash clear
```

## `hyprfill`

`hyprfill` places workspaces and applications onto the correct monitors given a configuration file.
The default configuration location is at `$HOME/.config/hypr/hyprfill.json`.

You can use `hyprfill --help` to get help on writing a configuration.
`hyprfill fill` executes placement.
By default, `hyprfill` expects `sinkgui`, a separate program to be installed.

> Because of IPC limitations, `hyprfill` should only ever spawn simple applications with one window.

### `sinkgui`

`sinkgui` is a <50 LOC X11 window that closes on interaction.
It exists to act as a placeholder as you set up your workspaces.

