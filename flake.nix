{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            (pkgs.lib.hasSuffix "\.mp3" path) ||
            (craneLib.filterCargoSources path type)
          ;
        };

        commonArgs = {
          version = "0.1.0";
          buildInputs = with pkgs; [ pkg-config alsa-lib xorg.libX11 ];

          LD_LIBRARY_PATH = builtins.concatStringsSep ":" [
            "${pkgs.xorg.libX11}/lib"
            "${pkgs.xorg.libXi}/lib"
            "${pkgs.libGL}/lib"
            "${pkgs.libxkbcommon}/lib"
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          inherit src;
          pname = "snake-workspace";
        });

        snake-tui = craneLib.buildPackage (commonArgs // {
          inherit src cargoArtifacts;
          pname = "snake-tui";
          cargoBuildCommand = "cargo build --profile release --package snake-tui";
        });

        snake-gui = craneLib.buildPackage (commonArgs // {
          inherit src cargoArtifacts;
          pname = "snake-gui";
          cargoBuildCommand = "cargo build --profile release --package snake-gui";
        });
      in
      with pkgs;
      {
        packages = {
          inherit snake-tui snake-gui;
          default = snake-tui;
        };

        devShells.default = mkShell {
          buildInputs = [
            rustToolchain
            pkg-config
            alsa-lib
          ];

          LD_LIBRARY_PATH = builtins.concatStringsSep ":" [
            "${pkgs.xorg.libX11}/lib"
            "${pkgs.xorg.libXi}/lib"
            "${pkgs.libGL}/lib"
            "${pkgs.libxkbcommon}/lib"
          ];
        };

        formatter = nixpkgs-fmt;
      }
    );
}
