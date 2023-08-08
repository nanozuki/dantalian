{
  description = "A nfo file generator for your anime. Source from Bangumi.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devshell.url = "github:numtide/devshell";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, naersk, devshell, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        devshell.flakeModule
      ];

      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }:
        let naersk-lib = pkgs.callPackage naersk { }; in
        {
          packages.dantalian = naersk-lib.buildPackage {
            src = ./.;
            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = with pkgs; [ openssl ];
          };
          packages.default = self'.packages.dantalian;
          devshells.default = {
            packages = with pkgs; [ cargo clippy rustfmt ];
          };

          formatter = pkgs.nixpkgs-fmt;
        };
    };
}
