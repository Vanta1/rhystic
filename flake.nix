{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    crane.url = "github:ipetkov/crane";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {nixpkgs, ...} @ inputs: let
    name = "rhystic";
    version = "0.0.1";
  in
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        system,
        stdenv,
        ...
      }: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = with nixpkgs;
          lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              (craneLib.fileset.commonCargoSources ./.)
              (lib.fileset.maybeMissing ./resources)
            ];
          };

        nativeBuildInputs = with pkgs; [rustToolchain pkg-config];
        buildInputs = [
          # extra dependencies here
        ];
        commonArgs = {
          inherit src buildInputs nativeBuildInputs;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        bin = craneLib.buildPackage (commonArgs
          // {
            inherit cargoArtifacts;
            pname = "${name}";
            version = "${version}";
          });
      in {
        devShells.default = pkgs.mkShell {
          inputsFrom = [bin];
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          shellHook = ''
            echo "entering ${name} devshell..."
          '';
        };
        packages = {
          inherit bin;
          default = bin;
        };
      };
    };
}
