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
#[derive(Clone)]
enum US0 {
    US0_Box,
    US0_Node,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US3 {
    US3_Rust,
    US3_Gleam,
    US3_Zig,
    US3_CBackend,
    US3_Lua,
    US3_Wasm,
    US3_Fsharp,
}
#[derive(Clone)]
enum US4 {
    US4_Compile,
    US4_Rustc,
    US4_Run,
    US4_SlotWait,
    US4_DotnetStartup,
    US4_Mount,
    US4_LockWait,
}
#[derive(Clone)]
enum US2 {
    US2_Duration(US3, US4, u64),
    US2_JobDuration(Rc<str>, u64),
}
#[derive(Clone)]
enum US5 {
    US5_PeakBytes(Rc<str>, u64),
}
#[derive(Clone)]
enum US6 {
    US6_Count(Rc<str>, u64),
}
#[derive(Clone)]
enum US1 {
    US1_TimedSample(US0, US2),
    US1_SizedSample(US0, US5),
    US1_CountedSample(US0, US6),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US1, Rc<UH1>),
}
#[derive(Clone)]
enum US7 {
    US7_Some(u64),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_Some(US1),
    US8_None,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
pub struct Mut1 { pub l0: Rc<str> }
fn joined_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_7() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_7();
    joined_5(v1, v0)
}
fn job_file_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_6(v0);
    joined_5(v2, v1)
}
fn text_or_empty_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method10(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US7> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US7> = Rc::new(move |mut v0: (u64)| -> US7 {
        let mut v1: u64 = (v0);
        US7::US7_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_9(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method10(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US7> = closure1();
    let mut v4: Option<US7> = v2.map(|x| v3(x));
    let mut v6: US7 = US7::US7_None;
    let v7: US7 = v6.clone();
    let v8: US7 = v4.unwrap_or(v7);
    match v8 {
        US7::US7_None => {
            0u64
        }
        US7::US7_Some(v9) => {
            let mut v9: u64 = v9;
            v9
        }
    }
}
fn now_ms_11() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_3(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_4(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_8(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = job_file_4(v0.clone(), v4);
    let mut v6: Rc<str> = text_or_empty_8(v5);
    let v7: u64 = decimal_u64_9(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = job_file_4(v0, v8);
    let v10: Rc<str> = text_or_empty_8(v9);
    let v11: u64 = decimal_u64_9(v10);
    let v12: u64 = now_ms_11();
    (v3, v7, v11, v12)
}
fn method12(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
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
fn method13(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
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
fn elapsed_of_14(mut v0: Rc<str>) -> u64 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_3(v0);
    let v5: bool = &*v1 == "";
    let v8: u64 = if v5 {
        v4
    } else {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.split(char::from(9u8)).nth(1).unwrap_or("0"));
        decimal_u64_9(v6)
    };
    let mut v9: bool = v8 > v3;
    if v9 {
        let v10: u64 = v8.wrapping_sub(v3);
        v10
    } else {
        0u64
    }
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method19(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method19(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method18(v16, v17)
    }
}
fn lane_of_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = split_list_17(v1, v0.clone());
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH0::UH0_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let mut v6: Rc<UH0> = v6;
                    v3
                }
                _ => {
                    v0
                }
            }
        }
        _ => {
            v0
        }
    }
}
fn job_sample_15(mut v0: US0, mut v1: Rc<str>, mut v2: u64, mut v3: bool) -> US8 {
    let v5: bool = if v3 {
        let mut v4: bool = v2 > 0u64;
        v4
    } else {
        false
    };
    if v5 {
        let mut v6: Rc<str> = lane_of_16(v1);
        let v7: US2 = US2::US2_JobDuration(v6, v2);
        let v8: US1 = US1::US1_TimedSample(v0, v7);
        US8::US8_Some(v8)
    } else {
        US8::US8_None
    }
}
fn method2(mut v0: US0, mut v1: u64, mut v2: Rc<UH0>, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let (mut v6, mut v7, mut v8, mut v9): (Rc<str>, u64, u64, u64) = job_record_of_3(v4.clone());
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
                let v11: Rc<str> = job_file_4(v4.clone(), v10);
                let v12: Rc<str> = text_or_empty_8(v11);
                let v13: i32 = (v12.clone().len() as i32);
                let v14: i32 = 0i32;
                let v15: i32 = method12(v12.clone(), v13, v14);
                let v16: i32 = v13.wrapping_sub(1i32);
                let v17: Rc<str> = string_slice(&v12, v15 as i64, v16 as i64);
                let v18: i32 = (v17.clone().len() as i32);
                let v19: i32 = method13(v17.clone(), v18);
                let v20: Rc<str> = string_slice(&v17, 0i32 as i64, v19 as i64);
                let v21: bool = &*v6 != "";
                let v23: bool = if v21 {
                    let v22: bool = v8 >= v1;
                    v22
                } else {
                    false
                };
                let v24: u64 = elapsed_of_14(v4);
                let v25: US8 = job_sample_15(v0.clone(), v20, v24, v23);
                let v29: Rc<UH1> = match v25 {
                    US8::US8_None => {
                        v3
                    }
                    US8::US8_Some(v26) => {
                        let v26: US1 = v26;
                        Rc::new(UH1::UH1_Cons(v26, v3))
                    }
                };
                (v2, v3) = (v5, v29);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn directory_exists_20(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method21(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            if v6 {
                Rc::new(UH0::UH0_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn job_samples_1(mut v0: US0, mut v1: u64) -> Rc<UH1> {
    let mut v2: Rc<str> = jobs_root_7();
    let mut v3: bool = directory_exists_20(v2.clone());
    let mut v4: bool = v3 == false;
    if v4 {
        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v7: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v2).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v9: Vec<std::string::String> = v7.unwrap();
        let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v12: bool = true; let _vec_fold_ = v9.into_iter().fold(v10, |acc, x| { //;
        let v14: Rc<UH0> = acc;
        let v16: std::string::String = x;
        let v18: Rc<str> = Rc::<str>::from(String::as_str(&v16));
        let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v14));
        let v21: bool = true; v20 });
        let v23: Rc<UH0> = _vec_fold_;
        let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v25: Rc<UH0> = method21(v23, v24);
        let v26: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method2(v0, v1, v25, v26)
    }
}
fn method24(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_23(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method24(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method22(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: US1 = v2;
            let mut v3: Rc<UH1> = v3;
            let mut v4: Rc<UH0> = method22(v3, v1);
            let v122: Rc<str> = match v2 {
                US1::US1_CountedSample(v98, v99) => {
                    let v98: US0 = v98;
                    let v99: US6 = v99;
                    let v103: Rc<str> = match v98 {
                        US0::US0_Box => {
                            let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("box"); } LIT.with(|lit| lit.clone()) };
                            v100
                        }
                        US0::US0_Node => {
                            let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node"); } LIT.with(|lit| lit.clone()) };
                            v101
                        }
                    };
                    let v104: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("machine=\""); } LIT.with(|lit| lit.clone()) }, v103));
                    let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v104, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                    match v99 {
                        US6::US6_Count(v106, v107) => {
                            let v106: Rc<str> = v106;
                            let v107: u64 = v107;
                            let v108: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("what=\""); } LIT.with(|lit| lit.clone()) }, v106));
                            let v109: Rc<str> = Rc::<str>::from(format!("{}{}", v108, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                            let v110: Rc<str> = format_real_23(v107);
                            let mut v111: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit_exec_count{"); } LIT.with(|lit| lit.clone()) };
                            v111.borrow_mut().push(v112);
                            v111.borrow_mut().push(v105);
                            let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            v111.borrow_mut().push(v113);
                            v111.borrow_mut().push(v109);
                            let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v111.borrow_mut().push(v114);
                            v111.borrow_mut().push(v110);
                            let v115: Rc<Vec<Rc<str>>> = Rc::new(v111.borrow().clone());
                            let v116: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v115).as_ref().clone()));
                            let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v118: Rc<str> = Rc::<str>::from((*v116.borrow()).join(&*v117));
                            v118
                        }
                    }
                }
                US1::US1_SizedSample(v76, v77) => {
                    let v76: US0 = v76;
                    let v77: US5 = v77;
                    let v81: Rc<str> = match v76 {
                        US0::US0_Box => {
                            let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("box"); } LIT.with(|lit| lit.clone()) };
                            v78
                        }
                        US0::US0_Node => {
                            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node"); } LIT.with(|lit| lit.clone()) };
                            v79
                        }
                    };
                    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("machine=\""); } LIT.with(|lit| lit.clone()) }, v81));
                    let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v82, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                    match v77 {
                        US5::US5_PeakBytes(v84, v85) => {
                            let v84: Rc<str> = v84;
                            let v85: u64 = v85;
                            let v86: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("what=\""); } LIT.with(|lit| lit.clone()) }, v84));
                            let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v86, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                            let v88: Rc<str> = format_real_23(v85);
                            let mut v89: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit_exec_peak_bytes{"); } LIT.with(|lit| lit.clone()) };
                            v89.borrow_mut().push(v90);
                            v89.borrow_mut().push(v83);
                            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            v89.borrow_mut().push(v91);
                            v89.borrow_mut().push(v87);
                            let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v89.borrow_mut().push(v92);
                            v89.borrow_mut().push(v88);
                            let v93: Rc<Vec<Rc<str>>> = Rc::new(v89.borrow().clone());
                            let v94: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v93).as_ref().clone()));
                            let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v96: Rc<str> = Rc::<str>::from((*v94.borrow()).join(&*v95));
                            v96
                        }
                    }
                }
                US1::US1_TimedSample(v5, v6) => {
                    let v5: US0 = v5;
                    let mut v6: US2 = v6;
                    let v10: Rc<str> = match v5 {
                        US0::US0_Box => {
                            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("box"); } LIT.with(|lit| lit.clone()) };
                            v7
                        }
                        US0::US0_Node => {
                            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node"); } LIT.with(|lit| lit.clone()) };
                            v8
                        }
                    };
                    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("machine=\""); } LIT.with(|lit| lit.clone()) }, v10));
                    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                    match v6 {
                        US2::US2_Duration(v13, v14, v15) => {
                            let v13: US3 = v13;
                            let v14: US4 = v14;
                            let v15: u64 = v15;
                            let v29: Rc<str> = match v13 {
                                US3::US3_CBackend => {
                                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c"); } LIT.with(|lit| lit.clone()) };
                                    v19
                                }
                                US3::US3_Fsharp => {
                                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fsharp"); } LIT.with(|lit| lit.clone()) };
                                    v22
                                }
                                US3::US3_Gleam => {
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
                                    v17
                                }
                                US3::US3_Lua => {
                                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lua"); } LIT.with(|lit| lit.clone()) };
                                    v20
                                }
                                US3::US3_Rust => {
                                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust"); } LIT.with(|lit| lit.clone()) };
                                    v16
                                }
                                US3::US3_Wasm => {
                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wasm"); } LIT.with(|lit| lit.clone()) };
                                    v21
                                }
                                US3::US3_Zig => {
                                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
                                    v18
                                }
                            };
                            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backend=\""); } LIT.with(|lit| lit.clone()) }, v29));
                            let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                            let v45: Rc<str> = match v14 {
                                US4::US4_Compile => {
                                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
                                    v32
                                }
                                US4::US4_DotnetStartup => {
                                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dotnet_startup"); } LIT.with(|lit| lit.clone()) };
                                    v36
                                }
                                US4::US4_LockWait => {
                                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lock_wait"); } LIT.with(|lit| lit.clone()) };
                                    v38
                                }
                                US4::US4_Mount => {
                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mount"); } LIT.with(|lit| lit.clone()) };
                                    v37
                                }
                                US4::US4_Run => {
                                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
                                    v34
                                }
                                US4::US4_Rustc => {
                                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustc"); } LIT.with(|lit| lit.clone()) };
                                    v33
                                }
                                US4::US4_SlotWait => {
                                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_wait"); } LIT.with(|lit| lit.clone()) };
                                    v35
                                }
                            };
                            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("phase=\""); } LIT.with(|lit| lit.clone()) }, v45));
                            let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                            let mut v48: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            v48.borrow_mut().push(v12);
                            v48.borrow_mut().push(v31);
                            v48.borrow_mut().push(v47);
                            let v49: Rc<Vec<Rc<str>>> = Rc::new(v48.borrow().clone());
                            let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v49).as_ref().clone()));
                            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            let v52: Rc<str> = Rc::<str>::from((*v50.borrow()).join(&*v51));
                            let v53: Rc<str> = format_real_23(v15);
                            let mut v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit_exec_duration_ms{"); } LIT.with(|lit| lit.clone()) };
                            v54.borrow_mut().push(v55);
                            v54.borrow_mut().push(v52);
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v54.borrow_mut().push(v56);
                            v54.borrow_mut().push(v53);
                            let v57: Rc<Vec<Rc<str>>> = Rc::new(v54.borrow().clone());
                            let v58: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v57).as_ref().clone()));
                            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v60: Rc<str> = Rc::<str>::from((*v58.borrow()).join(&*v59));
                            v60
                        }
                        US2::US2_JobDuration(v61, v62) => {
                            let v61: Rc<str> = v61;
                            let v62: u64 = v62;
                            let v63: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lane=\""); } LIT.with(|lit| lit.clone()) }, v61));
                            let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                            let v65: Rc<str> = format_real_23(v62);
                            let mut v66: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit_job_duration_ms{"); } LIT.with(|lit| lit.clone()) };
                            v66.borrow_mut().push(v67);
                            v66.borrow_mut().push(v12);
                            let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            v66.borrow_mut().push(v68);
                            v66.borrow_mut().push(v64);
                            let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v66.borrow_mut().push(v69);
                            v66.borrow_mut().push(v65);
                            let v70: Rc<Vec<Rc<str>>> = Rc::new(v66.borrow().clone());
                            let v71: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v70).as_ref().clone()));
                            let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v73: Rc<str> = Rc::<str>::from((*v71.borrow()).join(&*v72));
                            v73
                        }
                    }
                }
            };
            Rc::new(UH0::UH0_Cons(v122, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method25(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn jobs_prom_0(mut v0: u64, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = &*v1 == "node";
    let v5: US0 = if v2 {
        US0::US0_Node
    } else {
        US0::US0_Box
    };
    let mut v6: Rc<UH1> = job_samples_1(v5, v0);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = method22(v6, v7);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v10: i32 = 0i32;
    let v11: i32 = method25(v9.clone(), v8, v10);
    let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
    let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v14));
    v15
}
fn closure0() -> Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> Rc<str> {
        jobs_prom_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_exec_jobs_prom(v0: &str, v1: u64) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), v1)
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
