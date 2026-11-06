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
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut1 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
enum US1 {
    US1_Ok(Vec<std::string::String>),
    US1_Error(std::io::Error),
}
pub struct Mut2 { pub l0: i32 }
pub struct Mut3 { pub l0: i32 }
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
enum US3 {
    US3_Ok,
    US3_Error(std::io::Error),
}
fn joined_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn get_environment_variable_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn heavy_queue_0() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_HEAVY_QUEUE"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = get_environment_variable_2(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = get_environment_variable_2(v2);
    let mut v4: bool = &*v3 != "";
    let v7: Rc<str> = if v4 {
        v3
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HOME"); } LIT.with(|lit| lit.clone()) };
        get_environment_variable_2(v5)
    };
    let v8: bool = &*v1 != "";
    if v8 {
        v1
    } else {
        let mut v9: bool = &*v7 == "";
        if v9 {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v10
        } else {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral-bin"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = joined_1(v7, v11);
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-queue"); } LIT.with(|lit| lit.clone()) };
            joined_1(v12, v13)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        heavy_queue_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method5(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_4(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn closure2(mut v0: i32, mut v1: Rc<str>, mut v2: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    Rc::new(move |mut v3: Rc<str>| -> Rc<str> {
        let mut v4: bool = v2 >= v0;
        if v4 {
            v3
        } else {
            let mut v5: i32 = v2.wrapping_add(1i32);
            let mut v6: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method6(v0, v1.clone(), v5);
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1.clone()));
            v6(v7)
        }
    })
}
fn method6(mut v0: i32, mut v1: Rc<str>, mut v2: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    closure2(v0, v1, v2)
}
fn text_written_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2
}
fn ticket_written_7(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v0).err().map(|error| format!("create {}: {error}", &*v0)).unwrap_or_default());
    let mut v3: bool = &*v2 == "";
    if v3 {
        let mut v4: Rc<str> = joined_1(v0, v1);
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = text_written_8(v4, v5);
        let v7: bool = &*v6 == "";
        v7
    } else {
        false
    }
}
fn ticket_created_3(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: std::time::Duration = std::time::UNIX_EPOCH.elapsed().unwrap_or_default();
    let mut v2: u64 = v1.as_secs();
    let mut v3: u64 = v2.wrapping_mul(10000000u64);
    let mut v4: u32 = v1.subsec_nanos();
    let mut v5: u64 = (v4 as u64);
    let mut v6: u64 = v5.wrapping_div(100u64);
    let v7: u64 = v3.wrapping_add(v6);
    let v8: u64 = v7.wrapping_add(621355968000000000u64);
    let mut v9: u32 = std::process::id();
    let mut v10: u64 = (v9 as u64);
    let mut v11: Rc<str> = format_real_4(v8);
    let v12: Rc<str> = Rc::<str>::from(((b'0') as char).to_string());
    let mut v13: i32 = (v11.clone().len() as i32);
    let mut v14: i32 = 20i32.wrapping_sub(v13);
    let mut v15: i32 = 0i32;
    let mut v16: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method6(v14, v12, v15);
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = v16(v17.clone());
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v11));
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
    let v21: Rc<str> = format_real_4(v10);
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".t"); } LIT.with(|lit| lit.clone()) }));
    let v24: bool = ticket_written_7(v0, v23.clone());
    if v24 {
        v23
    } else {
        v17
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        ticket_created_3(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_12(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".t"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(2i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method15(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method15(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method14(v16, v17)
    }
}
fn method16(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure4() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn process_alive_17(mut v0: u64) -> bool {
    let mut v1: bool = kit_process_alive::kit_process_alive(v0);
    v1
}
fn first_live_11(mut v0: Rc<UH0>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let mut v5: Rc<str> = loop_12(v2.clone());
                let mut v6: Rc<UH0> = split_list_13(v4, v5);
                let v23: u64 = match Rc::unwrap_or_clone(v6) {
                    UH0::UH0_Cons(v7, v8) => {
                        let v7: Rc<str> = v7;
                        let v8: Rc<UH0> = v8;
                        match Rc::unwrap_or_clone(v8) {
                            UH0::UH0_Cons(v9, v10) => {
                                let mut v9: Rc<str> = v9;
                                let mut v10: Rc<UH0> = v10;
                                let mut v11: Option<u64> = (v9).trim().parse::<u64>().ok();
                                let v12: Option<u64> = method16(v11);
                                let mut v13: Rc<dyn Fn((u64)) -> US0> = closure4();
                                let mut v14: Option<US0> = v12.map(|x| v13(x));
                                let mut v16: US0 = US0::US0_None;
                                let v17: US0 = v16.clone();
                                let v18: US0 = v14.unwrap_or(v17);
                                match v18 {
                                    US0::US0_None => {
                                        0u64
                                    }
                                    US0::US0_Some(v19) => {
                                        let v19: u64 = v19;
                                        v19
                                    }
                                }
                            }
                            _ => {
                                0u64
                            }
                        }
                    }
                    _ => {
                        0u64
                    }
                };
                let v24: bool = process_alive_17(v23);
                if v24 {
                    return v2;
                } else {
                    v0 = v3;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v1;
            }
        }
    }
}
fn method19(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let mut v5: Rc<str> = (v0)[v4].clone();
            let mut v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn ok_c5() -> Rc<dyn Fn(Vec<std::string::String>) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<std::string::String>) -> US1> = Rc::new(move |mut v0: Vec<std::string::String>| -> US1 {
        US1::US1_Ok(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method20() -> Rc<dyn Fn(Vec<std::string::String>) -> US1> {
    ok_c5()
}
fn error_c6() -> Rc<dyn Fn(std::io::Error) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US1> = Rc::new(move |mut v0: std::io::Error| -> US1 {
        US1::US1_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method21() -> Rc<dyn Fn(std::io::Error) -> US1> {
    error_c6()
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method22(v3, v1);
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".t"); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.ends_with(&*v5);
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
fn method23(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn closure7() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let mut v1: Rc<str> = (v0);
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method24() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure7()
}
fn method25(mut v0: i32, mut v1: &Rc<RefCell<Mut2>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn method26(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: &Rc<RefCell<Mut3>>) -> bool {
    let mut v3: i32 = v2.borrow().l0;
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let mut v5: i32 = v2.borrow().l0;
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let v7: Rc<str> = v0.borrow()[v6 as usize].clone();
        let v8: bool = &*v7 < &*v1;
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
        let v12: bool = v11 == 1i32;
        v12
    } else {
        false
    }
}
fn tickets_sorted_18(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v3: Rc<dyn Fn(Vec<std::string::String>) -> US1> = method20();
    let mut v4: Rc<dyn Fn(std::io::Error) -> US1> = method21();
    let mut v5: US1 = match v2 { Ok(x) => v3(x), Err(e) => v4(e) };
    let v26: Rc<UH0> = match v5 {
        US1::US1_Error(v23) => {
            let v23: std::io::Error = v23;
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US1::US1_Ok(v6) => {
            let mut v6: Vec<std::string::String> = v6;
            let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v9: bool = true; let _vec_fold_ = v6.into_iter().fold(v7, |acc, x| { //;
            let mut v11: Rc<UH0> = acc;
            let mut v13: std::string::String = x;
            let mut v15: Rc<str> = Rc::<str>::from(String::as_str(&v13));
            let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v11));
            let v18: bool = true; v17 });
            let v20: Rc<UH0> = _vec_fold_;
            let v21: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            method14(v20, v21)
        }
    };
    let mut v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v28: Rc<UH0> = method22(v26, v27);
    let v29: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v30: i32 = 0i32;
    let mut v31: i32 = method23(v29.clone(), v28, v30);
    let mut v32: Rc<Vec<Rc<str>>> = Rc::new(v29.borrow().clone());
    let v33: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v32).as_ref().clone()));
    let v34: Rc<RefCell<Vec<Rc<str>>>> = v33 ;
    let mut v35: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method24();
    let mut v36: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v34.borrow().clone()));
    let mut v37: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v34.borrow().iter().map(|x| v35(x.clone())).collect::<Vec<_>>()));
    let v38: i32 = (v34.borrow().len() as i32);
    let mut v39: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: 1i32 }));
    while method25(v38, &v39) {
        let v41: i32 = v39.borrow().l0;
        let v42: Rc<str> = v36.borrow()[v41 as usize].clone();
        let v43: Rc<str> = v37.borrow()[v41 as usize].clone();
        let mut v44: Rc<RefCell<Mut3>> = Rc::new(RefCell::new(Mut3 { l0: v41 }));
        while method26(&v37, v43.clone(), &v44) {
            let v46: i32 = v44.borrow().l0;
            let v47: i32 = v46.wrapping_sub(1i32);
            let v48: Rc<str> = v36.borrow()[v47 as usize].clone();
            v36.borrow_mut()[v46 as usize] = v48;
            let v49: Rc<str> = v37.borrow()[v47 as usize].clone();
            v37.borrow_mut()[v46 as usize] = v49;
            v44.borrow_mut().l0 = v47;
            ()
        };
        let mut v50: i32 = v44.borrow().l0;
        v36.borrow_mut()[v50 as usize] = v42;
        let v51: i32 = v44.borrow().l0;
        v37.borrow_mut()[v51 as usize] = v43;
        let v52: i32 = v41.wrapping_add(1i32);
        v39.borrow_mut().l0 = v52;
        ()
    };
    let v53: Rc<RefCell<Vec<Rc<str>>>> = v36 ;
    let v54: Rc<Vec<Rc<str>>> = Rc::new((v53).borrow().clone());
    let v55: usize = (v54).len();
    let v56: i32 = ((v55) as i32);
    let v57: i32 = v56.wrapping_sub(1i32);
    let v58: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method19(v54, v57, v58)
}
fn first_live_ticket_10(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<UH0> = tickets_sorted_18(v0);
    first_live_11(v1)
}
fn front_ticket_9(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = first_live_ticket_10(v0.clone());
    let mut v2: bool = &*v1 == "";
    let v22: bool = if v2 {
        true
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = loop_12(v1.clone());
        let mut v5: Rc<UH0> = split_list_13(v3, v4);
        let v20: u64 = match Rc::unwrap_or_clone(v5) {
            UH0::UH0_Cons(v6, v7) => {
                let mut v6: Rc<str> = v6;
                let v7: Rc<UH0> = v7;
                let v8: Option<u64> = (v6).trim().parse::<u64>().ok();
                let mut v9: Option<u64> = method16(v8);
                let mut v10: Rc<dyn Fn((u64)) -> US0> = closure4();
                let mut v11: Option<US0> = v9.map(|x| v10(x));
                let mut v13: US0 = US0::US0_None;
                let mut v14: US0 = v13.clone();
                let mut v15: US0 = v11.unwrap_or(v14);
                match v15 {
                    US0::US0_None => {
                        0u64
                    }
                    US0::US0_Some(v16) => {
                        let mut v16: u64 = v16;
                        v16
                    }
                }
            }
            UH0::UH0_Nil => {
                0u64
            }
        };
        let v21: bool = v20 == 0u64;
        v21
    };
    if v22 {
        ticket_created_3(v0)
    } else {
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = loop_12(v1);
        let v26: Rc<UH0> = split_list_13(v24, v25);
        let v41: u64 = match Rc::unwrap_or_clone(v26) {
            UH0::UH0_Cons(v27, v28) => {
                let mut v27: Rc<str> = v27;
                let v28: Rc<UH0> = v28;
                let v29: Option<u64> = (v27).trim().parse::<u64>().ok();
                let v30: Option<u64> = method16(v29);
                let mut v31: Rc<dyn Fn((u64)) -> US0> = closure4();
                let mut v32: Option<US0> = v30.map(|x| v31(x));
                let v34: US0 = US0::US0_None;
                let mut v35: US0 = v34.clone();
                let mut v36: US0 = v32.unwrap_or(v35);
                match v36 {
                    US0::US0_None => {
                        0u64
                    }
                    US0::US0_Some(v37) => {
                        let mut v37: u64 = v37;
                        v37
                    }
                }
            }
            UH0::UH0_Nil => {
                0u64
            }
        };
        let v42: u32 = std::process::id();
        let v43: u64 = (v42 as u64);
        let mut v44: u64 = v41.wrapping_sub(1u64);
        let v45: Rc<str> = format_real_4(v44);
        let v46: Rc<str> = Rc::<str>::from(((b'0') as char).to_string());
        let v47: i32 = (v45.clone().len() as i32);
        let v48: i32 = 20i32.wrapping_sub(v47);
        let v49: i32 = 0i32;
        let mut v50: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method6(v48, v46, v49);
        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v52: Rc<str> = v50(v51.clone());
        let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v45));
        let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v53, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
        let v55: Rc<str> = format_real_4(v43);
        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v55));
        let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".t"); } LIT.with(|lit| lit.clone()) }));
        let v58: bool = ticket_written_7(v0, v57.clone());
        if v58 {
            v57
        } else {
            v51
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        front_ticket_9(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        first_live_ticket_10(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn live_rank_28(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = &*v3 == &*v1;
                if v5 {
                    return v2;
                } else {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let v7: Rc<str> = loop_12(v3);
                    let v8: Rc<UH0> = split_list_13(v6, v7);
                    let v25: u64 = match Rc::unwrap_or_clone(v8) {
                        UH0::UH0_Cons(v9, v10) => {
                            let mut v9: Rc<str> = v9;
                            let mut v10: Rc<UH0> = v10;
                            match Rc::unwrap_or_clone(v10) {
                                UH0::UH0_Cons(v11, v12) => {
                                    let mut v11: Rc<str> = v11;
                                    let v12: Rc<UH0> = v12;
                                    let mut v13: Option<u64> = (v11).trim().parse::<u64>().ok();
                                    let mut v14: Option<u64> = method16(v13);
                                    let mut v15: Rc<dyn Fn((u64)) -> US0> = closure4();
                                    let mut v16: Option<US0> = v14.map(|x| v15(x));
                                    let v18: US0 = US0::US0_None;
                                    let v19: US0 = v18.clone();
                                    let v20: US0 = v16.unwrap_or(v19);
                                    match v20 {
                                        US0::US0_None => {
                                            0u64
                                        }
                                        US0::US0_Some(v21) => {
                                            let v21: u64 = v21;
                                            v21
                                        }
                                    }
                                }
                                _ => {
                                    0u64
                                }
                            }
                        }
                        _ => {
                            0u64
                        }
                    };
                    let v26: bool = process_alive_17(v25);
                    let v28: i32 = if v26 {
                        let mut v27: i32 = v2.wrapping_add(1i32);
                        v27
                    } else {
                        v2
                    };
                    (v0, v2) = (v4, v28);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return -1i32;
            }
        }
    }
}
fn ticket_rank_27(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<UH0> = tickets_sorted_18(v1);
    let mut v3: i32 = 0i32;
    live_rank_28(v2, v0, v3)
}
fn closure9() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        ticket_rank_27(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn jobs_root_30() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn text_or_empty_31(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method32(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method33(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn method34(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure11() -> Rc<dyn Fn((i32)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US2> = Rc::new(move |mut v0: (i32)| -> US2 {
        let mut v1: i32 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn heavy_slots_29() -> i32 {
    let mut v0: Rc<str> = jobs_root_30();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-slots"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = joined_1(v0, v1);
    let mut v3: Rc<str> = text_or_empty_31(v2);
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: i32 = 0i32;
    let mut v6: i32 = method32(v3.clone(), v4, v5);
    let v7: i32 = v4.wrapping_sub(1i32);
    let v8: Rc<str> = string_slice(&v3, v6 as i64, v7 as i64);
    let mut v9: i32 = (v8.clone().len() as i32);
    let mut v10: i32 = method33(v8.clone(), v9);
    let mut v11: Rc<str> = string_slice(&v8, 0i32 as i64, v10 as i64);
    let v12: Option<i32> = (v11).trim().parse::<i32>().ok();
    let mut v13: Option<i32> = method34(v12);
    let mut v14: Rc<dyn Fn((i32)) -> US2> = closure11();
    let mut v15: Option<US2> = v13.map(|x| v14(x));
    let v17: US2 = US2::US2_None;
    let v18: US2 = v17.clone();
    let v19: US2 = v15.unwrap_or(v18);
    let v22: i32 = match v19 {
        US2::US2_None => {
            1i32
        }
        US2::US2_Some(v20) => {
            let v20: i32 = v20;
            v20
        }
    };
    let v23: bool = v22 < 1i32;
    if v23 {
        1i32
    } else {
        v22
    }
}
fn closure10() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        heavy_slots_29()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn ok_c13() -> Rc<dyn Fn() -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US3> = Rc::new(move || -> US3 {
        US3::US3_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method37() -> Rc<dyn Fn() -> US3> {
    ok_c13()
}
fn error_c14() -> Rc<dyn Fn(std::io::Error) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US3> = Rc::new(move |mut v0: std::io::Error| -> US3 {
        US3::US3_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method38() -> Rc<dyn Fn(std::io::Error) -> US3> {
    error_c14()
}
fn renamed_36(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v3: Result<(), std::io::Error> = std::fs::rename(&*v0, &*v1);
    let mut v4: Rc<dyn Fn() -> US3> = method37();
    let mut v5: Rc<dyn Fn(std::io::Error) -> US3> = method38();
    let mut v6: US3 = match v3 { Ok(()) => v4(), Err(e) => v5(e) };
    match v6 {
        US3::US3_Error(v7) => {
            let v7: std::io::Error = v7;
            false
        }
        US3::US3_Ok => {
            true
        }
    }
}
fn handed_ticket_owned_35(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = loop_12(v0.clone());
    let mut v4: Rc<UH0> = split_list_13(v2, v3);
    let v19: u64 = match Rc::unwrap_or_clone(v4) {
        UH0::UH0_Cons(v5, v6) => {
            let mut v5: Rc<str> = v5;
            let mut v6: Rc<UH0> = v6;
            let v7: Option<u64> = (v5).trim().parse::<u64>().ok();
            let v8: Option<u64> = method16(v7);
            let mut v9: Rc<dyn Fn((u64)) -> US0> = closure4();
            let mut v10: Option<US0> = v8.map(|x| v9(x));
            let v12: US0 = US0::US0_None;
            let mut v13: US0 = v12.clone();
            let mut v14: US0 = v10.unwrap_or(v13);
            match v14 {
                US0::US0_None => {
                    0u64
                }
                US0::US0_Some(v15) => {
                    let mut v15: u64 = v15;
                    v15
                }
            }
        }
        UH0::UH0_Nil => {
            0u64
        }
    };
    let v20: u32 = std::process::id();
    let v21: u64 = (v20 as u64);
    let v22: Rc<str> = format_real_4(v19);
    let v23: Rc<str> = Rc::<str>::from(((b'0') as char).to_string());
    let v24: i32 = (v22.clone().len() as i32);
    let v25: i32 = 20i32.wrapping_sub(v24);
    let v26: i32 = 0i32;
    let mut v27: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method6(v25, v23, v26);
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = v27(v28.clone());
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v22));
    let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
    let mut v32: Rc<str> = format_real_4(v21);
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".t"); } LIT.with(|lit| lit.clone()) }));
    let mut v35: bool = &*v0 != "";
    let mut v39: bool = if v35 {
        let mut v36: Rc<str> = joined_1(v1.clone(), v0);
        let mut v37: Rc<str> = joined_1(v1, v34.clone());
        renamed_36(v36, v37)
    } else {
        false
    };
    if v39 {
        v34
    } else {
        v28
    }
}
fn closure12() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        handed_ticket_owned_35(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn ticket_mine_39(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = loop_12(v0);
    let mut v3: Rc<UH0> = split_list_13(v1, v2);
    let v20: u64 = match Rc::unwrap_or_clone(v3) {
        UH0::UH0_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4;
            let mut v5: Rc<UH0> = v5;
            match Rc::unwrap_or_clone(v5) {
                UH0::UH0_Cons(v6, v7) => {
                    let mut v6: Rc<str> = v6;
                    let v7: Rc<UH0> = v7;
                    let v8: Option<u64> = (v6).trim().parse::<u64>().ok();
                    let mut v9: Option<u64> = method16(v8);
                    let mut v10: Rc<dyn Fn((u64)) -> US0> = closure4();
                    let mut v11: Option<US0> = v9.map(|x| v10(x));
                    let mut v13: US0 = US0::US0_None;
                    let mut v14: US0 = v13.clone();
                    let mut v15: US0 = v11.unwrap_or(v14);
                    match v15 {
                        US0::US0_None => {
                            0u64
                        }
                        US0::US0_Some(v16) => {
                            let mut v16: u64 = v16;
                            v16
                        }
                    }
                }
                _ => {
                    0u64
                }
            }
        }
        _ => {
            0u64
        }
    };
    let v21: u32 = std::process::id();
    let v22: u64 = (v21 as u64);
    let v23: bool = v20 == v22;
    v23
}
fn closure15() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        ticket_mine_39(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_ticket_queue() -> Rc<str> {
    closure0()()
}
pub fn kit_job_ticket_created(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
pub fn kit_job_ticket_front(v0: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0))
}
pub fn kit_job_ticket_first_live(v0: &str) -> Rc<str> {
    closure8()(Rc::<str>::from(v0))
}
pub fn kit_job_ticket_rank(v0: &str, v1: &str) -> i32 {
    closure9()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_job_ticket_slots() -> i32 {
    closure10()()
}
pub fn kit_job_ticket_handed(v0: &str, v1: &str) -> Rc<str> {
    closure12()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_job_ticket_mine(v0: &str) -> bool {
    closure15()(Rc::<str>::from(v0))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
pub type SpiralShape_652accfc6a8d = Mut1;
pub type SpiralShape_2617b4e49837 = Mut2;
