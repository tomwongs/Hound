pkgname='hound'
pkgver='2.2.0'
pkgrel='2'
arch=('any')
pkgdesc='Hound is a Linux package made to destroy sensible datas, with no chances of recovery.'
depends=('rust')
source=('hound.rs')
sha256sums=(bf039be3f868dea3f3207be5b682f8c757080ac3a43e46cd590ccb2938524d38)
package() {
  sudo rustc "${srcdir}/hound.rs" -o "${srcdir}/hound"

  mkdir -p "${pkgdir}/usr/bin"
  sudo mv "${srcdir}/hound" "${pkgdir}/usr/bin"

  echo 'Hound is in the system!'
}
