{
  description = "autismal dev shell and package";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, utils, rust-overlay, ... }: let
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs {
      system = "x86_64-linux";
      inherit overlays;
    };
    autismal = pkgs.callPackage ./nix/default.nix { };
  in {
    packages.x86_64-linux = {
      default = autismal;
    };

    dockerImage = pkgs.dockerTools.buildImage {
      name = "autismal";
      tag = "latest";

      config = {
        Cmd = [ "./autismal" ];
        WorkingDir = "/bin";
      };

      copyToRoot = pkgs.buildEnv {
        name = "autismal-docker-root";
        paths = [ autismal ];
        pathsToLink = [ "/bin" ];
      };
    };

    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        pkg-config
        openssl

        (rust-bin.stable.latest.default.override {
          extensions = [
            "clippy"
            "rust-src"
            "rust-analyzer"
          ];
          targets = [ "x86_64-unknown-linux-gnu" ];
        })
      ];
    };
  };
}