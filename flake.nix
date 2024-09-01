{
  inputs = {
    unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    unstable,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import unstable {
        inherit system;
      };
    in {
      devShell = pkgs.mkShell {
        RUST_BACKTRACE = 1;
        TS_RS_EXPORT_DIR = "../web/src/bindings";

        shellHook = ''
          PATH="$HOME/.rustup/bin:$PATH"
          if ! type -p cargo-watch &>/dev/null; then
            cargo install cargo-watch
          fi
          export APISIX_CONFIG_FILE="$PWD/scripts/apisix_conf.yaml"
        '';
        packages = with pkgs;
          [
            bun
            openssl
            pkg-config
            rustup
            wasm-bindgen-cli
            wasm-pack
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin [
            iconv

            # Required for cargo-watch
            darwin.apple_sdk.frameworks.SystemConfiguration
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.CoreFoundation
            darwin.apple_sdk.frameworks.CoreServices
            darwin.apple_sdk.frameworks.Foundation
            darwin.apple_sdk.frameworks.Cocoa
          ];
      };
    });
}
