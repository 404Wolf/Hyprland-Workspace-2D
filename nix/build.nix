{ rustPlatform, lib }:

rustPlatform.buildRustPackage {
  pname = "workspace2d-rs";
  version = "0.1.0";

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  meta = with lib; {
    description = "2D workspace navigation for Hyprland (Rust version)";
    license = licenses.mit;
    platforms = platforms.linux;
  };
}
