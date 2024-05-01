#! /bin/bash

function die {
    echo "$1"
    exit 1
}


TARGETS=( \
'x86_64-apple-darwin' \
'x86_64-pc-windows-gnu' \
'x86_64-unknown-linux-gnu' \
'x86_64-unknown-linux-musl' \
'x86_64-unknown-freebsd' \
'aarch64-apple-darwin' \
'aarch64-unknown-linux-gnu' \
'i686-pc-windows-gnu' \
'i686-unknown-linux-gnu' \
'i686-unknown-linux-gnu' \
'i686-unknown-freebsd' \
)

export RUSTFLAGS="-D warnings"

echo "======================================"
echo "Checking targets installed"
echo "======================================"

TARGETS_NEEDED=0

for TARGET in "${TARGETS[@]}"
do
    if ! rustup target list --installed | grep "$TARGET"; then
        echo "Target $TARGET not installed. Please use 'rustup target add $TARGET'."
        export TARGETS_NEEDED=1
    fi
done

if [ "$TARGETS_NEEDED" -eq "1" ]; then
    exit 1
fi

echo "======================================"
echo "Checking native target"
echo "======================================"

# Local target first
cargo check  || die "Native target failed cargo check."
cargo clippy || die "Native target failed cargo clippy."

# Target list can be found in https://doc.rust-lang.org/rustc/platform-support.html
#
# Targets can be installed with `rustup target add $TARGET`


for TARGET in "${TARGETS[@]}"
do
    echo "======================================"
    echo "Checking $TARGET"
    echo "======================================"

    cargo check --target "$TARGET"  || die "Target $TARGET failed cargo check."
    cargo clippy --target "$TARGET" || die "Target $TARGET failed cargo clippy."
done

echo "======================================"
echo "Checking cargo fmt"
echo "======================================"

cargo fmt --check > /dev/null 2> /dev/null || die "Please run 'cargo fmt'"


echo "======================================"
echo "Completed: ALL OK!"
echo "======================================"

