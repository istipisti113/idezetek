{
  description = "Syncronised quotes with rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: 
    flake-utils.lib.quotes {
      inherit self nixpkgs;
      packages = {
        default = pkgs: pkgs.rustPlatform.buildRustPackage{
          pname = "quotes";
          version = "0.1.0";
          src=./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      };
    };
    devShells = {
      default = pkgs: pkgs.mkShell {
        buildInputs = [pkgs.rustc pkgs.cargo];
      };
    };
}

