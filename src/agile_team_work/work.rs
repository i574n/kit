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
    UH0_NoTasks,
    UH0_TaskThen(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_FileReadable(Rc<str>),
    US0_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_ScanPlain,
    US1_ScanQuoted,
    US1_ScanEscaped,
}
#[derive(Clone)]
enum US2 {
    US2_MainAgent,
    US2_Subagent,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum US3 {
    US3_Some(i32),
    US3_None,
}
#[derive(Clone)]
enum UH2 {
    UH2_NoRanked,
    UH2_RankedThen(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, bool, Rc<UH3>),
}
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn field_end_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US1) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US1::US1_ScanPlain => {
                    let v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let v10: i32 = v1.wrapping_add(1i32);
                let v25: US1 = match v3 {
                    US1::US1_ScanEscaped => {
                        US1::US1_ScanQuoted
                    }
                    US1::US1_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US1::US1_ScanQuoted
                        } else {
                            US1::US1_ScanPlain
                        }
                    }
                    US1::US1_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US1::US1_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US1::US1_ScanPlain
                            } else {
                                US1::US1_ScanQuoted
                            }
                        }
                    }
                };
                (v1, v3) = (v10, v25);
                continue;
            }
        }
    }
}
fn skip_spaces_7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let mut v3: bool = v2 == 32i32;
        if v3 {
            let v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US1 = US1::US1_ScanPlain;
            let v6: i32 = field_end_6(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_7(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn backlog_task_at_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) {
    let mut v3: i32 = 0i32;
    let v4: i32 = field_start_5(v0.clone(), v1, v2, v3);
    let v5: US1 = US1::US1_ScanPlain;
    let v6: i32 = field_end_6(v0.clone(), v4, v2, v5);
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let v8: i32 = (v7.clone().len() as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let v11: i32 = 1i32;
    let v12: i32 = field_start_5(v0.clone(), v1, v2, v11);
    let v13: US1 = US1::US1_ScanPlain;
    let mut v14: i32 = field_end_6(v0.clone(), v12, v2, v13);
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: i32 = (v15.clone().len() as i32);
    let v17: i32 = v16.wrapping_sub(1i32);
    let v18: Rc<str> = std::rc::Rc::<str>::from(v15.get(1i32 as usize..v17 as usize).unwrap_or(""));
    let v19: i32 = 2i32;
    let v20: i32 = field_start_5(v0.clone(), v1, v2, v19);
    let v21: US1 = US1::US1_ScanPlain;
    let v22: i32 = field_end_6(v0.clone(), v20, v2, v21);
    let v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let v24: i32 = 3i32;
    let v25: i32 = field_start_5(v0.clone(), v1, v2, v24);
    let v26: US1 = US1::US1_ScanPlain;
    let v27: i32 = field_end_6(v0.clone(), v25, v2, v26);
    let v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let v29: i32 = (v28).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
    let v30: i32 = 4i32;
    let v31: i32 = field_start_5(v0.clone(), v1, v2, v30);
    let v32: US1 = US1::US1_ScanPlain;
    let v33: i32 = field_end_6(v0.clone(), v31, v2, v32);
    let v34: Rc<str> = std::rc::Rc::<str>::from(v0.get(v31 as usize..v33 as usize).unwrap_or(""));
    let v35: i32 = (v34).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
    let v36: i32 = 6i32;
    let v37: i32 = field_start_5(v0.clone(), v1, v2, v36);
    let v38: US1 = US1::US1_ScanPlain;
    let v39: i32 = field_end_6(v0.clone(), v37, v2, v38);
    let v40: Rc<str> = std::rc::Rc::<str>::from(v0.get(v37 as usize..v39 as usize).unwrap_or(""));
    let v41: i32 = 7i32;
    let v42: i32 = field_start_5(v0.clone(), v1, v2, v41);
    let v43: US1 = US1::US1_ScanPlain;
    let v44: i32 = field_end_6(v0.clone(), v42, v2, v43);
    let v45: Rc<str> = std::rc::Rc::<str>::from(v0.get(v42 as usize..v44 as usize).unwrap_or(""));
    let v46: i32 = (v45.clone().len() as i32);
    let v47: i32 = v46.wrapping_sub(1i32);
    let v48: Rc<str> = std::rc::Rc::<str>::from(v45.get(1i32 as usize..v47 as usize).unwrap_or(""));
    let v49: i32 = 8i32;
    let v50: i32 = field_start_5(v0.clone(), v1, v2, v49);
    let v51: US1 = US1::US1_ScanPlain;
    let v52: i32 = field_end_6(v0.clone(), v50, v2, v51);
    let v53: Rc<str> = std::rc::Rc::<str>::from(v0.get(v50 as usize..v52 as usize).unwrap_or(""));
    let v54: i32 = (v53.clone().len() as i32);
    let v55: i32 = v54.wrapping_sub(1i32);
    let v56: Rc<str> = std::rc::Rc::<str>::from(v53.get(1i32 as usize..v55 as usize).unwrap_or(""));
    let v57: i32 = agile_planning_fields::kit_agile_planning_minutes_near(&v0, &v10, v1);
    let v58: i32 = 5i32;
    let v59: i32 = field_start_5(v0.clone(), v1, v2, v58);
    let v60: US1 = US1::US1_ScanPlain;
    let v61: i32 = field_end_6(v0.clone(), v59, v2, v60);
    let v62: Rc<str> = std::rc::Rc::<str>::from(v0.get(v59 as usize..v61 as usize).unwrap_or(""));
    let v63: i32 = (v62).trim().trim_end_matches("u32").parse::<i32>().unwrap_or(0);
    (v10, v18, v23, v29, v35, v40, v48, v56, v57, v63)
}
fn tasks_from_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v4)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v6: bool = v5 < 0i32;
        if v6 {
            return v3;
        } else {
            let v7: i32 = v5.wrapping_add(6i32);
            let v8: i32 = v0.get(..v5 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v5 as usize).unwrap_or(""));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v11: bool = v9.starts_with(&*v10);
            if v11 {
                let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) = backlog_task_at_4(v0.clone(), v7, v2);
                let v22: Rc<UH0> = Rc::new(UH0::UH0_TaskThen(v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v3));
                (v1, v3) = (v7, v22);
                continue;
            } else {
                v1 = v7;
                continue;
            }
        }
    }
}
fn tasks_of_1(mut v0: Rc<str>) -> (Rc<str>, Rc<UH0>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = concat5_2(v0, v1, v2.clone(), v2.clone(), v2.clone());
    let v4: bool = std::path::Path::new(&*v3).is_file();
    let v7: US0 = if v4 {
        US0::US0_FileReadable(v3)
    } else {
        US0::US0_FileMissing(v3)
    };
    match v7 {
        US0::US0_FileMissing(v8) => {
            let v8: Rc<str> = v8;
            let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoTasks); } CASE.with(|case| case.clone()) };
            (v2, v9)
        }
        US0::US0_FileReadable(v10) => {
            let v10: Rc<str> = v10;
            let v11: Rc<str> = std::fs::read_to_string(&*v10).unwrap_or_default().into();
            let v12: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v11);
            let v13: i32 = 0i32;
            let mut v14: i32 = (v12.clone().len() as i32);
            let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoTasks); } CASE.with(|case| case.clone()) };
            let v16: Rc<UH0> = tasks_from_3(v12.clone(), v13, v14, v15);
            (v12, v16)
        }
    }
}
fn method9(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method9(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team:"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(v2.replace(&*v5, &*v6));
            Rc::new(UH1::UH1_Cons(v7, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method11(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH1> = v3;
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method12(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method12(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH1> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method11(v16, v17)
    }
}
fn method14(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
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
fn method15(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
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
fn method13(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method13(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let v6: i32 = 0i32;
            let v7: i32 = method14(v2.clone(), v5, v6);
            let v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
            let v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method15(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            Rc::new(UH1::UH1_Cons(v12, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method16(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method16(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team:"); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            if v6 {
                Rc::new(UH1::UH1_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn team_words_8(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH1> = split_list_10(v1, v0);
    let mut v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH1> = method13(v2, v3);
    let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH1> = method16(v4, v5);
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method9(v6, v7)
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US3> {
    Rc::new(move |mut v2: Rc<UH1>| -> US3 {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US3::US3_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US3> = method17(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH1::UH1_Nil => {
                US3::US3_None
            }
        }
    })
}
fn method17(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US3> {
    closure2(v0, v1)
}
fn light_work_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32, mut v10: i32) -> bool {
    let v11: bool = &*v3 != "Epic";
    let v16: bool = if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user"); } LIT.with(|lit| lit.clone()) };
        let mut v14: bool = agile_planning_filters::kit_agile_planning_selected(&v0, &v1, &v12, &v13);
        let v15: bool = v14 == false;
        v15
    } else {
        false
    };
    if v16 {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("light"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
        let v19: bool = agile_planning_filters::kit_agile_planning_selected(&v0, &v1, &v17, &v18);
        if v19 {
            true
        } else {
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
            let v21: bool = agile_planning_filters::kit_agile_planning_selected(&v0, &v1, &v20, &v18);
            v21
        }
    } else {
        false
    }
}
fn own_c1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) -> bool> {
    Rc::new(move |mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: i32, mut v6: i32, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: i32, mut v11: i32| -> bool {
        let v12: Rc<UH1> = team_words_8(v9.clone());
        let v13: i32 = 0i32;
        let mut v14: Rc<dyn Fn(Rc<UH1>) -> US3> = method17(v0.clone(), v13);
        let v15: US3 = v14(v12);
        let v16: bool = match v15 {
            US3::US3_None => {
                true
            }
            _ => {
                false
            }
        };
        let v17: bool = v16 != true;
        if v17 {
            light_work_18(v1.clone(), v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
        } else {
            false
        }
    })
}
fn task_open_20(mut v0: &Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: &Rc<str>, mut v7: &Rc<str>, mut v8: i32, mut v9: i32) -> bool {
    let v10: bool = &*v5 == "Planned";
    if v10 {
        true
    } else {
        let v11: bool = &*v5 == "Active";
        if v11 {
            true
        } else {
            let v12: bool = &*v5 == "Blocked";
            if v12 {
                false
            } else {
                let v13: bool = &*v5 == "Paused";
                if v13 {
                    false
                } else {
                    false
                }
            }
        }
    }
}
fn status_of_22(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_NoTasks => {
                let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v2;
            }
            UH0::UH0_TaskThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: i32 = v6;
                let v7: i32 = v7;
                let v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: i32 = v11;
                let v12: i32 = v12;
                let v13: Rc<UH0> = v13;
                let mut v14: bool = &*v3 == &*v1;
                if v14 {
                    return v8;
                } else {
                    v0 = v13;
                    continue;
                }
            }
        }
    }
}
fn deps_settled_21(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: i32 = (v1.clone().len() as i32);
        let v4: bool = v3 <= v2;
        if v4 {
            return true;
        } else {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v6: i32 = v1.get(v2 as usize..).and_then(|rest| rest.find(&*v5)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let v7: bool = v6 < 0i32;
            let v8: i32 = if v7 {
                v3
            } else {
                v6
            };
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v1.get(v2 as usize..v8 as usize).unwrap_or(""));
            let v10: Rc<str> = std::rc::Rc::<str>::from(v9.trim());
            let v11: Rc<str> = status_of_22(v0.clone(), v10);
            let v12: bool = &*v11 == "";
            let mut v14: bool = if v12 {
                true
            } else {
                let v13: bool = &*v11 == "Done";
                v13
            };
            if v14 {
                let v15: i32 = v8.wrapping_add(1i32);
                v2 = v15;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn open_deps_25(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = (v1.clone().len() as i32);
        let v5: bool = v4 <= v2;
        if v5 {
            return v3;
        } else {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v7: i32 = v1.get(v2 as usize..).and_then(|rest| rest.find(&*v6)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let v8: bool = v7 < 0i32;
            let mut v9: i32 = if v8 {
                v4
            } else {
                v7
            };
            let v10: Rc<str> = std::rc::Rc::<str>::from(v1.get(v2 as usize..v9 as usize).unwrap_or(""));
            let v11: Rc<str> = std::rc::Rc::<str>::from(v10.trim());
            let v12: Rc<str> = status_of_22(v0.clone(), v11);
            let v13: i32 = v9.wrapping_add(1i32);
            let mut v14: bool = &*v12 == "";
            let v16: bool = if v14 {
                true
            } else {
                let v15: bool = &*v12 == "Done";
                v15
            };
            let v18: i32 = if v16 {
                v3
            } else {
                let v17: i32 = v3.wrapping_add(1i32);
                v17
            };
            (v2, v3) = (v13, v18);
            continue;
        }
    }
}
fn unblocked_weight_24(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_NoTasks => {
                return v2;
            }
            UH0::UH0_TaskThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: Rc<str> = v5.clone();
                let v6: i32 = *v6;
                let v7: i32 = *v7;
                let v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let v10: Rc<str> = v10.clone();
                let v11: i32 = *v11;
                let v12: i32 = *v12;
                let v13: Rc<UH0> = v13.clone();
                let mut v14: bool = task_open_20(&v3, &v4, &v5, v6, v7, v8, &v9, &v10, v11, v12);
                let v21: bool = if v14 {
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                    let v17: Rc<str> = Rc::<str>::from(v9.replace(&*v15, &*v16));
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v19: Rc<str> = Rc::<str>::from(v17.replace(&*v18, &*v16));
                    let v20: bool = v19.split("|").any(|item| item == &*v1);
                    v20
                } else {
                    false
                };
                let v29: i32 = if v21 {
                    let v22: i32 = 0i32;
                    let v23: i32 = 0i32;
                    let v24: i32 = open_deps_25(v0, v9, v22, v23);
                    let v25: i32 = 1i32 << (2 * (5 - (v6).clamp(0, 5)));
                    let v26: bool = v24 < 1i32;
                    let v27: i32 = if v26 {
                        1i32
                    } else {
                        v24
                    };
                    let v28: i32 = v25.wrapping_div(v27);
                    v28
                } else {
                    0i32
                };
                let v30: i32 = v2.wrapping_add(v29);
                (v0, v2) = (v13, v30);
                continue;
            }
        }
    }
}
fn ranked_of_23(mut v0: US2, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: &Rc<str>, mut v5: i32, mut v6: i32, mut v7: &Rc<str>, mut v8: &Rc<str>, mut v9: &Rc<str>, mut v10: i32, mut v11: i32) -> (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32) {
    let v12: bool = 0i32 < v10;
    let v30: i32 = if v12 {
        v10
    } else {
        let v13: bool = 0i32 == v6;
        let v28: i32 = if v13 {
            15i32
        } else {
            let mut v14: bool = 1i32 == v6;
            if v14 {
                30i32
            } else {
                let v15: bool = 2i32 == v6;
                if v15 {
                    60i32
                } else {
                    let v16: bool = 3i32 == v6;
                    if v16 {
                        180i32
                    } else {
                        let v17: bool = 4i32 == v6;
                        if v17 {
                            300i32
                        } else {
                            let v18: bool = 5i32 == v6;
                            if v18 {
                                600i32
                            } else {
                                let v19: bool = 6i32 == v6;
                                if v19 {
                                    1200i32
                                } else {
                                    let v20: bool = 7i32 == v6;
                                    if v20 {
                                        2400i32
                                    } else {
                                        3600i32
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        let v29: i32 = ((v28 as i64) * (100 - (v11).clamp(0, 100)) as i64 / 100).max(15) as i32;
        v29
    };
    let v31: i32 = 0i32;
    let v32: i32 = unblocked_weight_24(v1, v2.clone(), v31);
    let v33: i32 = 1i32 << (2 * (5 - (v5).clamp(0, 5)));
    let v34: i32 = v33.wrapping_add(v32);
    let v40: i32 = match v0 {
        US2::US2_MainAgent => {
            let v35: i32 = ((v34 as i64) * 6000 / (v30).max(1) as i64).min(i32::MAX as i64) as i32;
            v35
        }
        US2::US2_Subagent => {
            let v36: i32 = v30.wrapping_mul(8i32);
            let v37: i32 = v36.wrapping_add(5i32);
            let v38: i32 = v37.wrapping_sub(v5);
            v38
        }
    };
    (v2, v3, v5, v6, v30, v12, v32, v40)
}
fn inserted_26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: bool, mut v6: i32, mut v7: i32, mut v8: Rc<UH2>) -> Rc<UH2> {
    match &*v8 {
        UH2::UH2_NoRanked => {
            let mut v9: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_NoRanked); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_RankedThen(v0, v1, v2, v3, v4, v5, v6, v7, v9))
        }
        UH2::UH2_RankedThen(v11, v12, v13, v14, v15, v16, v17, v18, v19) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = v12.clone();
            let v13: i32 = *v13;
            let mut v14: i32 = *v14;
            let v15: i32 = *v15;
            let v16: bool = *v16;
            let v17: i32 = *v17;
            let v18: i32 = *v18;
            let v19: Rc<UH2> = v19.clone();
            let v20: bool = v18 < v7;
            if v20 {
                Rc::new(UH2::UH2_RankedThen(v0, v1, v2, v3, v4, v5, v6, v7, v8))
            } else {
                let v22: Rc<UH2> = inserted_26(v0, v1, v2, v3, v4, v5, v6, v7, v19);
                Rc::new(UH2::UH2_RankedThen(v11, v12, v13, v14, v15, v16, v17, v18, v22))
            }
        }
    }
}
fn ranked_from_19(mut v0: US2, mut v1: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) -> bool>, mut v2: Rc<UH0>, mut v3: Rc<UH0>, mut v4: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH0::UH0_NoTasks => {
                return v4;
            }
            UH0::UH0_TaskThen(v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) => {
                let v5: Rc<str> = v5;
                let v6: Rc<str> = v6;
                let v7: Rc<str> = v7;
                let v8: i32 = v8;
                let mut v9: i32 = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<str> = v11;
                let v12: Rc<str> = v12;
                let v13: i32 = v13;
                let mut v14: i32 = v14;
                let v15: Rc<UH0> = v15;
                let v16: bool = task_open_20(&v5, &v6, &v7, v8, v9, v10.clone(), &v11, &v12, v13, v14);
                let v20: bool = if v16 {
                    match &v0 {
                        US2::US2_MainAgent => {
                            true
                        }
                        US2::US2_Subagent => {
                            let v17: bool = &*v7 != "Epic";
                            v17
                        }
                    }
                } else {
                    false
                };
                let v29: bool = if v20 {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user-batch"); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = Rc::<str>::from(v12.replace(&*v22, &*v23));
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = Rc::<str>::from(v24.replace(&*v25, &*v23));
                    let v27: bool = v26.split("|").any(|item| item == &*v21);
                    let v28: bool = v27 == false;
                    v28
                } else {
                    false
                };
                let v32: bool = if v29 {
                    let v30: i32 = 0i32;
                    deps_settled_21(v2.clone(), v11.clone(), v30)
                } else {
                    false
                };
                let v34: bool = if v32 {
                    v1(v5.clone(), v6.clone(), v7.clone(), v8, v9, v10.clone(), v11.clone(), v12.clone(), v13, v14)
                } else {
                    false
                };
                if v34 {
                    let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32) = ranked_of_23(v0.clone(), v2.clone(), v5, v6, &v7, v8, v9, &v10, &v11, &v12, v13, v14);
                    let v43: Rc<UH2> = inserted_26(v35, v36, v37, v38, v39, v40, v41, v42, v4);
                    (v3, v4) = (v15, v43);
                    continue;
                } else {
                    v3 = v15;
                    continue;
                }
            }
        }
    }
}
fn ranked_words_27(mut v0: Rc<UH2>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_NoRanked => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
        }
        UH2::UH2_RankedThen(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let v4: i32 = v4;
            let v5: i32 = v5;
            let v6: i32 = v6;
            let v7: bool = v7;
            let v8: i32 = v8;
            let mut v9: i32 = v9;
            let v10: Rc<UH2> = v10;
            let v11: Rc<UH1> = ranked_words_27(v10);
            Rc::new(UH1::UH1_Cons(v2, v11))
        }
    }
}
fn taken_28(mut v0: i32, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH1> = v3;
            let v4: bool = v0 > 0i32;
            if v4 {
                let v5: i32 = v0.wrapping_sub(1i32);
                let v6: Rc<UH1> = taken_28(v5, v3);
                Rc::new(UH1::UH1_Cons(v2, v6))
            } else {
                { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
            }
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method29(mut v0: Rc<UH1>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH1> = v3;
            let v4: Rc<UH3> = method29(v3, v1);
            Rc::new(UH3::UH3_Cons(v2, false, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method30(mut v0: Rc<RefCell<Vec<(Rc<str>, bool)>>>, mut v1: Rc<UH3>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH3::UH3_Cons(v3, v4, v5) => {
                let mut v3: Rc<str> = v3;
                let v4: bool = v4;
                let v5: Rc<UH3> = v5;
                v0.borrow_mut().push((v3, v4));
                let v6: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v5, v6);
                continue;
            }
            UH3::UH3_Nil => {
                return v2;
            }
        }
    }
}
fn gate_rows_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, bool)>>> {
    let (mut v2, mut v3): (Rc<str>, Rc<UH0>) = tasks_of_1(v1);
    let v4: US2 = US2::US2_MainAgent;
    let v5: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) -> bool> = own_c1(v0, v2);
    let v6: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_NoRanked); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH2> = ranked_from_19(v4, v5, v3.clone(), v3, v6);
    let v8: Rc<UH1> = ranked_words_27(v7);
    let mut v9: i32 = 3i32;
    let v10: Rc<UH1> = taken_28(v9, v8);
    let v11: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH3> = method29(v10, v11);
    let v13: Rc<RefCell<Vec<(Rc<str>, bool)>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v14: i32 = 0i32;
    let v15: i32 = method30(v13.clone(), v12, v14);
    let v16: Rc<Vec<(Rc<str>, bool)>> = Rc::new(v13.borrow().clone());
    let v17: Rc<RefCell<Vec<(Rc<str>, bool)>>> = Rc::new(RefCell::new((v16).as_ref().clone()));
    v17
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, bool)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, bool)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, bool)>>> {
        gate_rows_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_gate_rows(v0: &str, v1: &str) -> Rc<RefCell<Vec<(Rc<str>, bool)>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_accbf6de6842 = Mut0;
