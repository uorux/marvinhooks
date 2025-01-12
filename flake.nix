{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.nightly.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        myRustBuild = rustPlatform.buildRustPackage {
          pname = "marvinhooks"; # make this what ever your cargo.toml package.name is
          version = "0.1.0";
          src = ./.; # the folder with the cargo.toml
          cargoLock.lockFile = ./Cargo.lock;
          buildInputs = with pkgs; [
            openssl
          ];
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
        };

        dockerImage = pkgs.dockerTools.buildImage {
          name = "marvinhooks";
          tag = "latest";

          copyToRoot = [ pkgs.cacert ];
          config = {
            Cmd = [ "${myRustBuild}/bin/marvinhooks" ];
          };
        };

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            openssl
          ];
          nativeBuildInputs = with pkgs; [
            rust-analyzer
            pkg-config
            (rustVersion.override { extensions = [ "rust-src" ]; })
          ];
        };
        packages = {
          rustPackage = myRustBuild;
          docker = dockerImage;
        };
        defaultPackage = dockerImage;
      }
    );
}
