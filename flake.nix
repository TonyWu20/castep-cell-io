{
  description = "rust environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devshell.url = "github:numtide/devshell";
    rushi-config = {
      url = "git+ssh://git@github.com/TonyWu20/rushi-config";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.fenix.follows = "fenix";
    };
  };
  outputs =
    {
      nixpkgs,
      fenix,
      devshell,
      rushi-config,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-darwin"
      ];
      pkgsFor =
        system:
        import nixpkgs {
          inherit system;
          overlays = [
            fenix.overlays.default
            devshell.overlays.default
          ];
        };

      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = pkgsFor system;
          rushi = rushi-config.packages.${system}.default;
          rushi-tui = rushi-config.packages.${system}.rushi-tui;
        in
        {
          default = pkgs.devshell.mkShell {
            packages =
              with pkgs;
              [
                (fenix.packages.${system}.stable.withComponents [
                  "cargo"
                  "clippy"
                  "rust-src"
                  "rustc"
                  "rustfmt"
                  "rust-analyzer"
                ])
                stdenv
                fish
                (python313.withPackages (
                  ps: with ps; [
                    beautifulsoup4
                    requests
                    pynvim
                  ]
                ))
                uv
                rushi
                rushi-tui
              ]
              ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
                pkgs.libiconv
              ];
            env = pkgs.lib.optionals pkgs.stdenv.isDarwin [
              {
                name = "RUSTFLAGS";
                value = "-C link-arg=-L${pkgs.libiconv}/lib";
              }
            ];
          };
        }
      );
    };
}
