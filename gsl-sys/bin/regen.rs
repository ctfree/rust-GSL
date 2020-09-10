use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::fs::{read_dir, write, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const HEADER_FILE: &str = "wrapper.h";
const GSL_REPOSITORY: &str = "git://git.savannah.gnu.org/gsl.git";

fn get_all_headers(folder: &Path, extra: &mut Vec<String>, headers: &mut Vec<String>) {
    println!("=> Entering `{:?}`", folder);
    for entry in read_dir(folder).expect("Failed to read gsl directory...") {
        let entry = entry.expect("failed to get entry...");
        let entry = entry.path();
        if entry.is_dir() {
            extra.push(entry.file_name().expect("failed to get file name").to_str().expect("failed to convert to str").to_owned());
            get_all_headers(&entry, extra, headers);
            extra.pop();
        } else if entry.is_file() && *entry.extension().as_ref().unwrap_or(&OsStr::new("")) == "h" {
            let file_name = entry.file_name().expect("failed to get file name").to_str().expect("failed to convert to str");
            headers.push(format!("#include \"{}/{}\"", extra.join("/"), file_name));
            println!("--> Added `{}` to the list!", file_name);
        }
    }
    println!("<= Leaving `{:?}`", folder);
}

fn create_header_file(folder: &Path) {
    println!("=> Creating header file...");
    let mut headers = Vec::new();
    let mut extra = vec!["gsl".to_owned()];

    get_all_headers(&folder.join("gsl"), &mut extra, &mut headers);
    write(
        HEADER_FILE,
        format!("#ifndef __WRAPPER__\n#define __WRAPPER__\n{}\n#endif\n", headers.join("\n")),
    ).expect("failed to write content to wrapper header file...");
    println!("<= Done");
}

fn run_bindgen(folder: &Path, commit_hash: String) {
    println!("=> Running bindgen...");
    let bindings = bindgen::Builder::default()
        .header(HEADER_FILE)
        .layout_tests(false)
        .clang_args(&[format!("-I{}", folder.display())])
        .generate()
        .expect("Unable to generate bindings");

    println!("<= Done");

    let mut consts = HashSet::new();
    let content = bindings.to_string();
    let mut content = content.lines().collect::<Vec<_>>();
    let mut pos = 0;
    while pos < content.len() {
        if content[pos].starts_with("pub const _") {
            content.remove(pos);
            continue;
        } else if content[pos].starts_with("pub const ") {
            if !consts.insert(content[pos].split(":").next().unwrap()) {
                content.remove(pos);
                continue;
            }
        }
        let should_remove = if let Some(fn_name) = content[pos].trim_start().split("(").next().unwrap().split("pub fn ").skip(1).next() {
            !fn_name.starts_with("gsl_") && !fn_name.starts_with("cblas_")
        } else {
            false
        };
        if should_remove {
            while !content[pos].starts_with("extern \"C\" {") {
                if pos > 0 {
                    pos -= 1;
                } else {
                    break;
                }
            }
            while !content[pos].starts_with("}") && pos < content.len() {
                content.remove(pos);
            }
            if pos < content.len() {
                content.remove(pos);
            }
            continue
        }
        pos += 1;
    }

    let out = "../src/auto.rs";
    println!("=> Writing content into `{}`...", out);

    let mut f = OpenOptions::new().truncate(true).create(true).write(true).open(out).expect("Failed to open binding file...");
    writeln!(f, "// Generated on commit {} from {}", commit_hash, GSL_REPOSITORY).unwrap();
    writeln!(f, "// DO NOT EDIT THIS FILE!!!", ).unwrap();
    writeln!(f, "").unwrap();
    writeln!(f, "{}", content.join("\n")).unwrap();

    println!("<= Done");
}

fn ready_gsl_lib(folder: &Path) {
    if Command::new("git")
        .arg("clone")
        .arg(GSL_REPOSITORY)
        .arg("--depth")
        .arg("1")
        .arg(folder.join("gsl").to_str().expect("failed to convert path to str"))
        .status()
        .is_err()
    {
        panic!("Failed to clone gsl repository...");
    }
    if Command::new("bash")
        .arg("-c")
        .arg(&format!("cd {}/gsl && ./autogen.sh", folder.display()))
        .status()
        .is_err()
    {
        panic!("Failed to run autogen.sh");
    }
    if Command::new("bash")
        .arg("-c")
        .arg(&format!("cd {}/gsl && ./configure", folder.display()))
        .status()
        .is_err()
    {
        panic!("Failed to run configure");
    }
    if Command::new("bash")
        .arg("-c")
        .arg(&format!("cd {}/gsl && make", folder.display()))
        .status()
        .is_err()
    {
        panic!("Failed to run make");
    }
}

fn get_current_commit_hash(folder: &Path) -> String {
    let commit_hash = Command::new("bash")
        .arg("-c")
        .arg(&format!("cd {} && git rev-parse --short HEAD", folder.display()))
        .output()
        .expect("Failed to retrieve current gsl commit hash");
    if !commit_hash.status.success() {
        panic!("Commit hash retrieval failed....");
    }
    String::from_utf8(commit_hash.stdout).expect("Invalid commit hash received...").trim().to_owned()
}

fn run_everything(folder: &Path, ready_gsl: bool) {
    if ready_gsl {
        ready_gsl_lib(folder);
    }
    create_header_file(folder);
    run_bindgen(folder, get_current_commit_hash(folder));
}

fn main() {
    if env::args().skip(1).count() != 0 {
        let dir = env::args().skip(1).next().unwrap();
        println!("Using `{}` path as gsl directory. No initialization will be performed on it", dir);

        run_everything(&Path::new(&dir), false);
        return;
    }

    let dir = tempfile::tempdir().expect("failed to create temporary directory");
    println!("Created temporary directory: {:?}", dir.path());

    run_everything(&dir.path(), true);
}
