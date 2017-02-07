with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "reql";
  buildInputs = [ gcc openssl gnumake ];
  CARGO_INCREMENTAL = 1;
}
