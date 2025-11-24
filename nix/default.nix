{
    rustPlatform,
    pkgs
}:



rustPlatform.buildRustPackage {
    name = "autismal";
    src = ../.;
    buildInputs = with pkgs; [ openssl ];
    nativeBuildInputs = with pkgs; [ pkgs.pkg-config ];
    cargoLock.lockFile = ../Cargo.lock;
    # env.OPENSSL_NO_VENDOR = "1";
}


