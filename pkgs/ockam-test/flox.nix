{
  # flox environment configuration
  #
  # flox.nix options: https://floxdev.com/docs/reference/flox-nix-config
  # Getting started with flox: https://floxdev.com/docs
  # Get help: https://discourse.floxdev.com
  #
  # Happy hacking!

  # Aliases available when environment is active
  # shell.aliases.cat = "bat";

  # Script run upon environment activation
  # Warning: Be careful when using `${}` in shell hook.
  #          Due to conflicts with Nix language you have to escape it with '' (two single quotes)
  #          Example: ` ''${ENV_VARIABLE} `
  # shell.hook = ''
  # '';

  # Environment variables
  environmentVariables.LANG = "en_US.UTF-8";

  # Packages
  # "version" is optional, otherwise the latest is used. Try `flox search`
  packages.nixpkgs-flox.apacheKafka = {version="2.13-3.3.1";};
  packages.nixpkgs-flox.jq = {};
  packages.nixpkgs-flox.bats = {};
  packages.nixpkgs-flox.nodejs = {};
  packages.nixpkgs-flox.commitlint = {};
  packages.nixpkgs-flox.mold = {};
  packages.nixpkgs-flox.cosign = {};
  packages.nixpkgs-flox.telegraf = {stability="staging"; version="1.26.0";};
  packages.nixpkgs-flox.influxdb2 = {};
  packages.nixpkgs-flox.postgresql_15 = {};
  packages.nixpkgs-flox.pipenv = {};
  packages.nixpkgs-flox.python3Full = {};
  packages.nixpkgs-flox.terragrunt = {};
  packages.nixpkgs-flox.shfmt = {};
  packages.nixpkgs-flox.gradle = {version="7.4.2";};
  packages.nixpkgs-flox.jdk = {version="17.0.3";};
  packages.nixpkgs-flox.rustup = {};
  packages.nixpkgs-flox.cmake = {};
  packages.nixpkgs-flox.eclint = {};
  packages.nixpkgs-flox.elixir = {};
  packages.nixpkgs-flox.erlang = {};
}
