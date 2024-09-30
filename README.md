# Hyprland Workspace Matrix

## A 2D matrix for [hyprland](https://hyprland.app)

Tons of credit to [Trevor](https://github.com/tnichols217) for this! This is mostly a clean-up of his script in his `nix` config, with a `flake.nix` export.

Usage:
`workspace2d.sh <direction> ["all" or ""] ["sync" or ""]`

`all` will move all monitors in that direction

`sync` will ensure that all monitors are on the same position within the matrix, following the current active monitor

`move_` moves the current active window to the workspace you are switching to

Direction can be:
  - `left`
  - `right`
  - `up`
  - `down`
  - `move_left`
  - `move_right`
  - `move_up`
  - `move_down`
