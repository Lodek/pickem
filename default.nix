let
  pkgs = import <nixpkgs> { };
in
{
  pickem = pkgs.callPackage ./pickem.nix { };
}
