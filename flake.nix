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
        default = workspace2d;
        workspace2d = pkgs.callPackage ./package.nix {};
      };

      apps = rec {
        default = workspace2d;
        workspace2d = flake-utils.lib.mkApp {
          name = "workspace2d";
          drv = packages.workspace2d;
        };
      };

      devShells.default = pkgs.mkShell {
        packages = [
          pkgs.jq
          pkgs.bash
        ];
      };
    });
}
