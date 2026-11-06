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
    US0_RemoteManifest,
    US0_RemotePlan,
    US0_RemoteSync,
    US0_RemoteUnpack,
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(US0),
    US2_None,
}
fn refused_1(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: mirror "); } LIT.with(|lit| lit.clone()) }, v0));
    eprintln!("{}", v1);
    2i32
}
fn directory_exists_4(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn listing_at_5(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>, i32) {
    let v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32)>>> = kit_mirror_local::kit_mirror_listing_of(&v0, &v1);
    let (mut v3, mut v4, mut v5): (Rc<str>, Rc<str>, i32) = v2.borrow()[0i32 as usize].clone();
    (v3, v4, v5)
}
fn method7(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_6(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method7(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn manifest_printed_3(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: bool = directory_exists_4(v0.clone());
    let v3: bool = v2 == false;
    if v3 {
        let v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("manifest needs an existing root, received "); } LIT.with(|lit| lit.clone()) }, v0));
        refused_1(v4)
    } else {
        let (mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, i32) = listing_at_5(v0, v1);
        print!("{}", v6);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let v9: Rc<str> = format_real_6(v8);
        let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit mirror manifest ok files="); } LIT.with(|lit| lit.clone()) }, v9));
        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" fingerprint="); } LIT.with(|lit| lit.clone()) }));
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v7));
        eprintln!("{}", v12);
        0i32
    }
}
fn plan_at_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) {
    let v4: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> = kit_mirror_plan::kit_mirror_plan_of(&v0, &v1, &v2, &v3);
    let (mut v5, mut v6, mut v7, mut v8, mut v9): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = v4.borrow()[0i32 as usize].clone();
    (v5, v6, v7, v8, v9)
}
fn method11(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let mut v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn listed_of_10(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<UH0> {
    let v1: Rc<Vec<Rc<str>>> = Rc::new((v0).borrow().clone());
    let v2: usize = (v1).len();
    let v3: i32 = ((v2) as i32);
    let v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method11(v1, v4, v5)
}
fn method12(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
                let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add\t"); } LIT.with(|lit| lit.clone()) }, v1));
                println!("{}", v3);
                v0 = v2;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method13(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
                let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("change\t"); } LIT.with(|lit| lit.clone()) }, v1));
                println!("{}", v3);
                v0 = v2;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method14(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
                let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("remove\t"); } LIT.with(|lit| lit.clone()) }, v1));
                println!("{}", v3);
                v0 = v2;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn plan_printed_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let (mut v4, mut v5, mut v6): (Rc<str>, Rc<str>, i32) = listing_at_5(v1, v3.clone());
    let (mut v7, mut v8, mut v9, mut v10, mut v11): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = plan_at_9(v0.clone(), v2, v3, v4);
    let v12: bool = &*v7 != "";
    if v12 {
        let v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan "); } LIT.with(|lit| lit.clone()) }, v0));
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) }));
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v7));
        refused_1(v15)
    } else {
        let v17: Rc<UH0> = listed_of_10(v8);
        method12(v17);
        let v18: Rc<UH0> = listed_of_10(v9);
        method13(v18);
        let v19: Rc<UH0> = listed_of_10(v10);
        method14(v19);
        let v20: bool = &*v11 == &*v5;
        let v23: Rc<str> = if v20 {
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in-step"); } LIT.with(|lit| lit.clone()) };
            v21
        } else {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("drift"); } LIT.with(|lit| lit.clone()) };
            v22
        };
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit mirror plan "); } LIT.with(|lit| lit.clone()) }, v23));
        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" machine="); } LIT.with(|lit| lit.clone()) }));
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v0));
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" local="); } LIT.with(|lit| lit.clone()) }));
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v5));
        eprintln!("{}", v28);
        if v20 {
            0i32
        } else {
            1i32
        }
    }
}
fn skip_of_15(mut v0: &Rc<UH0>) -> US1 {
    match &**v0 {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<UH0> = v4.clone();
            match Rc::unwrap_or_clone(v4) {
                UH0::UH0_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let mut v6: Rc<UH0> = v6;
                    match Rc::unwrap_or_clone(v6) {
                        UH0::UH0_Nil => {
                            let v7: bool = &*v3 == "--skip";
                            if v7 {
                                US1::US1_Some(v5)
                            } else {
                                US1::US1_None
                            }
                        }
                        _ => {
                            US1::US1_None
                        }
                    }
                }
                _ => {
                    US1::US1_None
                }
            }
        }
        UH0::UH0_Nil => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            US1::US1_Some(v1)
        }
    }
}
fn command_ran_2(mut v0: US0, mut v1: Rc<UH0>) -> i32 {
    match v0 {
        US0::US0_RemoteManifest => {
            match Rc::unwrap_or_clone(v1) {
                UH0::UH0_Cons(v16, v17) => {
                    let v16: Rc<str> = v16;
                    let v17: Rc<UH0> = v17;
                    let v18: US1 = skip_of_15(&v17);
                    match v18 {
                        US1::US1_None => {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror manifest <root> | plan|sync <ssh-alias> <local-root> <remote-root> | unpack <root> <removal-list>; [--skip a,b]"); } LIT.with(|lit| lit.clone()) };
                            refused_1(v21)
                        }
                        US1::US1_Some(v19) => {
                            let v19: Rc<str> = v19;
                            manifest_printed_3(v16, v19)
                        }
                    }
                }
                _ => {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror manifest <root> | plan|sync <ssh-alias> <local-root> <remote-root> | unpack <root> <removal-list>; [--skip a,b]"); } LIT.with(|lit| lit.clone()) };
                    refused_1(v25)
                }
            }
        }
        US0::US0_RemotePlan => {
            match Rc::unwrap_or_clone(v1) {
                UH0::UH0_Cons(v28, v29) => {
                    let v28: Rc<str> = v28;
                    let v29: Rc<UH0> = v29;
                    match Rc::unwrap_or_clone(v29) {
                        UH0::UH0_Cons(v30, v31) => {
                            let v30: Rc<str> = v30;
                            let v31: Rc<UH0> = v31;
                            match Rc::unwrap_or_clone(v31) {
                                UH0::UH0_Cons(v32, v33) => {
                                    let v32: Rc<str> = v32;
                                    let v33: Rc<UH0> = v33;
                                    let v34: US1 = skip_of_15(&v33);
                                    match v34 {
                                        US1::US1_None => {
                                            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror manifest <root> | plan|sync <ssh-alias> <local-root> <remote-root> | unpack <root> <removal-list>; [--skip a,b]"); } LIT.with(|lit| lit.clone()) };
                                            refused_1(v37)
                                        }
                                        US1::US1_Some(v35) => {
                                            let v35: Rc<str> = v35;
                                            plan_printed_8(v28, v30, v32, v35)
                                        }
                                    }
                                }
                                _ => {
                                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror manifest <root> | plan|sync <ssh-alias> <local-root> <remote-root> | unpack <root> <removal-list>; [--skip a,b]"); } LIT.with(|lit| lit.clone()) };
                                    refused_1(v41)
                                }
                            }
                        }
                        _ => {
                            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror manifest <root> | plan|sync <ssh-alias> <local-root> <remote-root> | unpack <root> <removal-list>; [--skip a,b]"); } LIT.with(|lit| lit.clone()) };
                            refused_1(v44)
                        }
                    }
                }
                _ => {
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror manifest <root> | plan|sync <ssh-alias> <local-root> <remote-root> | unpack <root> <removal-list>; [--skip a,b]"); } LIT.with(|lit| lit.clone()) };
                    refused_1(v47)
                }
            }
        }
        US0::US0_RemoteSync => {
            match Rc::unwrap_or_clone(v1) {
                UH0::UH0_Cons(v50, v51) => {
                    let v50: Rc<str> = v50;
                    let v51: Rc<UH0> = v51;
                    match Rc::unwrap_or_clone(v51) {
                        UH0::UH0_Cons(v52, v53) => {
                            let v52: Rc<str> = v52;
                            let v53: Rc<UH0> = v53;
                            match Rc::unwrap_or_clone(v53) {
                                UH0::UH0_Cons(v54, v55) => {
                                    let v54: Rc<str> = v54;
                                    let v55: Rc<UH0> = v55;
                                    let v56: US1 = skip_of_15(&v55);
                                    match v56 {
                                        US1::US1_None => {
                                            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror manifest <root> | plan|sync <ssh-alias> <local-root> <remote-root> | unpack <root> <removal-list>; [--skip a,b]"); } LIT.with(|lit| lit.clone()) };
                                            refused_1(v59)
                                        }
                                        US1::US1_Some(v57) => {
                                            let v57: Rc<str> = v57;
                                            let v58: i32 = kit_mirror_sync::kit_mirror_sync_printed(&v50, &v52, &v54, &v57);
                                            v58
                                        }
                                    }
                                }
                                _ => {
                                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror manifest <root> | plan|sync <ssh-alias> <local-root> <remote-root> | unpack <root> <removal-list>; [--skip a,b]"); } LIT.with(|lit| lit.clone()) };
                                    refused_1(v63)
                                }
                            }
                        }
                        _ => {
                            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror manifest <root> | plan|sync <ssh-alias> <local-root> <remote-root> | unpack <root> <removal-list>; [--skip a,b]"); } LIT.with(|lit| lit.clone()) };
                            refused_1(v66)
                        }
                    }
                }
                _ => {
                    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror manifest <root> | plan|sync <ssh-alias> <local-root> <remote-root> | unpack <root> <removal-list>; [--skip a,b]"); } LIT.with(|lit| lit.clone()) };
                    refused_1(v69)
                }
            }
        }
        US0::US0_RemoteUnpack => {
            match Rc::unwrap_or_clone(v1) {
                UH0::UH0_Cons(v2, v3) => {
                    let v2: Rc<str> = v2;
                    let v3: Rc<UH0> = v3;
                    match Rc::unwrap_or_clone(v3) {
                        UH0::UH0_Cons(v4, v5) => {
                            let v4: Rc<str> = v4;
                            let v5: Rc<UH0> = v5;
                            match Rc::unwrap_or_clone(v5) {
                                UH0::UH0_Nil => {
                                    let mut v6: i32 = kit_mirror_unpack::kit_mirror_unpack_printed(&v2, &v4);
                                    v6
                                }
                                _ => {
                                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror manifest <root> | plan|sync <ssh-alias> <local-root> <remote-root> | unpack <root> <removal-list>; [--skip a,b]"); } LIT.with(|lit| lit.clone()) };
                                    refused_1(v7)
                                }
                            }
                        }
                        _ => {
                            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror manifest <root> | plan|sync <ssh-alias> <local-root> <remote-root> | unpack <root> <removal-list>; [--skip a,b]"); } LIT.with(|lit| lit.clone()) };
                            refused_1(v10)
                        }
                    }
                }
                _ => {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror manifest <root> | plan|sync <ssh-alias> <local-root> <remote-root> | unpack <root> <removal-list>; [--skip a,b]"); } LIT.with(|lit| lit.clone()) };
                    refused_1(v13)
                }
            }
        }
    }
}
fn command_line_argument_count_17() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_18(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_16(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_17();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US1 = if v4 {
        let v5: Rc<str> = command_line_argument_18(v0);
        US1::US1_Some(v5)
    } else {
        US1::US1_None
    };
    match v8 {
        US1::US1_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US1::US1_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_16(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn remote_of_19(mut v0: Rc<str>) -> US2 {
    let v1: bool = "manifest" == &*v0;
    if v1 {
        let v2: US0 = US0::US0_RemoteManifest;
        US2::US2_Some(v2)
    } else {
        let v4: bool = "plan" == &*v0;
        if v4 {
            let v5: US0 = US0::US0_RemotePlan;
            US2::US2_Some(v5)
        } else {
            let v7: bool = "sync" == &*v0;
            if v7 {
                let v8: US0 = US0::US0_RemoteSync;
                US2::US2_Some(v8)
            } else {
                let v10: bool = "unpack" == &*v0;
                if v10 {
                    let v11: US0 = US0::US0_RemoteUnpack;
                    US2::US2_Some(v11)
                } else {
                    US2::US2_None
                }
            }
        }
    }
}
fn remote_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_16(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    let mut v6: US2 = remote_of_19(v4.clone());
                    match v6 {
                        US2::US2_None => {
                            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown command "); } LIT.with(|lit| lit.clone()) }, v4));
                            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; check|manifest|plan|sync|unpack"); } LIT.with(|lit| lit.clone()) }));
                            refused_1(v10)
                        }
                        US2::US2_Some(v7) => {
                            let v7: US0 = v7;
                            command_ran_2(v7, v5)
                        }
                    }
                }
                _ => {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected a command: check|manifest|plan|sync|unpack"); } LIT.with(|lit| lit.clone()) };
                    refused_1(v14)
                }
            }
        }
        _ => {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected a command: check|manifest|plan|sync|unpack"); } LIT.with(|lit| lit.clone()) };
            refused_1(v17)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        remote_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_mirror_remote_run() -> i32 {
    closure0()()
}
use kit_mirror_local::SpiralShape_97df63ebabc3 as Mut0;
