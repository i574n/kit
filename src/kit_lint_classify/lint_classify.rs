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
    US0_TermMacro,
    US0_TypeMacro,
}
#[derive(Clone)]
enum US1 {
    US1_InlineMacro,
    US1_GlobalDeclaration,
    US1_GlobalBody,
}
#[derive(Clone)]
enum US3 {
    US3_MacroLong,
    US3_MacroBlock,
    US3_MacroFlow,
    US3_MacroCast,
    US3_MacroArith,
}
#[derive(Clone)]
enum US2 {
    US2_Some(US3),
    US2_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>),
    US4_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn method5(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method6(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method6(v7, &v9) {
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
        method5(v16, v17)
    }
}
fn escaped_at_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let v3: bool = v1 < 0i32;
        let mut v9: bool = if v3 {
            true
        } else {
            let v4: usize = ((v1) as usize);
            let v5: i32 = v1.wrapping_add(1i32);
            let v6: usize = ((v5) as usize);
            let mut v7: Rc<str> = Rc::<str>::from(&(v0)[v4..v6]);
            let mut v8: bool = &*v7 != "\\";
            v8
        };
        if v9 {
            let v10: i32 = v2.wrapping_rem(2i32);
            let v11: bool = v10 == 1i32;
            return v11;
        } else {
            let v12: i32 = v1.wrapping_sub(1i32);
            let v13: i32 = v2.wrapping_add(1i32);
            (v1, v2) = (v12, v13);
            continue;
        }
    }
}
fn quote_merged_8(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let v4: i32 = (v2.clone().len() as i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: i32 = 0i32;
            let mut v7: bool = escaped_at_9(v2.clone(), v5, v6);
            if v7 {
                let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v1));
                Rc::new(UH0::UH0_Cons(v9, v3))
            } else {
                Rc::new(UH0::UH0_Cons(v1, v0))
            }
        }
        UH0::UH0_Nil => {
            let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v1, v13))
        }
    }
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH0> = quote_merged_8(v1, v2);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: i32, mut v2: Rc<str>) -> (i32, Rc<str>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: i32 = v1.wrapping_add(1i32);
                let v6: i32 = v1.wrapping_rem(2i32);
                let mut v7: bool = v6 == 0i32;
                let v10: Rc<str> = if v7 {
                    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v3));
                    v8
                } else {
                    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("literal"); } LIT.with(|lit| lit.clone()) }));
                    v9
                };
                (v0, v1, v2) = (v4, v5, v10);
                continue;
            }
            UH0::UH0_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn outside_strings_3(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = split_list_4(v1, v0);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH0> = method7(v2, v3);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = method5(v4, v5);
    let mut v7: i32 = 0i32;
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v9, mut v10): (i32, Rc<str>) = method10(v6, v7, v8);
    v10
}
fn method13(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
                let v5: Option<usize> = v2.find(&*v4);
                let v6: Option<usize> = method13(v5);
                let mut v7: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v8: Option<i32> = v6.map(|x| v7(x));
                let v10: i32 = v8.unwrap_or(-1i32);
                let v11: bool = v10 < 0i32;
                let v21: Rc<str> = if v11 {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v12
                } else {
                    let v13: i32 = v10.wrapping_add(1i32);
                    let v14: i32 = (v2.clone().len() as i32);
                    let v15: i32 = v14.wrapping_sub(v10);
                    let v16: i32 = v15.wrapping_sub(1i32);
                    let v17: usize = ((v13) as usize);
                    let v18: i32 = v13.wrapping_add(v16);
                    let v19: usize = ((v18) as usize);
                    let v20: Rc<str> = Rc::<str>::from(&(v2)[v17..v19]);
                    v20
                };
                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v21));
                (v0, v1) = (v3, v22);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn outside_brackets_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = split_list_4(v1, v0);
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            method12(v4, v3)
        }
        UH0::UH0_Nil => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v6
        }
    }
}
fn words_from_15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: i32 = v1.wrapping_add(1i32);
            let mut v7: bool = &*v5 == "";
            let mut v9: Rc<UH0> = if v7 {
                v2
            } else {
                Rc::new(UH0::UH0_Cons(v5, v2))
            };
            (v1, v2) = (v6, v9);
            continue;
        }
    }
}
fn words_of_14(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v2));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v2));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v2));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v2));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v2));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(v13.replace(&*v14, &*v2));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(v15.replace(&*v16, &*v2));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(v17.replace(&*v18, &*v2));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(v19.replace(&*v20, &*v2));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(v21.replace(&*v22, &*v2));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = Rc::<str>::from(v23.replace(&*v24, &*v2));
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = Rc::<str>::from(v25.replace(&*v26, &*v2));
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("+"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = Rc::<str>::from(v27.replace(&*v28, &*v2));
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = Rc::<str>::from(v29.replace(&*v30, &*v2));
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = Rc::<str>::from(v31.replace(&*v32, &*v2));
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%"); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = Rc::<str>::from(v33.replace(&*v34, &*v2));
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
    let v37: Rc<str> = Rc::<str>::from(v35.replace(&*v36, &*v2));
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<str> = Rc::<str>::from(v37.replace(&*v38, &*v2));
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
    let v41: Rc<str> = Rc::<str>::from(v39.replace(&*v40, &*v2));
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = Rc::<str>::from(v41.replace(&*v42, &*v2));
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("?"); } LIT.with(|lit| lit.clone()) };
    let v45: Rc<str> = Rc::<str>::from(v43.replace(&*v44, &*v2));
    let v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v45.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v47: i32 = 0i32;
    let v48: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    words_from_15(v46, v47, v48)
}
fn method16(mut v0: Rc<UH0>) -> US4 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: bool = &*v2 == "as";
                if v4 {
                    return US4::US4_Some(v2);
                } else {
                    v0 = v3;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US4::US4_None;
            }
        }
    }
}
fn method19(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
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
fn operator_between_18(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> bool {
    loop {
        let mut v2: i32 = (v0.clone().borrow().len() as i32);
        let v3: i32 = v1.wrapping_add(1i32);
        let v4: bool = v2 <= v3;
        if v4 {
            return false;
        } else {
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: i32 = (v5.clone().len() as i32);
            let mut v7: i32 = 0i32;
            let mut v8: i32 = method19(v5.clone(), v6, v7);
            let mut v9: i32 = v6.wrapping_sub(1i32);
            let v10: Rc<str> = string_slice(&v5, v8 as i64, v9 as i64);
            let v11: i32 = (v10.clone().len() as i32);
            let v12: i32 = method20(v10.clone(), v11);
            let v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
            let v14: i32 = (v13.clone().len() as i32);
            let v15: bool = v14 == 0i32;
            let v22: Rc<str> = if v15 {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v16
            } else {
                let v17: i32 = v14.wrapping_sub(1i32);
                let v18: usize = ((v17) as usize);
                let v19: i32 = v17.wrapping_add(1i32);
                let v20: usize = ((v19) as usize);
                let v21: Rc<str> = Rc::<str>::from(&(v13)[v18..v20]);
                v21
            };
            let v23: Rc<str> = v0.borrow()[v3 as usize].clone();
            let v24: i32 = (v23.clone().len() as i32);
            let v25: i32 = 0i32;
            let v26: i32 = method19(v23.clone(), v24, v25);
            let v27: i32 = v24.wrapping_sub(1i32);
            let v28: Rc<str> = string_slice(&v23, v26 as i64, v27 as i64);
            let v29: i32 = (v28.clone().len() as i32);
            let v30: i32 = method20(v28.clone(), v29);
            let v31: Rc<str> = string_slice(&v28, 0i32 as i64, v30 as i64);
            let v32: bool = &*v31 == "";
            let v37: Rc<str> = if v32 {
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v33
            } else {
                let v34: usize = ((0i32) as usize);
                let v35: usize = ((1i32) as usize);
                let v36: Rc<str> = Rc::<str>::from(&(v31)[v34..v35]);
                v36
            };
            let v38: bool = &*v22 != "";
            let v41: bool = if v38 {
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_"); } LIT.with(|lit| lit.clone()) };
                let v40: bool = v39.contains(&*v22);
                v40
            } else {
                false
            };
            let v43: bool = if v41 {
                true
            } else {
                let v42: bool = &*v22 == ")";
                v42
            };
            let v52: bool = if v43 {
                let mut v44: bool = &*v37 != "";
                let v47: bool = if v44 {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_"); } LIT.with(|lit| lit.clone()) };
                    let v46: bool = v45.contains(&*v37);
                    v46
                } else {
                    false
                };
                let v49: bool = if v47 {
                    true
                } else {
                    let v48: bool = &*v37 == "!";
                    v48
                };
                if v49 {
                    true
                } else {
                    let v50: bool = &*v37 == "(";
                    v50
                }
            } else {
                false
            };
            if v52 {
                return true;
            } else {
                v1 = v3;
                continue;
            }
        }
    }
}
fn arith_in_17(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("+"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = 0i32;
    let v4: bool = operator_between_18(v2, v3);
    let mut v9: bool = if v4 {
        true
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = 0i32;
        operator_between_18(v6, v7)
    };
    let v14: bool = if v9 {
        true
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v10).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v12: i32 = 0i32;
        operator_between_18(v11, v12)
    };
    let v19: bool = if v14 {
        true
    } else {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v15).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v17: i32 = 0i32;
        operator_between_18(v16, v17)
    };
    if v19 {
        true
    } else {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v20).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v22: i32 = 0i32;
        operator_between_18(v21, v22)
    }
}
fn term_class_2(mut v0: Rc<str>, mut v1: bool) -> US2 {
    let mut v2: Rc<str> = outside_strings_3(v0);
    if v1 {
        let v3: US3 = US3::US3_MacroFlow;
        US2::US2_Some(v3)
    } else {
        let v5: Rc<str> = outside_brackets_11(v2.clone());
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v5.contains(&*v6);
        if v7 {
            let mut v8: US3 = US3::US3_MacroBlock;
            US2::US2_Some(v8)
        } else {
            let v10: Rc<UH0> = words_of_14(v2.clone());
            let v11: US4 = method16(v10);
            let v12: bool = match v11 {
                US4::US4_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v13: bool = v12 != true;
            if v13 {
                let v14: US3 = US3::US3_MacroCast;
                US2::US2_Some(v14)
            } else {
                let v16: bool = arith_in_17(v2.clone());
                let v19: bool = if v16 {
                    true
                } else {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("=="); } LIT.with(|lit| lit.clone()) };
                    let v18: bool = v2.contains(&*v17);
                    v18
                };
                let v22: bool = if v19 {
                    true
                } else {
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!="); } LIT.with(|lit| lit.clone()) };
                    let v21: bool = v2.contains(&*v20);
                    v21
                };
                if v22 {
                    let v23: US3 = US3::US3_MacroArith;
                    US2::US2_Some(v23)
                } else {
                    US2::US2_None
                }
            }
        }
    }
}
fn macro_body_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = v1 >= 3i32;
    if v2 {
        let v3: i32 = v1.wrapping_sub(3i32);
        let v4: usize = ((2i32) as usize);
        let v5: i32 = 2i32.wrapping_add(v3);
        let v6: usize = ((v5) as usize);
        let mut v7: Rc<str> = Rc::<str>::from(&(v0)[v4..v6]);
        v7
    } else {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v8
    }
}
fn class_of_1(mut v0: i32, mut v1: &Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1, mut v6: bool) -> US2 {
    let mut v7: i32 = (v3.clone().len() as i32);
    let mut v8: bool = v7 > 120i32;
    match v5 {
        US1::US1_GlobalBody => {
            let mut v9: US3 = US3::US3_MacroBlock;
            US2::US2_Some(v9)
        }
        US1::US1_GlobalDeclaration => {
            if v8 {
                let v11: US3 = US3::US3_MacroLong;
                US2::US2_Some(v11)
            } else {
                US2::US2_None
            }
        }
        US1::US1_InlineMacro => {
            if v8 {
                let v15: US3 = US3::US3_MacroLong;
                US2::US2_Some(v15)
            } else {
                let v17: bool = v4 == false;
                if v17 {
                    let v18: US3 = US3::US3_MacroBlock;
                    US2::US2_Some(v18)
                } else {
                    match v2 {
                        US0::US0_TermMacro => {
                            let v21: Rc<str> = macro_body_21(v3);
                            term_class_2(v21, v6)
                        }
                        US0::US0_TypeMacro => {
                            US2::US2_None
                        }
                    }
                }
            }
        }
    }
}
fn classified_0(mut v0: Rc<str>, mut v1: bool, mut v2: bool, mut v3: i32, mut v4: i32) -> i32 {
    let v5: bool = v3 == 1i32;
    let mut v8: US0 = if v5 {
        US0::US0_TypeMacro
    } else {
        US0::US0_TermMacro
    };
    let mut v9: bool = v4 == 1i32;
    let v15: US1 = if v9 {
        US1::US1_GlobalDeclaration
    } else {
        let v11: bool = v4 == 2i32;
        if v11 {
            US1::US1_GlobalBody
        } else {
            US1::US1_InlineMacro
        }
    };
    let v16: i32 = 0i32;
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v18: US2 = class_of_1(v16, &v17, v8, v0, v1, v15, v2);
    match v18 {
        US2::US2_None => {
            -1i32
        }
        US2::US2_Some(v19) => {
            let v19: US3 = v19;
            match v19 {
                US3::US3_MacroArith => {
                    4i32
                }
                US3::US3_MacroBlock => {
                    1i32
                }
                US3::US3_MacroCast => {
                    3i32
                }
                US3::US3_MacroFlow => {
                    2i32
                }
                US3::US3_MacroLong => {
                    0i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, bool, bool, i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, bool, bool, i32, i32) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: bool, mut v2: bool, mut v3: i32, mut v4: i32| -> i32 {
        classified_0(v0, v1, v2, v3, v4)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method24(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_23(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method24(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn tsv_row_22(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    let v5: bool = v3 == 0i32;
    let v25: US2 = if v5 {
        let v6: US3 = US3::US3_MacroLong;
        US2::US2_Some(v6)
    } else {
        let mut v8: bool = v3 == 1i32;
        if v8 {
            let mut v9: US3 = US3::US3_MacroBlock;
            US2::US2_Some(v9)
        } else {
            let v11: bool = v3 == 2i32;
            if v11 {
                let v12: US3 = US3::US3_MacroFlow;
                US2::US2_Some(v12)
            } else {
                let v14: bool = v3 == 3i32;
                if v14 {
                    let v15: US3 = US3::US3_MacroCast;
                    US2::US2_Some(v15)
                } else {
                    let v17: bool = v3 == 4i32;
                    if v17 {
                        let v18: US3 = US3::US3_MacroArith;
                        US2::US2_Some(v18)
                    } else {
                        US2::US2_None
                    }
                }
            }
        }
    };
    let v35: Rc<str> = match v25 {
        US2::US2_None => {
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v33
        }
        US2::US2_Some(v26) => {
            let v26: US3 = v26;
            match v26 {
                US3::US3_MacroArith => {
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arith"); } LIT.with(|lit| lit.clone()) };
                    v31
                }
                US3::US3_MacroBlock => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("block"); } LIT.with(|lit| lit.clone()) };
                    v28
                }
                US3::US3_MacroCast => {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cast"); } LIT.with(|lit| lit.clone()) };
                    v30
                }
                US3::US3_MacroFlow => {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("flow"); } LIT.with(|lit| lit.clone()) };
                    v29
                }
                US3::US3_MacroLong => {
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("long"); } LIT.with(|lit| lit.clone()) };
                    v27
                }
            }
        }
    };
    let v36: Rc<str> = format_real_23(v1);
    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<str> = Rc::<str>::from(v4.replace(&*v37, &*v38));
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v41: Rc<str> = Rc::<str>::from(v39.replace(&*v40, &*v38));
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = Rc::<str>::from(v41.replace(&*v42, &*v38));
    let mut v44: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v44.borrow_mut().push(v0);
    v44.borrow_mut().push(v36);
    v44.borrow_mut().push(v2);
    v44.borrow_mut().push(v35);
    v44.borrow_mut().push(v43);
    let v45: Rc<Vec<Rc<str>>> = Rc::new(v44.borrow().clone());
    let v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v45).as_ref().clone()));
    let v47: Rc<str> = Rc::<str>::from((*v46.borrow()).join(&*v37));
    v47
}
fn closure2() -> Rc<dyn Fn(Rc<str>, i32, Rc<str>, i32, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, Rc<str>, i32, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>| -> Rc<str> {
        tsv_row_22(v0, v1, v2, v3, v4)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_class(v0: &str, v1: bool, v2: bool, v3: i32, v4: i32) -> i32 {
    closure0()(Rc::<str>::from(v0), v1, v2, v3, v4)
}
pub fn kit_lint_tsv_row(v0: &str, v1: i32, v2: &str, v3: i32, v4: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2), v3, Rc::<str>::from(v4))
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
