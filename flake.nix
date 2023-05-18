{
  description = "projectional";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };


  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # From https://github.com/loophp/rust-shell
        rustInfo = ({
          version,
          profile,
        }: let
          rust = pkgs.rust-bin.${version}.latest.${profile}.override {extensions = ["rust-src"];};
        in {
          name = "rust-" + version + "-" + profile;

          # From https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/11
          path = "${rust}/lib/rustlib/src/rust/library";

          drvs = [
            pkgs.just
            pkgs.openssl
            pkgs.pkgconfig
            pkgs.rust-analyzer
            rust
          ];
        }) {
          version = "stable";
          profile = "default";
        };
      in
      with pkgs;

      {
        devShells.default = mkShell {
          buildInputs = [
            # openssl
            # pkg-config
            rustInfo.drvs
            nodejs-18_x
            entr
            emscripten
            llvmPackages_14.clang
            tree-sitter
            graphviz
          ];
          shellHook = ''
            export PATH=$(pwd)/puddlejumper/target/release:$PATH
          '';
        };
        RUST_SRC_PATH = rustInfo.path;
      }
    );
}
