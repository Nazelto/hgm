{
  description = "Nix Rust 开发环境 (flake-parts 模块化版本)";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
    flake-parts.url = "github:hercules-ci/flake-parts";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
    };

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      self,
      flake-parts,
      crane,
      fenix,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } (
      top@{
        config,
        lib,
        inputs,
        ...
      }:
      {
        systems = [
          "x86_64-linux"
          "aarch64-linux"
          "x86_64-darwin"
          "aarch64-darwin"
        ];

        perSystem =
          {
            config,
            self',
            inputs',
            pkgs,
            system,
            ...
          }:
          let
            pchLib = inputs.pre-commit-hooks.lib.${system};
            toolchain = inputs'.fenix.packages.stable.toolchain;
            craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;
            isProjectInitialized = (builtins.pathExists ./Cargo.toml) && (builtins.pathExists ./Cargo.lock);
            rustConfig =
              if isProjectInitialized then
                import ./nix/rust.nix {
                  inherit
                    config
                    pkgs
                    toolchain
                    craneLib
                    ; # 👈 传递 craneLib
                }
              else
                null;

          in
          {
            _module.args = {
              inherit
                pchLib
                toolchain
                isProjectInitialized
                rustConfig
                ;
            };

            imports = [
              ./nix/packages.nix
              ./nix/devshells.nix
              ./nix/checks.nix
            ];
          };

        flake =
          let
            nixpkgs = inputs.nixpkgs;
          in
          {
            bundlers = nixpkgs.lib.genAttrs [ "x86_64-linux" ] (
              system: import ./nix/bundle.nix { inherit nixpkgs system; }
            );

          };
      }
    );
}
