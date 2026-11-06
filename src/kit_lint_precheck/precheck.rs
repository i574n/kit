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
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_WidthOver(Rc<str>, i32, i32),
    US0_OpenString(Rc<str>, i32),
    US0_ReservedOutput(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US0, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(u8, Rc<UH2>),
}
pub struct Mut1 { pub l0: Rc<str> }
fn method1(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method5(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH1> = method5(v0.clone(), v4, v2);
            let v6: US0 = US0::US0_ReservedOutput(v0, v3);
            Rc::new(UH1::UH1_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn manifest_findings_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("output = \"lib.rs\""); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v1.contains(&*v2);
    let v8: Rc<UH0> = if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lib.rs"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v4, v5))
    } else {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("output = \"main.rs\""); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v1.contains(&*v9);
    let v13: Rc<UH0> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("main.rs"); } LIT.with(|lit| lit.clone()) };
        Rc::new(UH0::UH0_Cons(v11, v8))
    } else {
        v8
    };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("output = \"build.rs\""); } LIT.with(|lit| lit.clone()) };
    let v15: bool = v1.contains(&*v14);
    let v18: Rc<UH0> = if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("build.rs"); } LIT.with(|lit| lit.clone()) };
        Rc::new(UH0::UH0_Cons(v16, v13))
    } else {
        v13
    };
    let v19: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method5(v0, v18, v19)
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method9(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method9(v7, &v9) {
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
        method8(v16, v17)
    }
}
fn method12(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method13(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn host_line_11(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.contains(&*v1);
    let v5: bool = if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$\""); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.contains(&*v3);
        v4
    };
    if v5 {
        true
    } else {
        let v6: i32 = (v0.clone().len() as i32);
        let v7: i32 = 0i32;
        let v8: i32 = method12(v0.clone(), v6, v7);
        let mut v9: i32 = v6.wrapping_sub(1i32);
        let v10: Rc<str> = string_slice(&v0, v8 as i64, v9 as i64);
        let v11: i32 = (v10.clone().len() as i32);
        let v12: i32 = method13(v10.clone(), v11);
        let v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v15: bool = v13.ends_with(&*v14);
        v15
    }
}
fn method15(mut v0: i64, mut v1: Rc<str>, mut v2: i64) -> Rc<UH2> {
    let v3: bool = v2 < v0;
    if v3 {
        let v4: u8 = v1.clone().as_bytes()[v2 as usize];
        let v5: i64 = v2.wrapping_add(1i64);
        let v6: Rc<UH2> = method15(v0, v1, v5);
        Rc::new(UH2::UH2_Cons(v4, v6))
    } else {
        { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
    }
}
fn quotes_from_16(mut v0: Rc<UH2>, mut v1: bool, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: u8 = v3;
                let v4: Rc<UH2> = v4;
                if v1 {
                    let v5: bool = false;
                    (v0, v1) = (v4, v5);
                    continue;
                } else {
                    let v7: bool = v3 == b'\\';
                    if v7 {
                        let v8: bool = true;
                        (v0, v1) = (v4, v8);
                        continue;
                    } else {
                        let v10: bool = v3 == b'"';
                        if v10 {
                            let v11: bool = false;
                            let v12: i32 = v2.wrapping_add(1i32);
                            (v0, v1, v2) = (v4, v11, v12);
                            continue;
                        } else {
                            let v14: bool = false;
                            (v0, v1) = (v4, v14);
                            continue;
                        }
                    }
                }
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn open_string_14(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'\"'"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: i64 = (v3.clone().len() as i64);
    let v5: i64 = 0i64;
    let v6: Rc<UH2> = method15(v4, v3, v5);
    let v7: bool = false;
    let v8: i32 = 0i32;
    let mut v9: i32 = quotes_from_16(v6, v7, v8);
    let v10: i32 = v9.wrapping_rem(2i32);
    let v11: bool = v10 == 1i32;
    v11
}
fn method17(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: US0 = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method17(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method10(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>, mut v3: i32, mut v4: bool) -> (Rc<UH1>, i32, bool) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v5, v6) => {
                let v5: Rc<str> = v5;
                let v6: Rc<UH0> = v6;
                let v7: i32 = (v5.clone().len() as i32);
                let v8: bool = v7 > 120i32;
                let v13: Rc<UH1> = if v8 {
                    let mut v9: US0 = US0::US0_WidthOver(v0.clone(), v3, v7);
                    let v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH1::UH1_Cons(v9, v10))
                } else {
                    { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
                };
                let v15: bool = if v4 {
                    true
                } else {
                    host_line_11(v5.clone())
                };
                let v25: bool = if v15 {
                    true
                } else {
                    let v16: i32 = 0i32;
                    let v17: i32 = method12(v5.clone(), v7, v16);
                    let v18: i32 = v7.wrapping_sub(1i32);
                    let v19: Rc<str> = string_slice(&v5.clone(), v17 as i64, v18 as i64);
                    let v20: i32 = (v19.clone().len() as i32);
                    let v21: i32 = method13(v19.clone(), v20);
                    let v22: Rc<str> = string_slice(&v19, 0i32 as i64, v21 as i64);
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///"); } LIT.with(|lit| lit.clone()) };
                    let v24: bool = v22.starts_with(&*v23);
                    v24
                };
                let v26: bool = v25 == false;
                let v28: bool = if v26 {
                    open_string_14(v5.clone())
                } else {
                    false
                };
                let v33: Rc<UH1> = if v28 {
                    let v29: US0 = US0::US0_OpenString(v0.clone(), v3);
                    let v30: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH1::UH1_Cons(v29, v30))
                } else {
                    { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
                };
                let v34: Rc<UH1> = method17(v13, v33);
                let v35: Rc<UH1> = method17(v2, v34);
                let v36: i32 = v3.wrapping_add(1i32);
                let v37: i32 = 0i32;
                let v38: i32 = method12(v5.clone(), v7, v37);
                let v39: i32 = v7.wrapping_sub(1i32);
                let v40: Rc<str> = string_slice(&v5, v38 as i64, v39 as i64);
                let v41: i32 = (v40.clone().len() as i32);
                let v42: i32 = method13(v40.clone(), v41);
                let v43: Rc<str> = string_slice(&v40, 0i32 as i64, v42 as i64);
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                let v45: bool = v43.ends_with(&*v44);
                (v1, v2, v3, v4) = (v6, v35, v36, v45);
                continue;
            }
            UH0::UH0_Nil => {
                return (v2, v3, v4);
            }
        }
    }
}
fn file_findings_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v1.replace(&*v3, &*v4));
    let v6: Rc<UH0> = split_list_7(v2, v5);
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v8: i32 = 1i32;
    let mut v9: bool = false;
    let (mut v10, mut v11, mut v12): (Rc<UH1>, i32, bool) = method10(v0, v6, v7, v8, v9);
    v10
}
fn path_findings_3(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default().as_str());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Cargo.toml"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v0.ends_with(&*v2);
    if v3 {
        manifest_findings_4(v0, v1)
    } else {
        file_findings_6(v0, v1)
    }
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH1> = path_findings_3(v2);
                let v5: Rc<UH1> = method17(v1, v4);
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method21(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_20(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method21(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn finding_line_19(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_OpenString(v16, v17) => {
            let v16: Rc<str> = v16;
            let v17: i32 = v17;
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
            let v19: Rc<str> = format_real_20(v17);
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": a string literal is left open at the line end (a broken \\n or \\\"?)"); } LIT.with(|lit| lit.clone()) }));
            v21
        }
        US0::US0_ReservedOutput(v1, v2) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": output "); } LIT.with(|lit| lit.clone()) }));
            let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v2));
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is a name Cargo reserves in a crate root (build.rs = build script); rename the module"); } LIT.with(|lit| lit.clone()) }));
            v5
        }
        US0::US0_WidthOver(v6, v7, v8) => {
            let v6: Rc<str> = v6;
            let v7: i32 = v7;
            let v8: i32 = v8;
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
            let v10: Rc<str> = format_real_20(v7);
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": width "); } LIT.with(|lit| lit.clone()) }));
            let v13: Rc<str> = format_real_20(v8);
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" > 120"); } LIT.with(|lit| lit.clone()) }));
            v15
        }
    }
}
fn method18(mut v0: Rc<UH1>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v1, v2) => {
                let mut v1: US0 = v1;
                let mut v2: Rc<UH1> = v2;
                let v3: Rc<str> = finding_line_19(v1);
                println!("{}", v3);
                v0 = v2;
                continue;
            }
            UH1::UH1_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method22(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: US0 = v2;
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
fn method23(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn precheck_run_0(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> i32 {
    let mut v1: Rc<Vec<Rc<str>>> = Rc::new((v0).borrow().clone());
    let mut v2: usize = (v1).len();
    let v3: i32 = ((v2) as i32);
    let v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = method1(v1, v4, v5);
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH1> = method2(v6.clone(), v7);
    method18(v8.clone());
    let mut v9: i32 = 0i32;
    let v10: i32 = method22(v8, v9);
    let v11: i32 = 0i32;
    let v12: i32 = method23(v6, v11);
    let v13: bool = v10 == 0i32;
    if v13 {
        let v14: Rc<str> = format_real_20(v12);
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit lint precheck ok files="); } LIT.with(|lit| lit.clone()) }, v14));
        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" findings=0"); } LIT.with(|lit| lit.clone()) }));
        println!("{}", v16);
        0i32
    } else {
        let v17: Rc<str> = format_real_20(v12);
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files="); } LIT.with(|lit| lit.clone()) }, v17));
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" findings="); } LIT.with(|lit| lit.clone()) }));
        let v20: Rc<str> = format_real_20(v10);
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit lint precheck refused: "); } LIT.with(|lit| lit.clone()) }, v21));
        println!("{}", v22);
        1i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<RefCell<Vec<Rc<str>>>>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<RefCell<Vec<Rc<str>>>>) -> i32> = Rc::new(move |mut v0: Rc<RefCell<Vec<Rc<str>>>>| -> i32 {
        precheck_run_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_precheck_run(v0: Rc<RefCell<Vec<Rc<str>>>>) -> i32 {
    closure0()(v0)
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
