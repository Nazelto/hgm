{
  config,
  pkgs,
  isProjectInitialized,
  rustConfig,
  ...
}:

let
  not_rust_project = pkgs.runcommand "rust-project-not-initialized" { } ''
    echo ""
    echo "❌ rust 项目尚未初始化"
    echo ""
    echo "💡 请执行以下步骤:"
    echo "   1. cargo init"
    echo "   2. cargo generate-lockfile"
    echo "   3. git add cargo.toml cargo.lock"
    echo "   4. direnv reload"
    echo ""
    exit 1
  '';

in
{
  packages = {
    default = if isProjectInitialized then rustConfig.packages.default else not_rust_project;

  };
}
