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
enum UH0 {
    UH0_Nil,
    UH0_Cons(u64, u64, bool, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_IdleOrphanHost(u64),
    US1_OrphanCompilerServer(u64, u64),
    US1_SettledButSupervised(u64, Rc<str>),
    US1_WaitOnSettled(u64, Rc<str>),
    US1_WaitOnMissing(u64, Rc<str>),
    US1_ChainToMissing(Rc<str>, Rc<str>),
    US1_HostEnvironmentDiffers(Rc<str>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US1, Rc<UH1>),
}
pub struct Mut0 { pub l0: Rc<str> }
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: u64 = v2;
                let v3: u64 = v3;
                let v4: bool = v4;
                let v5: Rc<UH0> = v5;
                let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method4(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method6(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn method7(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn servers_from_2(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method3(v3, v5);
        } else {
            let v7: Rc<str> = v0[v1 as usize].3.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--server-tcp"); } LIT.with(|lit| lit.clone()) };
            let v9: bool = v7.contains(&*v8);
            let v12: bool = if v9 {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral-cells-"); } LIT.with(|lit| lit.clone()) };
                let v11: bool = v7.contains(&*v10);
                v11
            } else {
                false
            };
            let v13: bool = v12 == false;
            if v13 {
                let v14: i32 = v1.wrapping_add(1i32);
                v1 = v14;
                continue;
            } else {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral-cells-"); } LIT.with(|lit| lit.clone()) };
                let v17: Option<usize> = v7.find(&*v16);
                let v18: Option<usize> = method4(v17);
                let mut v19: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v20: Option<i32> = v18.map(|x| v19(x));
                let v22: i32 = v20.unwrap_or(-1i32);
                let v23: bool = v22 < 0i32;
                let v48: Rc<str> = if v23 {
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v24
                } else {
                    let v25: i32 = (v16.len() as i32);
                    let v26: i32 = v22.wrapping_add(v25);
                    let v27: i32 = (v7.clone().len() as i32);
                    let v28: i32 = v27.wrapping_sub(v22);
                    let v29: i32 = v28.wrapping_sub(v25);
                    let v30: usize = ((v26) as usize);
                    let v31: i32 = v26.wrapping_add(v29);
                    let v32: usize = ((v31) as usize);
                    let v33: Rc<str> = Rc::<str>::from(&(v7)[v30..v32]);
                    let v34: i32 = (v33.clone().len() as i32);
                    let v35: i32 = 0i32;
                    let v36: i32 = method5(v33.clone(), v34, v35);
                    let v37: i32 = v34.wrapping_sub(1i32);
                    let v38: Rc<str> = string_slice(&v33, v36 as i64, v37 as i64);
                    let v39: i32 = (v38.clone().len() as i32);
                    let v40: i32 = method6(v38.clone(), v39);
                    let v41: Rc<str> = string_slice(&v38, 0i32 as i64, v40 as i64);
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v41.split(&*v42).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                    let v44: Rc<str> = v43.borrow()[0i32 as usize].clone();
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v47: Rc<str> = Rc::<str>::from(v44.replace(&*v45, &*v46));
                    v47
                };
                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v48.split(&*v49).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let v51: Rc<str> = v50.borrow()[0i32 as usize].clone();
                let v52: Option<u64> = (v51).trim().parse::<u64>().ok();
                let v53: Option<u64> = method7(v52);
                let mut v54: Rc<dyn Fn((u64)) -> US0> = closure2();
                let mut v55: Option<US0> = v53.map(|x| v54(x));
                let v57: US0 = US0::US0_None;
                let v58: US0 = v57.clone();
                let v59: US0 = v55.unwrap_or(v58);
                let v62: u64 = match v59 {
                    US0::US0_None => {
                        0u64
                    }
                    US0::US0_Some(v60) => {
                        let v60: u64 = v60;
                        v60
                    }
                };
                let v63: u64 = v0[v1 as usize].0;
                let v64: i32 = v0.iter().position(|row| row.0 == v62).map(|at| at as i32).unwrap_or(-1);
                let v65: bool = v64 >= 0i32;
                let v66: i32 = v1.wrapping_add(1i32);
                let v67: Rc<UH0> = Rc::new(UH0::UH0_Cons(v63, v62, v65, v3));
                (v1, v3) = (v66, v67);
                continue;
            }
        }
    }
}
fn process_rows_sampled_8(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> {
    let mut v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = kit_machine_sample::kit_machine_sampled(v0);
    v1
}
fn server_facts_1() -> Rc<UH0> {
    let mut v0: u64 = 50u64;
    let mut v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = process_rows_sampled_8(v0);
    let mut v2: i32 = 0i32;
    let v3: i32 = v1.len() as i32;
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    servers_from_2(v1, v2, v3, v4)
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: u64 = v2;
                let v3: u64 = v3;
                let v4: bool = v4;
                let v5: Rc<UH0> = v5;
                let v7: bool = if v4 {
                    true
                } else {
                    let v6: bool = v3 == 0u64;
                    v6
                };
                let v10: Rc<UH1> = if v7 {
                    v1
                } else {
                    let v8: US1 = US1::US1_OrphanCompilerServer(v2, v3);
                    Rc::new(UH1::UH1_Cons(v8, v1))
                };
                (v0, v1) = (v5, v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method10(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: US1 = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method13(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_12(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method13(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method11(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: US1 = v2;
                let v3: Rc<UH1> = v3;
                let v4: bool = &*v1 == "";
                let v95: Rc<str> = if v4 {
                    match v2 {
                        US1::US1_ChainToMissing(v35, v36) => {
                            let v35: Rc<str> = v35;
                            let v36: Rc<str> = v36;
                            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chain-to-missing job="); } LIT.with(|lit| lit.clone()) }, v35));
                            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after="); } LIT.with(|lit| lit.clone()) }));
                            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v36));
                            v39
                        }
                        US1::US1_HostEnvironmentDiffers(v40) => {
                            let v40: Rc<str> = v40;
                            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-environment-differs "); } LIT.with(|lit| lit.clone()) }, v40));
                            v41
                        }
                        US1::US1_IdleOrphanHost(v5) => {
                            let v5: u64 = v5;
                            let v6: Rc<str> = format_real_12(v5);
                            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("idle-orphan-host pid="); } LIT.with(|lit| lit.clone()) }, v6));
                            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (its parent is gone and it serves no job)"); } LIT.with(|lit| lit.clone()) }));
                            v8
                        }
                        US1::US1_OrphanCompilerServer(v9, v10) => {
                            let v9: u64 = v9;
                            let v10: u64 = v10;
                            let v11: Rc<str> = format_real_12(v9);
                            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan-compiler-server pid="); } LIT.with(|lit| lit.clone()) }, v11));
                            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" owner="); } LIT.with(|lit| lit.clone()) }));
                            let v14: Rc<str> = format_real_12(v10);
                            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
                            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (the erl named in its socket is gone; send quit on the socket or stop the pid)"); } LIT.with(|lit| lit.clone()) }));
                            v16
                        }
                        US1::US1_SettledButSupervised(v17, v18) => {
                            let v17: u64 = v17;
                            let v18: Rc<str> = v18;
                            let mut v19: Rc<str> = format_real_12(v17);
                            let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled-but-supervised pid="); } LIT.with(|lit| lit.clone()) }, v19));
                            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) }));
                            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v18));
                            v22
                        }
                        US1::US1_WaitOnMissing(v29, v30) => {
                            let v29: u64 = v29;
                            let v30: Rc<str> = v30;
                            let v31: Rc<str> = format_real_12(v29);
                            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-on-missing pid="); } LIT.with(|lit| lit.clone()) }, v31));
                            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) }));
                            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v30));
                            v34
                        }
                        US1::US1_WaitOnSettled(v23, v24) => {
                            let v23: u64 = v23;
                            let v24: Rc<str> = v24;
                            let v25: Rc<str> = format_real_12(v23);
                            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-on-settled pid="); } LIT.with(|lit| lit.clone()) }, v25));
                            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) }));
                            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v24));
                            v28
                        }
                    }
                } else {
                    let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                    let v93: Rc<str> = match v2 {
                        US1::US1_ChainToMissing(v80, v81) => {
                            let v80: Rc<str> = v80;
                            let v81: Rc<str> = v81;
                            let v82: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chain-to-missing job="); } LIT.with(|lit| lit.clone()) }, v80));
                            let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v82, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after="); } LIT.with(|lit| lit.clone()) }));
                            let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v81));
                            v84
                        }
                        US1::US1_HostEnvironmentDiffers(v85) => {
                            let v85: Rc<str> = v85;
                            let v86: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-environment-differs "); } LIT.with(|lit| lit.clone()) }, v85));
                            v86
                        }
                        US1::US1_IdleOrphanHost(v50) => {
                            let v50: u64 = v50;
                            let v51: Rc<str> = format_real_12(v50);
                            let v52: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("idle-orphan-host pid="); } LIT.with(|lit| lit.clone()) }, v51));
                            let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (its parent is gone and it serves no job)"); } LIT.with(|lit| lit.clone()) }));
                            v53
                        }
                        US1::US1_OrphanCompilerServer(v54, v55) => {
                            let mut v54: u64 = v54;
                            let mut v55: u64 = v55;
                            let v56: Rc<str> = format_real_12(v54);
                            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan-compiler-server pid="); } LIT.with(|lit| lit.clone()) }, v56));
                            let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" owner="); } LIT.with(|lit| lit.clone()) }));
                            let v59: Rc<str> = format_real_12(v55);
                            let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
                            let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (the erl named in its socket is gone; send quit on the socket or stop the pid)"); } LIT.with(|lit| lit.clone()) }));
                            v61
                        }
                        US1::US1_SettledButSupervised(v62, v63) => {
                            let v62: u64 = v62;
                            let v63: Rc<str> = v63;
                            let v64: Rc<str> = format_real_12(v62);
                            let v65: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled-but-supervised pid="); } LIT.with(|lit| lit.clone()) }, v64));
                            let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) }));
                            let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v63));
                            v67
                        }
                        US1::US1_WaitOnMissing(v74, v75) => {
                            let v74: u64 = v74;
                            let v75: Rc<str> = v75;
                            let v76: Rc<str> = format_real_12(v74);
                            let v77: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-on-missing pid="); } LIT.with(|lit| lit.clone()) }, v76));
                            let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v77, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) }));
                            let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v75));
                            v79
                        }
                        US1::US1_WaitOnSettled(v68, v69) => {
                            let v68: u64 = v68;
                            let v69: Rc<str> = v69;
                            let v70: Rc<str> = format_real_12(v68);
                            let v71: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-on-settled pid="); } LIT.with(|lit| lit.clone()) }, v70));
                            let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v71, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) }));
                            let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v69));
                            v73
                        }
                    };
                    let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v93));
                    v94
                };
                (v0, v1) = (v3, v95);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method14(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: US1 = v2;
                let v3: Rc<UH1> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn server_report_0() -> (i32, Rc<str>) {
    let mut v0: Rc<UH0> = server_facts_1();
    let mut v1: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH1> = method9(v0, v1);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH1> = method10(v2, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = method11(v4.clone(), v5);
    let v7: i32 = 0i32;
    let v8: i32 = method14(v4, v7);
    (v8, v6)
}
fn closure0() -> Rc<dyn Fn() -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (i32, Rc<str>)> = Rc::new(move || -> (i32, Rc<str>) {
        server_report_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_doctor_server_report() -> (i32, Rc<str>) {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
