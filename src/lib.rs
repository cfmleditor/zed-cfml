use zed_extension_api as zed;

const SERVER_PATH: &str = "cfmleditor-lsp";
const GITHUB_REPO: &str = "cfmleditor/cfmleditor-lsp";
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
        if let Some(path) = &self.cached_binary_path {
            if std::fs::metadata(path).map_or(false, |m| m.is_file()) {
                return Ok(path.clone());
            }
        }

        if let Some(path) = worktree.which(SERVER_PATH) {
            return Ok(path);
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

        let ext = if os == zed::Os::Windows { "zip" } else { "tar.gz" };
        let asset_name = format!("cfmleditor-lsp-{os_str}-{arch_str}.{ext}");
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {asset_name}"))?;

        let version_dir = format!("cfmleditor-lsp-{}", release.version);
        let binary_name = if os == zed::Os::Windows {
            format!("{SERVER_PATH}.exe")
        } else {
            SERVER_PATH.to_string()
        };
        let binary_path = format!("{version_dir}/{binary_name}");

        if !std::fs::metadata(&binary_path).map_or(false, |m| m.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let file_type = if os == zed::Os::Windows {
                zed::DownloadedFileType::Zip
            } else {
                zed::DownloadedFileType::GzipTar
            };

            zed::download_file(
                &asset.download_url,
                &version_dir,
                file_type,
            )
            .map_err(|e| format!("failed to download: {e}"))?;

            zed::make_file_executable(&binary_path)
                .map_err(|e| format!("failed to make executable: {e}"))?;
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
