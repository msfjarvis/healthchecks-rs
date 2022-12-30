{
  description = "healthchecks-rs monorepo";

  inputs = {
    nixpkgs = {url = "github:NixOS/nixpkgs/nixpkgs-unstable";};
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    flake-utils = {url = "github:numtide/flake-utils";};

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-compat.follows = "flake-compat";
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
      };
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    advisory-db,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      rustStable =
        pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      # Keep in sync with healthchecks/Cargo.toml
      rustMsrv = pkgs.rust-bin.stable."1.64.0".default;

      craneLib = (crane.mkLib pkgs).overrideToolchain rustStable;
      commonArgs = {
        src = ./.;
        buildInputs = [];
        nativeBuildInputs = [];
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      };
      src = ./.;
      hcctlArgs = "-p hcctl";
      healthchecksArgs = "-p healthchecks";
      monitorArgs = "-p healthchecks-monitor";

      audit = craneLib.cargoAudit {
        inherit src advisory-db;
      };
      cargoArtifacts = craneLib.buildDepsOnly (commonArgs
        // {
          pname = "workspace-deps";
        });
      fmt = craneLib.cargoFmt (commonArgs // {
        inherit cargoArtifacts;
      });
      hcctl-clippy = craneLib.cargoClippy (commonArgs
        // {
          pname = "hcctl";
          cargoArtifacts = fmt;
          cargoExtraArgs = hcctlArgs;
        });
      hcctl = craneLib.buildPackage (
        commonArgs
        // {
          pname = "hcctl";
          cargoArtifacts = hcctl-clippy;
          cargoExtraArgs = hcctlArgs;
          doCheck = false;
        }
      );
      hcctl-nextest = craneLib.cargoNextest (commonArgs
        // {
          pname = "hcctl";
          cargoArtifacts = hcctl;
          cargoExtraArgs = hcctlArgs;
          partitions = 1;
          partitionType = "count";
        });

      monitor-clippy = craneLib.cargoClippy (commonArgs
        // {
          pname = "monitor";
          cargoArtifacts = fmt;
          cargoExtraArgs = monitorArgs;
        });
      monitor = craneLib.buildPackage (
        commonArgs
        // {
          pname = "monitor";
          cargoArtifacts = monitor-clippy;
          cargoExtraArgs = monitorArgs;
          doCheck = false;
        }
      );
      monitor-nextest = craneLib.cargoNextest (commonArgs
        // {
          pname = "monitor";
          cargoArtifacts = monitor;
          cargoExtraArgs = monitorArgs;
          partitions = 1;
          partitionType = "count";
        });

      healthchecks-clippy = craneLib.cargoClippy (commonArgs
        // {
          pname = "healthchecks";
          cargoArtifacts = fmt;
          cargoExtraArgs = healthchecksArgs;
        });
      healthchecks = craneLib.buildPackage (
        commonArgs
        // {
          pname = "healthchecks";
          cargoArtifacts = healthchecks-clippy;
          cargoExtraArgs = healthchecksArgs;
          doCheck = false;
        }
      );
      healthchecks-nextest = craneLib.cargoNextest (commonArgs
        // {
          pname = "healthchecks";
          cargoArtifacts = healthchecks;
          cargoExtraArgs = healthchecksArgs;
          partitions = 1;
          partitionType = "count";
        });
      healthchecks-msrv = ((crane.mkLib pkgs).overrideToolchain rustMsrv).buildPackage (commonArgs
        // {
          pname = "healthchecks-msrv";
          cargoArtifacts = healthchecks;
          cargoExtraArgs = healthchecksArgs;
          doCheck = false;
        });
    in {
      checks = {
        inherit
          audit
          fmt
          hcctl
          hcctl-clippy
          hcctl-nextest
          healthchecks
          healthchecks-msrv
          healthchecks-clippy
          healthchecks-nextest
          monitor
          monitor-clippy
          monitor-nextest
          ;
      };

      packages.default = hcctl;

      apps.hcctl = flake-utils.lib.mkApp {drv = hcctl;};
      apps.monitor = flake-utils.lib.mkApp {drv = monitor;};
      apps.default = flake-utils.lib.mkApp {drv = hcctl;};

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;

        nativeBuildInputs = with pkgs; [
          cargo-nextest
          cargo-release
          nil
          rustStable
        ];
      };
    });
}
