# Maintainer: dlurak
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=ccrs-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="CCRS allows you to easily convert colors from one format into another"
url="https://github.com/Dlurak/ccrs"
license=("mit")
arch=("x86_64")
provides=("ccrs")
conflicts=("ccrs")
source=("https://github.com/Dlurak/ccrs/releases/download/v$pkgver/ccrs-$pkgver-x86_64.tar.gz")
sha256sums=("875af2b9f896dff90107881489b88842fc22e6b793d3512ebe6911e9b922fabe")

package() {
    install -Dm755 ccrs -t "$pkgdir/usr/bin"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}