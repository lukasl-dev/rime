{
  description = "rime: MCP server (Rust) with Nix flake packaging";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";

    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs =
    {
      self,
      nixpkgs,
      devenv,
      systems,
      crane,
      rust-overlay,
    }@inputs:
    let
      pname = "rime";
      version = builtins.readFile ./VERSION;

      mkPkgs =
        system:
        import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

      mkRime =
        pkgs:
        let
          rustToolchain = pkgs.rust-bin.nightly.latest.default;
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          commonArgs = { inherit src pname version; };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        in
        craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });

      overlay = final: prev: {
        rime = mkRime (mkPkgs prev.stdenv.hostPlatform.system);
      };

      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = forEachSystem (system: rec {
        default = mkRime (mkPkgs system);
        rime = default;
      });

      apps = forEachSystem (system: rec {
        default = {
          program = "${mkRime (mkPkgs system)}/bin/rime";
          type = "app";
        };
        rime = default;
      });

      overlays.default = overlay;

      devShells = forEachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [
              (
                { pkgs, ... }:
                {
                  packages = [ pkgs.pkg-config ];
                  languages.rust = {
                    enable = true;
                    channel = "nightly";
                    targets = [
                      "x86_64-unknown-linux-musl"
                      "x86_64-unknown-linux-gnu"
                    ];
                  };
                }
              )
            ];
          };
        }
      );
    };
}
