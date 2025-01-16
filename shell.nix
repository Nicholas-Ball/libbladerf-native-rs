{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell{
	buildInputs = with pkgs; [
		gcc
		rustup

		pkg-config
		fontconfig
	];
	LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [
    ]}";
}
