{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = [ 
      pkgs.buildPackages.elixir 
      pkgs.buildPackages.erlang 
      pkgs.buildPackages.rebar3
   ];
}