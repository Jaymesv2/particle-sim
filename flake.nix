{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    #rust-flake.url = "github:juspay/rust-flake/c1b660962d211910cfbe3429bccb6a4dd5d479e3";
    #rust-flake.inputs.nixpkgs.follows = "nixpkgs";
    #process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    #cargo-doc-live.url = "github:srid/cargo-doc-live";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs = { nixpkgs.follows = "nixpkgs"; };

    devenv.url = "github:cachix/devenv";

    # Dev tools
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.devenv.flakeModule
      ];
      perSystem = { config, self', pkgs, lib, ... }: {
        treefmt.config = {
          projectRootFile = "flake.nix";
          programs = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
        };
        devenv.shells.default = rec {
          languages.rust = {
            enable = true;
            #mold.enable = true;
            targets = [ "wasm32-unknown-unknown" ];
            channel = "stable";
          };
          languages.javascript.enable = true;
          packages = with pkgs; [ 
            cargo-watch 
            mold 
            openssl
            trunk

            # GUI libs
            libxkbcommon
            libGL
            fontconfig

            # wayland libraries
            wayland

            # x11 libraries
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11
          ];

          env = {
          #   RUSTFLAGS = lib.mkForce "-Clink-arg=-fuse-ld=${pkgs.mold}/bin/mold";
                    LD_LIBRARY_PATH = "${lib.makeLibraryPath packages}";
          };

          dotenv.enable = true;

          # pre-commit.hooks = {
          #   # lint shell scripts
          #   shellcheck.enable = true;

          #   clippy.enable = true;
          #   clippy.packageOverrides.cargo = pkgs.cargo;
          #   clippy.packageOverrides.clippy = pkgs.clippy;
          #   # some hooks provide settings
          #   clippy.settings.allFeatures = true;
          # };
        };

      };
    };

}
