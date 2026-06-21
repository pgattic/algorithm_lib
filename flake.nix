{
  description = "Algorithm-lib dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];

    perSystem = { pkgs, ... }: {
      devShells.default = pkgs.mkShell {
        packages = [
          pkgs.cargo
          pkgs.clippy
          pkgs.rust-analyzer
          pkgs.rustc
          pkgs.rustfmt
        ];
      };
    };
  };
}

