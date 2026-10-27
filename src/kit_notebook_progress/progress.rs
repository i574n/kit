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
#[derive(Clone)]
enum US0 {
    US0_SeenWritten(u64),
    US0_SeenPending,
    US0_SeenShared,
}
fn method3(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_2(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method3(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn shared_text_4(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: u64) -> Rc<str> {
    let mut v5: bool = v2 > 0i32;
    if v5 {
        let mut v6: Rc<str> = format_real_2(v2);
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v6));
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" share a report name, not counted)"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
        v10.clone()
    } else {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v11.clone()
    }
}
fn format_real_6(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method3(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn minutes_text_5(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(60000u64);
    format_real_6(v1)
}
fn pace_text_7(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: u64 = v4.wrapping_sub(v5);
    let mut v7: u64 = (v0 as u64);
    let mut v8: u64 = v6.wrapping_div(v7);
    let mut v9: u64 = v8.wrapping_div(1000u64);
    let mut v10: Rc<str> = format_real_6(v9);
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" s per notebook"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    v12.clone()
}
fn batch_progress_text_1(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64) -> Rc<str> {
    let mut v7: i32 = v0.wrapping_add(v1);
    let mut v8: i32 = v7.wrapping_add(v2);
    let mut v9: Rc<str> = format_real_2(v0);
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notebook batch: "); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: Rc<str> = format_real_2(v8);
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" written"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = shared_text_4(v0, v1, v2, v3.clone(), v4);
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let v20: bool = v0 == 0i32;
    if v20 {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", none yet after "); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v21));
        let v23: u64 = v6.wrapping_sub(v5);
        let v24: Rc<str> = minutes_text_5(v23);
        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v24));
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min"); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
        v27.clone()
    } else {
        let v28: u64 = v6.wrapping_sub(v4);
        let v29: Rc<str> = minutes_text_5(v28);
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", last "); } LIT.with(|lit| lit.clone()) };
        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v30)), v3));
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v35: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v32, v33)), v29));
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min ago, ~"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
        let v38: Rc<str> = pace_text_7(v0, v1, v2, v3.clone(), v4, v5);
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
        v39.clone()
    }
}
fn method10(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method11(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
                let v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method12(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn flag_value_until_next_flag_9(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --"); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method10(v2.clone());
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let mut v8: bool = v7 < 0i32;
    let v12: Rc<str> = if v8 {
        v0.clone()
    } else {
        let mut v9: usize = ((0i32) as usize);
        let mut v10: usize = ((v7) as usize);
        let v11: Rc<str> = Rc::<str>::from(&(v0)[v9..v10]);
        v11.clone()
    };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(v12.replace(&*v13, &*v14));
    let v16: i32 = (v15.clone().len() as i32);
    let v17: i32 = 0i32;
    let v18: i32 = method11(v15.clone(), v16, v17);
    let v19: i32 = v16.wrapping_sub(1i32);
    let v20: Rc<str> = string_slice(&v15.clone(), v18 as i64, v19 as i64);
    let v21: i32 = (v20.clone().len() as i32);
    let v22: i32 = method12(v20.clone(), v21);
    let v23: Rc<str> = string_slice(&v20.clone(), 0i32 as i64, v22 as i64);
    v23.clone()
}
fn batch_output_dir_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --output-dir "); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method10(v2.clone());
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let mut v8: bool = v7 < 0i32;
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        let mut v10: i32 = v7.wrapping_add(14i32);
        let v11: i32 = (v0.clone().len() as i32);
        let v12: i32 = v11.wrapping_sub(v10);
        let v13: usize = ((v10) as usize);
        let mut v14: i32 = v10.wrapping_add(v12);
        let v15: usize = ((v14) as usize);
        let v16: Rc<str> = Rc::<str>::from(&(v0)[v13..v15]);
        flag_value_until_next_flag_9(v16.clone())
    }
}
fn method14(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: &str = &*v0;
    let mut v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let mut v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method14(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn last_write_ms_15(mut v0: Rc<str>) -> u64 {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v0);
    let v2: bool = v1.is_ok();
    let v3: bool = v2 == false;
    if v3 {
        0u64
    } else {
        let mut v4: std::fs::Metadata = v1.unwrap();
        let mut v5: std::io::Result<std::time::SystemTime> = v4.modified();
        let mut v6: bool = v5.is_ok();
        if v6 {
            let mut v7: std::time::SystemTime = v5.unwrap();
            let mut v8: std::time::Duration = v7.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
            let mut v9: u64 = v8.as_secs();
            let mut v10: u64 = v9.wrapping_mul(1000u64);
            let v11: u32 = v8.subsec_millis();
            let v12: u64 = (v11 as u64);
            let v13: u64 = v10.wrapping_add(v12);
            v13
        } else {
            0u64
        }
    }
}
fn closure2(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> u64> {
    Rc::new(move |mut v1: Rc<str>| -> u64 {
        let v2: Rc<str> = method13(v0.clone(), v1.clone());
        last_write_ms_15(v2.clone())
    })
}
fn batch_paths_17(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --path "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    v2.clone()
}
fn closure3() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn file_name_of_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let mut v7: Option<usize> = v6.rfind(&*v5);
    let mut v8: Option<usize> = method10(v7.clone());
    let mut v9: Rc<dyn Fn((usize)) -> i32> = closure3();
    let mut v10: Option<i32> = v8.map(|x| v9(x));
    let v12: i32 = v10.unwrap_or(-1i32);
    let v13: bool = v12 < 0i32;
    if v13 {
        v6.clone()
    } else {
        let mut v14: i32 = v12.wrapping_add(1i32);
        let v15: i32 = (v6.clone().len() as i32);
        let v16: i32 = v15.wrapping_sub(v12);
        let v17: i32 = v16.wrapping_sub(1i32);
        let v18: usize = ((v14) as usize);
        let v19: i32 = v14.wrapping_add(v17);
        let v20: usize = ((v19) as usize);
        let v21: Rc<str> = Rc::<str>::from(&(v6)[v18..v20]);
        v21.clone()
    }
}
fn report_name_of_20(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = file_name_of_21(v0.clone());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".ipynb"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v2));
    v3.clone()
}
fn chunk_name_19(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<dyn Fn(Rc<str>) -> u64>, mut v2: u64, mut v3: i32) -> Rc<str> {
    let mut v4: Rc<str> = v0.borrow()[v3 as usize].clone();
    let mut v5: Rc<str> = flag_value_until_next_flag_9(v4);
    report_name_of_20(v5)
}
fn names_matching_23(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<dyn Fn(Rc<str>) -> u64>, mut v2: u64, mut v3: Rc<str>, mut v4: i32, mut v5: i32) -> i32 {
    loop {
        let mut v6: i32 = (v0.clone().borrow().len() as i32);
        let mut v7: bool = v6 <= v4;
        if v7 {
            return v5;
        } else {
            let mut v8: i32 = v4.wrapping_add(1i32);
            let mut v9: Rc<str> = chunk_name_19(v0.clone(), v1.clone(), v2, v4);
            let mut v10: bool = &*v9 == &*v3;
            let v12: i32 = if v10 {
                let v11: i32 = v5.wrapping_add(1i32);
                v11
            } else {
                v5
            };
            (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2, v3.clone(), v8, v12);
            continue;
        }
    }
}
fn notebook_seen_of_22(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<dyn Fn(Rc<str>) -> u64>, mut v2: u64, mut v3: Rc<str>) -> US0 {
    let mut v4: i32 = 1i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = names_matching_23(v0, v1.clone(), v2, v3.clone(), v4, v5);
    let mut v7: bool = v6 > 1i32;
    if v7 {
        US0::US0_SeenShared
    } else {
        let mut v9: u64 = v1(v3);
        let mut v10: bool = v9 > 0u64;
        let v12: bool = if v10 {
            let v11: bool = v2 <= v9;
            v11
        } else {
            false
        };
        if v12 {
            US0::US0_SeenWritten(v9)
        } else {
            US0::US0_SeenPending
        }
    }
}
fn progress_counted_24(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<str>, mut v6: US0) -> (i32, i32, i32, Rc<str>, u64) {
    let (mut v27, mut v28, mut v29, mut v30, mut v31): (i32, i32, i32, Rc<str>, u64) = match &v6 {
        US0::US0_SeenWritten(v7) => {
            let mut v7: u64 = *v7;
            let mut v8: i32 = v0.wrapping_add(1i32);
            let mut v9: bool = v4 <= v7;
            if v9 {
                (v8, v1, v2, v5, v7)
            } else {
                (v8, v1, v2, v3.clone(), v4)
            }
        }
        _ => {
            let (mut v22, mut v23, mut v24, mut v25, mut v26): (i32, i32, i32, Rc<str>, u64) = match &v6 {
                US0::US0_SeenPending => {
                    let v15: i32 = v1.wrapping_add(1i32);
                    (v0, v15, v2, v3.clone(), v4)
                }
                _ => {
                    let (mut v17, mut v18, mut v19, mut v20, mut v21): (i32, i32, i32, Rc<str>, u64) = match &v6 {
                        US0::US0_SeenShared => {
                            let v16: i32 = v2.wrapping_add(1i32);
                            (v0, v1, v16, v3, v4)
                        }
                        _ => unreachable!(),
                    };
                    (v17, v18, v19, v20, v21)
                }
            };
            (v22, v23, v24, v25, v26)
        }
    };
    (v27, v28, v29, v30, v31)
}
fn progress_from_18(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<dyn Fn(Rc<str>) -> u64>, mut v2: u64, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: Rc<str>, mut v8: u64) -> (i32, i32, i32, Rc<str>, u64) {
    let mut v9: i32 = (v0.clone().borrow().len() as i32);
    let mut v10: bool = v9 <= v3;
    let (mut v24, mut v25, mut v26, mut v27, mut v28): (i32, i32, i32, Rc<str>, u64) = if v10 {
        (v4, v5, v6, v7.clone(), v8)
    } else {
        let v11: Rc<str> = chunk_name_19(v0.clone(), v1.clone(), v2, v3);
        let v12: i32 = v3.wrapping_add(1i32);
        let v13: US0 = notebook_seen_of_22(v0.clone(), v1.clone(), v2, v11.clone());
        let (mut v14, mut v15, mut v16, mut v17, mut v18): (i32, i32, i32, Rc<str>, u64) = progress_counted_24(v4, v5, v6, v7, v8, v11, v13);
        let (mut v19, mut v20, mut v21, mut v22, mut v23): (i32, i32, i32, Rc<str>, u64) = progress_from_18(v0, v1, v2, v12, v14, v15, v16, v17, v18);
        (v19, v20, v21, v22, v23)
    };
    (v24, v25, v26, v27, v28)
}
fn batch_progress_of_16(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> u64>, mut v2: u64) -> (i32, i32, i32, Rc<str>, u64) {
    let v3: Rc<RefCell<Vec<Rc<str>>>> = batch_paths_17(v0.clone());
    let mut v4: i32 = 1i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = 0i32;
    let mut v7: i32 = 0i32;
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: u64 = 0u64;
    let (mut v10, mut v11, mut v12, mut v13, mut v14): (i32, i32, i32, Rc<str>, u64) = progress_from_18(v3.clone(), v1.clone(), v2, v4, v5, v6, v7, v8.clone(), v9);
    (v10, v11, v12, v13.clone(), v14)
}
fn notebook_batch_line_0(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> Rc<str> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run_notebook.ps1"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v0.contains(&*v3);
    let mut v7: bool = if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --output-dir "); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = v0.contains(&*v5);
        v6
    } else {
        false
    };
    let mut v8: bool = v7 == false;
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        let mut v10: Rc<str> = batch_output_dir_8(v0.clone());
        let v11: u64 = v1.wrapping_add(1u64);
        let v12: u64 = v11.wrapping_mul(60000u64);
        let v13: u64 = v2.wrapping_sub(v12);
        let mut v14: Rc<dyn Fn(Rc<str>) -> u64> = closure2(v10.clone());
        let (mut v15, mut v16, mut v17, mut v18, mut v19): (i32, i32, i32, Rc<str>, u64) = batch_progress_of_16(v0.clone(), v14.clone(), v13);
        batch_progress_text_1(v15, v16, v17, v18.clone(), v19, v13, v2)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> Rc<str> {
        let v2: i64 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros() as i64);
        let v3: i64 = v2.wrapping_div(1000i64);
        let mut v4: u64 = (v3 as u64);
        notebook_batch_line_0(v0.clone(), v1, v4)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_notebook_batch_line(v0: &str, v1: u64) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), v1)
}
