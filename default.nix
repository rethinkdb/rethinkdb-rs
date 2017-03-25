with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "reql";
  buildInputs = [ gcc openssl gnumake inotify-tools ];
  CARGO_INCREMENTAL = 1;
  OPENSSL_DIR = "${openssl.dev}";
  OPENSSL_LIB_DIR = "${openssl.out}/lib";
}
