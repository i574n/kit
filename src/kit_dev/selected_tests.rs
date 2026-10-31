#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
const DEV_BIN: &str = env!("CARGO_BIN_EXE_kit-dev");
#[cfg(test)] mod native_selected_tests {
struct Fixture(std::path::PathBuf);
impl Fixture { fn new() -> Self { let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos(); let root=std::env::temp_dir().join(format!("kit native workflow ü {} {stamp}",std::process::id())); std::fs::create_dir(&root).unwrap(); Self(root) } }
impl Drop for Fixture { fn drop(&mut self) { let _=std::fs::remove_dir_all(&self.0); } }
#[test] fn selected_packages_preserve_native_build_contracts() { let fixture=Fixture::new(); assert_eq!(super::kit_dev_selected_scenarios(fixture.0.to_str().unwrap()),0); }
}

fn write_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> () {
    { let path=std::path::Path::new(&*v0).join(&*v1); std::fs::create_dir_all(path.parent().unwrap()).unwrap(); std::fs::write(path,v2.as_bytes()).unwrap(); };
    ()
}
fn client_manifest_2(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[package]\nname='kit-coverage-prune'\nversion='0.0.0'\nedition='2024'\n[lib]\npath='lib.rs'\n[package.metadata.kit]\ntest-requires-cli="); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v0));
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v3));
    v4
}
fn require_3(mut v0: bool, mut v1: Rc<str>) -> () {
    if v0 { () } else { std::panic::panic_any(v1.to_string()) };
    ()
}
fn selected_args_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--skip-release"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let v6: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--offline"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let v10: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--package"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let v14: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v13, v14)), v0));
    let v17: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--target-dir"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let v21: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target with spaces ü"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
    v24
}
fn capture_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let mut command=std::process::Command::new(DEV_BIN); command.current_dir(&*v0).env_remove("RUSTFLAGS").env_remove("LLVM_PROFILE_FILE").args(v1.split(char::from(0u8))); let capture=kit_process::run_bounded_capture_with_input(&mut command,120000,None).unwrap(); let status=if capture.status.success() { "success
" } else { "failure
" }; std::rc::Rc::<str>::from([status,&String::from_utf8_lossy(&capture.stdout),&String::from_utf8_lossy(&capture.stderr)].concat()) };
    v2
}
fn selected_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool, mut v3: Rc<str>) -> () {
    let v4: Rc<str> = selected_args_5(v1);
    let v5: Rc<str> = capture_6(v0, v4);
    let v8: Rc<str> = if v2 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("success\n"); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failure\n"); } LIT.with(|lit| lit.clone()) };
        v7
    };
    let v9: bool = v5.starts_with(&*v8);
    require_3(v9, v5.clone());
    let v10: bool = v5.contains(&*v3);
    require_3(v10, v5)
}
fn scenarios_0(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/Cargo.toml"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[workspace]\nmembers=['cli','client','isolated']\nresolver='3'\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v1.clone(), v2.clone());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/cli/Cargo.toml"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[package]\nname='kit-cli'\nversion='0.0.0'\nedition='2024'\n[[bin]]\nname='kit'\npath='main.rs'\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v3.clone(), v4.clone());
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/cli/main.rs"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn main() { println!(\"first\"); }\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v5.clone(), v6.clone());
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/client/Cargo.toml"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("true"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = client_manifest_2(v8.clone());
    write_1(v0.clone(), v7.clone(), v9.clone());
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/client/lib.rs"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#[test] fn fresh_cli_is_available() { let current=std::env::current_exe().unwrap(); let name=format!(\"kit{}\",std::env::consts::EXE_SUFFIX); let binary=current.ancestors().map(|folder| folder.join(&name)).find(|candidate| candidate.is_file()).expect(\"selected test requires the CLI\"); let output=std::process::Command::new(binary).output().expect(\"selected test requires the CLI\"); assert!(output.status.success()); assert_eq!(String::from_utf8(output.stdout).unwrap().trim(),include_str!(\"expected.txt\").trim()); }\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v10.clone(), v11.clone());
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/client/expected.txt"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("first"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v12.clone(), v13.clone());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/isolated/Cargo.toml"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[package]\nname='isolated'\nversion='0.0.0'\nedition='2024'\n[lib]\npath='lib.rs'\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v14.clone(), v15.clone());
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/isolated/lib.rs"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#[test] fn independent() { assert_eq!(2+2,4); }\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v16.clone(), v17.clone());
    let v18: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("generate-lockfile"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--offline"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v24: i32 = { let mut command=std::process::Command::new("cargo"); command.current_dir(std::path::Path::new(&*v0).join("src")); command.args(v22.split(char::from(0u8))); if v23.len() > 0 { command.env("KIT_SPIRAL_COMPILE", &*v23); } if std::env::var_os("CARGO_BUILD_JOBS").is_none() { command.env("CARGO_BUILD_JOBS", "2"); } if std::env::var_os("RUST_TEST_THREADS").is_none() { command.env("RUST_TEST_THREADS", "1"); } match kit_process::run_bounded(&mut command,60000u64) { Ok(status) => status.code().unwrap_or(1), Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); 2 } } };
    let v25: bool = v24 == 0i32;
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fixture lock generation failed"); } LIT.with(|lit| lit.clone()) };
    require_3(v25, v26.clone());
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-coverage-prune"); } LIT.with(|lit| lit.clone()) };
    let v28: bool = true;
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test result: ok"); } LIT.with(|lit| lit.clone()) };
    selected_4(v0.clone(), v27.clone(), v28, v29.clone());
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn main() { println!(\"second\"); }\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v5.clone(), v30.clone());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("second"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v12.clone(), v31.clone());
    let v32: bool = true;
    selected_4(v0.clone(), v27.clone(), v32, v29.clone());
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile_error!(\"unselected CLI must not compile\"); fn main() {}\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v5.clone(), v33.clone());
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("isolated"); } LIT.with(|lit| lit.clone()) };
    let v35: bool = true;
    selected_4(v0.clone(), v34.clone(), v35, v29.clone());
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing-package"); } LIT.with(|lit| lit.clone()) };
    let v37: bool = false;
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unknown selected Cargo package"); } LIT.with(|lit| lit.clone()) };
    selected_4(v0.clone(), v36.clone(), v37, v38.clone());
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'yes'"); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = client_manifest_2(v39.clone());
    write_1(v0.clone(), v7.clone(), v40.clone());
    let v41: bool = false;
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-requires-cli must be a boolean"); } LIT.with(|lit| lit.clone()) };
    selected_4(v0.clone(), v27.clone(), v41, v42.clone());
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--skip-release"); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = capture_6(v0.clone(), v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("require --test"); } LIT.with(|lit| lit.clone()) };
    let v46: bool = v44.contains(&*v45);
    require_3(v46, v44.clone());
    let v47: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--timeout-ms"); } LIT.with(|lit| lit.clone()) };
    let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v47));
    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
    let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v50));
    let v52: Rc<str> = capture_6(v0.clone(), v51.clone());
    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--timeout-ms must be between"); } LIT.with(|lit| lit.clone()) };
    let v54: bool = v52.contains(&*v53);
    require_3(v54, v52.clone());
    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--package"); } LIT.with(|lit| lit.clone()) };
    let v56: Rc<str> = capture_6(v0.clone(), v55.clone());
    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing value"); } LIT.with(|lit| lit.clone()) };
    let v58: bool = v56.contains(&*v57);
    require_3(v58, v56.clone());
    let v59: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(DEV_BIN).parent().unwrap().parent().unwrap().to_str().unwrap());
    let v60: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--target-dir"); } LIT.with(|lit| lit.clone()) };
    let v63: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v61, v60)), v59));
    let v64: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v64));
    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test"); } LIT.with(|lit| lit.clone()) };
    let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v66));
    let v68: Rc<str> = capture_6(v0.clone(), v67.clone());
    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bootstrap kit-dev with a separate --target-dir"); } LIT.with(|lit| lit.clone()) };
    let v70: bool = v68.contains(&*v69);
    require_3(v70, v68.clone());
    let v71: Rc<str> = selected_args_5(v34.clone());
    let v72: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v72));
    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dry-run"); } LIT.with(|lit| lit.clone()) };
    let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v74));
    let v76: Rc<str> = capture_6(v0.clone(), v75.clone());
    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target with spaces ü"); } LIT.with(|lit| lit.clone()) };
    let v78: bool = v76.contains(&*v77);
    require_3(v78, v76.clone());
    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cargo \"test\""); } LIT.with(|lit| lit.clone()) };
    let v80: bool = v76.contains(&*v79);
    require_3(v80, v76.clone());
    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#[test] fn deliberate_failure() { panic!(\"deliberate native workflow test failure\"); }\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v16.clone(), v81.clone());
    let v82: bool = false;
    let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deliberate native workflow test failure"); } LIT.with(|lit| lit.clone()) };
    selected_4(v0.clone(), v34.clone(), v82, v83.clone());
    0i32
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        scenarios_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_dev_selected_scenarios(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
