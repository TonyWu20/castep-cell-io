{
  description = "rust environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devshell.url = "github:numtide/devshell";
  };
  outputs = { nixpkgs, fenix, devshell, ... }:
    let
      systems = [ "x86_64-linux" "aarch64-darwin" ];
      pkgsFor = system: import nixpkgs { inherit system; overlays = [ fenix.overlays.default devshell.overlays.default ]; };

      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = pkgsFor system;
        in
        {
          default = pkgs.devshell.mkShell
            {
              packages = with pkgs; [
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
                (python313.withPackages (ps: with ps; [
                  beautifulsoup4
                  requests
                  pynvim
                ]))
                uv
              ];
              commands = [
                {
                  name = "claude-deepseek";
                  command = ''
                    ANTHROPIC_BASE_URL=$DEEPSEEK_BASE_URL \
                    ANTHROPIC_AUTH_TOKEN=$DEEPSEEK_TOKEN \
                    CLAUDE_CODE_ATTRIBUTION_HEADER="0" \
                    CLAUDE_CODE_EFFORT_LEVEL=max \
                    ANTHROPIC_DEFAULT_OPUS_MODEL=deepseek-v4-pro \
                    ANTHROPIC_DEFAULT_SONNET_MODEL=deepseek-v4-flash \
                    ANTHROPIC_DEFAULT_HAIKU_MODEL=deepseek-v4-flash \
                    claude --model "opusplan [1m]"
                  '';
                }
              ];
            };
        }
      );
    };
}
