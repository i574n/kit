#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
fn refused_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: mirror "); } LIT.with(|lit| lit.clone()) }, v0));
    eprintln!("{}", v1);
    2i32
}
fn listing_at_2(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>, i32) {
    let mut v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32)>>> = kit_mirror_local::kit_mirror_listing_of(&v0, &v1);
    let (mut v3, mut v4, mut v5): (Rc<str>, Rc<str>, i32) = v2.borrow()[0i32 as usize].clone();
    (v3, v4, v5)
}
fn plan_at_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) {
    let v4: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> = kit_mirror_plan::kit_mirror_plan_of(&v0, &v1, &v2, &v3);
    let (mut v5, mut v6, mut v7, mut v8, mut v9): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = v4.borrow()[0i32 as usize].clone();
    (v5, v6, v7, v8, v9)
}
fn method5(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn listed_of_4(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<UH0> {
    let mut v1: Rc<Vec<Rc<str>>> = Rc::new((v0).borrow().clone());
    let mut v2: usize = (v1).len();
    let v3: i32 = ((v2) as i32);
    let v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method5(v1, v4, v5)
}
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method6(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method7(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method9(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_8(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method9(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method12(v3, v1);
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method13(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn lines_text_11(mut v0: Rc<UH0>) -> Rc<str> {
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH0> = method12(v0, v1);
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v4: i32 = 0i32;
    let v5: i32 = method13(v3.clone(), v2, v4);
    let v6: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from((*v7.borrow()).join(&*v8));
    v9
}
fn method17(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn get_temp_path_16() -> Rc<str> {
    let mut v1: std::path::PathBuf = std::env::temp_dir();
    let mut v2: std::path::PathBuf = method17(v1);
    let v4: std::path::Display = v2.display();
    let v6: std::string::String = format!("{}", v4);
    let v8: Rc<str> = Rc::<str>::from(String::as_str(&v6));
    let v9: i32 = (v8.clone().len() as i32);
    let v10: bool = v9 > 1i32;
    let v16: bool = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v12: bool = v8.ends_with(&*v11);
        if v12 {
            true
        } else {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v14: bool = v8.ends_with(&*v13);
            v14
        }
    } else {
        false
    };
    if v16 {
        let mut v17: i32 = v9.wrapping_sub(1i32);
        let v18: usize = ((0i32) as usize);
        let v19: usize = ((v17) as usize);
        let v20: Rc<str> = Rc::<str>::from(&(v8)[v18..v19]);
        v20
    } else {
        v8
    }
}
fn format_real_18(mut v0: u32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method9(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn temp_named_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = get_temp_path_16();
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v0));
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
    let v5: u32 = std::process::id();
    let v6: Rc<str> = format_real_18(v5);
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v6));
    v7
}
fn ship_list_14() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-mirror-ship"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = temp_named_15(v0);
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".list"); } LIT.with(|lit| lit.clone()) }));
    v2
}
fn removal_name_19() -> Rc<str> {
    let mut v0: u32 = std::process::id();
    let mut v1: Rc<str> = format_real_18(v0);
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-mirror-removed-"); } LIT.with(|lit| lit.clone()) }, v1));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".list"); } LIT.with(|lit| lit.clone()) }));
    v3
}
fn tar_path_20() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-mirror"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = temp_named_15(v0);
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".tar"); } LIT.with(|lit| lit.clone()) }));
    v2
}
fn get_environment_variable_22(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn tar_program_21() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SystemRoot"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = get_environment_variable_22(v0);
    let mut v2: bool = &*v1 == "";
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tar"); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/System32/tar.exe"); } LIT.with(|lit| lit.clone()) }));
        v4
    }
}
fn method23(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method24(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
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
fn packed_10(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> bool {
    let v3: Rc<str> = lines_text_11(v1);
    let v4: Rc<str> = ship_list_14();
    std::fs::write(&*v4, &*v3).unwrap();
    let v6: Rc<str> = lines_text_11(v2);
    let v7: Rc<str> = get_temp_path_16();
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v9: Rc<str> = removal_name_19();
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    std::fs::write(&*v10, &*v6).unwrap();
    let mut v12: Rc<str> = tar_path_20();
    let v13: Rc<str> = ship_list_14();
    let v14: Rc<str> = get_temp_path_16();
    let v15: Rc<str> = removal_name_19();
    let v16: Rc<str> = tar_program_21();
    let mut v17: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v16)));
    let v18: bool = &*v12 == "";
    let v26: Rc<UH0> = if v18 {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-cf"); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v19, v20))
    } else {
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-cf"); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v22, v23));
        Rc::new(UH0::UH0_Cons(v12, v24))
    };
    let v27: bool = &*v0 == "";
    let v33: Rc<UH0> = if v27 {
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
        Rc::new(UH0::UH0_Cons(v28, v26))
    } else {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<UH0> = Rc::new(UH0::UH0_Cons(v30, v26));
        Rc::new(UH0::UH0_Cons(v0, v31))
    };
    let v34: bool = &*v13 == "";
    let v40: Rc<UH0> = if v34 {
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-T"); } LIT.with(|lit| lit.clone()) };
        Rc::new(UH0::UH0_Cons(v35, v33))
    } else {
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-T"); } LIT.with(|lit| lit.clone()) };
        let v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v37, v33));
        Rc::new(UH0::UH0_Cons(v13, v38))
    };
    let v41: bool = &*v14 == "";
    let v47: Rc<UH0> = if v41 {
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
        Rc::new(UH0::UH0_Cons(v42, v40))
    } else {
        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
        let v45: Rc<UH0> = Rc::new(UH0::UH0_Cons(v44, v40));
        Rc::new(UH0::UH0_Cons(v14, v45))
    };
    let v48: bool = &*v15 == "";
    let v50: Rc<UH0> = if v48 {
        v47
    } else {
        Rc::new(UH0::UH0_Cons(v15, v47))
    };
    let v51: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v52: Rc<UH0> = method23(v50, v51);
    method24(v17.clone(), v52);
    let mut v53: Rc<std::io::Result<std::process::Output>> = Rc::new(v17.borrow_mut().output());
    let v54: bool = v53.is_ok();
    let (mut v60, mut v61, mut v62): (bool, i32, Rc<str>) = if v54 {
        let v55: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v53).as_ref().unwrap().stderr).as_ref());
        let v56: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v53).as_ref().unwrap().stdout).as_ref());
        let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v55));
        let v58: i32 = (&*v53).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v58, v57)
    } else {
        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v59)
    };
    if v60 {
        let v63: bool = v61 == 0i32;
        v63
    } else {
        false
    }
}
fn ssh_options_26() -> Rc<UH0> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-o"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BatchMode=yes"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ConnectTimeout=12"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3));
    let v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0.clone(), v4));
    let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v5));
    Rc::new(UH0::UH0_Cons(v0, v6))
}
fn method27(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: bool = &*v2 == "";
                let v6: Rc<UH0> = if v4 {
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
fn shipped_25(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/.kit-mirror-incoming.tar"); } LIT.with(|lit| lit.clone()) }));
    let v5: Rc<UH0> = ssh_options_26();
    let v6: Rc<str> = tar_path_20();
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v7));
    let v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v8));
    let v10: Rc<UH0> = method6(v5, v9);
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("scp"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v11)));
    let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v14: Rc<UH0> = method27(v10, v13);
    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH0> = method23(v14, v15);
    method24(v12.clone(), v16);
    let mut v17: Rc<std::io::Result<std::process::Output>> = Rc::new(v12.borrow_mut().output());
    let v18: bool = v17.is_ok();
    let (mut v24, mut v25, mut v26): (bool, i32, Rc<str>) = if v18 {
        let v19: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v17).as_ref().unwrap().stderr).as_ref());
        let v20: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v17).as_ref().unwrap().stdout).as_ref());
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
        let v22: i32 = (&*v17).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v22, v21)
    } else {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v23)
    };
    if v24 {
        let v27: bool = v25 == 0i32;
        v27
    } else {
        false
    }
}
fn unpacked_28(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<UH0> = ssh_options_26();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit mirror unpack "); } LIT.with(|lit| lit.clone()) }, v1));
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
    let v5: Rc<str> = removal_name_19();
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v7));
    let v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v8));
    let v10: Rc<UH0> = method6(v2, v9);
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v11)));
    let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v14: Rc<UH0> = method27(v10, v13);
    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH0> = method23(v14, v15);
    method24(v12.clone(), v16);
    let mut v17: Rc<std::io::Result<std::process::Output>> = Rc::new(v12.borrow_mut().output());
    let v18: bool = v17.is_ok();
    let (mut v24, mut v25, mut v26): (bool, i32, Rc<str>) = if v18 {
        let v19: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v17).as_ref().unwrap().stderr).as_ref());
        let v20: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v17).as_ref().unwrap().stdout).as_ref());
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
        let v22: i32 = (&*v17).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v22, v21)
    } else {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v23)
    };
    if v24 {
        let v27: bool = v25 == 0i32;
        v27
    } else {
        false
    }
}
fn cleaned_29() -> () {
    let mut v0: Rc<str> = ship_list_14();
    let mut v1: Rc<str> = tar_path_20();
    let mut v2: Rc<str> = get_temp_path_16();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v4: Rc<str> = removal_name_19();
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let v6: bool = std::fs::remove_file(&*v0).is_ok() || !std::path::Path::new(&*v0).exists();
    let v7: bool = std::fs::remove_file(&*v1).is_ok() || !std::path::Path::new(&*v1).exists();
    let v8: bool = std::fs::remove_file(&*v5).is_ok() || !std::path::Path::new(&*v5).exists();
    ()
}
fn sync_printed_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let (mut v4, mut v5, mut v6): (Rc<str>, Rc<str>, i32) = listing_at_2(v2.clone(), v0.clone());
    let (mut v7, mut v8, mut v9, mut v10, mut v11): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = plan_at_3(v3.clone(), v1.clone(), v0.clone(), v4.clone());
    let mut v12: Rc<UH0> = listed_of_4(v8);
    let v13: Rc<UH0> = listed_of_4(v9);
    let v14: Rc<UH0> = method6(v12, v13);
    let v15: Rc<UH0> = listed_of_4(v10);
    let v16: i32 = 0i32;
    let mut v17: i32 = method7(v14.clone(), v16);
    let v18: Rc<str> = format_real_8(v17);
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shipped="); } LIT.with(|lit| lit.clone()) }, v18));
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" removed="); } LIT.with(|lit| lit.clone()) }));
    let v21: i32 = 0i32;
    let v22: i32 = method7(v15.clone(), v21);
    let v23: Rc<str> = format_real_8(v22);
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v23));
    let v25: bool = &*v7 != "";
    if v25 {
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sync "); } LIT.with(|lit| lit.clone()) }, v3));
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) }));
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v7));
        refused_1(v28)
    } else {
        let v30: bool = &*v11 == &*v5;
        if v30 {
            let v31: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit mirror sync in-step machine="); } LIT.with(|lit| lit.clone()) }, v3));
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" fingerprint="); } LIT.with(|lit| lit.clone()) }));
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v5));
            eprintln!("{}", v33);
            0i32
        } else {
            let v34: bool = packed_10(v2, v14, v15);
            let v36: bool = if v34 {
                shipped_25(v3.clone(), v1.clone())
            } else {
                false
            };
            let v38: bool = if v36 {
                unpacked_28(v3.clone(), v1.clone())
            } else {
                false
            };
            cleaned_29();
            let v45: Rc<str> = if v38 {
                let (mut v39, mut v40, mut v41, mut v42, mut v43): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = plan_at_3(v3.clone(), v1, v0, v4);
                v43
            } else {
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v44
            };
            let v46: bool = &*v45 == &*v5;
            if v46 {
                let v47: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit mirror sync ok machine="); } LIT.with(|lit| lit.clone()) }, v3));
                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v24));
                let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" fingerprint="); } LIT.with(|lit| lit.clone()) }));
                let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v5));
                eprintln!("{}", v50);
                0i32
            } else {
                let mut v53: Rc<str> = if v38 {
                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the remote fingerprint still differs"); } LIT.with(|lit| lit.clone()) };
                    v51
                } else {
                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pack, copy or unpack failed"); } LIT.with(|lit| lit.clone()) };
                    v52
                };
                let v54: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: mirror sync "); } LIT.with(|lit| lit.clone()) }, v3));
                let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) }));
                let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v53));
                let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v24));
                eprintln!("{}", v57);
                1i32
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> i32 {
        sync_printed_0(v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_mirror_sync_printed(v0: &str, v1: &str, v2: &str, v3: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
use kit_mirror_local::SpiralShape_97df63ebabc3 as Mut0;
