{ pkgs ? import
    (builtins.fetchTarball {
      url = "https://github.com/NixOS/nixpkgs/archive/317484b1ead87b9c1b8ac5261a8d2dd748a0492d.tar.gz";
      sha256 = "0hnjff59x61f0bxnh33n84jyb1z5l9b859z2ngzsykzcqgg8k8cq";
    })
    { }
}:
with pkgs;
pkgs.mkShell {
  buildInputs = [ pkgs.cargo pkgs.rustc pkgs.rustfmt pkgs.rustup ];
}