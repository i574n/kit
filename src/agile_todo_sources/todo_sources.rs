#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
fn walked_names_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: i32 = (v2.clone().len() as i32);
        let v6: bool = v5 <= v3;
        if v6 {
            return v4.clone();
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v2.get(v3 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v3 as usize) as i32).unwrap_or(-1);
            let v9: bool = v8 < 0i32;
            let v10: i32 = if v9 {
                v5
            } else {
                v8
            };
            let v11: Rc<str> = std::rc::Rc::<str>::from(v2.get(v3 as usize..v10 as usize).unwrap_or(""));
            let v12: i32 = v10.wrapping_add(1i32);
            let v13: bool = &*v0 == "";
            let v17: Rc<str> = if v13 {
                v11.clone()
            } else {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v14)), v11));
                v16.clone()
            };
            let v18: bool = &*v1 == "";
            let v22: Rc<str> = if v18 {
                v11.clone()
            } else {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v19)), v11));
                v21.clone()
            };
            let v23: bool = &*v11 == "";
            let v46: Rc<str> = if v23 {
                v4.clone()
            } else {
                let v24: i32 = std::fs::symlink_metadata(&*v17).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
                let v25: bool = v24 == 2i32;
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules|vendor|third_party|target|dist|_build|.zig-cache|zig-out|.lake|__pycache__|.venv|venv|.gradle|.next|bin|obj|deps|.git|.cache|elm-stuff|bower_components|.dart_tool|.mypy_cache|.pytest_cache|.tox"); } LIT.with(|lit| lit.clone()) };
                let v27: bool = v26.split("|").any(|item| item == &*v11);
                let v28: bool = v25 && v27;
                if v28 {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                    let v30: bool = &*v22 == "";
                    let v34: Rc<str> = if v30 {
                        v29.clone()
                    } else {
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v22, v31)), v29));
                        v33.clone()
                    };
                    let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v34));
                    let v36: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
                    v37.clone()
                } else {
                    let v38: i32 = std::fs::symlink_metadata(&*v17).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
                    let v39: bool = v38 == 2i32;
                    if v39 {
                        walked_directory_1(v17.clone(), v22.clone(), v4.clone())
                    } else {
                        let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v22));
                        let v42: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
                        v43.clone()
                    }
                }
            };
            (v3, v4) = (v12, v46.clone());
            continue;
        }
    }
}
fn walked_directory_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_list_names(std::path::Path::new(&*v0)).unwrap_or_default().join(&char::from(10u8).to_string()).as_str());
    let v4: i32 = 0i32;
    walked_names_2(v0, v1, v3, v4, v2)
}
fn todo_sources_0(mut v0: Rc<str>) -> (bool, Rc<str>) {
    let mut v1: i32 = std::fs::symlink_metadata(&*v0).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
    let mut v2: bool = v1 == 2i32;
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rev-parse\t--is-inside-work-tree"); } LIT.with(|lit| lit.clone()) };
        let (mut v4, mut v5): (bool, Rc<str>) = { let mut command = std::process::Command::new("git"); command.arg("-C").arg(&*v0).args(v3.split(char::from(9u8))); match kit_process::run_bounded_capture_with_input(&mut command, 60000u64, None) { Ok(capture) => (capture.termination == kit_process::ProcessTermination::Exited(0), std::rc::Rc::<str>::from(String::from_utf8_lossy(&capture.stdout).as_ref())), Err(error) => (false, std::rc::Rc::<str>::from(error)) } };
        if v4 {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ls-files\t-co\t--exclude-standard\t-z"); } LIT.with(|lit| lit.clone()) };
            let (mut v7, mut v8): (bool, Rc<str>) = { let mut command = std::process::Command::new("git"); command.arg("-C").arg(&*v0).args(v6.split(char::from(9u8))); match kit_process::run_bounded_capture_with_input(&mut command, 60000u64, None) { Ok(capture) => (capture.termination == kit_process::ProcessTermination::Exited(0), std::rc::Rc::<str>::from(String::from_utf8_lossy(&capture.stdout).as_ref())), Err(error) => (false, std::rc::Rc::<str>::from(error)) } };
            (v7, v8.clone())
        } else {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = walked_directory_1(v0.clone(), v9.clone(), v9.clone());
            (true, v10.clone())
        }
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, v13.clone())
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (bool, Rc<str>) {
        todo_sources_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method5(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn git_head_3(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rev-parse\tHEAD"); } LIT.with(|lit| lit.clone()) };
    let (mut v2, mut v3): (bool, Rc<str>) = { let mut command = std::process::Command::new("git"); command.arg("-C").arg(&*v0).args(v1.split(char::from(9u8))); match kit_process::run_bounded_capture_with_input(&mut command, 60000u64, None) { Ok(capture) => (capture.termination == kit_process::ProcessTermination::Exited(0), std::rc::Rc::<str>::from(String::from_utf8_lossy(&capture.stdout).as_ref())), Err(error) => (false, std::rc::Rc::<str>::from(error)) } };
    if v2 {
        let v4: i32 = (v3.clone().len() as i32);
        let v5: i32 = 0i32;
        let v6: i32 = method4(v3.clone(), v4, v5);
        let v7: i32 = v4.wrapping_sub(1i32);
        let v8: Rc<str> = string_slice(&v3, v6 as i64, v7 as i64);
        let v9: i32 = (v8.clone().len() as i32);
        let v10: i32 = method5(v8.clone(), v9);
        let v11: Rc<str> = string_slice(&v8, 0i32 as i64, v10 as i64);
        v11
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        git_head_3(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn git_changed_since_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff\t--name-only\t--relative\t-z\t"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    let (mut v4, mut v5): (bool, Rc<str>) = { let mut command = std::process::Command::new("git"); command.arg("-C").arg(&*v0).args(v3.split(char::from(9u8))); match kit_process::run_bounded_capture_with_input(&mut command, 60000u64, None) { Ok(capture) => (capture.termination == kit_process::ProcessTermination::Exited(0), std::rc::Rc::<str>::from(String::from_utf8_lossy(&capture.stdout).as_ref())), Err(error) => (false, std::rc::Rc::<str>::from(error)) } };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ls-files\t-o\t--exclude-standard\t-z"); } LIT.with(|lit| lit.clone()) };
    let (mut v7, mut v8): (bool, Rc<str>) = { let mut command = std::process::Command::new("git"); command.arg("-C").arg(&*v0).args(v6.split(char::from(9u8))); match kit_process::run_bounded_capture_with_input(&mut command, 60000u64, None) { Ok(capture) => (capture.termination == kit_process::ProcessTermination::Exited(0), std::rc::Rc::<str>::from(String::from_utf8_lossy(&capture.stdout).as_ref())), Err(error) => (false, std::rc::Rc::<str>::from(error)) } };
    let v9: bool = v4 && v7;
    if v9 {
        let v10: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v5)), v8));
        v12
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        git_changed_since_6(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_todo_sources(v0: &str) -> (bool, Rc<str>) {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_agile_todo_git_head(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
pub fn kit_agile_todo_git_changed(v0: &str, v1: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
