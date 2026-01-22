{
  config,
  pkgs,
  craneLib,
  ...
}:

let
  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
  commonArgs = {
    pname = cargoToml.package.name;
    src = craneLib.cleanCargoSource ../.;
    buildInputs = [ ];
    cargoVendorDir = craneLib.vendorCargoDeps {
      cargoLock = ../Cargo.lock;
      cargoToml = ../Cargo.toml;
    };
    version = cargoToml.package.version;
  };
  # windows 交叉编译配置
  windowsTarget = "x86_64-windows-gnu";
  # windows 交叉编译工具链
  windowsPkgs = pkgs.pkgsCross.mingW64;

  windowsCommonArgs = commonArgs // {
    strictDeps = true;
    doCheck = false;
    CARGO_BUILD_TARGET = windowsTarget;
    # 设置交叉编译环境变量
    CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "${windowsPkgs.stdenv.cc.targetPrefix}cc";

    depsBuildBuild = with pkgs; [
      windowsPkgs.stdenv.cc
      windowsPkgs.windows.pthreads
    ];

    buildInputs = with windowsPkgs; [
      windows.pthreads
    ];
  };
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  packages = {
    default = craneLib.buildPackage (
      commonArgs
      // {
        inherit cargoArtifacts;
      }

    );
    x86_64-windows = craneLib.buildPackage (
      windowsCommonArgs
      // {
        cargoArtifacts = craneLib.buildDepsOnly windowsCommonArgs;
        postInstall = ''
          # 确保输出文件有 .exe文件
          for file in $out/bin/*; do
            if [[ ! "$file" == *.exe ]]; then
              mv "$file" "$file.exe"
            fi
          done
        '';
      }
    );
  };

in
{
  # 暴露给其他模块使用
  inherit commonArgs packages craneLib;
}
