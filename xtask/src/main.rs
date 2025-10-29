use std::{
    env,
    fs,
    path::Path,
    process::{Command, exit},
};

fn main() {
    // 获取主 crate 名称
    let manifest_path = Path::new("../Cargo.toml");
    let crate_name = get_crate_name(manifest_path).unwrap_or_else(|| {
        eprintln!("⚠️ 解析 Cargo.toml 时失败");
        "rust_project".to_string()
    });

    let target_dir = Path::new("../target/thumbv7m-none-eabi/debug");
    let elf_path = target_dir.join(&crate_name);

    let elf_file = elf_path.with_extension("elf");
    if elf_file.exists() {
        fs::remove_file(&elf_file).ok();
    }
    let built_file = elf_path.with_extension(""); // 原始 target 文件名
    if !built_file.exists() {
        eprintln!("❌ 构建完成后未找到 ELF 文件: {:?}", built_file);
        exit(1);
    }
    fs::rename(&built_file, &elf_file).expect("无法重命名 ELF 文件");
    println!("✅ ELF 文件生成: {:?}", elf_file);

    println!("📊 统计 Flash/RAM 使用情况 ...");
    let output = Command::new("arm-none-eabi-size")
        .arg(&elf_file)
        .output()
        .expect("无法执行 arm-none-eabi-size");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);

    // 输出占用率
    // STM32F103ZET6: Flash 512KB, RAM 64KB
    const FLASH_TOTAL: f64 = 512.0 * 1024.0;
    const RAM_TOTAL: f64 = 64.0 * 1024.0;

    if let Some(line) = stdout.lines().nth(1) {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() >= 4 {
            let text: f64 = cols[0].parse().unwrap_or(0.0);
            let data: f64 = cols[1].parse().unwrap_or(0.0);
            let bss: f64 = cols[2].parse().unwrap_or(0.0);
            let flash_used = text + data;
            let ram_used = data + bss;
            println!(
                "Flash 使用: {:.2}% ({:.0}/{:.0} bytes)",
                flash_used / FLASH_TOTAL * 100.0,
                flash_used,
                FLASH_TOTAL
            );
            println!(
                "RAM 使用: {:.2}% ({:.0}/{:.0} bytes)",
                ram_used / RAM_TOTAL * 100.0,
                ram_used,
                RAM_TOTAL
            );
        }
    }
}

// 从 Cargo.toml 读取 crate 名称
fn get_crate_name(manifest_path: &Path) -> Option<String> {
    let content = fs::read_to_string(manifest_path).ok()?;
    for line in content.lines() {
        if line.trim_start().starts_with("name") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                let name = parts[1].trim().trim_matches('"').to_string();
                return Some(name);
            }
        }
    }
    None
}
