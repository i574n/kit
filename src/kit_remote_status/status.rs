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
    US0_RemoteUnreachable(Rc<str>),
    US0_RemoteReported(u64, u64, u64),
}
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_DiskAbove(u64),
    US3_DiskBelow(u64),
    US3_DiskUnknown(Rc<str>),
}
fn summary_marker_3() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" window list ok "); } LIT.with(|lit| lit.clone()) };
    v0
}
fn method4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method5(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn method7(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let mut v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn field_value_6(mut v0: Rc<str>, mut v1: Rc<str>) -> u64 {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }, v1.clone()));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) }));
    let mut v4: i32 = v0.get(0i32 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + 0i32 as usize) as i32).unwrap_or(-1);
    let v5: bool = v4 < 0i32;
    if v5 {
        0u64
    } else {
        let mut v6: i32 = (v1.len() as i32);
        let v7: i32 = v4.wrapping_add(v6);
        let v8: i32 = v7.wrapping_add(2i32);
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v10: i32 = v0.get(v8 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v8 as usize) as i32).unwrap_or(-1);
        let mut v11: bool = v10 < 0i32;
        let v13: i32 = if v11 {
            let v12: i32 = (v0.clone().len() as i32);
            v12
        } else {
            v10
        };
        let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v13 as usize).unwrap_or(""));
        let mut v15: Option<u64> = (v14).trim().parse::<u64>().ok();
        let mut v16: Option<u64> = method7(v15);
        let mut v17: Rc<dyn Fn((u64)) -> US1> = closure1();
        let mut v18: Option<US1> = v16.map(|x| v17(x));
        let v20: US1 = US1::US1_None;
        let v21: US1 = v20.clone();
        let v22: US1 = v18.unwrap_or(v21);
        match v22 {
            US1::US1_None => {
                0u64
            }
            US1::US1_Some(v23) => {
                let v23: u64 = v23;
                v23
            }
        }
    }
}
fn status_of_2(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = summary_marker_3();
    let mut v2: i32 = v0.get(0i32 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + 0i32 as usize) as i32).unwrap_or(-1);
    let v3: bool = v2 < 0i32;
    if v3 {
        let mut v4: i32 = (v0.clone().len() as i32);
        let v5: i32 = 0i32;
        let mut v6: i32 = method4(v0.clone(), v4, v5);
        let v7: i32 = v4.wrapping_sub(1i32);
        let v8: Rc<str> = string_slice(&v0, v6 as i64, v7 as i64);
        let mut v9: i32 = (v8.clone().len() as i32);
        let v10: i32 = method5(v8.clone(), v9);
        let mut v11: Rc<str> = string_slice(&v8, 0i32 as i64, v10 as i64);
        US0::US0_RemoteUnreachable(v11)
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v14: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v13)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v15: bool = v14 < 0i32;
        let mut v17: i32 = if v15 {
            let mut v16: i32 = (v0.clone().len() as i32);
            v16
        } else {
            v14
        };
        let mut v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v17 as usize).unwrap_or(""));
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("windows"); } LIT.with(|lit| lit.clone()) };
        let v20: u64 = field_value_6(v18.clone(), v19);
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("free_mb"); } LIT.with(|lit| lit.clone()) };
        let v22: u64 = field_value_6(v18.clone(), v21);
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy_slots"); } LIT.with(|lit| lit.clone()) };
        let v24: u64 = field_value_6(v18, v23);
        US0::US0_RemoteReported(v20, v22, v24)
    }
}
fn ssh_arguments_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" window list"); } LIT.with(|lit| lit.clone()) }));
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-o"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BatchMode=yes"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ConnectTimeout=10"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v6));
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v7));
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v8));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v9));
    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v10));
    Rc::new(UH0::UH0_Cons(v3, v11))
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v2 == "";
                let mut v6: Rc<UH0> = if v4 {
                    v1
                } else {
                    Rc::new(UH0::UH0_Cons(v2, v1))
                };
                (v0, v1) = (v3, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method11(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                drop(v0.borrow_mut().arg(&*v2));
                v1 = v3;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn remote_status_of_1(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let mut v2: Rc<UH0> = ssh_arguments_8(v0, v1);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v3)));
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = method9(v2, v5);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = method10(v6, v7);
    method11(v4.clone(), v8);
    let mut v9: Rc<std::io::Result<std::process::Output>> = Rc::new(v4.borrow_mut().output());
    let v10: bool = v9.is_ok();
    let (mut v16, mut v17, mut v18): (bool, i32, Rc<str>) = if v10 {
        let mut v11: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v9).as_ref().unwrap().stderr).as_ref());
        let v12: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v9).as_ref().unwrap().stdout).as_ref());
        let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v11));
        let v14: i32 = (&*v9).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v14, v13)
    } else {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v15)
    };
    let v20: bool = if v16 {
        let v19: bool = v17 == 0i32;
        v19
    } else {
        false
    };
    let v23: US2 = if v20 {
        US2::US2_Some(v18)
    } else {
        US2::US2_None
    };
    match v23 {
        US2::US2_None => {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh did not run"); } LIT.with(|lit| lit.clone()) };
            US0::US0_RemoteUnreachable(v26)
        }
        US2::US2_Some(v24) => {
            let v24: Rc<str> = v24;
            status_of_2(v24)
        }
    }
}
fn method14(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_13(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method14(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn status_line_12(mut v0: Rc<str>, mut v1: US0) -> Rc<str> {
    match v1 {
        US0::US0_RemoteReported(v2, v3, v4) => {
            let mut v2: u64 = v2;
            let v3: u64 = v3;
            let mut v4: u64 = v4;
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit remote status ok machine="); } LIT.with(|lit| lit.clone()) }, v0));
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" windows="); } LIT.with(|lit| lit.clone()) }));
            let v7: Rc<str> = format_real_13(v2);
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free_mb="); } LIT.with(|lit| lit.clone()) }));
            let v10: Rc<str> = format_real_13(v3);
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" heavy_slots="); } LIT.with(|lit| lit.clone()) }));
            let v13: Rc<str> = format_real_13(v4);
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
            v14
        }
        US0::US0_RemoteUnreachable(v15) => {
            let mut v15: Rc<str> = v15;
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit remote status unreachable machine="); } LIT.with(|lit| lit.clone()) }, v0));
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason="); } LIT.with(|lit| lit.clone()) }));
            let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v15));
            v18
        }
    }
}
fn jobs_arguments_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job list"); } LIT.with(|lit| lit.clone()) }));
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-o"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BatchMode=yes"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ConnectTimeout=10"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v6));
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v7));
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v8));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v9));
    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v10));
    Rc::new(UH0::UH0_Cons(v3, v11))
}
fn disk_arguments_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" machine health --gate --min-disk-gb 0 --max-busy-pct 1000"); } LIT.with(|lit| lit.clone()) }));
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-o"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BatchMode=yes"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ConnectTimeout=10"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v6));
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v7));
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v8));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v9));
    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v10));
    Rc::new(UH0::UH0_Cons(v3, v11))
}
fn disk_line_17(mut v0: Rc<str>, mut v1: US3) -> Rc<str> {
    let mut v2: u64 = 10u64;
    let v3: Rc<str> = format_real_13(v2);
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" floor_gb="); } LIT.with(|lit| lit.clone()) }, v3));
    match v1 {
        US3::US3_DiskAbove(v5) => {
            let v5: u64 = v5;
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit remote status disk machine="); } LIT.with(|lit| lit.clone()) }, v0));
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free_gb="); } LIT.with(|lit| lit.clone()) }));
            let v8: Rc<str> = format_real_13(v5);
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v4));
            v10
        }
        US3::US3_DiskBelow(v11) => {
            let mut v11: u64 = v11;
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit remote status DISK-LOW machine="); } LIT.with(|lit| lit.clone()) }, v0));
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free_gb="); } LIT.with(|lit| lit.clone()) }));
            let v14: Rc<str> = format_real_13(v11);
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v4));
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (prune finished staged workspaces before a heavy job)"); } LIT.with(|lit| lit.clone()) }));
            v17
        }
        US3::US3_DiskUnknown(v18) => {
            let mut v18: Rc<str> = v18;
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit remote status disk unknown machine="); } LIT.with(|lit| lit.clone()) }, v0));
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason="); } LIT.with(|lit| lit.clone()) }));
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v18));
            v21
        }
    }
}
fn disk_of_18(mut v0: Rc<str>) -> US3 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit machine health gate="); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = v0.get(0i32 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + 0i32 as usize) as i32).unwrap_or(-1);
    let v3: bool = v2 < 0i32;
    let v7: Rc<str> = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let v5: i32 = (v0.clone().len() as i32);
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v5 as usize).unwrap_or(""));
        v6
    };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disk"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = kit_remote_activity::kit_remote_field_text(&v7, &v8);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gb"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v11));
    let v13: Option<u64> = (v12).trim().parse::<u64>().ok();
    let v14: Option<u64> = method7(v13);
    let mut v15: Rc<dyn Fn((u64)) -> US1> = closure1();
    let mut v16: Option<US1> = v14.map(|x| v15(x));
    let mut v18: US1 = US1::US1_None;
    let v19: US1 = v18.clone();
    let v20: US1 = v16.unwrap_or(v19);
    match v20 {
        US1::US1_None => {
            let v26: i32 = (v0.clone().len() as i32);
            let v27: i32 = 0i32;
            let v28: i32 = method4(v0.clone(), v26, v27);
            let v29: i32 = v26.wrapping_sub(1i32);
            let v30: Rc<str> = string_slice(&v0, v28 as i64, v29 as i64);
            let mut v31: i32 = (v30.clone().len() as i32);
            let v32: i32 = method5(v30.clone(), v31);
            let v33: Rc<str> = string_slice(&v30, 0i32 as i64, v32 as i64);
            US3::US3_DiskUnknown(v33)
        }
        US1::US1_Some(v21) => {
            let v21: u64 = v21;
            let v22: bool = v21 < 10u64;
            if v22 {
                US3::US3_DiskBelow(v21)
            } else {
                US3::US3_DiskAbove(v21)
            }
        }
    }
}
fn remote_status_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: US0 = remote_status_of_1(v0.clone(), v1.clone());
    let v3: Rc<str> = status_line_12(v0.clone(), v2.clone());
    println!("{}", v3);
    let mut v4: Rc<UH0> = jobs_arguments_15(v0.clone(), v1.clone());
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v5)));
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = method9(v4, v7);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH0> = method10(v8, v9);
    method11(v6.clone(), v10);
    let mut v11: Rc<std::io::Result<std::process::Output>> = Rc::new(v6.borrow_mut().output());
    let v12: bool = v11.is_ok();
    let (mut v18, mut v19, mut v20): (bool, i32, Rc<str>) = if v12 {
        let v13: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v11).as_ref().unwrap().stderr).as_ref());
        let v14: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v11).as_ref().unwrap().stdout).as_ref());
        let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
        let mut v16: i32 = (&*v11).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v16, v15)
    } else {
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v17)
    };
    let v22: bool = if v18 {
        let v21: bool = v19 == 0i32;
        v21
    } else {
        false
    };
    let v25: US2 = if v22 {
        US2::US2_Some(v20)
    } else {
        US2::US2_None
    };
    match v25 {
        US2::US2_None => {
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit remote status jobs unknown machine="); } LIT.with(|lit| lit.clone()) }, v0.clone()));
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason=ssh did not run"); } LIT.with(|lit| lit.clone()) }));
            println!("{}", v29);
            ()
        }
        US2::US2_Some(v26) => {
            let v26: Rc<str> = v26;
            let v27: Rc<str> = kit_remote_activity::kit_remote_activity_text(&v0, &v26);
            println!("{}", v27);
            ()
        }
    };
    let v30: Rc<UH0> = disk_arguments_16(v0.clone(), v1);
    let mut v31: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v5)));
    let v32: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v33: Rc<UH0> = method9(v30, v32);
    let v34: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v35: Rc<UH0> = method10(v33, v34);
    method11(v31.clone(), v35);
    let mut v36: Rc<std::io::Result<std::process::Output>> = Rc::new(v31.borrow_mut().output());
    let v37: bool = v36.is_ok();
    let (mut v43, mut v44, mut v45): (bool, i32, Rc<str>) = if v37 {
        let v38: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v36).as_ref().unwrap().stderr).as_ref());
        let v39: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v36).as_ref().unwrap().stdout).as_ref());
        let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v38));
        let v41: i32 = (&*v36).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v41, v40)
    } else {
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v42)
    };
    let v47: bool = if v43 {
        let v46: bool = v44 == 0i32;
        v46
    } else {
        false
    };
    let v50: US2 = if v47 {
        US2::US2_Some(v45)
    } else {
        US2::US2_None
    };
    match v50 {
        US2::US2_None => {
            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh did not run"); } LIT.with(|lit| lit.clone()) };
            let v55: US3 = US3::US3_DiskUnknown(v54);
            let v56: Rc<str> = disk_line_17(v0, v55);
            println!("{}", v56);
            ()
        }
        US2::US2_Some(v51) => {
            let v51: Rc<str> = v51;
            let v52: US3 = disk_of_18(v51);
            let v53: Rc<str> = disk_line_17(v0, v52);
            println!("{}", v53);
            ()
        }
    };
    match v2 {
        US0::US0_RemoteReported(v57, v58, v59) => {
            let v57: u64 = v57;
            let v58: u64 = v58;
            let v59: u64 = v59;
            0i32
        }
        US0::US0_RemoteUnreachable(v60) => {
            let v60: Rc<str> = v60;
            1i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        remote_status_run_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_remote_status_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
use kit_remote_activity::SpiralShape_97df63ebabc3 as Mut0;
