{
  description = "Algorithm-lib dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];

    perSystem = { pkgs, self', ... }: {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo rustc rustfmt rust-analyzer
        ];
      };
    };
  };
}

