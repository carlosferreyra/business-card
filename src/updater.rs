use anyhow::Context;
use self_update::backends::github::{Update, UpdateBuilder};
use std::ffi::OsString;
use std::time::Duration;

const RESTART_MARKER: &str = "--__business-card-update-restarted";

fn configuration() -> UpdateBuilder {
    let target = self_update::get_target();
    let archive = format!("carlosferreyra-{target}.tar.xz");
    let mut builder = Update::configure();
    builder
        .repo_owner("carlosferreyra")
        .repo_name("business-card")
        .bin_name("carlosferreyra")
        .bin_path_in_archive(format!("carlosferreyra-{target}/carlosferreyra"))
        .current_version(env!("CARGO_PKG_VERSION"))
        .asset_matcher(move |assets| assets.iter().find(|asset| asset.name() == archive).cloned())
        .checksum_from_asset("sha256.sum")
        .check_install_path_writable(true)
        .timeout(Duration::from_millis(1500))
        .no_confirm(true)
        .show_output(false);
    builder
}

fn update(builder: &mut UpdateBuilder) -> anyhow::Result<Option<String>> {
    let releases = builder.build()?.get_latest_release()?;
    let release = releases.latest().context("No stable release found")?;
    if !self_update::version::bump_is_greater(
        releases
            .current_version()
            .unwrap_or(env!("CARGO_PKG_VERSION")),
        release.version(),
    )? {
        return Ok(None);
    }
    let version = release.version().to_owned();
    eprintln!("Updating business card to {version}…");
    let status = builder
        .release_tag(format!("v{version}"))
        .timeout(Duration::from_secs(30))
        .build()?
        .update()?;
    Ok(status.is_updated().then_some(version))
}

fn take_restart_marker(args: &mut Vec<OsString>) -> bool {
    if let Some(index) = args.iter().skip(1).position(|arg| arg == RESTART_MARKER) {
        args.remove(index + 1);
        true
    } else {
        false
    }
}

pub fn check_and_restart(args: &mut Vec<OsString>) {
    if take_restart_marker(args) {
        return;
    }
    match update(&mut configuration()) {
        Ok(Some(_)) => {
            let restarted_args = args
                .iter()
                .skip(1)
                .cloned()
                .chain([OsString::from(RESTART_MARKER)]);
            match self_update::restart::restart_with(restarted_args) {
                Err(error) => eprintln!(
                    "Update installed; restart unavailable. Continuing this invocation: {error}"
                ),
                Ok(never) => match never {},
            }
        }
        Ok(None) => {}
        Err(_) => eprintln!("Update unavailable; continuing with the installed card."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use sha2::{Digest, Sha256};
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::path::Path;
    use std::process::Command;
    use std::thread;
    use std::time::Instant;

    fn server(replies: Vec<(u16, Vec<u8>)>) -> (String, thread::JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        start_server(listener, replies)
    }

    fn start_server(
        listener: TcpListener,
        replies: Vec<(u16, Vec<u8>)>,
    ) -> (String, thread::JoinHandle<Vec<String>>) {
        let base = format!("http://{}", listener.local_addr().unwrap());
        listener.set_nonblocking(true).unwrap();
        let handle = thread::spawn(move || {
            let mut paths = Vec::new();
            for (status, body) in replies {
                let deadline = Instant::now() + Duration::from_secs(5);
                let mut stream = loop {
                    if let Ok((stream, _)) = listener.accept() {
                        break stream;
                    }
                    assert!(
                        Instant::now() < deadline,
                        "expected HTTP request was not received"
                    );
                    thread::sleep(Duration::from_millis(5));
                };
                stream
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let mut request = [0; 8192];
                let count = stream.read(&mut request).unwrap();
                paths.push(
                    String::from_utf8_lossy(&request[..count])
                        .lines()
                        .next()
                        .unwrap()
                        .to_owned(),
                );
                write!(
                    stream,
                    "HTTP/1.1 {status} OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                )
                .unwrap();
                stream.write_all(&body).unwrap();
            }
            paths
        });
        (base, handle)
    }

    fn release(version: &str) -> Vec<u8> {
        serde_json::to_vec(&json!({"tag_name": format!("v{version}"), "name": version, "created_at": "2026-10-04T00:00:00Z", "assets": []})).unwrap()
    }

    #[test]
    fn checks_every_time_and_does_not_downgrade() {
        for version in [env!("CARGO_PKG_VERSION"), "1.0.0"] {
            let (base, handle) = server(vec![(200, release(version)), (200, release(version))]);
            let mut builder = configuration();
            builder.api_base_url(base);
            assert!(update(&mut builder).unwrap().is_none());
            assert!(update(&mut builder).unwrap().is_none());
            assert_eq!(handle.join().unwrap().len(), 2);
        }
    }

    #[test]
    fn failed_check_keeps_installed_file() {
        let dir = tempfile::tempdir().unwrap();
        let installed = dir.path().join("card");
        std::fs::write(&installed, "original").unwrap();
        let (base, handle) = server(vec![(403, b"rate limited".to_vec())]);
        let mut builder = configuration();
        builder.api_base_url(base).bin_install_path(&installed);
        assert!(update(&mut builder).is_err());
        assert_eq!(std::fs::read_to_string(installed).unwrap(), "original");
        handle.join().unwrap();
    }

    #[test]
    fn stalled_check_has_a_short_timeout() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let handle = thread::spawn(move || {
            let _stream = listener.accept().unwrap();
            thread::sleep(Duration::from_millis(400));
        });
        let mut builder = configuration();
        builder
            .api_base_url(base)
            .timeout(Duration::from_millis(100));
        let start = Instant::now();
        assert!(update(&mut builder).is_err());
        assert!(start.elapsed() < Duration::from_millis(350));
        handle.join().unwrap();
    }

    fn upgrade_fixture(
        destination: &Path,
        bad_checksum: bool,
    ) -> (UpdateBuilder, thread::JoinHandle<Vec<String>>, String) {
        let dir = tempfile::tempdir().unwrap();
        let target = self_update::get_target();
        let folder = format!("carlosferreyra-{target}");
        std::fs::create_dir(dir.path().join(&folder)).unwrap();
        let binary = dir.path().join(&folder).join("carlosferreyra");
        std::fs::write(
            &binary,
            "#!/bin/sh\nprintf 'updated-fixture\\n'\nprintf '%s\\n' \"$@\"\n",
        )
        .unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&binary, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        let archive = dir.path().join("card.tar.xz");
        assert!(
            Command::new("tar")
                .args(["-cJf"])
                .arg(&archive)
                .arg("-C")
                .arg(dir.path())
                .arg(&folder)
                .status()
                .unwrap()
                .success()
        );
        let bytes = std::fs::read(archive).unwrap();
        let digest = if bad_checksum {
            "0".repeat(64)
        } else {
            format!("{:x}", Sha256::digest(&bytes))
        };
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let asset = format!("{folder}.tar.xz");
        let record = serde_json::to_vec(&json!({
            "tag_name": "v9.0.0", "name": "fixture", "created_at": "2026-10-04T00:00:00Z",
            "assets": [
                {"name": format!("{folder}-update"), "url": format!("{base}/wrong-updater")},
                {"name": format!("{asset}.sha256"), "url": format!("{base}/wrong-checksum")},
                {"name": asset, "url": format!("{base}/archive")},
                {"name": "sha256.sum", "url": format!("{base}/sums")}
            ]
        }))
        .unwrap();
        let (base, handle) = start_server(
            listener,
            vec![
                (200, record.clone()),
                (200, record),
                (200, format!("{digest}  {asset}\n").into_bytes()),
                (200, bytes),
            ],
        );
        let mut builder = configuration();
        builder.api_base_url(&base).bin_install_path(destination);
        (builder, handle, base)
    }

    #[test]
    fn upgrades_exact_platform_archive_with_verified_checksum() {
        let dir = tempfile::tempdir().unwrap();
        let installed = dir.path().join("card");
        std::fs::write(&installed, "original").unwrap();
        let (mut builder, handle, _) = upgrade_fixture(&installed, false);
        assert_eq!(update(&mut builder).unwrap().as_deref(), Some("9.0.0"));
        let paths = handle.join().unwrap();
        assert!(paths[0].contains("/releases/latest"));
        assert!(paths[1].contains("/releases/tags/v9.0.0"));
        assert!(paths[2].contains("/sums"));
        assert!(paths[3].contains("/archive"));
        assert!(
            std::fs::read_to_string(installed)
                .unwrap()
                .contains("updated-fixture")
        );
    }

    #[test]
    fn checksum_failure_preserves_installed_binary() {
        let dir = tempfile::tempdir().unwrap();
        let installed = dir.path().join("card");
        std::fs::write(&installed, "original").unwrap();
        let (mut builder, handle, _) = upgrade_fixture(&installed, true);
        assert!(update(&mut builder).is_err());
        handle.join().unwrap();
        assert_eq!(std::fs::read_to_string(installed).unwrap(), "original");
    }

    #[test]
    fn restarted_invocation_preserves_user_arguments_and_skips_one_check() {
        let mut args: Vec<OsString> = ["card", "--open", "portfolio", RESTART_MARKER]
            .map(OsString::from)
            .into();
        assert!(take_restart_marker(&mut args));
        assert_eq!(args, ["card", "--open", "portfolio"].map(OsString::from));
        assert!(!take_restart_marker(&mut args));
    }

    // Spawn a disposable copy of the test executable so replacement never touches the developer's CLI.
    #[test]
    fn restart_child() {
        let Ok(base) = std::env::var("BUSINESS_CARD_SMOKE_API") else {
            return;
        };
        let mut builder = configuration();
        builder.api_base_url(base);
        assert!(update(&mut builder).unwrap().is_some());
        assert!(self_update::restart::restart_with(["--version", RESTART_MARKER]).is_ok());
    }

    #[test]
    fn upgrade_restarts_into_replacement_with_original_arguments() {
        let dir = tempfile::tempdir().unwrap();
        let executable = dir.path().join("smoke-child");
        std::fs::copy(std::env::current_exe().unwrap(), &executable).unwrap();
        let (_, handle, base) = upgrade_fixture(&executable, false);
        let output = Command::new(&executable)
            .args(["--exact", "updater::tests::restart_child", "--nocapture"])
            .env("BUSINESS_CARD_SMOKE_API", base)
            .output()
            .unwrap();
        handle.join().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("updated-fixture\n--version\n"), "{stdout}");
    }
}
