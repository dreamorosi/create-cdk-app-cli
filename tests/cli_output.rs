#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

struct Fixture(PathBuf);

impl Fixture {
    fn new(npm_script: &str) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let sequence = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "cdk-cli-test-{}-{id}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(dir.join("tools")).unwrap();
        let npm = dir.join("tools/npm");
        fs::write(&npm, format!("#!/bin/sh\n{npm_script}\n")).unwrap();
        fs::set_permissions(npm, fs::Permissions::from_mode(0o755)).unwrap();
        Self(dir)
    }

    fn run(&self, no_install: bool) -> Output {
        let mut paths = vec![self.0.join("tools")];
        paths.extend(std::env::split_paths(&std::env::var_os("PATH").unwrap()));
        let mut command = Command::new(env!("CARGO_BIN_EXE_create-cdk-app"));
        command
            .arg("output-check")
            .current_dir(&self.0)
            .env("PATH", std::env::join_paths(paths).unwrap());
        if no_install {
            command.arg("--no-install");
        }
        command.output().unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn redirected_output_has_no_spinner_and_includes_npm_output() {
    let fixture = Fixture::new(
        "[ \"$1\" = ci ] || exit 1\nsleep 0.3\necho 'npm stdout message'\necho 'npm stderr message' >&2",
    );
    let output = fixture.run(false);
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(stdout.matches("Installing npm packages...").count(), 1);
    assert!(!stdout.contains('\r'), "Spinner redraws found: {stdout:?}");
    assert!(!stdout.contains('\u{1b}'), "ANSI escapes found: {stdout:?}");
    assert!(stdout.contains("npm stdout message"));
    assert!(stderr.contains("npm stderr message"));
    assert!(stdout.contains("New CDK project created successfully!"));
}

#[test]
fn npm_failure_preserves_diagnostics_and_exit_status() {
    let fixture = Fixture::new("echo 'npm install failed' >&2\nexit 7");
    let output = fixture.run(false);
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("npm install failed"));
    assert!(stderr.contains("npm ci failed with exit code: Some(7)"));
    assert!(stderr.contains("running 'npm ci' manually"));
}

#[test]
fn no_install_skips_npm_and_prints_plain_progress() {
    let fixture = Fixture::new("echo 'npm should not run' >&2\nexit 1");
    let output = fixture.run(true);
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.stderr.is_empty());
    assert!(stdout.contains("Scaffolding complete!"));
    assert!(!stdout.contains("Installing npm packages"));
    assert!(!stdout.contains('\r'));
}
