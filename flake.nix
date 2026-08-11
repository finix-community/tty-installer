{
  description = "finix installer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          libdrm
          mesa
          seatd
          libinput
          libxkbcommon
          fontconfig

          wayland
          wayland-protocols
          libGL
          libx11
          libxcursor
          libxrandr
          libxi
        ];

        systemListsEnv = {
          FINIX_TZDATA_ZONE1970 = "${pkgs.tzdata}/share/zoneinfo/zone1970.tab";
          FINIX_LOCALE_SUPPORTED = "${pkgs.glibcLocales}/share/i18n/SUPPORTED";
          XKB_CONFIG_ROOT = "${pkgs.xkeyboard-config}/share/X11/xkb";
        };
      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;
          packages = [ rustToolchain ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          inherit (systemListsEnv) FINIX_TZDATA_ZONE1970 FINIX_LOCALE_SUPPORTED XKB_CONFIG_ROOT;

          shellHook = ''
            echo "finix-installer dev shell ready"
            echo "  cargo run -p installer-ui # desktop dev (backend-winit), ts buggy but works"
            echo "  cargo test -p installer-core"
          '';
        };

        packages.default = rustPlatform.buildRustPackage {
          pname = "finix-installer";
          version = "0.1.0";
          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = nativeBuildInputs ++ [ pkgs.makeWrapper ];
          inherit buildInputs;

          buildNoDefaultFeatures = true;
          buildFeatures = [ "kms" ];
          cargoBuildFlags = [ "-p" "installer-ui" ];

          postInstall = ''
            wrapProgram $out/bin/installer-ui \
              --set FINIX_TZDATA_ZONE1970 "${systemListsEnv.FINIX_TZDATA_ZONE1970}" \
              --set FINIX_LOCALE_SUPPORTED "${systemListsEnv.FINIX_LOCALE_SUPPORTED}" \
              --set XKB_CONFIG_ROOT "${systemListsEnv.XKB_CONFIG_ROOT}"
          '';

          meta = {
            description = "finix OS installer";
            mainProgram = "installer-ui";
          };
        };
      });
}
