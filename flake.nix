{
  description = "Hyprland workspace matrix";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in rec {
      packages = rec {
        default = workspace2d-rs;
        workspace2d-rs = pkgs.callPackage ./nix/build.nix {};
      };

      apps = rec {
        default = workspace2d-rs;
        workspace2d-rs = flake-utils.lib.mkApp {
          name = "workspace2d";
          drv = packages.workspace2d-rs;
        };
      };

      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          jq
          bash
          shellcheck
          cargo
          rustc
          rust-analyzer
          clippy
        ];
      };
    });
}
