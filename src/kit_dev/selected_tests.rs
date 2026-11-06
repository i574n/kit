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
    let v1: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[package]\nname='kit-coverage-prune'\nversion='0.0.0'\nedition='2024'\n[lib]\npath='lib.rs'\n[package.metadata.kit]\ntest-requires-cli="); } LIT.with(|lit| lit.clone()) }, v0));
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
    v2
}
fn require_3(mut v0: bool, mut v1: Rc<str>) -> () {
    if v0 { () } else { std::panic::panic_any(v1.to_string()) };
    ()
}
fn selected_args_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test"); } LIT.with(|lit| lit.clone()) }, v1));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--skip-release"); } LIT.with(|lit| lit.clone()) }));
    let v4: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--offline"); } LIT.with(|lit| lit.clone()) }));
    let v7: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--package"); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v0));
    let v13: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--target-dir"); } LIT.with(|lit| lit.clone()) }));
    let v16: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target with spaces ü"); } LIT.with(|lit| lit.clone()) }));
    v18
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
    write_1(v0.clone(), v1, v2);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/cli/Cargo.toml"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[package]\nname='kit-cli'\nversion='0.0.0'\nedition='2024'\n[[bin]]\nname='kit'\npath='main.rs'\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v3, v4);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/cli/main.rs"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn main() { println!(\"first\"); }\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v5.clone(), v6);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/client/Cargo.toml"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("true"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = client_manifest_2(v8);
    write_1(v0.clone(), v7.clone(), v9);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/client/lib.rs"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#[test] fn fresh_cli_is_available() { let current=std::env::current_exe().unwrap(); let name=format!(\"kit{}\",std::env::consts::EXE_SUFFIX); let binary=current.ancestors().map(|folder| folder.join(&name)).find(|candidate| candidate.is_file()).expect(\"selected test requires the CLI\"); let output=std::process::Command::new(binary).output().expect(\"selected test requires the CLI\"); assert!(output.status.success()); assert_eq!(String::from_utf8(output.stdout).unwrap().trim(),include_str!(\"expected.txt\").trim()); }\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v10, v11);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/client/expected.txt"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("first"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v12.clone(), v13);
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/isolated/Cargo.toml"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[package]\nname='isolated'\nversion='0.0.0'\nedition='2024'\n[lib]\npath='lib.rs'\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v14, v15);
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/isolated/lib.rs"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#[test] fn independent() { assert_eq!(2+2,4); }\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v16.clone(), v17);
    let v18: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("generate-lockfile"); } LIT.with(|lit| lit.clone()) }, v18));
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--offline"); } LIT.with(|lit| lit.clone()) }));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v22: i32 = { let mut command=std::process::Command::new("cargo"); command.current_dir(std::path::Path::new(&*v0).join("src")); command.args(v20.split(char::from(0u8))); if v21.len() > 0 { command.env("KIT_SPIRAL_COMPILE", &*v21); } if std::env::var_os("CARGO_BUILD_JOBS").is_none() { command.env("CARGO_BUILD_JOBS", "2"); } if std::env::var_os("RUST_TEST_THREADS").is_none() { command.env("RUST_TEST_THREADS", "1"); } match kit_process::run_bounded(&mut command,60000u64) { Ok(status) => status.code().unwrap_or(1), Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); 2 } } };
    let v23: bool = v22 == 0i32;
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fixture lock generation failed"); } LIT.with(|lit| lit.clone()) };
    require_3(v23, v24);
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-coverage-prune"); } LIT.with(|lit| lit.clone()) };
    let v26: bool = true;
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test result: ok"); } LIT.with(|lit| lit.clone()) };
    selected_4(v0.clone(), v25.clone(), v26, v27.clone());
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn main() { println!(\"second\"); }\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v5.clone(), v28);
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("second"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v12, v29);
    let v30: bool = true;
    selected_4(v0.clone(), v25.clone(), v30, v27.clone());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile_error!(\"unselected CLI must not compile\"); fn main() {}\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v5, v31);
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("isolated"); } LIT.with(|lit| lit.clone()) };
    let v33: bool = true;
    selected_4(v0.clone(), v32.clone(), v33, v27);
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing-package"); } LIT.with(|lit| lit.clone()) };
    let v35: bool = false;
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unknown selected Cargo package"); } LIT.with(|lit| lit.clone()) };
    selected_4(v0.clone(), v34, v35, v36);
    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'yes'"); } LIT.with(|lit| lit.clone()) };
    let v38: Rc<str> = client_manifest_2(v37);
    write_1(v0.clone(), v7, v38);
    let v39: bool = false;
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-requires-cli must be a boolean"); } LIT.with(|lit| lit.clone()) };
    selected_4(v0.clone(), v25, v39, v40);
    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--skip-release"); } LIT.with(|lit| lit.clone()) };
    let v42: Rc<str> = capture_6(v0.clone(), v41);
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("require --test"); } LIT.with(|lit| lit.clone()) };
    let v44: bool = v42.contains(&*v43);
    require_3(v44, v42);
    let v45: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v46: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--timeout-ms"); } LIT.with(|lit| lit.clone()) }, v45));
    let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }));
    let v48: Rc<str> = capture_6(v0.clone(), v47);
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--timeout-ms must be between"); } LIT.with(|lit| lit.clone()) };
    let v50: bool = v48.contains(&*v49);
    require_3(v50, v48);
    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--package"); } LIT.with(|lit| lit.clone()) };
    let v52: Rc<str> = capture_6(v0.clone(), v51);
    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing value"); } LIT.with(|lit| lit.clone()) };
    let v54: bool = v52.contains(&*v53);
    require_3(v54, v52);
    let v55: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(DEV_BIN).parent().unwrap().parent().unwrap().to_str().unwrap());
    let v56: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v57: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--target-dir"); } LIT.with(|lit| lit.clone()) }, v56));
    let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v55));
    let v59: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
    let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test"); } LIT.with(|lit| lit.clone()) }));
    let v62: Rc<str> = capture_6(v0.clone(), v61);
    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bootstrap kit-dev with a separate --target-dir"); } LIT.with(|lit| lit.clone()) };
    let v64: bool = v62.contains(&*v63);
    require_3(v64, v62);
    let v65: Rc<str> = selected_args_5(v32.clone());
    let v66: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v66));
    let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dry-run"); } LIT.with(|lit| lit.clone()) }));
    let v69: Rc<str> = capture_6(v0.clone(), v68);
    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target with spaces ü"); } LIT.with(|lit| lit.clone()) };
    let v71: bool = v69.contains(&*v70);
    require_3(v71, v69.clone());
    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cargo \"test\""); } LIT.with(|lit| lit.clone()) };
    let v73: bool = v69.contains(&*v72);
    require_3(v73, v69);
    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#[test] fn deliberate_failure() { panic!(\"deliberate native workflow test failure\"); }\n"); } LIT.with(|lit| lit.clone()) };
    write_1(v0.clone(), v16, v74);
    let v75: bool = false;
    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deliberate native workflow test failure"); } LIT.with(|lit| lit.clone()) };
    selected_4(v0, v32, v75, v76);
    0i32
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        scenarios_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_dev_selected_scenarios(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
