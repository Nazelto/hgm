{ nixpkgs, system, ... }:
{
  toDockerImage =
    drv:
    nixpkgs.legacyPackages.${system}.dockerTools.buildLayeredImage {
      name = drv.pname or drv.name or "image";
      tag = "latest";
      contents =
        if drv ? outPath then
          drv
        else
          throw "provided installable is not a derivation and not coercible to an outPath";
      config = {
        Cmd = [ "/bin/${drv.pname}" ];
      };
    };
}
