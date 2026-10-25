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
    US0_AfterAny,
    US0_AfterOk,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<UH1>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_ChainPlanned(US0, Rc<str>, Rc<UH0>),
    US2_ChainRefused(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(US0, Rc<str>),
    US3_None,
}
fn method2(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, Rc<str>) {
    let (mut v12, mut v13): (Rc<str>, Rc<str>) = match &*v1 {
        UH1::UH1_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<UH1> = v5.clone();
            let (mut v6, mut v7): (Rc<str>, Rc<str>) = method2(v0.clone(), v5.clone(), v2.clone(), v3.clone());
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v7));
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v6));
            (v9.clone(), v0.clone())
        }
        _ => {
            let (mut v10, mut v11): (Rc<str>, Rc<str>) = match &*v1 {
                UH1::UH1_Nil => {
                    (v2.clone(), v3.clone())
                }
                _ => unreachable!(),
            };
            (v10.clone(), v11.clone())
        }
    };
    (v12.clone(), v13.clone())
}
fn method3(mut v0: Option<usize>) -> Option<usize> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
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
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
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
fn method6(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH1> = v3.clone();
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method8(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3.clone();
    ()
}
fn format_real_7(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method8(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn method9(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH1> = v3.clone();
                let mut v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method10(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, Rc<str>) {
    let (mut v12, mut v13): (Rc<str>, Rc<str>) = match &*v0 {
        UH1::UH1_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<UH1> = v4.clone();
            let (mut v5, mut v6): (Rc<str>, Rc<str>) = method10(v4.clone(), v1.clone(), v2.clone());
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v6));
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v5));
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            (v8.clone(), v9.clone())
        }
        _ => {
            let (mut v10, mut v11): (Rc<str>, Rc<str>) = match &*v0 {
                UH1::UH1_Nil => {
                    (v1.clone(), v2.clone())
                }
                _ => unreachable!(),
            };
            (v10.clone(), v11.clone())
        }
    };
    (v12.clone(), v13.clone())
}
fn chain_started_1(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: Rc<UH1>) -> i32 {
    loop {
        match &*v3 {
            UH0::UH0_Cons(v18, v19) => {
                let mut v18: Rc<UH1> = v18.clone();
                let mut v19: Rc<UH0> = v19.clone();
                let mut v20: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<UH1> = if v20 {
                    v18.clone()
                } else {
                    let mut v24: Rc<str> = match &v1 {
                        US0::US0_AfterAny => {
                            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after"); } LIT.with(|lit| lit.clone()) };
                            v21.clone()
                        }
                        US0::US0_AfterOk => {
                            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after-ok"); } LIT.with(|lit| lit.clone()) };
                            v22.clone()
                        }
                    };
                    let mut v25: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v18.clone()));
                    Rc::new(UH1::UH1_Cons(v24.clone(), v25.clone()))
                };
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let (mut v30, mut v31): (Rc<str>, Rc<str>) = method2(v28.clone(), v27.clone(), v29.clone(), v29.clone());
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v31));
                let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v30));
                let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v34));
                let (mut v38, mut v39): (bool, Rc<str>) = match std::process::Command::new(&*v0).args((&*v37).split(&*v28).filter(|a| a.len() > 0)).output() { Ok(out) if out.status.success() => { let mut text = String::from_utf8_lossy(&out.stdout).into_owned(); text.push_str(&String::from_utf8_lossy(&out.stderr)); (true, Rc::<str>::from(text)) } _ => (false, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                let mut v42: US1 = if v38 {
                    US1::US1_Some(v39.clone())
                } else {
                    US1::US1_None
                };
                let mut v45: Rc<str> = match &v42 {
                    US1::US1_None => {
                        v29.clone()
                    }
                    US1::US1_Some(v43) => {
                        let mut v43: Rc<str> = v43.clone();
                        v43.clone()
                    }
                };
                let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id=job-"); } LIT.with(|lit| lit.clone()) };
                let mut v47: Option<usize> = v45.find(&*v46);
                let mut v48: Option<usize> = method3(v47.clone());
                let mut v49: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v50: Option<i32> = v48.map(|x| v49(x));
                let mut v52: i32 = v50.unwrap_or(-1i32);
                let mut v53: bool = v52 < 0i32;
                let mut v73: Rc<str> = if v53 {
                    v29.clone()
                } else {
                    let mut v54: i32 = v52.wrapping_add(3i32);
                    let mut v55: i32 = (v45.clone().len() as i32);
                    let mut v56: i32 = v55.wrapping_sub(v52);
                    let mut v57: i32 = v56.wrapping_sub(3i32);
                    let mut v58: usize = ((v54) as usize);
                    let mut v59: i32 = v54.wrapping_add(v57);
                    let mut v60: usize = ((v59) as usize);
                    let mut v61: Rc<str> = Rc::<str>::from(&(v45)[v58..v60]);
                    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let mut v63: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v61.split(&*v62).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                    let mut v64: Rc<str> = v63.clone().borrow()[0i32 as usize].clone();
                    let mut v65: i32 = (v64.clone().len() as i32);
                    let mut v66: i32 = 0i32;
                    let mut v67: i32 = method4(v64.clone(), v65, v66);
                    let mut v68: i32 = v65.wrapping_sub(1i32);
                    let mut v69: Rc<str> = string_slice(&v64.clone(), v67 as i64, v68 as i64);
                    let mut v70: i32 = (v69.clone().len() as i32);
                    let mut v71: i32 = method5(v69.clone(), v70);
                    let mut v72: Rc<str> = string_slice(&v69.clone(), 0i32 as i64, v71 as i64);
                    v72.clone()
                };
                let mut v74: bool = v73.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v74 {
                    let mut v75: i32 = 0i32;
                    let mut v76: i32 = method6(v4.clone(), v75);
                    let mut v77: i32 = v76.wrapping_add(1i32);
                    let mut v78: Rc<str> = format_real_7(v77);
                    let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: job chain stopped: link "); } LIT.with(|lit| lit.clone()) };
                    let mut v80: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v78));
                    let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" did not start ("); } LIT.with(|lit| lit.clone()) };
                    let mut v82: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v81));
                    let mut v83: i32 = (v45.clone().len() as i32);
                    let mut v84: i32 = 0i32;
                    let mut v85: i32 = method4(v45.clone(), v83, v84);
                    let mut v86: i32 = v83.wrapping_sub(1i32);
                    let mut v87: Rc<str> = string_slice(&v45.clone(), v85 as i64, v86 as i64);
                    let mut v88: i32 = (v87.clone().len() as i32);
                    let mut v89: i32 = method5(v87.clone(), v88);
                    let mut v90: Rc<str> = string_slice(&v87.clone(), 0i32 as i64, v89 as i64);
                    let mut v91: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v90));
                    let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("); started before it: "); } LIT.with(|lit| lit.clone()) };
                    let mut v93: Rc<str> = Rc::<str>::from(format!("{}{}", v91, v92));
                    let mut v94: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let mut v95: Rc<UH1> = method9(v4.clone(), v94.clone());
                    let (mut v96, mut v97): (Rc<str>, Rc<str>) = method10(v95.clone(), v29.clone(), v29.clone());
                    let mut v98: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v96));
                    eprintln!("{}", v98);
                    return 2i32;
                } else {
                    let mut v99: i32 = (v45.clone().len() as i32);
                    let mut v100: i32 = 0i32;
                    let mut v101: i32 = method4(v45.clone(), v99, v100);
                    let mut v102: i32 = v99.wrapping_sub(1i32);
                    let mut v103: Rc<str> = string_slice(&v45.clone(), v101 as i64, v102 as i64);
                    let mut v104: i32 = (v103.clone().len() as i32);
                    let mut v105: i32 = method5(v103.clone(), v104);
                    let mut v106: Rc<str> = string_slice(&v103.clone(), 0i32 as i64, v105 as i64);
                    println!("{}", v106);
                    let mut v107: Rc<UH1> = Rc::new(UH1::UH1_Cons(v73.clone(), v4.clone()));
                    (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v73.clone(), v19.clone(), v107.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let mut v5: i32 = 0i32;
                let mut v6: i32 = method6(v4.clone(), v5);
                let mut v7: Rc<str> = format_real_7(v6);
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie job chain ok jobs="); } LIT.with(|lit| lit.clone()) };
                let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ids="); } LIT.with(|lit| lit.clone()) };
                let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
                let mut v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let mut v13: Rc<UH1> = method9(v4.clone(), v12.clone());
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let (mut v15, mut v16): (Rc<str>, Rc<str>) = method10(v13.clone(), v14.clone(), v14.clone());
                let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v15));
                println!("{}", v17);
                return 0i32;
            }
        }
    }
}
fn command_line_argument_count_11() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_13(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3.clone()
}
fn arguments_from_12(mut v0: i32, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v3: bool = v1 <= v0;
        if v3 {
            let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            return method9(v2.clone(), v4.clone());
        } else {
            let mut v6: i32 = v0.wrapping_add(1i32);
            let mut v7: bool = v0 >= 0i32;
            let mut v10: bool = if v7 {
                let mut v8: i32 = command_line_argument_count_11();
                let mut v9: bool = v0 < v8;
                v9
            } else {
                false
            };
            let mut v14: US1 = if v10 {
                let mut v11: Rc<str> = command_line_argument_13(v0);
                US1::US1_Some(v11.clone())
            } else {
                US1::US1_None
            };
            let mut v18: Rc<str> = match &v14 {
                US1::US1_None => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                }
                US1::US1_Some(v15) => {
                    let mut v15: Rc<str> = v15.clone();
                    v15.clone()
                }
            };
            let mut v19: Rc<UH1> = Rc::new(UH1::UH1_Cons(v18.clone(), v2.clone()));
            (v0, v1, v2) = (v6, v1, v19.clone());
            continue;
        }
    }
}
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<UH1> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn segments_14(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v8, v9) => {
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH1> = v9.clone();
                let mut v10: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let mut v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let mut v13: Rc<UH1> = method9(v1.clone(), v12.clone());
                    let mut v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v2.clone()));
                    (v0, v1, v2) = (v9.clone(), v11.clone(), v14.clone());
                    continue;
                } else {
                    let mut v16: Rc<UH1> = Rc::new(UH1::UH1_Cons(v8.clone(), v1.clone()));
                    (v0, v1, v2) = (v9.clone(), v16.clone(), v2.clone());
                    continue;
                }
            }
            UH1::UH1_Nil => {
                let mut v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let mut v4: Rc<UH1> = method9(v1.clone(), v3.clone());
                let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v5.clone()));
                return method15(v2.clone(), v6.clone());
            }
        }
    }
}
fn short_link_16(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<UH1> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: i32 = 0i32;
                let mut v5: i32 = method6(v2.clone(), v4);
                let mut v6: bool = v5 < 3i32;
                if v6 {
                    return v1;
                } else {
                    let mut v7: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v3.clone(), v7);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return 0i32;
            }
        }
    }
}
fn method17(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<UH1> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn current_executable_18() -> Rc<str> {
    let mut v0: Rc<str> = std::env::current_exe().map(|path| Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    v0.clone()
}
fn chain_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: i32 = command_line_argument_count_11();
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH1> = arguments_from_12(v0, v1, v2.clone());
    let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = segments_14(v3.clone(), v4.clone(), v5.clone());
    let mut v69: US2 = match &*v6 {
        UH0::UH0_Cons(v9, v10) => {
            let mut v9: Rc<UH1> = v9.clone();
            let mut v10: Rc<UH0> = v10.clone();
            let mut v42: US3 = match &*v9 {
                UH1::UH1_Cons(v14, v15) => {
                    let mut v14: Rc<str> = v14.clone();
                    let mut v15: Rc<UH1> = v15.clone();
                    match &*v15 {
                        UH1::UH1_Cons(v27, v28) => {
                            let mut v27: Rc<str> = v27.clone();
                            let mut v28: Rc<UH1> = v28.clone();
                            match &*v28 {
                                UH1::UH1_Nil => {
                                    let mut v29: bool = v14.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ok"); } LIT.with(|lit| lit.clone()) };
                                    let mut v32: bool = if v29 {
                                        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
                                        let mut v31: bool = v27.starts_with(&*v30);
                                        v31
                                    } else {
                                        false
                                    };
                                    if v32 {
                                        let mut v33: US0 = US0::US0_AfterOk;
                                        US3::US3_Some(v33.clone(), v27.clone())
                                    } else {
                                        US3::US3_None
                                    }
                                }
                                _ => {
                                    US3::US3_None
                                }
                            }
                        }
                        UH1::UH1_Nil => {
                            let mut v16: bool = v14.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ok"); } LIT.with(|lit| lit.clone()) };
                            if v16 {
                                let mut v17: US0 = US0::US0_AfterOk;
                                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                US3::US3_Some(v17.clone(), v18.clone())
                            } else {
                                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
                                let mut v21: bool = v14.starts_with(&*v20);
                                if v21 {
                                    let mut v22: US0 = US0::US0_AfterAny;
                                    US3::US3_Some(v22.clone(), v14.clone())
                                } else {
                                    US3::US3_None
                                }
                            }
                        }
                    }
                }
                UH1::UH1_Nil => {
                    let mut v11: US0 = US0::US0_AfterAny;
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    US3::US3_Some(v11.clone(), v12.clone())
                }
            };
            match &v42 {
                US3::US3_None => {
                    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job chain [--ok] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id)"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_ChainRefused(v43.clone())
                }
                US3::US3_Some(v45, v46) => {
                    let mut v45: US0 = v45.clone();
                    let mut v46: Rc<str> = v46.clone();
                    let mut v47: i32 = 1i32;
                    let mut v48: i32 = short_link_16(v10.clone(), v47);
                    let mut v49: i32 = 0i32;
                    let mut v50: i32 = method17(v10.clone(), v49);
                    let mut v51: bool = v50 == 0i32;
                    if v51 {
                        let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no job to start; "); } LIT.with(|lit| lit.clone()) };
                        let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job chain [--ok] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id)"); } LIT.with(|lit| lit.clone()) };
                        let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no job to start; job chain [--ok] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id)"); } LIT.with(|lit| lit.clone()) };
                        US2::US2_ChainRefused(v54.clone())
                    } else {
                        let mut v56: bool = v48 > 0i32;
                        if v56 {
                            let mut v57: Rc<str> = format_real_7(v48);
                            let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("link "); } LIT.with(|lit| lit.clone()) };
                            let mut v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v57));
                            let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs <label> <timeout-ms> <program> [args...]"); } LIT.with(|lit| lit.clone()) };
                            let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v60));
                            US2::US2_ChainRefused(v61.clone())
                        } else {
                            US2::US2_ChainPlanned(v45.clone(), v46.clone(), v10.clone())
                        }
                    }
                }
            }
        }
        UH0::UH0_Nil => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job chain [--ok] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id)"); } LIT.with(|lit| lit.clone()) };
            US2::US2_ChainRefused(v7.clone())
        }
    };
    match &v69 {
        US2::US2_ChainPlanned(v73, v74, v75) => {
            let mut v73: US0 = v73.clone();
            let mut v74: Rc<str> = v74.clone();
            let mut v75: Rc<UH0> = v75.clone();
            let mut v76: Rc<str> = current_executable_18();
            let mut v77: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            chain_started_1(v76.clone(), v73.clone(), v74.clone(), v75.clone(), v77.clone())
        }
        US2::US2_ChainRefused(v70) => {
            let mut v70: Rc<str> = v70.clone();
            let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: job chain refused: "); } LIT.with(|lit| lit.clone()) };
            let mut v72: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v70));
            eprintln!("{}", v72);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        chain_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_chain_run() -> i32 {
    closure0()()
}
