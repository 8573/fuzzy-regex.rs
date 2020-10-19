let
  nixpkgs-mozilla = (import <nixpkgs> {}).fetchFromGitHub {
    owner = "mozilla";
    repo = "nixpkgs-mozilla";
    # This revision is dated 2020-02-19.
    rev = "e912ed483e980dfb4666ae0ed17845c4220e5e7c";
    sha256 = "08fvzb8w80bkkabc1iyhzd15f4sm7ra10jn32kfch5klgl0gj3j3";
  };

  rust-overlay = "${nixpkgs-mozilla}/rust-overlay.nix";
in

with import <nixpkgs> {
  overlays = [
    (import rust-overlay)
  ];
};

let
  Rust-version-str = "1.44.1";
in

stdenv.mkDerivation rec {
  name = "fuzzy-regex.rs";

  nativeBuildInputs = [
    (rustChannelOf {
      channel = Rust-version-str;
    }).rust
  ];
}
