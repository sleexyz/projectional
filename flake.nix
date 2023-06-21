{
  description = "projectional";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };

        # From https://github.com/loophp/rust-shell
        rustInfo =
          with pkgs;
          let
            rust = rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
              # From https://gist.github.com/oxalica/310d9a1ba69fd10123f2d70dc6e00f0b
              targets = [ "wasm32-unknown-unknown" ];
            };
          in
          {
            # From https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/11
            path = "${rust}/lib/rustlib/src/rust/library";
            drvs = [
              rust-analyzer
              rust
            ];
          };

        clang-wasi32 = pkgs.writeShellScriptBin "clang-wasi32" ''
          ${pkgs.pkgsCross.wasi32.buildPackages.llvmPackages.clang-unwrapped}/bin/clang $@
        '';

        just-watch = pkgs.writeScriptBin "just-watch" ''
          #!/usr/bin/env bash
          list-files() {
            cat <(git ls-files) <(git ls-files --others --exclude-standard) 
          }
          # NOTE: we cannot use `-r` because it doesn't work with ag.
          list-files | ${pkgs.entr}/bin/entr -cs "${pkgs.just}/bin/just $@"
        '';

      in
      with pkgs;
      {
        devShell = mkShell {
          nativeBuildInputs = [
            # Web stuff:
            nodejs-18_x

            # General dev stuff:
            just
            just-watch
            entr
            silver-searcher

            # Rust:
            rustInfo.drvs
            rustfmt

            # Tree-sitter:
            tree-sitter
            graphviz

            # Wasm cross-compilation:
            wasm-pack
            wasm-bindgen-cli
            lld
            llvmPackages.llvm
            llvmPackages.clang
            clang-wasi32

            # Bazel:
            bazel
            buildifier
            bazel-watcher
            sccache
            rsync
            xorg.lndir
          ];
          shellHook = ''
            export PATH=$(pwd)/puddlejumper/target/release:$PATH
          '';
        };
      }
    );
}
