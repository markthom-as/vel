{ nixpkgs ? <nixpkgs> }:

let
  pkgs = import nixpkgs {
    config.allowUnfreePredicate = pkg:
      builtins.elem pkg.pname [
        "obsidian"
      ];
  };
in
pkgs.mkShell {
  packages = with pkgs; [
    cargo
    rustc
    rustfmt
    clippy
    nodejs
    nodePackages.npm
    pkg-config
    openssl
    obsidian
    obsidian-export
    swift
    swiftpm
    python3Packages.huggingface-hub
  ];

  shellHook = ''
    export OPENSSL_DIR="${pkgs.openssl.dev}"
    export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
    export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include"
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig''${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
      pkgs.openssl
      pkgs.swiftPackages.Dispatch
      pkgs.swiftPackages.Foundation
    ]}''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  '';
}
