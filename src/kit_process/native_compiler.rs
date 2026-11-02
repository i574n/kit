fn native_spiral_host(path: &std::path::Path) -> bool {
    path.file_stem().and_then(|name| name.to_str()).is_some_and(|name| name.eq_ignore_ascii_case("SpiralCompiler"))
}

fn spiral_dotnet_from(env: impl Fn(&str) -> Option<std::ffi::OsString>) -> std::ffi::OsString {
    if let Some(value) = env("KIT_DOTNET").or_else(|| env("SPIRAL_DOTNET")).filter(|value| !value.is_empty()) {
        return value;
    }
    let cache = env("SPIRAL_BIN_CACHE_DIR").filter(|value| !value.is_empty()).map(std::path::PathBuf::from).or_else(|| {
        if cfg!(windows) {
            env("LOCALAPPDATA").map(|root| std::path::PathBuf::from(root).join("spiral-bin"))
        } else {
            env("HOME").map(|home| std::path::PathBuf::from(home).join(".cache").join("spiral-bin"))
        }
    });
    let host = if cfg!(windows) { "dotnet.exe" } else { "dotnet" };
    cache
        .map(|root| root.join("toolchains").join("dotnet").join(host))
        .filter(|candidate| candidate.is_file())
        .map(std::path::PathBuf::into_os_string)
        .unwrap_or_else(|| "dotnet".into())
}

fn spiral_compiler_command(path: &std::path::Path) -> std::process::Command {
    let managed = path.extension().and_then(|value| value.to_str()).is_some_and(|value| value.eq_ignore_ascii_case("dll"));
    let mut command = if managed {
        let dotnet = spiral_dotnet_from(|name| std::env::var_os(name));
        let mut command = std::process::Command::new(dotnet);
        command.arg(path);
        command
    } else {
        std::process::Command::new(path)
    };
    if let Some(root) = std::env::var_os("KIT_SPIRAL_BUNDLE") {
        command.env("SPIRAL_WORKSPACE_ROOT", root);
    }
    command
}

#[cfg(test)]
mod native_compiler_tests {
    use super::*;
    #[test]
    fn native_host_supports_attestation_commands() {
        for name in ["SpiralCompiler.dll", "SpiralCompiler.exe", "SpiralCompiler"] {
            let path = std::path::Path::new(name);
            assert!(native_spiral_host(path));
        }
        assert!(!native_spiral_host(std::path::Path::new("spiral-compile")));
        let command = spiral_compiler_command(std::path::Path::new("SpiralCompiler.dll"));
        assert_eq!(command.get_args().next().unwrap(), "SpiralCompiler.dll");
    }

    #[test]
    fn dotnet_resolves_from_the_spiral_cache_before_path() {
        let root = std::env::temp_dir().join(format!("kit-dotnet-cache-{}", std::process::id()));
        let host = if cfg!(windows) { "dotnet.exe" } else { "dotnet" };
        let dotnet = root.join("toolchains").join("dotnet").join(host);
        std::fs::create_dir_all(dotnet.parent().unwrap()).unwrap();
        std::fs::write(&dotnet, b"host").unwrap();
        let cache = root.clone().into_os_string();
        let found = spiral_dotnet_from(|name| (name == "SPIRAL_BIN_CACHE_DIR").then(|| cache.clone()));
        assert_eq!(std::path::PathBuf::from(found), dotnet);
        let explicit = spiral_dotnet_from(|name| (name == "KIT_DOTNET").then(|| "/opt/dotnet/dotnet".into()));
        assert_eq!(explicit, "/opt/dotnet/dotnet");
        let fallback = spiral_dotnet_from(|_| None);
        assert_eq!(fallback, "dotnet");
        let _ = std::fs::remove_dir_all(&root);
    }
}
