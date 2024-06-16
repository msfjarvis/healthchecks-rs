{
  description = "healthchecks-rs monorepo";

  inputs.nixpkgs.url = "github:msfjarvis/nixpkgs/nixpkgs-unstable";

  inputs.systems.url = "github:msfjarvis/flake-systems";

  inputs.advisory-db.url = "github:rustsec/advisory-db";
  inputs.advisory-db.flake = false;

  inputs.crane.url = "github:ipetkov/crane";
  inputs.crane.inputs.nixpkgs.follows = "nixpkgs";

  inputs.devshell.url = "github:numtide/devshell";
  inputs.devshell.inputs.nixpkgs.follows = "nixpkgs";
  inputs.devshell.inputs.flake-utils.follows = "flake-utils";

  inputs.fenix.url = "github:nix-community/fenix/staging";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.flake-utils.inputs.systems.follows = "systems";

  inputs.flake-compat.url = "github:nix-community/flake-compat";
  inputs.flake-compat.flake = false;

  # Keep in sync with healthchecks/Cargo.toml
  inputs.rust-msrv.url = "https://static.rust-lang.org/dist/channel-rust-1.67.0.toml";
  inputs.rust-msrv.flake = false;

  outputs = {
    nixpkgs,
    devshell,
    fenix,
    crane,
    flake-utils,
    advisory-db,
    rust-msrv,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [devshell.overlays.default];
      };

      rustStable = (import fenix {inherit pkgs;}).fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-Ngiz76YP4HTY75GGdH2P+APE/DEIx2R/Dn+BwwOyzZU=";
      };
      rustMsrv = (fenix.packages.${system}.fromManifestFile rust-msrv).minimalToolchain;

      craneLib = (crane.mkLib pkgs).overrideToolchain rustStable;
      markdownFilter = path: _type: builtins.match ".*md$" path != null;
      markdownOrCargo = path: type:
        (markdownFilter path type) || (craneLib.filterCargoSources path type);

      commonArgs = {
        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = markdownOrCargo;
        };
        buildInputs = [];
        nativeBuildInputs = [];
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        # https://github.com/ipetkov/crane/issues/312
        extraDummyScript = "rm -f $(find $out | grep bin/crane-dummy/main.rs)";
      };

      hcctlName = craneLib.crateNameFromCargoToml {
        cargoToml = ./hcctl/Cargo.toml;
      };
      healthchecksName = craneLib.crateNameFromCargoToml {
        cargoToml = ./healthchecks/Cargo.toml;
      };
      monitorName = craneLib.crateNameFromCargoToml {
        cargoToml = ./monitor/Cargo.toml;
      };
      workspaceName = {
        version = "1.0.0";
        pname = "healthchecks-rs";
      };

      audit = craneLib.cargoAudit (commonArgs
        // {
          inherit advisory-db;
          inherit (workspaceName) pname version;
        });
      cargoArtifacts = craneLib.buildDepsOnly (commonArgs
        // {
          inherit (workspaceName) pname version;
        });
      fmt = craneLib.cargoFmt (commonArgs
        // {
          inherit (workspaceName) pname version;
        });

      hcctl = craneLib.buildPackage (
        commonArgs
        // {
          inherit (hcctlName) pname version;
          inherit cargoArtifacts;
          cargoExtraArgs = "-p hcctl";
          doCheck = false;
        }
      );

      monitor = craneLib.buildPackage (
        commonArgs
        // {
          inherit (monitorName) pname version;
          inherit cargoArtifacts;
          cargoExtraArgs = "-p healthchecks-monitor";
          doCheck = false;
        }
      );

      workspace = craneLib.buildPackage (
        commonArgs
        // {
          inherit (healthchecksName) pname version;
          inherit cargoArtifacts;
          doCheck = false;
        }
      );
      workspace-clippy = craneLib.cargoClippy (commonArgs
        // {
          inherit (healthchecksName) pname version;
          inherit cargoArtifacts;
        });
      workspace-nextest = craneLib.cargoNextest (commonArgs
        // {
          inherit (healthchecksName) pname version;
          inherit cargoArtifacts;
          partitions = 1;
          partitionType = "count";
        });
      healthchecks-msrv = ((crane.mkLib pkgs).overrideToolchain rustMsrv).buildPackage (commonArgs
        // {
          inherit (healthchecksName) version;
          pname = "healthchecks-msrv";
          cargoExtraArgs = "-p healthchecks";
          doCheck = false;
        });
    in {
      checks = {
        inherit audit fmt workspace workspace-clippy healthchecks-msrv workspace-nextest;
      };

      packages = {
        inherit hcctl monitor;
      };

      apps.hcctl = flake-utils.lib.mkApp {drv = hcctl;};
      apps.monitor = flake-utils.lib.mkApp {drv = monitor;};

      devShells.default = pkgs.devshell.mkShell {
        bash = {interactive = "";};

        env = [
          {
            name = "DEVSHELL_NO_MOTD";
            value = 1;
          }
        ];

        packages = with pkgs; [
          cargo-nextest
          cargo-release
          cargo-semver-checks
          fenix.packages.${system}.rust-analyzer
          nil
          rustStable
          stdenv.cc
        ];
      };
    });
}
