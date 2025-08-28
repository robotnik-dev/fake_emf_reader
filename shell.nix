let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-25.05";
  pkgs = import nixpkgs {
    config = { };
    overlays = [ ];
  };
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
    gdb
    usbutils
    cargo-binutils
    probe-rs-tools
    minicom
  ];

  shellHook = ''
    if ! rustup target list --installed | grep -q thumbv7em-none-eabihf; then
      rustup target add thumbv7em-none-eabihf
    fi
  '';
}
