{
  description = "healthchecks-rs monorepo";

  inputs.nixpkgs.url = "github:msfjarvis/nixpkgs/nixpkgs-unstable";

  inputs.systems.url = "github:msfjarvis/flake-systems";

  inputs.advisory-db.url = "github:rustsec/advisory-db";
  inputs.advisory-db.flake = false;

  inputs.crane.url = "github:ipetkov/crane";

  inputs.devshell.url = "github:numtide/devshell";
  inputs.devshell.inputs.nixpkgs.follows = "nixpkgs";

  inputs.fenix.url = "github:nix-community/fenix/staging";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.flake-utils.inputs.systems.follows = "systems";

  inputs.flake-compat.url = "git+https://git.lix.systems/lix-project/flake-compat";
  inputs.flake-compat.flake = false;

  # Keep in sync with healthchecks/Cargo.toml
  inputs.rust-msrv.url = "https://static.rust-lang.org/dist/channel-rust-1.82.0.toml";
  inputs.rust-msrv.flake = false;

  outputs =
    {
      nixpkgs,
      devshell,
      fenix,
      crane,
      flake-utils,
      advisory-db,
      rust-msrv,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ devshell.overlays.default ];
        };

        rustMsrv = (fenix.packages.${system}.fromManifestFile rust-msrv).minimalToolchain;

        rustStable = (import fenix { inherit pkgs; }).fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-Qxt8XAuaUR2OMdKbN4u8dBJOhSHxS+uS06Wl9+flVEk=";
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustStable;
        commonArgs = {
          src =
            with pkgs.lib.fileset;
            toSource {
              root = ./.;
              fileset = unions [
                (fileFilter (file: file.name == "README.md") ./.)
                (craneLib.fileset.commonCargoSources ./.)
              ];
            };
          buildInputs = [ ];
          nativeBuildInputs = [ ];
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
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

        audit = craneLib.cargoAudit (
          commonArgs
          // {
            inherit advisory-db;
            inherit (workspaceName) pname version;
          }
        );
        cargoArtifacts = craneLib.buildDepsOnly (
          commonArgs
          // {
            inherit (workspaceName) pname version;
          }
        );
        fmt = craneLib.cargoFmt (
          commonArgs
          // {
            inherit (workspaceName) pname version;
          }
        );

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
        workspace-clippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit (healthchecksName) pname version;
            inherit cargoArtifacts;
          }
        );
        workspace-nextest = craneLib.cargoNextest (
          commonArgs
          // {
            inherit (healthchecksName) pname version;
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          }
        );
        healthchecks-msrv = ((crane.mkLib pkgs).overrideToolchain rustMsrv).buildPackage (
          commonArgs
          // {
            inherit (healthchecksName) version;
            pname = "healthchecks-msrv";
            cargoExtraArgs = "-p healthchecks";
            doCheck = false;
          }
        );
      in
      {
        checks = {
          inherit
            audit
            fmt
            workspace
            workspace-clippy
            healthchecks-msrv
            workspace-nextest
            ;
        };

        packages = {
          inherit hcctl monitor;
        };

        apps.hcctl = flake-utils.lib.mkApp { drv = hcctl; };
        apps.monitor = flake-utils.lib.mkApp { drv = monitor; };

        devShells.default = pkgs.devshell.mkShell {
          bash = {
            interactive = "";
          };

          env = [
            {
              name = "DEVSHELL_NO_MOTD";
              value = 1;
            }
          ];

          packages = with pkgs; [
            bacon
            cargo-nextest
            cargo-release
            fenix.packages.${system}.rust-analyzer
            nil
            rustStable
            stdenv.cc
          ];
        };
      }
    );
}
