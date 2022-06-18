{
  description = "Jay is a web-based smart home inventory system. Named after California Scrub Jays";

  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-21.05;
    fenix.url = github:nix-community/fenix;
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {self, nixpkgs, fenix, flake-utils, ...}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust-pkg = fenix.packages.${system}.stable.withComponents [ 
          "cargo"
          "rustc"
          "rustfmt"
          "rust-src"
        ];
        commonBuildInputs = [
          pkgs.nodejs-16_x
          rust-pkg
        ];
      in
        {
          defaultPackages = pkgs.stdenv.mkDerivation {
            pname = "jay";
            version = "0.1.0";
            src = ./.;
            buildInputs = commonBuildInputs;
          };

          devShell = pkgs.mkShell {
            packages = [pkgs.sqlite] ++ commonBuildInputs;
          };
        }
    );
}
