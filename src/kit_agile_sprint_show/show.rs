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
    US0_ShowItems,
    US0_ShowNotes,
}
#[derive(Clone)]
enum US1 {
    US1_Some(i64),
    US1_None,
}
fn depth_word_2(mut v0: US0) -> Rc<str> {
    let mut v1: bool = match v0 {
        US0::US0_ShowNotes => {
            true
        }
        _ => {
            false
        }
    };
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("retro"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
        v3
    }
}
fn committed_from_3(mut v0: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let (mut v5, mut v6, mut v7): (Rc<str>, i32, bool) = v0.borrow()[v1 as usize].clone();
            let mut v8: i32 = v1.wrapping_add(1i32);
            let v10: i32 = if v7 {
                let mut v9: i32 = v2.wrapping_add(v6);
                v9
            } else {
                v2
            };
            (v1, v2) = (v8, v10);
            continue;
        }
    }
}
fn method5(mut v0: Option<i64>) -> Option<i64> {
    v0
}
fn closure1() -> Rc<dyn Fn((i64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i64)) -> US1> = Rc::new(move |mut v0: (i64)| -> US1 {
        let mut v1: i64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn days_of_4(mut v0: Rc<str>) -> i64 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = v1 < 10i32;
    if v2 {
        0i64
    } else {
        let v3: usize = ((0i32) as usize);
        let v4: usize = ((4i32) as usize);
        let v5: Rc<str> = Rc::<str>::from(&(v0)[v3..v4]);
        let v6: Option<i64> = (v5).trim().parse::<i64>().ok();
        let v7: Option<i64> = method5(v6);
        let mut v8: Rc<dyn Fn((i64)) -> US1> = closure1();
        let mut v9: Option<US1> = v7.map(|x| v8(x));
        let v11: US1 = US1::US1_None;
        let v12: US1 = v11.clone();
        let v13: US1 = v9.unwrap_or(v12);
        let v16: i64 = match v13 {
            US1::US1_None => {
                0i64
            }
            US1::US1_Some(v14) => {
                let v14: i64 = v14;
                v14
            }
        };
        let v17: usize = ((5i32) as usize);
        let v18: usize = ((7i32) as usize);
        let v19: Rc<str> = Rc::<str>::from(&(v0)[v17..v18]);
        let v20: Option<i64> = (v19).trim().parse::<i64>().ok();
        let v21: Option<i64> = method5(v20);
        let mut v22: Option<US1> = v21.map(|x| v8(x));
        let v24: US1 = US1::US1_None;
        let v25: US1 = v24.clone();
        let v26: US1 = v22.unwrap_or(v25);
        let v29: i64 = match v26 {
            US1::US1_None => {
                0i64
            }
            US1::US1_Some(v27) => {
                let v27: i64 = v27;
                v27
            }
        };
        let v30: usize = ((8i32) as usize);
        let v31: usize = ((10i32) as usize);
        let v32: Rc<str> = Rc::<str>::from(&(v0)[v30..v31]);
        let v33: Option<i64> = (v32).trim().parse::<i64>().ok();
        let v34: Option<i64> = method5(v33);
        let mut v35: Option<US1> = v34.map(|x| v8(x));
        let v37: US1 = US1::US1_None;
        let v38: US1 = v37.clone();
        let v39: US1 = v35.unwrap_or(v38);
        let v42: i64 = match v39 {
            US1::US1_None => {
                0i64
            }
            US1::US1_Some(v40) => {
                let v40: i64 = v40;
                v40
            }
        };
        let v43: bool = v29 <= 2i64;
        let v45: i64 = if v43 {
            let v44: i64 = v16.wrapping_sub(1i64);
            v44
        } else {
            v16
        };
        let v46: i64 = v45.wrapping_div(400i64);
        let v47: i64 = v45.wrapping_rem(400i64);
        let v48: bool = v47 != 0i64;
        let v50: bool = if v48 {
            let v49: bool = v45 < 0i64;
            v49
        } else {
            false
        };
        let v52: i64 = if v50 {
            let v51: i64 = v46.wrapping_sub(1i64);
            v51
        } else {
            v46
        };
        let v53: i64 = v52.wrapping_mul(400i64);
        let v54: i64 = v45.wrapping_sub(v53);
        let v55: i64 = v54.wrapping_mul(365i64);
        let v56: i64 = v54.wrapping_div(4i64);
        let v57: i64 = v55.wrapping_add(v56);
        let v58: i64 = v54.wrapping_div(100i64);
        let v59: i64 = v57.wrapping_sub(v58);
        let v60: bool = v29 > 2i64;
        let v63: i64 = if v60 {
            let v61: i64 = v29.wrapping_sub(3i64);
            v61
        } else {
            let v62: i64 = v29.wrapping_add(9i64);
            v62
        };
        let v64: i64 = 153i64.wrapping_mul(v63);
        let v65: i64 = v64.wrapping_add(2i64);
        let v66: i64 = v65.wrapping_div(5i64);
        let v67: i64 = v66.wrapping_add(v42);
        let v68: i64 = v67.wrapping_sub(1i64);
        let v69: i64 = v59.wrapping_add(v68);
        let v70: i64 = v52.wrapping_mul(146097i64);
        let v71: i64 = v70.wrapping_add(v69);
        let v72: i64 = v71.wrapping_sub(719468i64);
        v72
    }
}
fn method7(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_6(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method7(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_9(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method7(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn clipped_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = v1 > 160i32;
    if v2 {
        let v3: usize = ((0i32) as usize);
        let v4: usize = ((160i32) as usize);
        let v5: Rc<str> = Rc::<str>::from(&(v0)[v3..v4]);
        let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("..."); } LIT.with(|lit| lit.clone()) }));
        v6
    } else {
        v0
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
fn last_note_line_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = agile_note_listing::kit_agile_last_note(&v0, &v1);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v2.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v5: i32 = (v4.clone().borrow().len() as i32);
    let v6: bool = v5 < 2i32;
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v7
    } else {
        let mut v8: Rc<str> = v4.borrow()[1i32 as usize].clone();
        let mut v9: i32 = (v8.clone().len() as i32);
        let v10: i32 = 0i32;
        let v11: i32 = method12(v8.clone(), v9, v10);
        let v12: i32 = v9.wrapping_sub(1i32);
        let v13: Rc<str> = string_slice(&v8, v11 as i64, v12 as i64);
        let v14: i32 = (v13.clone().len() as i32);
        let v15: i32 = method13(v13.clone(), v14);
        let v16: Rc<str> = string_slice(&v13, 0i32 as i64, v15 as i64);
        clipped_11(v16)
    }
}
fn items_shown_8(mut v0: US0, mut v1: Rc<str>, mut v2: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v3: i32, mut v4: i32) -> i32 {
    loop {
        let v5: i32 = (v2.clone().borrow().len() as i32);
        let v6: bool = v3 >= v5;
        if v6 {
            return v4;
        } else {
            let (mut v7, mut v8, mut v9): (Rc<str>, i32, bool) = v2.borrow()[v3 as usize].clone();
            let v10: Rc<str> = agile_task_census::kit_agile_task_status_of(&v1, &v7);
            let v13: Rc<str> = if v9 {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("committed"); } LIT.with(|lit| lit.clone()) };
                v11
            } else {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
                v12
            };
            let v14: bool = &*v10 == "";
            let v16: Rc<str> = if v14 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown"); } LIT.with(|lit| lit.clone()) };
                v15
            } else {
                v10.clone()
            };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }, v7.clone()));
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let v19: Rc<str> = format_real_9(v8);
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m "); } LIT.with(|lit| lit.clone()) }));
            let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v16));
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v13));
            println!("{}", v24);
            let v25: bool = match &v0 {
                US0::US0_ShowNotes => {
                    true
                }
                _ => {
                    false
                }
            };
            let v28: Rc<str> = if v25 {
                last_note_line_10(v1.clone(), v7)
            } else {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v27
            };
            let v29: bool = &*v28 != "";
            if v29 {
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) }, v28));
                println!("{}", v30);
                ()
            };
            let v31: bool = &*v10 == "Done";
            let v32: bool = v31 && v9;
            let v34: i32 = if v32 {
                let v33: i32 = v4.wrapping_add(v8);
                v33
            } else {
                v4
            };
            let mut v35: i32 = v3.wrapping_add(1i32);
            (v3, v4) = (v35, v34);
            continue;
        }
    }
}
fn sprint_shown_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US0) -> i32 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v0.replace(&*v3, &*v4));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) }));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/sprints.spi"); } LIT.with(|lit| lit.clone()) }));
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v7).unwrap_or_default().as_str());
    let mut v9: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>> = kit_agile_sprint_state::kit_agile_sprint_head(&v8, &v1);
    let v10: i32 = (v9.clone().borrow().len() as i32);
    let v11: bool = v10 == 0i32;
    if v11 {
        let v12: bool = &*v1 == "";
        let v15: Rc<str> = if v12 {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open sprint"); } LIT.with(|lit| lit.clone()) };
            v13
        } else {
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sprint "); } LIT.with(|lit| lit.clone()) }, v1));
            v14
        };
        let v16: Rc<str> = depth_word_2(v2);
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile sprint "); } LIT.with(|lit| lit.clone()) }, v16));
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": no "); } LIT.with(|lit| lit.clone()) }));
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v15));
        eprintln!("{}", v19);
        1i32
    } else {
        let (mut v20, mut v21, mut v22, mut v23, mut v24): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) = v9.borrow()[0i32 as usize].clone();
        let v25: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> = kit_agile_sprint_state::kit_agile_sprint_items(&v8, &v20);
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/agile.spi"); } LIT.with(|lit| lit.clone()) }));
        let v27: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v26).unwrap_or_default().as_str());
        let v28: i32 = 0i32;
        let v29: i32 = 0i32;
        let v30: i32 = committed_from_3(&v25, v28, v29);
        let v31: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let v32: i64 = (v31 as i64);
        let v33: i64 = v32.wrapping_div(86400000i64);
        let v34: i64 = days_of_4(v23.clone());
        let mut v35: i64 = v34.wrapping_sub(v33);
        let v38: Rc<str> = if v24 {
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open"); } LIT.with(|lit| lit.clone()) };
            v36
        } else {
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("closed"); } LIT.with(|lit| lit.clone()) };
            v37
        };
        let v39: Rc<str> = depth_word_2(v2.clone());
        let v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile sprint "); } LIT.with(|lit| lit.clone()) }, v39));
        let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ok id="); } LIT.with(|lit| lit.clone()) }));
        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v20));
        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" status="); } LIT.with(|lit| lit.clone()) }));
        let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v38));
        let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" start="); } LIT.with(|lit| lit.clone()) }));
        let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v22));
        let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" finish="); } LIT.with(|lit| lit.clone()) }));
        let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v23));
        let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" days_left="); } LIT.with(|lit| lit.clone()) }));
        let v50: Rc<str> = format_real_6(v35);
        let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v50));
        let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" goal="); } LIT.with(|lit| lit.clone()) }));
        let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v21));
        println!("{}", v53);
        let v54: i32 = 0i32;
        let v55: i32 = 0i32;
        let v56: i32 = items_shown_8(v2, v27, &v25, v54, v55);
        let v57: Rc<str> = format_real_9(v30);
        let v58: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  committed="); } LIT.with(|lit| lit.clone()) }, v57));
        let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m done="); } LIT.with(|lit| lit.clone()) }));
        let v60: Rc<str> = format_real_9(v56);
        let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v60));
        let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
        println!("{}", v62);
        0i32
    }
}
fn shown_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: US0 = US0::US0_ShowItems;
    sprint_shown_1(v1, v0, v2)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        shown_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn retro_14(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: US0 = US0::US0_ShowNotes;
    sprint_shown_1(v1, v0, v2)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        retro_14(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_sprint_shown(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_agile_sprint_retro(v0: &str, v1: &str) -> i32 {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
use agile_task_census::SpiralShape_97df63ebabc3 as Mut0;
