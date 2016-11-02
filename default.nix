with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "reql";
  buildInputs = [ gcc openssl gnumake ];
}
