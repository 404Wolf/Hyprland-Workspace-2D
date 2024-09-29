# Hyprland Workspace Matrix

## A 2D matrix for [hyprland](https://hyprland.app)

Tons of credit to [Trevor](https://github.com/tnichols217) for this! This is mostly a clean-up of his script in his `nix` config, with a `flake.nix` export.

Usage:
`workspace2d.sh <direction> ["all" or ""] ["sync" or ""]`

`all` will move all workspaces in the same direction
`sync` will

Direction can be:
  - `left`
  - `right`
  - `up`
  - `down`
  - `move_left`
  - `move_right`
  - `move_up`
  - `move_down`
