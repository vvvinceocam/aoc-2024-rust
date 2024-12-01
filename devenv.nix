{ pkgs, config, lib, ... }:

{
  packages = with pkgs; [
    cargo-nextest bacon openssl
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };

  pre-commit.hooks = {
    cargo-check.enable = true;
    clippy.enable = true;
    markdownlint = {
      enable = true;
      settings.configuration = {
         MD033 = false;
         MD013 = {
            line_length = 120;
         };
         MD041 = false;
      };
    };
    rustfmt.enable = true;
    rust-tests = {
      enable = true;
      name = "Rust unit tests";
      entry = "cargo nextest run";
      files = "\\.rs$";
    };
  };

  enterShell = ''
    export CARGO_HOME="$PWD/.cargo"

    # Link the rust stdlib sources to a defined path to ease IDEs integration
    ln -sfT "$RUST_SRC_PATH" "$PWD/.rust-src"

    echo
    echo 💡 Helper scripts to ease development process:
    echo
    ${pkgs.gnused}/bin/sed -e 's| |••|g' -e 's|=| |' <<EOF | ${pkgs.util-linuxMinimal}/bin/column -t | ${pkgs.gnused}/bin/sed -e 's|^|• |' -e 's|••| |g'
    ${lib.generators.toKeyValue {} (lib.mapAttrs (name: value: value.description) config.scripts)}
    EOF
    echo
  '';
}
