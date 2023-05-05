# Maintainer: Samuel Sehnert 
pkgname='pactag-git'
pkgver=0.1
pkgrel=1
pkgdesc="A small command line tool to give the pacman package manager a tag feature that can be used to mark packages"
arch=('any')
url="https://github.com/SamuelSehnert/pactag"
license=('GPL3')
makedepends=(cargo)
sha512sums=('SKIP')

source=("git+$url")

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/pactag"
}
