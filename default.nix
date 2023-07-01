{ lib
, naersk
, stdenv
, hostPlatform
, targetPlatform
, cargo
, rustc
}:
let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
in
naersk.lib."${targetPlatform.system}".buildPackage rec {
  src = ./.;
  buildInputs = [
    cargo
    rustc
  ];
  cargoBuildOptions = final: final ++ [ "--all-features" ];
  CARGO_BUILD_INCREMENTAL = "false";
  copyLibs = true;
  name = cargoToml.package.name;
  version = cargoToml.package.version;
  meta = with lib; {
    description = cargoToml.package.description;
    homepage = cargoToml.package.homepage;
    license = with licenses; [ mit ];
    maintainers = with maintainers; [ ];
    mainProgram = "tera";
  };
}
