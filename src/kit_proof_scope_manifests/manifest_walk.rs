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
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(i32),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Other,
    US1_Packages,
    US1_Modules,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(i32, Rc<str>, Rc<UH2>),
}
fn joined_with_2(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<str> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match &*v3 {
                UH0::UH0_Nil => {
                    v2
                }
                _ => {
                    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v0.clone()));
                    let v5: Rc<str> = joined_with_2(v0, v3);
                    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
                    v6
                }
            }
        }
        UH0::UH0_Nil => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v8
        }
    }
}
fn method3(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let mut v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method5(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
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
fn up_one_6(mut v0: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<UH0> = v2.clone();
            let v3: bool = &*v1 == "..";
            if v3 {
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".."); } LIT.with(|lit| lit.clone()) };
                Rc::new(UH0::UH0_Cons(v4, v0))
            } else {
                v2
            }
        }
        UH0::UH0_Nil => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".."); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v7, v8))
        }
    }
}
fn normal_parts_4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v2 == "";
                let mut v6: bool = if v4 {
                    true
                } else {
                    let v5: bool = &*v2 == ".";
                    v5
                };
                if v6 {
                    v0 = v3;
                    continue;
                } else {
                    let v8: bool = &*v2 == "..";
                    if v8 {
                        let mut v9: Rc<UH0> = up_one_6(v1);
                        (v0, v1) = (v3, v9);
                        continue;
                    } else {
                        let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                        (v0, v1) = (v3, v11);
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method5(v1, v15);
            }
        }
    }
}
fn normal_path_1(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v5: Rc<Vec<Rc<str>>> = Rc::new((v4).borrow().clone());
    let mut v6: usize = (v5).len();
    let v7: i32 = ((v6) as i32);
    let v8: i32 = v7.wrapping_sub(1i32);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH0> = method3(v5, v8, v9);
    let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH0> = normal_parts_4(v10, v11);
    joined_with_2(v2, v12)
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    Rc::new(move |mut v2: Rc<UH0>| -> US0 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US0::US0_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US0> = method9(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US0::US0_None
            }
        }
    })
}
fn method9(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    closure1(v0, v1)
}
fn directory_exists_10(mut v0: Rc<str>) -> bool {
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
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v4.clone()));
                let v8: i32 = 0i32;
                let mut v9: Rc<dyn Fn(Rc<UH0>) -> US0> = method9(v4, v8);
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target"); } LIT.with(|lit| lit.clone()) };
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".git"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules"); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".cache"); } LIT.with(|lit| lit.clone()) };
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_build"); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bin"); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("obj"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17, v18));
                let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v19));
                let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v20));
                let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v14, v21));
                let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v22));
                let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v23));
                let v25: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v24));
                let v26: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v25));
                let v27: US0 = v9(v26);
                let v28: bool = match v27 {
                    US0::US0_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v29: bool = v28 != true;
                let v32: bool = if v29 {
                    true
                } else {
                    let v30: bool = directory_exists_10(v7.clone());
                    let v31: bool = v30 == false;
                    v31
                };
                let v35: Rc<UH1> = if v32 {
                    v3
                } else {
                    let v33: i32 = v1.wrapping_add(1i32);
                    manifests_in_7(v7, v33, v3)
                };
                (v2, v3) = (v5, v35);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn file_exists_11(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method12(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn indent_of_18(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v3: bool = v0.starts_with(&*v2);
        if v3 {
            let mut v4: i32 = (v0.clone().len() as i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 1i32 as i64, v5 as i64);
            let v7: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v6, v7);
            continue;
        } else {
            return v1;
        }
    }
}
fn method19(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
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
fn method20(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
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
fn value_after_21(mut v0: &Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: i32 = (v1.clone().len() as i32);
    let mut v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v1, v2 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = 0i32;
    let v8: i32 = method19(v5.clone(), v6, v7);
    let mut v9: i32 = v6.wrapping_sub(1i32);
    let v10: Rc<str> = string_slice(&v5, v8 as i64, v9 as i64);
    let v11: i32 = (v10.clone().len() as i32);
    let v12: i32 = method20(v10.clone(), v11);
    let v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
    v13
}
fn header_of_22(mut v0: Rc<str>) -> US1 {
    let mut v1: bool = &*v0 == "packages:";
    if v1 {
        US1::US1_Packages
    } else {
        let v3: bool = &*v0 == "modules:";
        if v3 {
            US1::US1_Modules
        } else {
            US1::US1_Other
        }
    }
}
fn popped_23(mut v0: Rc<UH2>, mut v1: i32) -> Rc<UH2> {
    loop {
        match &*v0 {
            UH2::UH2_Cons(v2, v3, v4) => {
                let mut v2: i32 = *v2;
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH2> = v4.clone();
                let v5: bool = v2 >= v1;
                if v5 {
                    v0 = v4;
                    continue;
                } else {
                    return v0;
                }
            }
            UH2::UH2_Nil => {
                return { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn prefix_of_24(mut v0: Rc<UH2>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v1, v2, v3) => {
            let mut v1: i32 = v1;
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH2> = v3;
            let mut v4: Rc<str> = prefix_of_24(v3);
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v2));
            v5
        }
        UH2::UH2_Nil => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v6
        }
    }
}
fn dep_name_25(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    let mut v6: Rc<str> = if v2 {
        let v3: i32 = (v0.clone().len() as i32);
        let mut v4: i32 = v3.wrapping_sub(1i32);
        let v5: Rc<str> = string_slice(&v0, 1i32 as i64, v4 as i64);
        v5
    } else {
        v0
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v8: bool = v6.ends_with(&*v7);
    if v8 {
        let mut v9: i32 = (v6.clone().len() as i32);
        let v10: i32 = v9.wrapping_sub(2i32);
        let v11: Rc<str> = string_slice(&v6, 0i32 as i64, v10 as i64);
        v11
    } else {
        v6
    }
}
enum Group15 {
    M15(Rc<UH0>, US1, Rc<UH2>, Rc<str>, Rc<UH0>, Rc<UH0>),
    M16(Rc<UH0>, Rc<str>, Rc<UH0>, Rc<UH0>, Rc<str>),
    M17(Rc<UH0>, US1, Rc<UH2>, Rc<str>, Rc<UH0>, Rc<UH0>, i32, Rc<str>),
}
fn group_15(mut st: Group15) -> (Rc<str>, Rc<UH0>, Rc<UH0>) {
    loop {
        match st {
            Group15::M15(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5) => {
                match Rc::unwrap_or_clone(v0) {
                    UH0::UH0_Cons(v10, v11) => {
                        let v10: Rc<str> = v10;
                        let v11: Rc<UH0> = v11;
                        let v12: i32 = 0i32;
                        let v13: i32 = indent_of_18(v10.clone(), v12);
                        let v14: i32 = (v10.clone().len() as i32);
                        let v15: i32 = 0i32;
                        let v16: i32 = method19(v10.clone(), v14, v15);
                        let v17: i32 = v14.wrapping_sub(1i32);
                        let v18: Rc<str> = string_slice(&v10, v16 as i64, v17 as i64);
                        let v19: i32 = (v18.clone().len() as i32);
                        let v20: i32 = method20(v18.clone(), v19);
                        let v21: Rc<str> = string_slice(&v18, 0i32 as i64, v20 as i64);
                        let v22: bool = &*v21 == "";
                        if v22 {
                            st = Group15::M15(v11, v1, v2, v3, v4, v5);
                            continue;
                        } else {
                            let v26: bool = v13 == 0i32;
                            if v26 {
                                st = Group15::M16(v11, v3, v4, v5, v21);
                                continue;
                            } else {
                                st = Group15::M17(v11, v1, v2, v3, v4, v5, v13, v21);
                                continue;
                            }
                        }
                    }
                    UH0::UH0_Nil => {
                        let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v7: Rc<UH0> = method5(v4, v6);
                        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let mut v9: Rc<UH0> = method5(v5, v8);
                        return (v3, v7, v9);
                    }
                }
            }
            Group15::M16(mut v0, mut v1, mut v2, mut v3, mut v4) => {
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packageDir:"); } LIT.with(|lit| lit.clone()) };
                let mut v6: bool = v4.starts_with(&*v5);
                if v6 {
                    let v7: US1 = US1::US1_Other;
                    let v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let mut v9: Rc<str> = value_after_21(&v5, v4);
                    st = Group15::M15(v0, v7, v8, v9, v2, v3);
                    continue;
                } else {
                    let v13: US1 = header_of_22(v4);
                    let v14: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    st = Group15::M15(v0, v13, v14, v1, v2, v3);
                    continue;
                }
            }
            Group15::M17(mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7) => {
                match &v1 {
                    US1::US1_Modules => {
                        let v13: Rc<UH2> = popped_23(v2, v6);
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                        let v15: bool = v7.ends_with(&*v14);
                        if v15 {
                            let v16: Rc<UH2> = Rc::new(UH2::UH2_Cons(v6, v7, v13));
                            st = Group15::M15(v0, v1, v16, v3, v4, v5);
                            continue;
                        } else {
                            let v20: Rc<str> = prefix_of_24(v13.clone());
                            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v7));
                            let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v21, v5));
                            st = Group15::M15(v0, v1, v13, v3, v4, v22);
                            continue;
                        }
                    }
                    US1::US1_Other => {
                        st = Group15::M15(v0, v1, v2, v3, v4, v5);
                        continue;
                    }
                    US1::US1_Packages => {
                        let v8: Rc<str> = dep_name_25(v7);
                        let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v4));
                        st = Group15::M15(v0, v1, v2, v3, v9, v5);
                        continue;
                    }
                }
            }
        }
    }
}
fn read_entry_17(mut v0: Rc<UH0>, mut v1: US1, mut v2: Rc<UH2>, mut v3: Rc<str>, mut v4: Rc<UH0>, mut v5: Rc<UH0>, mut v6: i32, mut v7: Rc<str>) -> (Rc<str>, Rc<UH0>, Rc<UH0>) {
    group_15(Group15::M17(v0, v1, v2, v3, v4, v5, v6, v7))
}
fn read_header_16(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH0>, mut v4: Rc<str>) -> (Rc<str>, Rc<UH0>, Rc<UH0>) {
    group_15(Group15::M16(v0, v1, v2, v3, v4))
}
fn read_lines_15(mut v0: Rc<UH0>, mut v1: US1, mut v2: Rc<UH2>, mut v3: Rc<str>, mut v4: Rc<UH0>, mut v5: Rc<UH0>) -> (Rc<str>, Rc<UH0>, Rc<UH0>) {
    group_15(Group15::M15(v0, v1, v2, v3, v4, v5))
}
fn text_lines_26(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new((v5).borrow().clone());
    let v7: usize = (v6).len();
    let v8: i32 = ((v7) as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method3(v6, v9, v10)
}
fn manifest_of_14(mut v0: Rc<str>) -> (Rc<str>, Rc<UH0>, Rc<UH0>) {
    let mut v1: Rc<UH0> = text_lines_26(v0);
    let mut v2: US1 = US1::US1_Other;
    let v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    read_lines_15(v1, v2, v3, v4, v5, v6)
}
fn joined_path_29(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    normal_path_1(v3)
}
fn resolved_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.starts_with(&*v3);
    if v4 {
        joined_path_29(v0, v2)
    } else {
        let mut v6: Rc<str> = joined_path_29(v0, v1);
        joined_path_29(v6, v2)
    }
}
fn method27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            let mut v6: Rc<UH0> = method27(v0.clone(), v1.clone(), v5, v3);
            let v7: Rc<str> = resolved_28(v0, v1, v4);
            Rc::new(UH0::UH0_Cons(v7, v6))
        }
        UH0::UH0_Nil => {
            v3
        }
    }
}
fn method31(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
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
fn listed_30(mut v0: Rc<UH0>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v2: i32 = 0i32;
    let v3: i32 = method31(v1.clone(), v0, v2);
    let mut v4: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v4).as_ref().clone()));
    v5
}
fn row_of_13(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) {
    let (mut v2, mut v3, mut v4): (Rc<str>, Rc<UH0>, Rc<UH0>) = manifest_of_14(v1);
    let v5: Rc<str> = normal_path_1(v0.clone());
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method27(v0, v2, v3, v6);
    let v8: Rc<RefCell<Vec<Rc<str>>>> = listed_30(v7);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = listed_30(v4);
    (v5, v8, v9)
}
fn child_names_32(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Vec<std::string::String> = v2.unwrap();
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: bool = true; let _vec_fold_ = v4.into_iter().fold(v5, |acc, x| { //;
    let mut v9: Rc<UH0> = acc;
    let v11: std::string::String = x;
    let v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v9));
    let v16: bool = true; v15 });
    let v18: Rc<UH0> = _vec_fold_;
    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method5(v18, v19)
}
fn manifests_in_7(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/package.spiproj"); } LIT.with(|lit| lit.clone()) }));
    let mut v4: bool = file_exists_11(v3.clone());
    let v20: Rc<UH1> = if v4 {
        let mut v6: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v3);
        let v8: Vec<u8> = v6.unwrap();
        let mut v9: Vec<u8> = method12(v8);
        let v11: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v9);
        let v13: std::string::String = v11.unwrap();
        let v15: Rc<str> = Rc::<str>::from(String::as_str(&v13));
        let (mut v16, mut v17, mut v18): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) = row_of_13(v0.clone(), v15);
        Rc::new(UH1::UH1_Cons(v16, v17, v18, v2))
    } else {
        v2
    };
    let v21: bool = v1 >= 8i32;
    if v21 {
        v20
    } else {
        let v22: Rc<UH0> = child_names_32(v0.clone());
        method8(v0, v1, v22, v20)
    }
}
fn method33(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = v4;
                let v5: Rc<RefCell<Vec<Rc<str>>>> = v5;
                let mut v6: Rc<UH1> = v6;
                v0.borrow_mut().push((v3, v4, v5));
                let v7: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn manifests_under_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    let mut v1: Rc<str> = normal_path_1(v0);
    let mut v2: i32 = 0i32;
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH1> = manifests_in_7(v1, v2, v3);
    let v5: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v6: i32 = 0i32;
    let v7: i32 = method33(v5.clone(), v4, v6);
    let v8: Rc<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>> = Rc::new(v5.borrow().clone());
    let mut v9: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
    v9
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
        manifests_under_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn anchored_36(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        true
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v1.contains(&*v4);
        v5
    };
    if v6 {
        normal_path_1(v1)
    } else {
        joined_path_29(v0, v1)
    }
}
fn method35(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method35(v0.clone(), v4, v2);
            let mut v6: Rc<str> = anchored_36(v0, v3);
            Rc::new(UH0::UH0_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn anchored_paths_34(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v2: Rc<str> = normal_path_1(v1);
    let v3: Rc<Vec<Rc<str>>> = Rc::new((v0).borrow().clone());
    let mut v4: usize = (v3).len();
    let v5: i32 = ((v4) as i32);
    let mut v6: i32 = v5.wrapping_sub(1i32);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = method3(v3, v6, v7);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH0> = method35(v2, v8, v9);
    listed_30(v10)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>| -> Rc<RefCell<Vec<Rc<str>>>> {
        anchored_paths_34(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_proof_scope_manifests_under(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_proof_scope_anchored_paths(v0: &str, v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure2()(Rc::<str>::from(v0), v1)
}
