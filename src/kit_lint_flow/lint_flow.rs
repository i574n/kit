#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
fn code_at_6(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> i32 {
    let mut v2: bool = v1 < 0i32;
    let mut v5: bool = if v2 {
        true
    } else {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v1 >= v3;
        v4
    };
    if v5 {
        -1i32
    } else {
        let mut v6: u8 = v0.borrow()[v1 as usize];
        let v7: i32 = (v6 as i32);
        v7
    }
}
fn word_code_7(mut v0: i32) -> bool {
    let mut v1: bool = v0 >= 97i32;
    let mut v3: bool = if v1 {
        let mut v2: bool = v0 <= 122i32;
        v2
    } else {
        false
    };
    let v7: bool = if v3 {
        true
    } else {
        let mut v4: bool = v0 >= 65i32;
        if v4 {
            let mut v5: bool = v0 <= 90i32;
            v5
        } else {
            false
        }
    };
    let v11: bool = if v7 {
        true
    } else {
        let v8: bool = v0 >= 48i32;
        if v8 {
            let v9: bool = v0 <= 57i32;
            v9
        } else {
            false
        }
    };
    if v11 {
        true
    } else {
        let mut v12: bool = v0 == 95i32;
        v12
    }
}
fn edge_at_5(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> bool {
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: i32 = code_at_6(v0.clone(), v2);
    let mut v4: bool = word_code_7(v3);
    let mut v5: i32 = code_at_6(v0, v1);
    let mut v6: bool = word_code_7(v5);
    let v7: bool = v4 != v6;
    v7
}
fn bytes_from_9(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: Rc<RefCell<Vec<u8>>>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let mut v4: i32 = (v1.clone().borrow().len() as i32);
        let mut v5: bool = v3 >= v4;
        if v5 {
            return true;
        } else {
            let mut v6: i32 = v2.wrapping_add(v3);
            let v7: i32 = code_at_6(v0.clone(), v6);
            let v8: i32 = code_at_6(v1.clone(), v3);
            let v9: bool = v7 == v8;
            if v9 {
                let v10: i32 = v3.wrapping_add(1i32);
                v3 = v10;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn text_at_8(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>) -> bool {
    let mut v3: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v2.as_bytes().to_vec()));
    let mut v4: i32 = 0i32;
    bytes_from_9(v0, v3, v1, v4)
}
fn word_from_4(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v2 > v3;
        if v4 {
            return false;
        } else {
            let mut v5: bool = edge_at_5(v0.clone(), v2);
            let v7: bool = if v5 {
                text_at_8(v0.clone(), v2, v1.clone())
            } else {
                false
            };
            let v11: bool = if v7 {
                let v8: i32 = (v1.clone().len() as i32);
                let v9: i32 = v2.wrapping_add(v8);
                edge_at_5(v0.clone(), v9)
            } else {
                false
            };
            if v11 {
                return true;
            } else {
                let mut v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            }
        }
    }
}
fn method3(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: Rc<UH0>) -> US0 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: i32 = 0i32;
                let mut v6: bool = word_from_4(v0.clone(), v3.clone(), v5);
                if v6 {
                    return US0::US0_Some(v3.clone());
                } else {
                    v1 = v4.clone();
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US0::US0_None;
            }
        }
    }
}
fn has_word_2(mut v0: Rc<str>, mut v1: Rc<UH0>) -> bool {
    let mut v2: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v0.as_bytes().to_vec()));
    let mut v3: US0 = method3(v2, v1);
    let mut v4: bool = match &v3 {
        US0::US0_None => {
            true
        }
        _ => {
            false
        }
    };
    let mut v5: bool = v4 != true;
    v5
}
fn method10(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn flow_region_1(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("? "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method10(v2.clone());
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    let v16: Rc<str> = if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        let v10: i32 = (v0.clone().len() as i32);
        let v11: i32 = v10.wrapping_sub(v7);
        let mut v12: usize = ((v7) as usize);
        let mut v13: i32 = v7.wrapping_add(v11);
        let v14: usize = ((v13) as usize);
        let v15: Rc<str> = Rc::<str>::from(&(v0)[v12..v14]);
        v15.clone()
    };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v18: bool = v16.contains(&*v17);
    let v21: bool = if v18 {
        true
    } else {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("=>"); } LIT.with(|lit| lit.clone()) };
        let v20: bool = v0.contains(&*v19);
        v20
    };
    let v24: bool = if v21 {
        true
    } else {
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("->"); } LIT.with(|lit| lit.clone()) };
        let v23: bool = v0.contains(&*v22);
        v23
    };
    if v24 {
        true
    } else {
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("if"); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("else"); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("match"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("for"); } LIT.with(|lit| lit.clone()) };
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("while"); } LIT.with(|lit| lit.clone()) };
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("case"); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("switch"); } LIT.with(|lit| lit.clone()) };
        let v32: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v33: Rc<UH0> = Rc::new(UH0::UH0_Cons(v31.clone(), v32.clone()));
        let v34: Rc<UH0> = Rc::new(UH0::UH0_Cons(v30.clone(), v33.clone()));
        let v35: Rc<UH0> = Rc::new(UH0::UH0_Cons(v29.clone(), v34.clone()));
        let v36: Rc<UH0> = Rc::new(UH0::UH0_Cons(v28.clone(), v35.clone()));
        let v37: Rc<UH0> = Rc::new(UH0::UH0_Cons(v27.clone(), v36.clone()));
        let v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v26.clone(), v37.clone()));
        let v39: Rc<UH0> = Rc::new(UH0::UH0_Cons(v25.clone(), v38.clone()));
        has_word_2(v0.clone(), v39.clone())
    }
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn flow_line_0(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Option<usize> = v1.find(&*v2);
    let mut v4: Option<usize> = method10(v3.clone());
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let v8: i32 = v6.unwrap_or(-1i32);
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let v10: Option<usize> = v1.rfind(&*v9);
    let v11: Option<usize> = method10(v10.clone());
    let mut v12: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v13: Option<i32> = v11.map(|x| v12(x));
    let v15: i32 = v13.unwrap_or(-1i32);
    let v16: bool = v8 >= 0i32;
    let v19: bool = if v16 {
        let v17: i32 = v8.wrapping_add(1i32);
        let v18: bool = v15 > v17;
        v18
    } else {
        false
    };
    if v19 {
        let v20: i32 = v8.wrapping_add(2i32);
        let v21: i32 = v15.wrapping_sub(v8);
        let v22: i32 = v21.wrapping_sub(2i32);
        let v23: usize = ((v20) as usize);
        let v24: i32 = v20.wrapping_add(v22);
        let v25: usize = ((v24) as usize);
        let v26: Rc<str> = Rc::<str>::from(&(v1)[v23..v25]);
        flow_region_1(v26.clone())
    } else {
        false
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        flow_line_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_flow(v0: &str) -> bool {
    closure0()(Rc::<str>::from(v0))
}
