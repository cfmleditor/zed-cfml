use zed_extension_api as zed;

const SERVER_PATH: &str = "clif";
/// The server's name before it was clif. A copy on PATH under this name is
/// still run, and a release from before the rename publishes only assets of
/// this name; every release since publishes them too.
const LEGACY_SERVER_PATH: &str = "cfmleditor-lsp";
const GITHUB_REPO: &str = "cfmleditor/clif";
const LSP_VERSION: &str = "latest";

struct CfmlExtension {
    cached_binary_path: Option<String>,
}

impl CfmlExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<String> {
        if let Some(path) = &self.cached_binary_path
            && std::fs::metadata(path).is_ok_and(|m| m.is_file())
        {
            return Ok(path.clone());
        }

        for name in [SERVER_PATH, LEGACY_SERVER_PATH] {
            if let Some(path) = worktree.which(name) {
                return Ok(path);
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = if LSP_VERSION == "latest" {
            zed::latest_github_release(
                GITHUB_REPO,
                zed::GithubReleaseOptions {
                    require_assets: true,
                    pre_release: false,
                },
            )
        } else {
            zed::github_release_by_tag_name(GITHUB_REPO, LSP_VERSION)
        }
        .map_err(|e| format!("failed to fetch release: {e}"))?;

        let (os, arch) = zed::current_platform();
        let os_str = match os {
            zed::Os::Mac => "darwin",
            zed::Os::Linux => "linux",
            zed::Os::Windows => "windows",
        };
        let arch_str = match arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X8664 => "amd64",
            zed::Architecture::X86 => "386",
        };

        // clif's asset first, then the one a release from before the rename
        // has; each archive holds a binary of its own name.
        let ext = if os == zed::Os::Windows {
            "zip"
        } else {
            "tar.gz"
        };
        let (server_name, asset) = [SERVER_PATH, LEGACY_SERVER_PATH]
            .into_iter()
            .find_map(|name| {
                let asset_name = format!("{name}-{os_str}-{arch_str}.{ext}");
                release
                    .assets
                    .iter()
                    .find(|a| a.name == asset_name)
                    .map(|a| (name, a))
            })
            .ok_or_else(|| {
                format!("no asset found matching {SERVER_PATH}-{os_str}-{arch_str}.{ext}")
            })?;

        let version_dir = format!("{server_name}-{}", release.version);
        let binary_name = if os == zed::Os::Windows {
            format!("{server_name}.exe")
        } else {
            server_name.to_string()
        };
        let binary_path = format!("{version_dir}/{binary_name}");

        if !std::fs::metadata(&binary_path).is_ok_and(|m| m.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let file_type = if os == zed::Os::Windows {
                zed::DownloadedFileType::Zip
            } else {
                zed::DownloadedFileType::GzipTar
            };

            zed::download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|e| format!("failed to download: {e}"))?;

            zed::make_file_executable(&binary_path)
                .map_err(|e| format!("failed to make executable: {e}"))?;

            // Clean up old version directories, ignoring errors
            if let Ok(entries) = std::fs::read_dir(".") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let name = entry.file_name();
                    let name = name.to_string_lossy();
                    let ours = [SERVER_PATH, LEGACY_SERVER_PATH]
                        .iter()
                        .any(|server| name.starts_with(&format!("{server}-")));
                    if ours && name.as_ref() != version_dir && path.is_dir() && !path.is_symlink() {
                        let _ = std::fs::remove_dir_all(&path);
                    }
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for CfmlExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(CfmlExtension);
