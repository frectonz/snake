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

        craneLib = crane.lib.${system};
        src = craneLib.cleanCargoSource (craneLib.path ./snake-tui);

        commonArgs = {
          inherit src;
          cargoVendorDir = craneLib.vendorCargoDeps { cargoLock = ./Cargo.lock; };
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        bin = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

      in
      with pkgs;
      {

        packages = {
          inherit bin;
          default = bin;
        };

        devShells.default = mkShell {
          buildInputs = [
            pkg-config
            alsa-lib
          ];
          packages = [
            rust-bin.stable.latest.default
            rust-analyzer
            nil
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
