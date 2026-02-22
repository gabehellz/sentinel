{ pkgs, ... }:

{
  dotenv.enable = true;
  
  languages.rust = {
    enable = true;
    toolchainFile = ./rust-toolchain.toml;
  };

  git-hooks.hooks = {
    rustfmt = {
      enable = true;
      settings.config-path = "./rustfmt.toml";
    };
  };
}
