{ lib, config, dream2nix, ... }: {
  imports = [
    dream2nix.modules.dream2nix.nodejs-package-json-v3
    dream2nix.modules.dream2nix.nodejs-granular-v3
  ];

  nodejs-granular-v3 = {
    buildScript = ''
      rollup --config
    '';
  };

  name = lib.mkForce "seams-js";
  version = lib.mkForce "3.0.0";

  mkDerivation = {
    src = lib.cleanSourceWith {
      src = ../.;
      filter =
        path: _: # path is of the format /nix/store/hash-whatever/Cargo.toml
        let rootDirName = builtins.elemAt (lib.splitString "/" path) 4;
        in builtins.elem (builtins.trace path rootDirName) [
          "js"
          "lock.json"
          "package-lock.json"
          "package.json"
          "rollup.config.mjs"
          "tsconfig.json"
        ];
    };
  };
}
