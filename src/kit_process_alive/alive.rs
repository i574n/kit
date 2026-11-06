#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
fn process_alive_0(mut v0: u64) -> bool {
    let v1: bool = v0 == 0u64;
    if v1 {
        true
    } else {
        let v2: u32 = ((v0) as u32);
        let v3: sysinfo::RefreshKind = sysinfo::RefreshKind::nothing().with_processes(Default::default());
        let mut v4: std::rc::Rc<sysinfo::System> = std::rc::Rc::new(sysinfo::System::new_with_specifics(v3));
        let mut v5: sysinfo::Pid = sysinfo::Pid::from_u32(v2);
        let v6: Option<sysinfo::ProcessStatus> = v4.process(v5).map(|p| p.status());
        let v7: Option<sysinfo::ProcessStatus> = None;
        let v8: bool = v6.eq(&v7);
        let mut v11: bool = if v8 {
            true
        } else {
            let mut v9: Option<sysinfo::ProcessStatus> = Some(sysinfo::ProcessStatus::Zombie);
            let mut v10: bool = v6.eq(&v9);
            v10
        };
        let v14: bool = if v11 {
            true
        } else {
            let v12: Option<sysinfo::ProcessStatus> = Some(sysinfo::ProcessStatus::Dead);
            let v13: bool = v6.eq(&v12);
            v13
        };
        let v15: bool = v14 == false;
        v15
    }
}
fn closure0() -> Rc<dyn Fn(u64) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64) -> bool> = Rc::new(move |mut v0: u64| -> bool {
        process_alive_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn owner_running_1(mut v0: u64, mut v1: u64) -> bool {
    let v2: bool = v1 > 0u64;
    if v2 {
        let v3: u32 = ((v1) as u32);
        let mut v4: sysinfo::RefreshKind = sysinfo::RefreshKind::nothing().with_processes(Default::default());
        let mut v5: std::rc::Rc<sysinfo::System> = std::rc::Rc::new(sysinfo::System::new_with_specifics(v4));
        let v6: sysinfo::Pid = sysinfo::Pid::from_u32(v3);
        let v7: u64 = v5.process(v6).map(|p| p.start_time()).unwrap_or(0);
        let v8: u64 = v7.wrapping_mul(1000u64);
        let mut v9: bool = v8 > 0u64;
        let v12: bool = if v9 {
            let mut v10: u64 = v8.wrapping_add(2000u64);
            let mut v11: bool = v10 > v0;
            v11
        } else {
            false
        };
        if v12 {
            let v13: u64 = v0.wrapping_add(2000u64);
            let v14: bool = v13 > v8;
            v14
        } else {
            false
        }
    } else {
        false
    }
}
fn closure1() -> Rc<dyn Fn(u64, u64) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, u64) -> bool> = Rc::new(move |mut v0: u64, mut v1: u64| -> bool {
        owner_running_1(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method5(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
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
        method4(v16, v17)
    }
}
fn method6(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure3() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn home_abandoned_2(mut v0: u64, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH0> = split_list_3(v2, v1);
    match Rc::unwrap_or_clone(v3) {
        UH0::UH0_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4;
            let mut v5: Rc<UH0> = v5;
            match Rc::unwrap_or_clone(v5) {
                UH0::UH0_Cons(v6, v7) => {
                    let v6: Rc<str> = v6;
                    let v7: Rc<UH0> = v7;
                    match Rc::unwrap_or_clone(v7) {
                        UH0::UH0_Nil => {
                            let v8: Option<u64> = (v4).trim().parse::<u64>().ok();
                            let mut v9: Option<u64> = method6(v8);
                            let mut v10: Rc<dyn Fn((u64)) -> US0> = closure3();
                            let mut v11: Option<US0> = v9.map(|x| v10(x));
                            let v13: US0 = US0::US0_None;
                            let v14: US0 = v13.clone();
                            let v15: US0 = v11.unwrap_or(v14);
                            let v18: u64 = match v15 {
                                US0::US0_None => {
                                    0u64
                                }
                                US0::US0_Some(v16) => {
                                    let v16: u64 = v16;
                                    v16
                                }
                            };
                            let v19: Option<u64> = (v6).trim().parse::<u64>().ok();
                            let v20: Option<u64> = method6(v19);
                            let mut v21: Option<US0> = v20.map(|x| v10(x));
                            let v23: US0 = US0::US0_None;
                            let v24: US0 = v23.clone();
                            let v25: US0 = v21.unwrap_or(v24);
                            let v28: u64 = match v25 {
                                US0::US0_None => {
                                    0u64
                                }
                                US0::US0_Some(v26) => {
                                    let v26: u64 = v26;
                                    v26
                                }
                            };
                            let v29: bool = owner_running_1(v28, v18);
                            let v30: bool = v29 == false;
                            if v30 {
                                let v31: bool = v0 == 0u64;
                                if v31 {
                                    true
                                } else {
                                    let v32: bool = process_alive_0(v0);
                                    let v33: bool = v32 == false;
                                    v33
                                }
                            } else {
                                false
                            }
                        }
                        _ => {
                            false
                        }
                    }
                }
                _ => {
                    false
                }
            }
        }
        _ => {
            false
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, u64) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> bool {
        home_abandoned_2(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_process_alive(v0: u64) -> bool {
    closure0()(v0)
}
pub fn kit_process_owner_running(v0: u64, v1: u64) -> bool {
    closure1()(v0, v1)
}
pub fn kit_process_home_abandoned(v0: &str, v1: u64) -> bool {
    closure2()(Rc::<str>::from(v0), v1)
}
pub type SpiralShape_652accfc6a8d = Mut0;
