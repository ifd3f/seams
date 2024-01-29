{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, naersk, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        buildPrograms = with pkgs; [ graphviz ];
      in {
        packages.default = naersk-lib.buildPackage ./.;
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy ]
              ++ buildPrograms;
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      });
}
