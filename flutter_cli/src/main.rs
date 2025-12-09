use clap::{Parser, ValueEnum};
use colored::*;
use inquire::{Select, Text};
use std::fs;
use std::process::{Command, Stdio};
use std::fmt;

// Windows'ta penceresiz işlem başlatmak için gerekli kütüphane
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

// --- CLI Argüman ve Enum Tanımları ---

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Proje ismi
    #[arg(short, long)]
    name: Option<String>,

    /// Mimari türü
    #[arg(short, long, value_enum)]
    arch: Option<Architecture>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Architecture {
    Clean,
    Mvvm,
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Architecture::Clean => write!(f, "Clean Architecture"),
            Architecture::Mvvm => write!(f, "MVVM"),
        }
    }
}

// IDE Seçenekleri (Güncellendi)
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum IdeOption {
    VsCode,
    Cursor,
    Custom, // <-- Yeni eklenen seçenek
    None,
}

impl fmt::Display for IdeOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IdeOption::VsCode => write!(f, "VS Code"),
            IdeOption::Cursor => write!(f, "Cursor"),
            IdeOption::Custom => write!(f, "Diğer (Komut Gir)"), // <-- Kullanıcıya görünecek metin
            IdeOption::None => write!(f, "Hiçbiri"),
        }
    }
}

// --- Ana Fonksiyon ---

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // 1. ADIM: Proje İsmini Al
    let project_name = match args.name {
        Some(name) => name,
        None => Text::new("Proje ismi nedir?").prompt()?,
    };

    // 2. ADIM: Mimariyi Seç
    let selected_arch = match args.arch {
        Some(arch) => arch,
        None => {
            let options = vec![Architecture::Clean, Architecture::Mvvm];
            Select::new("Hangi mimariyi kullanmak istersin?", options).prompt()?
        }
    };

    // 3. ADIM: IDE Seç
    let selected_ide_option = Select::new(
        "Projeyi oluşturduktan sonra nerede açmak istersiniz?",
        vec![IdeOption::VsCode, IdeOption::Cursor, IdeOption::Custom, IdeOption::None],
    )
    .prompt()?;

    // Eğer "Diğer" seçildiyse komutu sor, yoksa seçileni al
    let ide_command = match selected_ide_option {
        IdeOption::VsCode => Some("code".to_string()),
        IdeOption::Cursor => Some("cursor".to_string()),
        IdeOption::Custom => {
            let cmd = Text::new("Editör komutunu giriniz (örn: nvim, subl, atom):").prompt()?;
            if cmd.trim().is_empty() {
                None 
            } else {
                Some(cmd)
            }
        },
        IdeOption::None => None,
    };

    println!(
        "{}",
        format!(
            "🚀 {} projesi {} ile hazırlanıyor...",
            project_name,
            selected_arch.to_string()
        )
        .green()
        .bold()
    );

    // 4. ADIM: Flutter Create Komutunu Çalıştır
    let flutter_cmd = if cfg!(target_os = "windows") {
        "flutter.bat"
    } else {
        "flutter"
    };

    let status = Command::new(flutter_cmd)
        .arg("create")
        .arg(&project_name)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Flutter komutu çalıştırılamadı. Flutter yüklü mü?");

    if !status.success() {
        println!("{}", "Flutter projesi oluşturulurken hata oluştu!".red());
        return Ok(());
    }

    // 5. ADIM: Klasör Yapısını Oluştur
    let base_path = format!("{}/lib", project_name);
    create_folders(&base_path, selected_arch)?;

    println!(
        "{}",
        "\n✅ İşlem Başarıyla Tamamlandı! Klasörler oluşturuldu.".green().bold()
    );

    // 6. ADIM: Seçilen IDE'yi Sessizce Aç
    if let Some(cmd) = ide_command {
        open_ide(&cmd, &project_name);
    } else {
        println!("Terminalden girmek için:");
        println!("cd {}\nflutter run", project_name);
    }

    Ok(())
}

// --- Yardımcı Fonksiyonlar ---

fn create_folders(base_path: &str, arch: Architecture) -> std::io::Result<()> {
    let folders = match arch {
        Architecture::Clean => vec![
            "core/error",
            "core/usecases",
            "core/util",
            "core/constants",
            "data/datasources/local",
            "data/datasources/remote",
            "data/models",
            "data/repositories",
            "domain/entities",
            "domain/repositories",
            "domain/usecases",
            "presentation/bloc",
            "presentation/pages",
            "presentation/widgets",
        ],
        Architecture::Mvvm => vec![
            "core/constants",
            "core/services",
            "models",
            "views",
            "viewmodels",
            "widgets",
        ],
    };

    for folder in folders {
        let path = format!("{}/{}", base_path, folder);
        fs::create_dir_all(&path)?;
    }
    
    println!("📂 {} mimarisine uygun klasörler eklendi.", arch);

    Ok(())
}

fn open_ide(command: &str, project_path: &str) {
    println!("🖥️  '{}' komutu ile editör başlatılıyor...", command);

    #[cfg(target_os = "windows")]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        // Burada spawn sonucunu kontrol ediyoruz (match ile)
        let result = Command::new("cmd")
            .args(["/C", command, "."])
            .current_dir(project_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();

        match result {
            Ok(_) => {}, // Başarılı, sessizce devam et
            Err(_) => {
                println!("{}", format!("⚠️  Uyarı: '{}' komutu bulunamadı veya çalıştırılamadı.", command).yellow());
                println!("Lütfen komutun PATH'e ekli olduğundan emin olun.");
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let result = Command::new(command)
            .arg(".")
            .current_dir(project_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        match result {
            Ok(_) => {},
            Err(_) => {
                println!("{}", format!("⚠️  Uyarı: '{}' komutu bulunamadı.", command).yellow());
            }
        }
    }
}