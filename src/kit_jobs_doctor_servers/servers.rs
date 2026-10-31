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
struct Mut0 { l0: Rc<str> }
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: u64 = *v2;
                let v3: u64 = *v3;
                let v4: bool = *v4;
                let v5: Rc<UH0> = v5.clone();
                let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v4, v1.clone()));
                (v0, v1) = (v5.clone(), v6.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method4(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
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
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method6(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
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
                (v0, v1) = (v0.clone(), v3);
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
        let v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn servers_from_2(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method3(v3.clone(), v5.clone());
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
                (v0, v1, v2, v3) = (v0.clone(), v14, v2, v3.clone());
                continue;
            } else {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral-cells-"); } LIT.with(|lit| lit.clone()) };
                let v17: Option<usize> = v7.find(&*v16);
                let v18: Option<usize> = method4(v17.clone());
                let mut v19: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v20: Option<i32> = v18.map(|x| v19(x));
                let v22: i32 = v20.unwrap_or(-1i32);
                let v23: bool = v22 < 0i32;
                let v48: Rc<str> = if v23 {
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v24.clone()
                } else {
                    let v25: i32 = (v16.clone().len() as i32);
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
                    let v38: Rc<str> = string_slice(&v33.clone(), v36 as i64, v37 as i64);
                    let v39: i32 = (v38.clone().len() as i32);
                    let v40: i32 = method6(v38.clone(), v39);
                    let v41: Rc<str> = string_slice(&v38.clone(), 0i32 as i64, v40 as i64);
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v41.split(&*v42).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                    let v44: Rc<str> = v43.clone().borrow()[0i32 as usize].clone();
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v47: Rc<str> = Rc::<str>::from(v44.replace(&*v45, &*v46));
                    v47.clone()
                };
                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v48.split(&*v49).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let v51: Rc<str> = v50.clone().borrow()[0i32 as usize].clone();
                let v52: Option<u64> = (v51).trim().parse::<u64>().ok();
                let v53: Option<u64> = method7(v52.clone());
                let mut v54: Rc<dyn Fn((u64)) -> US0> = closure2();
                let mut v55: Option<US0> = v53.map(|x| v54(x));
                let v57: US0 = US0::US0_None;
                let v58: US0 = v57.clone();
                let v59: US0 = v55.unwrap_or(v58);
                let v62: u64 = match &v59 {
                    US0::US0_None => {
                        0u64
                    }
                    US0::US0_Some(v60) => {
                        let v60: u64 = *v60;
                        v60
                    }
                };
                let v63: u64 = v0[v1 as usize].0;
                let v64: i32 = v0.iter().position(|row| row.0 == v62).map(|at| at as i32).unwrap_or(-1);
                let v65: bool = v64 >= 0i32;
                let v66: i32 = v1.wrapping_add(1i32);
                let v67: Rc<UH0> = Rc::new(UH0::UH0_Cons(v63, v62, v65, v3.clone()));
                (v0, v1, v2, v3) = (v0.clone(), v66, v2, v67.clone());
                continue;
            }
        }
    }
}
fn process_rows_sampled_8(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> {
    let v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = kit_machine_sample::kit_machine_sampled(v0);
    v1
}
fn server_facts_1() -> Rc<UH0> {
    let mut v0: u64 = 50u64;
    let v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = process_rows_sampled_8(v0);
    let v2: i32 = 0i32;
    let v3: i32 = v1.len() as i32;
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    servers_from_2(v1, v2, v3, v4)
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: u64 = *v2;
                let v3: u64 = *v3;
                let v4: bool = *v4;
                let v5: Rc<UH0> = v5.clone();
                let v7: bool = if v4 {
                    true
                } else {
                    let v6: bool = v3 == 0u64;
                    v6
                };
                let v10: Rc<UH1> = if v7 {
                    v1.clone()
                } else {
                    let v8: US1 = US1::US1_OrphanCompilerServer(v2, v3);
                    Rc::new(UH1::UH1_Cons(v8.clone(), v1.clone()))
                };
                (v0, v1) = (v5.clone(), v10.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method10(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: US1 = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method13(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_12(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method13(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method11(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: US1 = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: bool = &*v1 == "";
                let v124: Rc<str> = if v4 {
                    match &v2 {
                        US1::US1_ChainToMissing(v46, v47) => {
                            let v46: Rc<str> = v46.clone();
                            let v47: Rc<str> = v47.clone();
                            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chain-to-missing job="); } LIT.with(|lit| lit.clone()) };
                            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v46));
                            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after="); } LIT.with(|lit| lit.clone()) };
                            let v52: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v49, v50)), v47));
                            v52.clone()
                        }
                        US1::US1_HostEnvironmentDiffers(v53) => {
                            let v53: Rc<str> = v53.clone();
                            let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-environment-differs "); } LIT.with(|lit| lit.clone()) };
                            let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v53));
                            v55.clone()
                        }
                        US1::US1_IdleOrphanHost(v5) => {
                            let v5: u64 = *v5;
                            let v6: Rc<str> = format_real_12(v5);
                            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("idle-orphan-host pid="); } LIT.with(|lit| lit.clone()) };
                            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v6));
                            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (its parent is gone and it serves no job)"); } LIT.with(|lit| lit.clone()) };
                            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
                            v10.clone()
                        }
                        US1::US1_OrphanCompilerServer(v11, v12) => {
                            let v11: u64 = *v11;
                            let v12: u64 = *v12;
                            let v13: Rc<str> = format_real_12(v11);
                            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan-compiler-server pid="); } LIT.with(|lit| lit.clone()) };
                            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
                            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" owner="); } LIT.with(|lit| lit.clone()) };
                            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                            let v18: Rc<str> = format_real_12(v12);
                            let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (the erl named in its socket is gone; send quit on the socket or stop the pid)"); } LIT.with(|lit| lit.clone()) };
                            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
                            v21.clone()
                        }
                        US1::US1_SettledButSupervised(v22, v23) => {
                            let v22: u64 = *v22;
                            let v23: Rc<str> = v23.clone();
                            let v24: Rc<str> = format_real_12(v22);
                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled-but-supervised pid="); } LIT.with(|lit| lit.clone()) };
                            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) };
                            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v26, v27)), v23));
                            v29.clone()
                        }
                        US1::US1_WaitOnMissing(v38, v39) => {
                            let v38: u64 = *v38;
                            let v39: Rc<str> = v39.clone();
                            let v40: Rc<str> = format_real_12(v38);
                            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-on-missing pid="); } LIT.with(|lit| lit.clone()) };
                            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v40));
                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) };
                            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v42, v43)), v39));
                            v45.clone()
                        }
                        US1::US1_WaitOnSettled(v30, v31) => {
                            let v30: u64 = *v30;
                            let v31: Rc<str> = v31.clone();
                            let v32: Rc<str> = format_real_12(v30);
                            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-on-settled pid="); } LIT.with(|lit| lit.clone()) };
                            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v32));
                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) };
                            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v34, v35)), v31));
                            v37.clone()
                        }
                    }
                } else {
                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v63));
                    let v122: Rc<str> = match &v2 {
                        US1::US1_ChainToMissing(v106, v107) => {
                            let v106: Rc<str> = v106.clone();
                            let v107: Rc<str> = v107.clone();
                            let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chain-to-missing job="); } LIT.with(|lit| lit.clone()) };
                            let v109: Rc<str> = Rc::<str>::from(format!("{}{}", v108, v106));
                            let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after="); } LIT.with(|lit| lit.clone()) };
                            let v112: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v109, v110)), v107));
                            v112.clone()
                        }
                        US1::US1_HostEnvironmentDiffers(v113) => {
                            let v113: Rc<str> = v113.clone();
                            let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-environment-differs "); } LIT.with(|lit| lit.clone()) };
                            let v115: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v113));
                            v115.clone()
                        }
                        US1::US1_IdleOrphanHost(v65) => {
                            let v65: u64 = *v65;
                            let v66: Rc<str> = format_real_12(v65);
                            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("idle-orphan-host pid="); } LIT.with(|lit| lit.clone()) };
                            let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v66));
                            let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (its parent is gone and it serves no job)"); } LIT.with(|lit| lit.clone()) };
                            let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v69));
                            v70.clone()
                        }
                        US1::US1_OrphanCompilerServer(v71, v72) => {
                            let v71: u64 = *v71;
                            let v72: u64 = *v72;
                            let v73: Rc<str> = format_real_12(v71);
                            let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan-compiler-server pid="); } LIT.with(|lit| lit.clone()) };
                            let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v73));
                            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" owner="); } LIT.with(|lit| lit.clone()) };
                            let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v76));
                            let v78: Rc<str> = format_real_12(v72);
                            let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v78));
                            let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (the erl named in its socket is gone; send quit on the socket or stop the pid)"); } LIT.with(|lit| lit.clone()) };
                            let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v80));
                            v81.clone()
                        }
                        US1::US1_SettledButSupervised(v82, v83) => {
                            let v82: u64 = *v82;
                            let v83: Rc<str> = v83.clone();
                            let v84: Rc<str> = format_real_12(v82);
                            let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled-but-supervised pid="); } LIT.with(|lit| lit.clone()) };
                            let v86: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v84));
                            let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) };
                            let v89: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v86, v87)), v83));
                            v89.clone()
                        }
                        US1::US1_WaitOnMissing(v98, v99) => {
                            let v98: u64 = *v98;
                            let v99: Rc<str> = v99.clone();
                            let v100: Rc<str> = format_real_12(v98);
                            let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-on-missing pid="); } LIT.with(|lit| lit.clone()) };
                            let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v100));
                            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) };
                            let v105: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v102, v103)), v99));
                            v105.clone()
                        }
                        US1::US1_WaitOnSettled(v90, v91) => {
                            let v90: u64 = *v90;
                            let v91: Rc<str> = v91.clone();
                            let v92: Rc<str> = format_real_12(v90);
                            let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-on-settled pid="); } LIT.with(|lit| lit.clone()) };
                            let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v92));
                            let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) };
                            let v97: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v94, v95)), v91));
                            v97.clone()
                        }
                    };
                    let v123: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v122));
                    v123.clone()
                };
                (v0, v1) = (v3.clone(), v124.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method14(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: US1 = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
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
    let v1: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v2: Rc<UH1> = method9(v0, v1);
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
