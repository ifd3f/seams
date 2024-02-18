{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, naersk, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        lib = pkgs.lib;
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        buildPrograms = with pkgs; [ graphviz nodePackages.katex ];

        seams = naersk-lib.buildPackage (builtins.filterSource
          (path: _: # path is of the format /nix/store/hash-whatever/Cargo.toml
            let rootDirName = builtins.elemAt (lib.splitString "/" path) 4;
            in builtins.elem rootDirName [
              ".cargo"
              "native"
              "src"

              "build.rs"
              "Cargo.lock"
              "Cargo.toml"
            ]) ./.);

        styles = pkgs.callPackage ./styles { };

        makeSite = name: content:
          with pkgs;
          runCommand name { buildInputs = [ seams ]; } ''
            shopt -s dotglob

            mkdir -p $out
            seams build ${content} -o $out
            cp -r ${styles}/* $out/
          '';

        python = pkgs.python3.withPackages (ps: with ps; [ pyyaml click ]);

      in {
        packages = rec {
          inherit styles seams;
          default = seams;
          test-site = makeSite "test-site" ./test_data/contentdir_example;
          astrid-dot-tech-test-site = makeSite "astrid-dot-tech-test-site"
            ./test_data/astrid_dot_tech_example;
        };
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
              sass
              just
              openssl
              pkg-config
              python
              iconv
              nodePackages.npm
              nodejs_21
            ] ++ buildPrograms
              ++ lib.optional (system != "aarch64-darwin") [ backblaze-b2 ]
              ++ lib.optional (system == "aarch64-darwin")
              [ darwin.apple_sdk.frameworks.SystemConfiguration ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      });
}
