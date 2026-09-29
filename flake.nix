{
  description = "CarX DRO2 calc(Rust + WASM)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        Graduate = with pkgs; [
          rustc
          cargo
          wasm-pack
          python3
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = Graduate;
          shellHook = ''
            echo "=== Добро пожаловать в nix develop ==="
            echo "Rust, cargo и wasm-pack готовы к работе."
            echo "Для сборки используй: wasm-pack build --target web"
            echo "Для теста локально используй: python3 -m http.server"
          '';
        };
      }
    );
}
