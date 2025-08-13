use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use walkdir::WalkDir;

fn main() -> Result<()> {
    println!("Starting benchmark data update...");

    let platform = detect_platform();
    println!("Detected platform: {}", platform);

    let project_root = find_project_root()?;
    println!("Project root: {:?}", project_root);
    let source_dir = project_root.join("target/criterion");
    let dest_base = project_root.join("assets/data");

    if !source_dir.exists() {
        anyhow::bail!("Source directory does not exist: {}", source_dir.display());
    }

    copy_benchmark_data(&source_dir, &dest_base, &platform)?;

    println!("Benchmark data update completed successfully!");
    Ok(())
}

fn detect_platform() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    }
}

fn find_project_root() -> Result<PathBuf> {
    // 使用CARGO_MANIFEST_DIR环境变量获取项目根目录
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .context("CARGO_MANIFEST_DIR environment variable not set")?;

    let mut path = PathBuf::from(manifest_dir);

    // xtask目录在项目根目录下的xtasks/xtask-update-bench-data
    // 所以需要返回两级上级目录
    if path.ends_with("xtasks/xtask-update-bench-data") {
        path.pop(); // 移除xtask-update-bench-data
        path.pop(); // 移除xtasks
    }

    Ok(path)
}

fn copy_benchmark_data(source_dir: &Path, dest_base: &Path, platform: &str) -> Result<()> {
    let walker = WalkDir::new(source_dir).into_iter();

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();

        // 查找estimates.json文件
        if path.file_name().and_then(|n| n.to_str()) == Some("new") && path.is_dir() {
            if let Some(scene_info) = extract_scene_info(path)? {
                println!("{:?}", scene_info);
                let (scene, engine, input) = scene_info;

                // 构建目标路径: assets/data/{platform}/criterion/{scene}/{engine}/{input}/new/estimates.json
                let dest_dir = dest_base
                    .join(platform)
                    .join("criterion")
                    .join(&scene)
                    .join(&engine)
                    .join(&input)
                    .join("new");

                for file in [
                    "benchmark.json",
                    "estimates.json",
                    "sample.json",
                    "tukey.json",
                ] {
                    let src_file = path.join(file);
                    let dest_file = dest_dir.join(file);

                    // 创建目标目录
                    fs::create_dir_all(&dest_dir).with_context(|| {
                        format!("Failed to create directory: {}", dest_dir.display())
                    })?;

                    // 复制文件
                    fs::copy(&src_file, &dest_file).with_context(|| {
                        format!(
                            "Failed to copy from {} to {}",
                            path.display(),
                            dest_file.display()
                        )
                    })?;

                    println!("Copied: {} -> {}", src_file.display(), dest_file.display());
                }
            }
        }
    }

    Ok(())
}

fn extract_scene_info(estimates_path: &Path) -> Result<Option<(String, String, String)>> {
    let components: Vec<_> = estimates_path.components().collect();

    // 查找criterion组件的位置
    if let Some(criterion_idx) = components
        .iter()
        .position(|c| c.as_os_str().to_string_lossy() == "criterion")
    {
        // 检查是否有足够的后续组件: scene/engine/input/new/estimates.json
        if criterion_idx + 4 < components.len() {
            let scene = components[criterion_idx + 1]
                .as_os_str()
                .to_string_lossy()
                .into_owned();
            let engine = components[criterion_idx + 2]
                .as_os_str()
                .to_string_lossy()
                .into_owned();
            let input = components[criterion_idx + 3]
                .as_os_str()
                .to_string_lossy()
                .into_owned();

            return Ok(Some((scene, engine, input)));
        }
    }

    Ok(None)
}
