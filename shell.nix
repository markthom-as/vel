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
    obsidian
    obsidian-export
    swift
    swiftpm
    python3Packages.huggingface-hub
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
      pkgs.swiftPackages.Dispatch
      pkgs.swiftPackages.Foundation
    ]}''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  '';
}
