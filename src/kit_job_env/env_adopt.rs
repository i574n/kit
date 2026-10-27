#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Ok,
    US0_Error(Rc<str>),
}
fn joined_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_4() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_3(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_4();
    joined_2(v1, v0)
}
fn job_file_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_3(v0);
    joined_2(v2, v1)
}
fn file_exists_5(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn text_or_empty_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn environment_restored_7(mut v0: Rc<str>, mut v1: Rc<str>) -> () {
    { let hidden = |key: &str| key.starts_with(char::from(61u8)); let wanted = (v0).split(char::from(0u8)).filter_map(|pair| pair.char_indices().skip(1).find(|(_, c)| *c == char::from(61u8)).map(|(at, _)| (pair[..at].to_owned(), pair[at + 1..].to_owned()))).filter(|(key, _)| !hidden(key)).collect::<std::collections::HashMap<String, String>>(); for (key, _) in std::env::vars_os() { let key = key.to_string_lossy().into_owned(); if !hidden(&key) && !key.starts_with(&*v1) && !wanted.contains_key(&key) { unsafe { std::env::remove_var(&key); } } } for (key, value) in &wanted { if std::env::var_os(key).map_or(true, |now| now.to_string_lossy() != value.as_str()) { unsafe { std::env::set_var(key, value); } } } };
    ()
}
fn file_exists_8(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn closure1() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> Rc<str>> = Rc::new(move |mut v0: std::io::Error| -> Rc<str> {
        let v1: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v0)).s() });
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    closure1()
}
fn ok_c2() -> Rc<dyn Fn() -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US0> = Rc::new(move || -> US0 {
        US0::US0_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10() -> Rc<dyn Fn() -> US0> {
    ok_c2()
}
fn error_c3() -> Rc<dyn Fn(Rc<str>) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US0> = Rc::new(move |mut v0: Rc<str>| -> US0 {
        US0::US0_Error(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method11() -> Rc<dyn Fn(Rc<str>) -> US0> {
    error_c3()
}
fn job_environment_adopted_0(mut v0: Rc<str>) -> () {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_1(v0.clone(), v1.clone());
    let v3: bool = file_exists_5(v2.clone());
    if v3 {
        let v4: Rc<str> = text_or_empty_6(v2.clone());
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_JOB_"); } LIT.with(|lit| lit.clone()) };
        environment_restored_7(v4.clone(), v5.clone());
        let mut v6: bool = file_exists_8(v2.clone());
        let mut v10: Result<(), std::io::Error> = if v6 {
            let v8: Result<(), std::io::Error> = std::fs::remove_file(&*v2);
            v8
        } else {
            let v9: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
            v9
        };
        let mut v11: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method9();
        let v13: Result<(), Rc<str>> = v10.map_err(|x| v11(x));
        let mut v14: Rc<dyn Fn() -> US0> = method10();
        let mut v15: Rc<dyn Fn(Rc<str>) -> US0> = method11();
        let v16: US0 = match v13 { Ok(()) => v14(), Err(e) => v15(e) };
        match &v16 {
            US0::US0_Error(v17) => {
                let v17: Rc<str> = v17.clone();
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.file_delete: "); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v17));
                eprintln!("{}", v19);
                ()
            }
            US0::US0_Ok => {
                ()
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> ()> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> ()> = Rc::new(move |mut v0: Rc<str>| -> () {
        job_environment_adopted_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_environment_adopted(v0: &str) -> () {
    closure0()(Rc::<str>::from(v0))
}
