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
      };
      hcctlArgs = "-p hcctl";
      healthchecksArgs = "-p healthchecks";
      monitorArgs = "-p healthchecks-monitor";

      hcctlName = craneLib.crateNameFromCargoToml {
        cargoToml = ./hcctl/Cargo.toml;
      };
      healthchecksName = craneLib.crateNameFromCargoToml {
        cargoToml = ./healthchecks/Cargo.toml;
      };
      monitorName = craneLib.crateNameFromCargoToml {
        cargoToml = ./monitor/Cargo.toml;
      };
      workspaceName = craneLib.crateNameFromCargoToml {cargoToml = ./Cargo.toml;};

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
      hcctl-clippy = craneLib.cargoClippy (commonArgs
        // {
          inherit (hcctlName) pname version;
          inherit cargoArtifacts;
          cargoExtraArgs = hcctlArgs;
        });
      hcctl = craneLib.buildPackage (
        commonArgs
        // {
          inherit (hcctlName) pname version;
          inherit cargoArtifacts;
          cargoExtraArgs = hcctlArgs;
          doCheck = false;
        }
      );
      hcctl-nextest = craneLib.cargoNextest (commonArgs
        // {
          inherit (hcctlName) pname version;
          inherit cargoArtifacts;
          cargoExtraArgs = hcctlArgs;
          partitions = 1;
          partitionType = "count";
        });

      monitor-clippy = craneLib.cargoClippy (commonArgs
        // {
          inherit (monitorName) pname version;
          inherit cargoArtifacts;
          cargoExtraArgs = monitorArgs;
        });
      monitor = craneLib.buildPackage (
        commonArgs
        // {
          inherit (monitorName) pname version;
          inherit cargoArtifacts;
          cargoExtraArgs = monitorArgs;
          doCheck = false;
        }
      );
      monitor-nextest = craneLib.cargoNextest (commonArgs
        // {
          inherit (monitorName) pname version;
          inherit cargoArtifacts;
          cargoExtraArgs = monitorArgs;
          partitions = 1;
          partitionType = "count";
        });

      healthchecks-clippy = craneLib.cargoClippy (commonArgs
        // {
          inherit (healthchecksName) pname version;
          inherit cargoArtifacts;
          cargoExtraArgs = healthchecksArgs;
        });
      healthchecks = craneLib.buildPackage (
        commonArgs
        // {
          inherit (healthchecksName) pname version;
          inherit cargoArtifacts;
          cargoExtraArgs = healthchecksArgs;
          doCheck = false;
        }
      );
      healthchecks-nextest = craneLib.cargoNextest (commonArgs
        // {
          inherit (healthchecksName) pname version;
          inherit cargoArtifacts;
          cargoExtraArgs = healthchecksArgs;
          partitions = 1;
          partitionType = "count";
        });
      healthchecks-msrv = ((crane.mkLib pkgs).overrideToolchain rustMsrv).buildPackage (commonArgs
        // {
          inherit (healthchecksName) version;
          inherit cargoArtifacts;
          pname = "healthchecks-msrv";
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
        inputsFrom = builtins.attrValues self.checks.${system};

        nativeBuildInputs = with pkgs; [
          cargo-nextest
          cargo-release
          nil
          rustStable
        ];

        CARGO_REGISTRIES_CRATES_IO_PROTOCOL = "sparse";
      };
    });
}
