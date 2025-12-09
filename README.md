# 🦀 Flutter CLI (Rust Edition)

**Flutter CLI**, yeni Flutter projelerinizi saniyeler içinde oluşturmanızı, mimari yapılarını (Clean Architecture veya MVVM) otomatik kurmanızı ve favori editörünüzde başlatmanızı sağlayan Rust tabanlı yüksek performanslı bir araçtır.

## 🚀 Neden Bu Aracı Kullanmalısınız?

* **⚡ Hız:** Rust ile yazıldığı için anında açılır ve işlem yapar.
* **🏗️ Mimari Desteği:** Klasörleri tek tek açmakla uğraşmayın. Clean Architecture veya MVVM yapısını otomatik kurar.
* **🤖 IDE Entegrasyonu:** Proje oluştuktan sonra VS Code veya Cursor gibi editörleri otomatik başlatır.
* **🪟 Cross-Platform:** Windows ve macOS/Linux üzerinde sorunsuz çalışır (Windows pencere yönetimi dahil).

## 📦 Kurulum

Bu aracı kullanmak için bilgisayarınızda Rust yüklü olmalıdır.

1.  **Repoyu klonlayın:**
2.  **Aracı derleyin ve kurun:**
    cargo install --path .
3.  **Kurulumu doğrulayın:**
    flutter_cli --help
## 💻 Kullanım
Aracı iki farklı şekilde kullanabilirsiniz: **İnteraktif Mod** veya **Argüman Modu**.

### 1. İnteraktif Mod (Önerilen)
Sadece komutu çalıştırın ve soruları cevaplayın:
flutter_cli
### 2.Argüman Modu
flutter_cli --name my_awesome_app --arch clean
