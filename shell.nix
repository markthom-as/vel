{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
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
