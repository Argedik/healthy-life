use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    // Linux ortamı için örnek bir dizin yolu
    // Kendi dizin yapınıza göre güncelleyin
    let root_path = "/home/argedik/Desktop/fe/healthy-life/backend/nutrition_service";
    // let root_path = "/home/argedik/Desktop/fe/healthy-life/backend";
    // let root_path = "/home/argedik/Desktop/fe/healthy-life/frontend/web";

    let mut output = Vec::new();
    let mut ignored = HashSet::new();

    // Örnek ignore listesi
    ignored.insert("target".to_string());
    ignored.insert("incremental".to_string());
    ignored.insert(".git".to_string());
    ignored.insert(".gitignore".to_string());
    ignored.insert("styles.css".to_string());
    ignored.insert("output.txt".to_string());
    ignored.insert("utilizable.html".to_string());
    ignored.insert("utilizable.markdown".to_string());
    ignored.insert("dist".to_string());
    ignored.insert("create_component.ps1".to_string());
    ignored.insert("README.md".to_string());

    list_files_only(root_path, root_path, &mut ignored, &mut output);

    let output_file_path = format!("{}/output.txt", root_path);

    // Dosyayı write + create + truncate modunda aç
    let mut output_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&output_file_path)
        .expect("Çıktı dosyası oluşturulamadı (OpenOptions).");

    for (file_path, content) in output {
        writeln!(output_file, "Dosya Yolu: {}", file_path).unwrap();
        writeln!(output_file, "Dosya İçeriği:\n{}\n", content).unwrap();
    }

    println!("Çıktılar '{}' dosyasına yazıldı.", output_file_path);
}

fn list_files_only(
    path: &str,
    root: &str,
    ignored: &mut HashSet<String>,
    output: &mut Vec<(String, String)>,
) {
    if let Ok(entries) = fs::read_dir(path) {
        // .gitignore yolunu doğrudan birleştiriyoruz
        let gitignore_path = format!("{}/.gitignore", path);
        if Path::new(&gitignore_path).exists() {
            update_ignored_from_gitignore(&gitignore_path, ignored, path);
        }

        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap_or_default();
                let file_path = entry.path().to_str().unwrap_or_default().to_string();

                // ignore listesinde varsa devam etme
                if ignored.contains(&file_name) || ignored.contains(&file_path) {
                    continue;
                }

                let file_type = entry.file_type().unwrap();
                if file_type.is_file() {
                    // root yerine "besin-uygulamasi" yazmak için
                    let relative_path = file_path.replace(root, "besin-uygulamasi");
                    let content = fs::read_to_string(&file_path)
                        .unwrap_or_else(|_| "Dosya okunamadı veya metin dosyası değil.".to_string());
                    output.push((relative_path, content));
                } else if file_type.is_dir() {
                    list_files_only(&file_path, root, ignored, output);
                }
            }
        }
    }
}

fn update_ignored_from_gitignore(
    gitignore_path: &str,
    ignored: &mut HashSet<String>,
    base_path: &str,
) {
    if let Ok(file) = fs::File::open(gitignore_path) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(mut line) = line {
                line = line.trim().to_string();
                // Boş ya da # ile başlayan satırları yok say
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let full_path = format!("{}/{}", base_path, line);
                ignored.insert(full_path);
            }
        }
    }
}
