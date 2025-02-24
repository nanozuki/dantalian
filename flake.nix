{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          self',
          pkgs,
          system,
          ...
        }:
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
            ];
          };
          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              pkg-config
              openssl
              (rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; })
              rust-analyzer
            ];
          };
          packages.dantalian =
            with pkgs;
            callPackage ./package.nix {
              rustPlatform = makeRustPlatform {
                cargo = rust-bin.stable.latest.default;
                rustc = rust-bin.stable.latest.default;
              };
            };
          packages.default = self'.packages.dantalian;
        };
      flake = { };
    };
}
