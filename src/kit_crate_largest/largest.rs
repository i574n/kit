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
struct Mut0 { l0: Rc<str> }
fn longer_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: i32) -> (Rc<str>, i32) {
    let v5: bool = &*v0 != "";
    let v8: bool = if v5 {
        let v6: i32 = v2.wrapping_sub(v1);
        let v7: bool = v6 > v4;
        v7
    } else {
        false
    };
    let (mut v10, mut v11): (Rc<str>, i32) = if v8 {
        let v9: i32 = v2.wrapping_sub(v1);
        (v0, v9)
    } else {
        (v3, v4)
    };
    (v10, v11)
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
fn method7(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn function_name_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: i32 = 0i32;
    let v3: i32 = method5(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method6(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let v9: i32 = (v8.clone().len() as i32);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pub fn "); } LIT.with(|lit| lit.clone()) };
    let v11: bool = v8.starts_with(&*v10);
    let v26: Rc<str> = if v11 {
        let v12: i32 = v9.wrapping_sub(7i32);
        let v13: usize = ((7i32) as usize);
        let v14: i32 = 7i32.wrapping_add(v12);
        let v15: usize = ((v14) as usize);
        let v16: Rc<str> = Rc::<str>::from(&(v8)[v13..v15]);
        v16.clone()
    } else {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn "); } LIT.with(|lit| lit.clone()) };
        let v18: bool = v8.starts_with(&*v17);
        if v18 {
            let v19: i32 = v9.wrapping_sub(3i32);
            let v20: usize = ((3i32) as usize);
            let v21: i32 = 3i32.wrapping_add(v19);
            let v22: usize = ((v21) as usize);
            let v23: Rc<str> = Rc::<str>::from(&(v8)[v20..v22]);
            v23.clone()
        } else {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24.clone()
        }
    };
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
    let v28: Option<usize> = v26.find(&*v27);
    let v29: Option<usize> = method7(v28.clone());
    let mut v30: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v31: Option<i32> = v29.map(|x| v30(x));
    let v33: i32 = v31.unwrap_or(-1i32);
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
    let v35: Option<usize> = v26.find(&*v34);
    let v36: Option<usize> = method7(v35.clone());
    let mut v37: Option<i32> = v36.map(|x| v30(x));
    let v39: i32 = v37.unwrap_or(-1i32);
    let v40: bool = v39 >= 0i32;
    let v44: bool = if v40 {
        let v41: bool = v33 < 0i32;
        if v41 {
            true
        } else {
            let v42: bool = v39 < v33;
            v42
        }
    } else {
        false
    };
    let v45: i32 = if v44 {
        v39
    } else {
        v33
    };
    let v46: bool = v45 <= 0i32;
    if v46 {
        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v47.clone()
    } else {
        let v48: usize = ((0i32) as usize);
        let v49: usize = ((v45) as usize);
        let v50: Rc<str> = Rc::<str>::from(&(v26)[v48..v49]);
        v50.clone()
    }
}
fn largest_from_2(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: i32) -> (Rc<str>, i32) {
    let v6: i32 = (v0.clone().borrow().len() as i32);
    let v7: bool = v6 <= v1;
    let (mut v23, mut v24): (Rc<str>, i32) = if v7 {
        let (mut v8, mut v9): (Rc<str>, i32) = longer_3(v2.clone(), v3, v1, v4.clone(), v5);
        (v8, v9)
    } else {
        let v10: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
        let v11: Rc<str> = function_name_4(v10);
        let v12: bool = &*v11 == "";
        let (mut v21, mut v22): (Rc<str>, i32) = if v12 {
            let v13: i32 = v1.wrapping_add(1i32);
            let (mut v14, mut v15): (Rc<str>, i32) = largest_from_2(v0.clone(), v13, v2.clone(), v3, v4.clone(), v5);
            (v14, v15)
        } else {
            let v16: i32 = v1.wrapping_add(1i32);
            let (mut v17, mut v18): (Rc<str>, i32) = longer_3(v2, v3, v1, v4, v5);
            let (mut v19, mut v20): (Rc<str>, i32) = largest_from_2(v0, v16, v11, v1, v17, v18);
            (v19, v20)
        };
        (v21, v22)
    };
    (v23, v24)
}
fn largest_function_of_1(mut v0: Rc<str>) -> (Rc<str>, i32) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v6: i32 = 0i32;
    let v7: i32 = 0i32;
    let v8: i32 = 0i32;
    let (mut v9, mut v10): (Rc<str>, i32) = largest_from_2(v5.clone(), v6, v2.clone(), v7, v2.clone(), v8);
    (v9.clone(), v10)
}
fn method9(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_8(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method9(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn largest_text_0(mut v0: Rc<str>) -> Rc<str> {
    let (mut v1, mut v2): (Rc<str>, i32) = largest_function_of_1(v0.clone());
    let v3: bool = &*v1 == "";
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" largest "); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v1));
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
        let v9: Rc<str> = format_real_8(v2);
        let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
        v10.clone()
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        largest_text_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_crate_largest(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
