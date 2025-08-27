{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.quotesApp =
      let
        pkgs = import nixpkgs {system = "x86_64-linux";};
      in pkgs.rustPlatform.buildRustPackage {
        pname = "asdsf";
        version = "0.1.0";
        src=./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
  packages.x86_64-linux.default = self.packages.x86_64-linux.quotesApp;
  };
}
