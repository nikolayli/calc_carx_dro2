{
  description = "Окружение разработки для CarX DRO2 калькулятора (Rust + WASM)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        dependencies = with pkgs; [
          rustc
          cargo
          wasm-pack
          python3
          llvm_18 # Добавили линковщик lld, без которого падает WASM-сборка
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = dependencies;

          # Прописываем пути к библиотекам, чтобы линковщик видел всё необходимое внутри Nix-окружения
          LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib";

          shellHook = ''
            echo "=== Добро пожаловать в nix develop ==="
            echo "Линковщик LLD и зависимости WASM успешно подключены."
            echo "Для сборки используй: wasm-pack build --target web"
            echo "Для теста локально используй: python3 -m http.server"
          '';
        };
      }
    );
}
