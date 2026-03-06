{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        tex = pkgs.texlive.combine {
          inherit (pkgs.texlive)
            babel
            background
            changepage
            chktex
            cm-unicode
            collection-fontsrecommended
            collection-langcyrillic
            enumitem
            environ
            etoolbox
            euenc
            everypage
            fancyhdr
            filehook
            fontspec
            forloop
            lh
            lm
            framed
            geometry
            graphics
            hyperref
            ifmtarg
            inter
            koma-script
            lastpage
            oberdiek
            paralist
            paratype
            parskip
            pbox
            pgf
            ragged2e
            scheme-medium
            setspace
            sourcesanspro
            t2
            tcolorbox
            titlesec
            tools
            trimspaces
            ucharcat
            unicode-math
            upquote
            url
            xcolor
            xifthen
            xkeyval
            xunicode
            ;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            tex
          ];
          shellHook = ''
            echo "Welcome to the devShell!"
          '';

          packages = [ pkgs.bashInteractive ];
        };
      }
    );
}
