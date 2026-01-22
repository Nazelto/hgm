{
  pchLib,
  system,
  isProjectInitialized,
  rustConfig,
  ...
}:
{
  checks = {
    pre-commit-checks =
      if isProjectInitialized then
        pchLib.run {
          # 使用 rustConfig 中的 commonArgs.src
          inherit (rustConfig.commonArgs) src;

          hooks = {
            # Rust 格式化
            rustfmt.enable = true;

            # Rust 静态分析
            clippy = {
              enable = true;
              # 可选：自定义 clippy 参数
              # settings.denyWarnings = true;
            };

            # 可选：检查 Cargo.toml 格式
            cargo-check.enable = true;
          };
        }
      else
        # 项目未初始化时返回空 derivation
        null;
  };
}
