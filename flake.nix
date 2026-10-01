{
  description = "Lush - Lua Shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{
      crane,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } (top: {
      systems = [ "x86_64-linux" ];
      perSystem =
        {
          self',
          config,
          pkgs,
          lib,
          ...
        }:
        let
          craneLib = crane.mkLib pkgs;
          unfilteredRoot = ./.;
          src = lib.fileset.toSource {
            root = unfilteredRoot;
            fileset = lib.fileset.unions [
              (craneLib.fileset.commonCargoSources unfilteredRoot)
              (lib.fileset.fileFilter (file: file.hasExt "lua") unfilteredRoot)
            ];
          };
          commonArgs = {
            inherit src;
            strictDeps = true;
            buildInputs = [ pkgs.luajit ];
            nativeBuildInputs = [ pkgs.pkg-config ];
            # Common arguments can be set here to avoid repeating them later
            # Note: changes here will rebuild all dependency crates
          };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          lush = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
              # Additional environment variables or build phases/hooks can be set
              # here *without* rebuilding all dependency crates
              # MY_CUSTOM_VAR = "some value";
            }
          );
        in
        {
          checks = {
            inherit lush;
            clippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
              }
            );
            nextest = craneLib.cargoNextest (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoNextestPartitionsExtraArgs = "--no-tests=pass";
              }
            );
          };

          packages.default = lush;

          devShells.default = craneLib.devShell {
            checks = self'.checks;

            # Additional dev-shell environment variables can be set directly
            # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

            # Extra inputs can be added here; cargo and rustc are provided by default.
            packages = [
              pkgs.bacon
            ];
          };
        };
    });
}
