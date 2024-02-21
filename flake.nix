{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    dream2nix.url = "github:nix-community/dream2nix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, naersk, dream2nix, ... }:
    let runtimePrograms = pkgs: with pkgs; [ graphviz nodePackages.katex ];
    in {
      lib.makeSite = { pkgs, name, content
        , seams ? self.packages.${pkgs.system}.seams
        , styles ? self.packages.${pkgs.system}.styles
        , js ? self.packages.${pkgs.system}.js }:
        pkgs.runCommand name {
          buildInputs = [ seams ] ++ runtimePrograms pkgs;
        } ''
          shopt -s dotglob

          mkdir -p $out
          seams build ${content} -o $out --script-assets ${./js}
          cp -r ${styles}/* $out/
          cp -r ${js}/* $out/
        '';

    } // flake-utils.lib.eachDefaultSystem (system:
      let
        lib = pkgs.lib;
        pkgs = import nixpkgs { inherit system; };

        naersk-lib = pkgs.callPackage naersk { };
        styles = pkgs.callPackage ./styles { };

        js-build = dream2nix.lib.evalModules {
          packageSets.nixpkgs = pkgs;
          modules = [
            ./nix/js.nix
            {
              paths.projectRoot = ./.;
              # can be changed to ".git" or "flake.nix" to get rid of .project-root
              paths.projectRootFile = "flake.nix";
              paths.package = ./.;
            }
          ];
        };

        js = pkgs.runCommand "seams-js" { } ''
          mkdir -p $out
          cp -r ${js-build}/lib/node_modules/seams-js/out-scripts/* $out
        '';

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
          inherit js styles seams;
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
              nodePackages.node2nix
              nodejs_21
            ] ++ rustDeps ++ runtimePrograms pkgs
              ++ lib.optional (system != "aarch64-darwin") [ backblaze-b2 ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      });
}
