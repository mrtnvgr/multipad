sudo modprobe uinput
export RUST_LOG=DEBUG
nix-shell -p pkg-config udev
