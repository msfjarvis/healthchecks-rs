with import <nixpkgs> { };
mkShell { buildInputs = [ cargo pkgconfig openssl ]; }
