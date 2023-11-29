pkgname='hound'
pkgver='2.2.0'
pkgrel='2'
arch=('any')
pkgdesc='Hound is a Linux package made to destroy sensible datas, with no chances of recovery.'
depends=('rust')
source=('hound.rs')
sha256sums=(376702fa8b41cf43d2507e2b82ef5a1769a0169d02b46ae3521b2c9d2c71cf33)
package() {
  sudo rustc "${srcdir}/hound.rs" -o "${srcdir}/hound"

  mkdir -p "${pkgdir}/usr/bin"
  sudo mv "${srcdir}/hound" "${pkgdir}/usr/bin"

  echo 'Hound is in the system!'
}
