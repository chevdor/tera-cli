{
  description = "A command line utility written in Rust to render templates from json|toml|yaml && ENV, using the tera templating engine";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = { self, nixpkgs, naersk }:
    let
      cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
    in
    {
      overlay = final: prev: {
        "${cargoToml.package.name}" = final.callPackage ./. { inherit naersk; };
      };
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              self.overlay
            ];
          };
        in
        {
          "${cargoToml.package.name}" = pkgs."${cargoToml.package.name}";
        });
      defaultPackage = forAllSystems (system: (import nixpkgs {
        inherit system;
        overlays = [ self.overlay ];
      })."${cargoToml.package.name}");
      devShell = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ self.overlay ];
          };
        in
        pkgs.mkShell {
          inputsFrom = with pkgs; [
            pkgs."${cargoToml.package.name}"
          ];
        });
    };
}
