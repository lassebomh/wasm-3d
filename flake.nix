{
  description = "wasm 3d";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          f = with fenix.packages.${system}; combine [
            latest.toolchain
            targets.wasm32-unknown-unknown.latest.rust-std
          ];
          
          startScript = pkgs.writeShellApplication {
            name = "start";
            runtimeInputs = [pkgs.nginx];
            text = ''cargo watch -i .gitignore -i "www/pkg/*" -s "wasm-pack build --no-pack --out-dir ./www/pkg --target web"'';
          };
        in
          {
            devShells.default = 
              pkgs.mkShell {
                name = "wasm-3d";

                systemPackages = with pkgs; [
                  rust-analyzer
                ];

                buildInputs = with pkgs; [
                  bashInteractive
                  nodePackages.pnpm
                  nodejs_20
                ];
                
                packages = with pkgs; [
                  f
                  linuxPackages_latest.perf
                  lldb
                  llvmPackages.bintools
                  nodePackages.typescript-language-server
                  nodejs_21
                  vscode-langservers-extracted
                  wasm-pack
                  startScript
                ];

                CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
              };
          }
      );
}
