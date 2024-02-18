{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, naersk, ... }:
    let runtimePrograms = pkgs: with pkgs; [ graphviz nodePackages.katex ];
    in {
      lib.makeSite = { pkgs, name, content
        , seams ? self.packages.${pkgs.system}.seams
        , styles ? self.packages.${pkgs.system}.styles }:
        pkgs.runCommand name {
          buildInputs = [ seams ] ++ runtimePrograms pkgs;
        } ''
          shopt -s dotglob

          mkdir -p $out
          seams build ${content} -o $out --script-assets ${./js}
          cp -r ${styles}/* $out/
        '';

    } // flake-utils.lib.eachDefaultSystem (system:
      let
        lib = pkgs.lib;
        pkgs = import nixpkgs { inherit system; };

        naersk-lib = pkgs.callPackage naersk { };
        styles = pkgs.callPackage ./styles { };

        rustDeps = with pkgs;
          [ openssl pkg-config iconv ]
          ++ lib.optional (system == "aarch64-darwin")
          [ darwin.apple_sdk.frameworks.SystemConfiguration ];

        seams = naersk-lib.buildPackage {
          src = builtins.filterSource
            (path: _: # path is of the format /nix/store/hash-whatever/Cargo.toml
              let rootDirName = builtins.elemAt (lib.splitString "/" path) 4;
              in builtins.elem rootDirName [
                ".cargo"
                "native"
                "src"
                "test_data"

                "build.rs"
                "Cargo.lock"
                "Cargo.toml"
              ]) ./.;
          buildInputs = rustDeps;
        };

        python = pkgs.python3.withPackages (ps: with ps; [ pyyaml click ]);

      in {
        packages = rec {
          inherit styles seams;
          default = seams;
          test-site = self.lib.makeSite {
            inherit pkgs;
            name = "test-site";
            content = ./test_data/contentdir_example;
          };
          astrid-dot-tech-test-site = self.lib.makeSite {
            inherit pkgs;
            name = "astrid-dot-tech-test-site";
            content = ./test_data/astrid_dot_tech_example;
          };
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
              python
              nodePackages.npm
              nodejs_21
            ] ++ rustDeps ++ runtimePrograms pkgs
              ++ lib.optional (system != "aarch64-darwin") [ backblaze-b2 ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      });
}
