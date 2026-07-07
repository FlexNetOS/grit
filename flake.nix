{
  description = "grit - coordination layer for parallel AI agents on top of git (file::symbol AST git-locks)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        grit = pkgs.rustPlatform.buildRustPackage {
          pname = "grit";
          version = "0.5.0";

          # Only git-tracked sources are copied for a path: flake, so the parent
          # workspace Cargo.toml at ../Cargo.toml is absent in the sandbox and grit
          # builds as a standalone package (no workspace.exclude edit required).
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          # rusqlite (bundled) + the tree-sitter grammars compile vendored C via the
          # `cc` crate (stdenv cc covers it); pkg-config/openssl cover the aws/azure TLS path.
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
          OPENSSL_NO_VENDOR = "1";

          # Tests need a git identity + network (S3/Azure); build the binary only.
          doCheck = false;

          meta = with pkgs.lib; {
            description = "Coordination layer for parallel AI agents on top of git";
            license = licenses.asl20;
            mainProgram = "grit";
            platforms = platforms.unix;
          };
        };
      in
      {
        packages = {
          default = grit;
          inherit grit;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = grit;
          name = "grit";
        };

        overlays.default = final: prev: { inherit grit; };
      });
}
