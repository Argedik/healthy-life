use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    // let root_path = "C:\\Users\\enes.gedik\\Desktop\\fe\\besin-uygulamasi";
    let root_path = "C:\\Users\\enes.gedik\\Desktop\\fe\\besin-uygulamasi\\frontend\\web";

    let mut output = Vec::new(); // Dosya yolu ve içeriğini saklamak için bir koleksiyon
    let mut ignored = HashSet::new(); // Hariç tutulacak dosya ve klasörlerin seti

    // Önceden belirlenmiş gereksiz klasör ve dosyalar
    ignored.insert("target".to_string());
    ignored.insert("incremental".to_string());
    ignored.insert(".git".to_string());
    ignored.insert(".gitignore".to_string());

    list_files_only(root_path, root_path, &mut ignored, &mut output);

    // Çıktıyı yazacağımız dosyanın yolu
    let output_file_path = format!("{}/output.txt", root_path.replace("\\", "/"));

    // Dosyayı oluştur ve üzerine yaz (mevcutsa siler)
    let mut output_file = File::create(&output_file_path)
        .expect("Çıktı dosyası oluşturulamadı.");

    // Tüm dosya yollarını ve içeriklerini dosyaya yazdır
    for (file_path, content) in output {
        writeln!(output_file, "Dosya Yolu: {}", file_path).unwrap();
        writeln!(output_file, "Dosya İçeriği:\n{}\n", content).unwrap();
    }
    println!("Çıktılar '{}' dosyasına yazıldı.", output_file_path);
}

/// Belirtilen dizindeki dosyaları listeler ve içeriklerini okur
fn list_files_only(
    path: &str,
    root: &str,
    ignored: &mut HashSet<String>,
    output: &mut Vec<(String, String)>,
) {
    if let Ok(entries) = fs::read_dir(path) {
        // `.gitignore` dosyasını kontrol et
        let gitignore_path = format!("{}/.gitignore", path.replace("\\", "/"));

        if Path::new(&gitignore_path).exists() {
            update_ignored_from_gitignore(&gitignore_path, ignored, path);
        }

        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();
                let file_path = entry.path().to_str().unwrap().replace("\\", "/");

                // Önceden belirlenmiş ve `.gitignore` ile hariç tutulan dosya/klasörleri atla
                if ignored.contains(&file_name) || ignored.contains(&file_path) {
                    continue;
                }

                let file_type = entry.file_type().unwrap();
                // Eğer dosyaysa
                if file_type.is_file() {
                    let relative_path = file_path.replace(root, "besin-uygulamasi");

                    // Dosya içeriğini oku
                    let content = fs::read_to_string(&file_path).unwrap_or_else(|_| {
                        "Dosya okunamadı veya metin dosyası değil.".to_string()
                    });

                    // Dosya yolu ve içeriğini sakla
                    output.push((relative_path, content));
                }
                // Eğer klasörse
                else if file_type.is_dir() {
                    // Alt klasörleri taramaya devam et
                    list_files_only(&file_path, root, ignored, output);
                }
            }
        }
    }
}

/// `.gitignore` dosyasını okuyup, hariç tutulacak dosya/klasörleri ekler
fn update_ignored_from_gitignore(
    gitignore_path: &str,
    ignored: &mut HashSet<String>,
    base_path: &str,
) {
    if let Ok(file) = fs::File::open(gitignore_path) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(mut line) = line {
                // Yorumları ve boş satırları atla
                line = line.trim().to_string();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                // Satırı tam yol haline getir ve `ignored` setine ekle
                let full_path = format!("{}/{}", base_path.replace("\\", "/"), line);
                ignored.insert(full_path);
            }
        }
    }
}
