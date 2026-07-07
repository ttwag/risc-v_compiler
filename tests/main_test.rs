use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

fn assert_compile(path: &Path) {
    for entry in fs::read_dir(&path).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_stem().unwrap().to_str().unwrap();
        let should_succeed = name.starts_with("ok_");

        let output = Command::new(env!("CARGO_BIN_EXE_compile"))
            .arg(&path)
            .output()
            .expect("compiler crashed");

        assert_eq!(
            output.status.success(),
            should_succeed,
            "[{name}] unexpected result: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn test_main_compile() {
    let temp_workspace = tempdir().unwrap();
    let temp_path = temp_workspace.path();
    let fixture_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");

    // copy all files from test fixtures to tempdir
    Command::new("cp")
        .arg("-r")
        .arg(fixture_src.join("."))
        .arg(temp_path)
        .status()
        .expect("failed to copy to a temporary test directory");

    // assert the files compile and return with the correct statuses
    assert_compile(temp_path);
}
