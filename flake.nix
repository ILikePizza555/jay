{
  description = "Jay is a web-based smart home inventory system. Named after California Scrub Jays";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {self, nixpkgs, flake-utils, ...}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        commonBuildInputs = [
          pkgs.nodejs-16_x
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
