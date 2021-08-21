with import <nixpkgs> { };
mkShell { buildInputs = [ clang_11 lld_11 cargo rustc pkgconfig openssl ]; }
