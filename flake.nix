{
  description = "healthchecks-rs monorepo";

  inputs = {
    nixpkgs = {url = "github:NixOS/nixpkgs/nixpkgs-unstable";};

    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    flake-utils = {url = "github:numtide/flake-utils";};

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-compat.follows = "flake-compat";
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    # Keep in sync with healthchecks/Cargo.toml
    rust-msrv = {
      url = "https://static.rust-lang.org/dist/channel-rust-1.64.0.toml";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    crane,
    flake-utils,
    advisory-db,
    rust-msrv,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};

      rustStable = (import fenix {inherit pkgs;}).fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-gdYqng0y9iHYzYPAdkC/ka3DRny3La/S5G8ASj0Ayyc=";
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
