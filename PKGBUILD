pkgname=hypr-cycle
giturl="https://github.com/christopherdolan/hypr-cycle.git"
pkgver=0.1.0
pkgrel=1
pkgdesc="A fast and monitor-aware workspace cycler for Hyprland, written in Rust"
arch=('x86_64')
url="${giturl}"
license=('0BSD')
makedepends=('cargo')
source=("${pkgname}::git+${giturl}")
sha256sums=('SKIP')

build() {
    cd "$pkgname"
    cargo build --release
}

package() {
    cd "$pkgname"
    install -Dm755 target/release/hypr-cycle "$pkgdir/usr/bin/hypr-cycle"
}
