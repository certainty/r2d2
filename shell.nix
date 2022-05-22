{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    buildInputs = [ 
      pkgs.buildPackages.ruby_3_1 
    ];
}
