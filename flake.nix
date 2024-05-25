{

inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

outputs = { self, nixpkgs }: let
  supportedSystems = nixpkgs.lib.systems.flakeExposed;
  allSystems = output: nixpkgs.lib.genAttrs supportedSystems
    (system: output nixpkgs.legacyPackages.${system});
in {
  packages = allSystems (pkgs: {
    default = pkgs.rustPlatform.buildRustPackage {
      pname = "askmod";
      version = "0.1.0";
      src = self;
      cargoLock.lockFile = ./Cargo.lock;
      nativeBuildInputs = [ pkgs.autoPatchelfHook ];
      buildInputs = [ pkgs.libgcc ];
      runtimeDependencies = with pkgs; [
        wayland
        libxkbcommon
        vulkan-loader
      ];
    };
  });

  devShells = allSystems (pkgs: {
    default = let
      thisPkg = self.packages.${pkgs.system}.default;
    in pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        cargo
        cargo-watch
        rustfmt
        clippy
      ];
      inherit (thisPkg) buildInputs;
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath thisPkg.runtimeDependencies;
      RUST_BACKTRACE = 1;
    };
  });
};

}
