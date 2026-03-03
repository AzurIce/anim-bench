{
  description = "Hello world flake using uv2nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";

    pyproject-nix = {
      url = "github:pyproject-nix/pyproject.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    uv2nix = {
      url = "github:pyproject-nix/uv2nix";
      inputs.pyproject-nix.follows = "pyproject-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    pyproject-build-systems = {
      url = "github:pyproject-nix/build-system-pkgs";
      inputs.pyproject-nix.follows = "pyproject-nix";
      inputs.uv2nix.follows = "uv2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        inherit (nixpkgs) lib;
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust-tools = pkgs.rust-bin.nightly."2026-01-01".default.override {
          extensions = [ "rust-src" ];
        };

        # Use Python 3.13 from nixpkgs
        python = pkgs.python313;
      in
      {
        # This example provides two different modes of development:
        # - Impurely using uv to manage virtual environments
        # - Pure development using uv2nix to manage virtual environments
        devShells.default =
          # It is of course perfectly OK to keep using an impure virtualenv workflow and only use uv2nix to build packages.
          # This devShell simply adds Python and undoes the dependency leakage done by Nixpkgs Python infrastructure.
          pkgs.mkShell rec {
            buildInputs = [
              pkgs.cairo
              pkgs.pango
              pkgs.portaudio
            ]
            ++ (
              with pkgs;
              pkgs.lib.optionals pkgs.stdenv.isLinux [
                libGL
                vulkan-loader
                vulkan-headers
                vulkan-tools
                vulkan-tools-lunarg
                vulkan-extension-layer
                vulkan-validation-layers
                openssl
              ]
            );
            packages =
              let
                tex = pkgs.texlive.combine {
                  inherit (pkgs.texlive) scheme-medium standalone preview;
                };
              in
              with pkgs;
              [
                pkg-config
                uv
                ninja
                ffmpeg
                typst
                vulkan-tools
                tex
              ]
              ++ [
                python
                rust-tools
              ];

            env = {
              UV_PYTHON_DOWNLOADS = "never";
              UV_PYTHON = python.interpreter;
            }
            // lib.optionalAttrs pkgs.stdenv.isLinux {
              LD_LIBRARY_PATH = lib.makeLibraryPath pkgs.pythonManylinuxPackages.manylinux1;
            };
            shellHook = ''
              unset PYTHONPATH
              ${lib.optionalString pkgs.stdenv.isLinux ''
                export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${toString (pkgs.lib.makeLibraryPath buildInputs)}"
              ''}
              ${lib.optionalString pkgs.stdenv.isDarwin ''
                export DYLD_LIBRARY_PATH="${toString (pkgs.lib.makeLibraryPath buildInputs)}"
              ''}
            '';
          };
      }
    );
}
