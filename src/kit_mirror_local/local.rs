#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
struct SpiralUse(std::rc::Rc<dyn Fn()>);
impl Drop for SpiralUse {
    fn drop(&mut self) { (self.0)() }
}
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
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i64, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(i32),
    US0_None,
}
pub struct Mut1 { pub l0: i32 }
pub struct Mut2 { pub l0: i32 }
pub struct Mut3 { pub l0: Rc<str> }
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method2(v3, v1);
            let v5: bool = &*v2 != "";
            if v5 {
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
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method5(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method5(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method4(v16, v17)
    }
}
fn method7(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
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
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method8(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
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
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method6(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let mut v6: i32 = 0i32;
            let mut v7: i32 = method7(v2.clone(), v5, v6);
            let mut v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
            let v10: i32 = (v9.clone().len() as i32);
            let mut v11: i32 = method8(v9.clone(), v10);
            let mut v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            Rc::new(UH0::UH0_Cons(v12, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn skipped_of_1(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: bool = &*v0 == "";
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".git"); } LIT.with(|lit| lit.clone()) };
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_build"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".cache"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v8));
        let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v9));
        let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v10));
        let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v11));
        let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v12));
        Rc::new(UH0::UH0_Cons(v2, v13))
    } else {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<UH0> = split_list_3(v15, v0);
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v18: Rc<UH0> = method6(v16, v17);
        let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method2(v18, v19)
    }
}
fn closure1() -> Rc<dyn Fn((u8)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u8)) -> Rc<str>> = Rc::new(move |mut v0: (u8)| -> Rc<str> {
        let mut v1: u8 = (v0);
        let v3: std::string::String = format!("{:02x}", v1);
        let v5: Rc<str> = Rc::<str>::from(String::as_str(&v3));
        v5
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method12(mut v0: Vec<Rc<str>>) -> Vec<Rc<str>> {
    v0
}
fn method13(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let mut v6: i32 = v1.wrapping_sub(1i32);
            let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method14() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    v0
}
fn method15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn entry_of_11(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, i64, Rc<str>) {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1.clone()));
    let v5: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v3);
    let mut v7: Vec<u8> = v5.unwrap();
    let mut v9: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(v7);
    let mut v11: std::io::BufReader<std::io::Cursor<Vec<u8>>> = std::io::BufReader::new(v9);
    let v13: bool = true; let mut v11 = v11;
    let v15: bool = true; let result : sha2::Sha256 = sha2::Digest::new();
    let mut v17: sha2::Sha256 = result;
    let v19: bool = true; let mut v17 = v17;
    let v20: usize = ((0i32) as usize);
    let mut v22: _ = [0u8; 1024i32 as usize];
    let v24: bool = true; loop { // rust.loop 1;
    let v26: bool = true; let mut v22 = v22;
    let v28: Result<usize, std::io::Error> = std::io::Read::read(&mut v11, &mut v22);
    let v30: usize = v28.unwrap();
    let v31: bool = v30 == v20 ;
    let v34: bool = if v31 {
        let v33: bool = true; break ();
        true
    } else {
        false
    };
    let v35: usize = ((v30) as usize);
    let v36: usize = (v35.clone());
    let v38: usize = v22.len();
    let v39: bool = v36 == v38 ;
    let v44: &_ = if v39 {
        let v41: &_ = &v22[v20..];
        v41
    } else {
        let v43: &_ = &v22[v20..v35];
        v43
    };
    let v46: bool = true; sha2::Digest::update(&mut v17, v44);
    let v48: bool = true; } // rust.loop 3;
    let v50: &[u8] = &sha2::Digest::finalize(v17);
    let v52: Vec<u8> = v50.iter().map(|x| *x).collect::<Vec<_>>();
    let mut v54: Rc<dyn Fn((u8)) -> Rc<str>> = closure1();
    let v55: Vec<Rc<str>> = v52.iter().map(|x| v54(x.clone())).collect::<Vec<_>>();
    let v56: Vec<Rc<str>> = method12(v55);
    let v57: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v56));
    let v58: Rc<Vec<Rc<str>>> = Rc::new((v57).borrow().clone());
    let v59: usize = (v58).len();
    let v60: i32 = ((v59) as i32);
    let v61: i32 = v60.wrapping_sub(1i32);
    let v62: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v63: Rc<UH0> = method13(v58, v61, v62);
    let v64: Rc<str> = method14();
    let v65: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v66: i32 = 0i32;
    let v67: i32 = method15(v65.clone(), v63, v66);
    let v68: Rc<Vec<Rc<str>>> = Rc::new(v65.borrow().clone());
    let v69: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v68).as_ref().clone()));
    let v70: Rc<str> = Rc::<str>::from((*v69.borrow()).join(&*v64));
    let v71: Result<Rc<str>, std::io::Error> = Ok::<Rc<str>, std::io::Error>(v70);
    let v73: Rc<str> = v71.unwrap();
    let mut v74: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v3);
    let v75: bool = v74.is_ok();
    let v76: bool = v75 == false;
    let v80: i64 = if v76 {
        0i64
    } else {
        let v77: std::fs::Metadata = v74.unwrap();
        let v78: u64 = v77.len();
        let v79: i64 = (v78 as i64);
        v79
    };
    (v1, v80, v73)
}
fn method10(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH1> = method10(v0.clone(), v4, v2);
            let (mut v6, mut v7, mut v8): (Rc<str>, i64, Rc<str>) = entry_of_11(v0, v3);
            Rc::new(UH1::UH1_Cons(v6, v7, v8, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn directory_exists_18(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    Rc::new(move |mut v2: Rc<UH0>| -> US0 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US0::US0_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US0> = method19(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US0::US0_None
            }
        }
    })
}
fn method19(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    closure2(v0, v1)
}
fn method17(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH0::UH0_Cons(v5, v6) => {
                let v5: Rc<str> = v5;
                let mut v6: Rc<UH0> = v6;
                let mut v7: bool = &*v2 == "";
                let v10: Rc<str> = if v7 {
                    v5.clone()
                } else {
                    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v5.clone()));
                    v9
                };
                let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v10.clone()));
                let v13: bool = directory_exists_18(v12);
                let v14: bool = v13 == false;
                let v23: Rc<UH0> = if v14 {
                    Rc::new(UH0::UH0_Cons(v10, v4))
                } else {
                    let mut v16: i32 = 0i32;
                    let mut v17: Rc<dyn Fn(Rc<UH0>) -> US0> = method19(v5, v16);
                    let v18: US0 = v17(v1.clone());
                    let v19: bool = match v18 {
                        US0::US0_None => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v20: bool = v19 != true;
                    if v20 {
                        v4
                    } else {
                        files_under_16(v0.clone(), v1.clone(), v10, v4)
                    }
                };
                (v3, v4) = (v6, v23);
                continue;
            }
            UH0::UH0_Nil => {
                return v4;
            }
        }
    }
}
fn child_names_20(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Vec<std::string::String> = v2.unwrap();
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: bool = true; let _vec_fold_ = v4.into_iter().fold(v5, |acc, x| { //;
    let mut v9: Rc<UH0> = acc;
    let mut v11: std::string::String = x;
    let v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v9));
    let mut v16: bool = true; v15 });
    let v18: Rc<UH0> = _vec_fold_;
    v18
}
fn files_under_16(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: Rc<UH0>) -> Rc<UH0> {
    let mut v4: bool = &*v2 == "";
    let mut v7: Rc<str> = if v4 {
        v0.clone()
    } else {
        let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2.clone()));
        v6
    };
    let mut v8: Rc<UH0> = child_names_20(v7);
    method17(v0, v1, v2, v8, v3)
}
fn closure3() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let mut v1: Rc<str> = (v0);
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method22() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure3()
}
fn method23(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method24(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: &Rc<RefCell<Mut2>>) -> bool {
    let v3: i32 = v2.borrow().l0;
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let v5: i32 = v2.borrow().l0;
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let mut v7: Rc<str> = v0.borrow()[v6 as usize].clone();
        let mut v8: bool = &*v7 < &*v1;
        let mut v11: i32 = if v8 {
            -1i32
        } else {
            let mut v9: bool = &*v7 > &*v1;
            if v9 {
                1i32
            } else {
                0i32
            }
        };
        let mut v12: bool = v11 == 1i32;
        v12
    } else {
        false
    }
}
fn sorted_21(mut v0: Rc<UH0>) -> Rc<UH0> {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v2: i32 = 0i32;
    let v3: i32 = method15(v1.clone(), v0, v2);
    let mut v4: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v4).as_ref().clone()));
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = v5 ;
    let mut v7: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method22();
    let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v6.borrow().clone()));
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v6.borrow().iter().map(|x| v7(x.clone())).collect::<Vec<_>>()));
    let v10: i32 = (v6.borrow().len() as i32);
    let mut v11: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 1i32 }));
    while method23(v10, &v11) {
        let v13: i32 = v11.borrow().l0;
        let v14: Rc<str> = v8.borrow()[v13 as usize].clone();
        let v15: Rc<str> = v9.borrow()[v13 as usize].clone();
        let mut v16: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v13 }));
        while method24(&v9, v15.clone(), &v16) {
            let v18: i32 = v16.borrow().l0;
            let v19: i32 = v18.wrapping_sub(1i32);
            let v20: Rc<str> = v8.borrow()[v19 as usize].clone();
            v8.borrow_mut()[v18 as usize] = v20;
            let v21: Rc<str> = v9.borrow()[v19 as usize].clone();
            v9.borrow_mut()[v18 as usize] = v21;
            v16.borrow_mut().l0 = v19;
            ()
        };
        let mut v22: i32 = v16.borrow().l0;
        v8.borrow_mut()[v22 as usize] = v14;
        let v23: i32 = v16.borrow().l0;
        v9.borrow_mut()[v23 as usize] = v15;
        let v24: i32 = v13.wrapping_add(1i32);
        v11.borrow_mut().l0 = v24;
        ()
    };
    let v25: Rc<RefCell<Vec<Rc<str>>>> = v8 ;
    let v26: Rc<Vec<Rc<str>>> = Rc::new((v25).borrow().clone());
    let v27: usize = (v26).len();
    let v28: i32 = ((v27) as i32);
    let v29: i32 = v28.wrapping_sub(1i32);
    let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method13(v26, v29, v30)
}
fn file_manifest_9(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<UH1> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = files_under_16(v0.clone(), v1, v2, v3);
    let v5: Rc<UH0> = sorted_21(v4);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method10(v0, v5, v6)
}
fn method28(mut v0: &Rc<RefCell<Mut3>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_27(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut3>> = Rc::new(RefCell::new(Mut3 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method28(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method26(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5) => {
            let mut v2: Rc<str> = v2;
            let v3: i64 = v3;
            let mut v4: Rc<str> = v4;
            let v5: Rc<UH1> = v5;
            let mut v6: Rc<UH0> = method26(v5, v1);
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let mut v8: Rc<str> = format_real_27(v3);
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v2));
            let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            Rc::new(UH0::UH0_Cons(v12, v6))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn manifest_text_25(mut v0: Rc<UH1>) -> Rc<str> {
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH0> = method26(v0, v1);
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v4: i32 = 0i32;
    let v5: i32 = method15(v3.clone(), v2, v4);
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let mut v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from((*v7.borrow()).join(&*v8));
    v9
}
fn hash_text_30(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v0.as_bytes().to_vec()));
    let mut v2: Vec<u8> = (v1).borrow().clone();
    let mut v4: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(v2);
    let mut v6: std::io::BufReader<std::io::Cursor<Vec<u8>>> = std::io::BufReader::new(v4);
    let mut v8: bool = true; let mut v6 = v6;
    let v10: bool = true; let result : sha2::Sha256 = sha2::Digest::new();
    let mut v12: sha2::Sha256 = result;
    let v14: bool = true; let mut v12 = v12;
    let v15: usize = ((0i32) as usize);
    let mut v17: _ = [0u8; 1024i32 as usize];
    let v19: bool = true; loop { // rust.loop 1;
    let v21: bool = true; let mut v17 = v17;
    let v23: Result<usize, std::io::Error> = std::io::Read::read(&mut v6, &mut v17);
    let v25: usize = v23.unwrap();
    let v26: bool = v25 == v15 ;
    let v29: bool = if v26 {
        let v28: bool = true; break ();
        true
    } else {
        false
    };
    let v30: usize = ((v25) as usize);
    let v31: usize = (v30.clone());
    let v33: usize = v17.len();
    let v34: bool = v31 == v33 ;
    let v39: &_ = if v34 {
        let v36: &_ = &v17[v15..];
        v36
    } else {
        let v38: &_ = &v17[v15..v30];
        v38
    };
    let v41: bool = true; sha2::Digest::update(&mut v12, v39);
    let v43: bool = true; } // rust.loop 3;
    let v45: &[u8] = &sha2::Digest::finalize(v12);
    let v47: Vec<u8> = v45.iter().map(|x| *x).collect::<Vec<_>>();
    let mut v49: Rc<dyn Fn((u8)) -> Rc<str>> = closure1();
    let v50: Vec<Rc<str>> = v47.iter().map(|x| v49(x.clone())).collect::<Vec<_>>();
    let v51: Vec<Rc<str>> = method12(v50);
    let v52: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v51));
    let v53: Rc<Vec<Rc<str>>> = Rc::new((v52).borrow().clone());
    let mut v54: usize = (v53).len();
    let v55: i32 = ((v54) as i32);
    let v56: i32 = v55.wrapping_sub(1i32);
    let v57: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v58: Rc<UH0> = method13(v53, v56, v57);
    let v59: Rc<str> = method14();
    let v60: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v61: i32 = 0i32;
    let v62: i32 = method15(v60.clone(), v58, v61);
    let v63: Rc<Vec<Rc<str>>> = Rc::new(v60.borrow().clone());
    let v64: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v63).as_ref().clone()));
    let v65: Rc<str> = Rc::<str>::from((*v64.borrow()).join(&*v59));
    let v66: Result<Rc<str>, std::io::Error> = Ok::<Rc<str>, std::io::Error>(v65);
    let v68: Rc<str> = v66.unwrap();
    v68
}
fn manifest_fingerprint_29(mut v0: Rc<UH1>) -> Rc<str> {
    let mut v1: Rc<str> = manifest_text_25(v0);
    hash_text_30(v1)
}
fn method31(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: i64 = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn listing_of_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32)>>> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: Rc<UH0> = skipped_of_1(v0);
    let mut v6: Rc<UH1> = file_manifest_9(v4, v5);
    let mut v7: Rc<str> = manifest_text_25(v6.clone());
    let mut v8: Rc<str> = manifest_fingerprint_29(v6.clone());
    let mut v9: i32 = 0i32;
    let v10: i32 = method31(v6, v9);
    Rc::new(RefCell::new(vec![(v7, v8, v10)]))
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32)>>> {
        listing_of_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_mirror_listing_of(v0: &str, v1: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32)>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_2617b4e49837 = Mut1;
pub type SpiralShape_97df63ebabc3 = Mut3;
