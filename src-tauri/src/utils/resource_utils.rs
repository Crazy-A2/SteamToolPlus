// 资源目录工具
// 提供获取资源目录的统一方法，避免重复代码

use std::path::PathBuf;
use tauri::{AppHandle, Manager};

/// 获取资源目录路径
/// 优先检查程序所在目录的 resources 文件夹，然后使用 Tauri 的 resource_dir
pub fn get_resource_dir(app: &AppHandle) -> Result<PathBuf, String> {
    // 首先检查程序所在目录的 resources 文件夹（打包后的标准结构）
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("无法获取程序路径: {}", e))?
        .parent()
        .ok_or("无法获取程序所在目录")?
        .to_path_buf();

    // 尝试小写的 resources
    let resources_path = exe_dir.join("resources");
    if resources_path.exists() && resources_path.is_dir() {
        return Ok(resources_path);
    }

    // 尝试大写的 Resources
    let resources_path_capital = exe_dir.join("Resources");
    if resources_path_capital.exists() && resources_path_capital.is_dir() {
        return Ok(resources_path_capital);
    }

    // 最后尝试使用 Tauri 的 resource_dir
    if let Ok(resource_dir) = app.path().resource_dir() {
        if resource_dir.exists() && resource_dir.is_dir() {
            return Ok(resource_dir);
        }
    }

    Err("无法找到资源目录".to_string())
}
