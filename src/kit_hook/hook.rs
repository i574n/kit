#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;

pub fn spiral_json_quote(s: &str) -> std::rc::Rc<str> {
    let (q, b) = (char::from(34u8), char::from(92u8));
    let mut out = String::from(q);
    for c in s.chars() {
        match c as u32 {
            34 | 92 => { out.push(b); out.push(c); }
            n @ (8 | 9 | 10 | 12 | 13) => { out.push(b); out.push(char::from(b"btn_fr"[(n - 8) as usize])); }
            n if n < 32 => out.push_str(&format!("{b}u{n:04x}")),
            _ => out.push(c),
        }
    }
    out.push(q);
    std::rc::Rc::from(out.as_str())
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
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Allow,
    US1_Deny(Rc<str>),
    US1_Advise(Rc<str>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US1, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_ToolEdit,
    US2_ToolWrite,
    US2_ToolMultiEdit,
    US2_ToolNotebookEdit,
    US2_ToolShell,
    US2_ToolOther,
}
#[derive(Clone)]
enum US3 {
    US3_Some(i32),
    US3_None,
}
fn hook_usage_1() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit hook pre [--repos-root d] [--closed-root d,..] [--refuse .x,..] [--advise .x]"); } LIT.with(|lit| lit.clone()) };
    eprintln!("{}", v0);
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("       kit hook writes   (reads a Bash/PowerShell hook payload; prints writes=yes|no)"); } LIT.with(|lit| lit.clone()) };
    eprintln!("{}", v1);
    2i32
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_2(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_3();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US0 = if v4 {
        let mut v5: Rc<str> = command_line_argument_4(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let mut v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_2(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn pre_output_7(mut v0: US1) -> Rc<str> {
    match v0 {
        US1::US1_Advise(v6) => {
            let mut v6: Rc<str> = v6;
            let v7: Rc<str> = spiral_json_quote(&*v6);
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"additionalContext\":"); } LIT.with(|lit| lit.clone()) }, v7));
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}}"); } LIT.with(|lit| lit.clone()) }));
            v9
        }
        US1::US1_Allow => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US1::US1_Deny(v2) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<str> = spiral_json_quote(&*v2);
            let v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"deny\",\"permissionDecisionReason\":"); } LIT.with(|lit| lit.clone()) }, v3));
            let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}}"); } LIT.with(|lit| lit.clone()) }));
            v5
        }
    }
}
fn flag_value_9(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let v4: Rc<str> = v4.clone();
                        let mut v5: Rc<UH0> = v5.clone();
                        let mut v6: bool = &*v2 == &*v1;
                        if v6 {
                            return v4;
                        } else {
                            v0 = v3;
                            continue;
                        }
                    }
                    _ => {
                        v0 = v3;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v11;
            }
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method11(v3, v1);
            let mut v5: bool = &*v2 != "";
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
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn method14(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method14(v7, &v9) {
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
        method13(v16, v17)
    }
}
fn method16(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
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
fn method17(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
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
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method15(v3, v1);
            let mut v5: i32 = (v2.clone().len() as i32);
            let mut v6: i32 = 0i32;
            let v7: i32 = method16(v2.clone(), v5, v6);
            let mut v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
            let mut v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method17(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            Rc::new(UH0::UH0_Cons(v12, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn comma_list_10(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = split_list_12(v1, v0);
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH0> = method15(v2, v3);
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method11(v4, v5)
}
fn policy_of_8(mut v0: Rc<UH0>) -> (Rc<str>, Rc<UH0>, Rc<UH0>, Rc<UH0>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--repos-root"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = flag_value_9(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--closed-root"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = flag_value_9(v0.clone(), v3);
    let mut v5: Rc<UH0> = comma_list_10(v4);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--refuse"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = flag_value_9(v0.clone(), v6);
    let mut v8: Rc<UH0> = comma_list_10(v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--advise"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = flag_value_9(v0, v9);
    let v11: Rc<UH0> = comma_list_10(v10);
    (v2, v5, v8, v11)
}
fn method20(mut v0: Rc<UH1>, mut v1: US1) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: US1 = v2;
                let mut v3: Rc<UH1> = v3;
                let v16: US1 = match &v1 {
                    US1::US1_Deny(v4) => {
                        let v4: Rc<str> = v4.clone();
                        v1
                    }
                    _ => {
                        match &v2 {
                            US1::US1_Deny(v5) => {
                                let mut v5: Rc<str> = v5.clone();
                                v2
                            }
                            _ => {
                                match &v1 {
                                    US1::US1_Advise(v6) => {
                                        let mut v6: Rc<str> = v6.clone();
                                        match v2 {
                                            US1::US1_Advise(v7) => {
                                                let v7: Rc<str> = v7;
                                                let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
                                                let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
                                                US1::US1_Advise(v9)
                                            }
                                            US1::US1_Allow => {
                                                v1
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    US1::US1_Allow => {
                                        v2
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                    }
                };
                (v0, v1) = (v3, v16);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn combined_19(mut v0: Rc<UH1>) -> US1 {
    let mut v1: US1 = US1::US1_Allow;
    method20(v0, v1)
}
fn tool_of_21(mut v0: Rc<str>) -> US2 {
    let mut v1: bool = &*v0 == "Edit";
    if v1 {
        US2::US2_ToolEdit
    } else {
        let mut v3: bool = &*v0 == "Write";
        if v3 {
            US2::US2_ToolWrite
        } else {
            let mut v5: bool = &*v0 == "MultiEdit";
            if v5 {
                US2::US2_ToolMultiEdit
            } else {
                let v7: bool = &*v0 == "NotebookEdit";
                if v7 {
                    US2::US2_ToolNotebookEdit
                } else {
                    let mut v9: bool = &*v0 == "Bash";
                    let v11: bool = if v9 {
                        true
                    } else {
                        let mut v10: bool = &*v0 == "PowerShell";
                        v10
                    };
                    if v11 {
                        US2::US2_ToolShell
                    } else {
                        US2::US2_ToolOther
                    }
                }
            }
        }
    }
}
fn edits_files_23(mut v0: US2) -> bool {
    match v0 {
        US2::US2_ToolEdit => {
            true
        }
        US2::US2_ToolMultiEdit => {
            true
        }
        US2::US2_ToolWrite => {
            true
        }
        _ => {
            false
        }
    }
}
fn normal_path_25(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = Rc::<str>::from(v3.to_lowercase());
    v4
}
fn method26(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn state_file_24(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = normal_path_25(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/docs/backlog/state/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Option<usize> = v1.find(&*v2);
    let v4: Option<usize> = method26(v3);
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let mut v8: i32 = v6.unwrap_or(-1i32);
    let mut v9: bool = v8 < 0i32;
    let v19: Rc<str> = if v9 {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v10
    } else {
        let v11: i32 = v8.wrapping_add(20i32);
        let v12: i32 = (v1.clone().len() as i32);
        let v13: i32 = v12.wrapping_sub(v8);
        let v14: i32 = v13.wrapping_sub(20i32);
        let v15: usize = ((v11) as usize);
        let v16: i32 = v11.wrapping_add(v14);
        let v17: usize = ((v16) as usize);
        let v18: Rc<str> = Rc::<str>::from(&(v1)[v15..v17]);
        v18
    };
    let v20: bool = v8 >= 0i32;
    let v23: bool = if v20 {
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let mut v22: bool = v19.ends_with(&*v21);
        v22
    } else {
        false
    };
    if v23 {
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v25: bool = v19.contains(&*v24);
        let v26: bool = v25 == false;
        v26
    } else {
        false
    }
}
fn state_verdict_22(mut v0: US2, mut v1: Rc<str>) -> US1 {
    let mut v2: bool = edits_files_23(v0);
    let v4: bool = if v2 {
        state_file_24(v1)
    } else {
        false
    };
    if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backlog state changes only through kit agile verbs (note, set, edit, add, claim), which order writes"); } LIT.with(|lit| lit.clone()) };
        US1::US1_Deny(v5)
    } else {
        US1::US1_Allow
    }
}
fn method29(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                (v0, v1) = (v3, v2);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn extension_of_28(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = normal_path_25(v0);
    let mut v3: Rc<UH0> = split_list_12(v1, v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = method29(v3, v4.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v7: Option<usize> = v5.rfind(&*v6);
    let mut v8: Option<usize> = method26(v7);
    let mut v9: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v10: Option<i32> = v8.map(|x| v9(x));
    let v12: i32 = v10.unwrap_or(-1i32);
    let v13: bool = v12 < 0i32;
    if v13 {
        v4
    } else {
        let v14: i32 = (v5.clone().len() as i32);
        let v15: i32 = v14.wrapping_sub(v12);
        let v16: usize = ((v12) as usize);
        let v17: i32 = v12.wrapping_add(v15);
        let v18: usize = ((v17) as usize);
        let v19: Rc<str> = Rc::<str>::from(&(v5)[v16..v18]);
        v19
    }
}
fn loop_32(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(1i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn under_31(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = normal_path_25(v0);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: bool = v2.ends_with(&*v3);
    let mut v6: Rc<str> = if v4 {
        v2.clone()
    } else {
        let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
        v5
    };
    let v7: Rc<str> = normal_path_25(v1.clone());
    let mut v8: bool = v7.starts_with(&*v6);
    if v8 {
        true
    } else {
        let mut v9: Rc<str> = normal_path_25(v1);
        let mut v10: Rc<str> = loop_32(v2);
        let v11: bool = &*v9 == &*v10;
        v11
    }
}
fn closure3(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        under_31(v1, v0.clone())
    })
}
fn method30(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure3(v0)
}
fn method33(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: Rc<UH0>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let mut v5: bool = v1(v3);
                if v5 {
                    return true;
                } else {
                    let mut v6: Rc<dyn Fn(Rc<str>) -> bool> = method30(v0.clone());
                    (v1, v2) = (v6, v4);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn closure4(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US3> {
    Rc::new(move |mut v2: Rc<UH0>| -> US3 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US3::US3_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US3> = method34(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US3::US3_None
            }
        }
    })
}
fn method34(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US3> {
    closure4(v0, v1)
}
fn new_file_verdict_27(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>, mut v4: US2, mut v5: Rc<str>, mut v6: bool) -> US1 {
    let v7: Rc<str> = extension_of_28(v5.clone());
    let mut v8: bool = match v4 {
        US2::US2_ToolWrite => {
            true
        }
        _ => {
            false
        }
    };
    let mut v9: bool = v8 != true;
    let mut v10: bool = v9 || v6;
    if v10 {
        US1::US1_Allow
    } else {
        let v12: Rc<dyn Fn(Rc<str>) -> bool> = method30(v5.clone());
        let v13: bool = method33(v5.clone(), v12, v1);
        if v13 {
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("new files are not written under "); } LIT.with(|lit| lit.clone()) }, v5));
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'s folder; put them in the repo that owns the work"); } LIT.with(|lit| lit.clone()) }));
            US1::US1_Deny(v15)
        } else {
            let v17: bool = under_31(v0, v5);
            let v18: bool = v17 == false;
            if v18 {
                US1::US1_Allow
            } else {
                let v20: i32 = 0i32;
                let mut v21: Rc<dyn Fn(Rc<UH0>) -> US3> = method34(v7.clone(), v20);
                let mut v22: US3 = v21(v2);
                let v23: bool = match v22 {
                    US3::US3_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v24: bool = v23 != true;
                if v24 {
                    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no new "); } LIT.with(|lit| lit.clone()) }, v7));
                    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files in repos; write the tool in Spiral"); } LIT.with(|lit| lit.clone()) }));
                    US1::US1_Deny(v26)
                } else {
                    let v28: i32 = 0i32;
                    let mut v29: Rc<dyn Fn(Rc<UH0>) -> US3> = method34(v7.clone(), v28);
                    let v30: US3 = v29(v3);
                    let v31: bool = match v30 {
                        US3::US3_None => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v32: bool = v31 != true;
                    if v32 {
                        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("new "); } LIT.with(|lit| lit.clone()) }, v7));
                        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" table in a repo: data a repo keeps is typed Spiral rows"); } LIT.with(|lit| lit.clone()) }));
                        US1::US1_Advise(v34)
                    } else {
                        US1::US1_Allow
                    }
                }
            }
        }
    }
}
fn pre_verdict_18(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: bool) -> US1 {
    let v7: US2 = tool_of_21(v4);
    let mut v8: US1 = state_verdict_22(v7.clone(), v5.clone());
    let mut v9: US1 = new_file_verdict_27(v0, v1, v2, v3, v7, v5, v6);
    let mut v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH1> = Rc::new(UH1::UH1_Cons(v9, v10));
    let v12: Rc<UH1> = Rc::new(UH1::UH1_Cons(v8, v11));
    combined_19(v12)
}
fn pre_text_6(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool) -> Rc<str> {
    let v4: bool = &*v2 == "";
    if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let (mut v6, mut v7, mut v8, mut v9): (Rc<str>, Rc<UH0>, Rc<UH0>, Rc<UH0>) = policy_of_8(v0);
        let mut v10: US1 = pre_verdict_18(v6, v7, v8, v9, v1, v2, v3);
        pre_output_7(v10)
    }
}
fn file_exists_35(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn pre_answered_5(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Option<serde_json::Value> = serde_json::from_str::<serde_json::Value>(&*v1).ok();
    let mut v3: std::rc::Rc<serde_json::Value> = std::rc::Rc::new(v2.unwrap_or_default());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/tool_name"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from(v3.pointer(&*v4).and_then(serde_json::Value::as_str).unwrap_or(""));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/tool_input/file_path"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = std::rc::Rc::<str>::from(v3.pointer(&*v6).and_then(serde_json::Value::as_str).unwrap_or(""));
    let mut v8: bool = &*v7 == "";
    let v11: Rc<str> = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/tool_input/notebook_path"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from(v3.pointer(&*v9).and_then(serde_json::Value::as_str).unwrap_or(""));
        v10
    } else {
        v7
    };
    let v12: bool = file_exists_35(v11.clone());
    pre_text_6(v0, v5, v11, v12)
}
fn writes_answered_36(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Option<serde_json::Value> = serde_json::from_str::<serde_json::Value>(&*v0).ok();
    let mut v2: std::rc::Rc<serde_json::Value> = std::rc::Rc::new(v1.unwrap_or_default());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/tool_input/command"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = std::rc::Rc::<str>::from(v2.pointer(&*v3).and_then(serde_json::Value::as_str).unwrap_or(""));
    let mut v5: Rc<str> = kit_hook_verbs::kit_hook_sign_line(&v4);
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit hook "); } LIT.with(|lit| lit.clone()) }, v5));
    v6
}
fn hook_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_2(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let mut v5: Rc<UH0> = v5;
                    let mut v6: bool = &*v4 == "pre";
                    if v6 {
                        let v7: std::io::Stdin = std::io::stdin();
                        let mut v8: Result<std::string::String, std::io::Error> = std::io::read_to_string(v7);
                        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v11: &str = &*v9;
                        let v13: std::string::String = String::from(v11);
                        let v15: std::string::String = v8.unwrap_or(v13);
                        let v17: Rc<str> = Rc::<str>::from(String::as_str(&v15));
                        let v18: Rc<str> = pre_answered_5(v5, v17);
                        let v19: bool = &*v18 == "";
                        if v19 {
                            ()
                        } else {
                            println!("{}", v18);
                            ()
                        };
                        0i32
                    } else {
                        let v20: bool = &*v4 == "writes";
                        if v20 {
                            let mut v21: std::io::Stdin = std::io::stdin();
                            let mut v22: Result<std::string::String, std::io::Error> = std::io::read_to_string(v21);
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v25: &str = &*v23;
                            let v27: std::string::String = String::from(v25);
                            let mut v29: std::string::String = v22.unwrap_or(v27);
                            let v31: Rc<str> = Rc::<str>::from(String::as_str(&v29));
                            let v32: Rc<str> = writes_answered_36(v31);
                            println!("{}", v32);
                            0i32
                        } else {
                            hook_usage_1()
                        }
                    }
                }
                _ => {
                    hook_usage_1()
                }
            }
        }
        _ => {
            hook_usage_1()
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        hook_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_hook_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
