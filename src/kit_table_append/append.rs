#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
struct Mut0 { l0: i32, l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_WholeSpi,
    US1_PlainTsv,
    US1_NotAppendable,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<UH0>, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_Outside,
    US2_InCell(Rc<str>),
    US2_AfterEscape(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_DeclaredHere,
    US3_OpenedFrom(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_Appended(Rc<UH0>, Rc<UH1>, Rc<str>),
    US4_AppendRefused(Rc<str>),
}
struct Mut1 { l0: Rc<str> }
fn refused_1(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table append: "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v0));
    eprintln!("{}", v2);
    2i32
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_2(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let v3: i32 = command_line_argument_count_3();
        let v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US0 = if v5 {
        let mut v6: Rc<str> = command_line_argument_4(v1);
        US0::US0_Some(v6)
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US0::US0_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10
        }
    }
}
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method7(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method7(v7, v9.clone()) {
            let mut v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method6(v16.clone(), v17.clone())
    }
}
fn cells_from_8(mut v0: i32, mut v1: i32) -> Rc<UH0> {
    let v2: bool = v1 <= v0;
    if v2 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v4: Rc<str> = argument_2(v0);
        let v5: i32 = v0.wrapping_add(1i32);
        let mut v6: Rc<UH0> = cells_from_8(v5, v1);
        Rc::new(UH0::UH0_Cons(v4, v6))
    }
}
fn file_exists_9(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method10(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn target_of_11(mut v0: Rc<str>) -> US1 {
    let v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v1.ends_with(&*v2);
    if v3 {
        US1::US1_WholeSpi
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".tsv"); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = v1.ends_with(&*v5);
        if v6 {
            US1::US1_PlainTsv
        } else {
            US1::US1_NotAppendable
        }
    }
}
fn method13(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn chars_from_14(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2.clone();
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: usize = ((v1) as usize);
            let mut v6: i32 = v1.wrapping_add(1i32);
            let v7: usize = ((v6) as usize);
            let v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
            let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8.clone(), v2.clone()));
            (v1, v2) = (v4, v9.clone());
            continue;
        }
    }
}
fn method15(mut v0: Rc<UH0>, mut v1: US2, mut v2: Rc<UH0>) -> (US2, Rc<UH0>) {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                let (mut v30, mut v31): (US2, Rc<UH0>) = match &v1 {
                    US2::US2_AfterEscape(v23) => {
                        let v23: Rc<str> = v23.clone();
                        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v3));
                        let v25: US2 = US2::US2_InCell(v24.clone());
                        (v25.clone(), v2.clone())
                    }
                    US2::US2_InCell(v11) => {
                        let mut v11: Rc<str> = v11.clone();
                        let v12: bool = &*v3 == "\\";
                        if v12 {
                            let v13: US2 = US2::US2_AfterEscape(v11.clone());
                            (v13.clone(), v2.clone())
                        } else {
                            let v14: bool = &*v3 == "\"";
                            if v14 {
                                let v15: US2 = US2::US2_Outside;
                                let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11.clone(), v2.clone()));
                                (v15.clone(), v16.clone())
                            } else {
                                let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v3));
                                let v18: US2 = US2::US2_InCell(v17.clone());
                                (v18.clone(), v2.clone())
                            }
                        }
                    }
                    US2::US2_Outside => {
                        let v5: bool = &*v3 == "\"";
                        if v5 {
                            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v7: US2 = US2::US2_InCell(v6.clone());
                            (v7.clone(), v2.clone())
                        } else {
                            let v8: US2 = US2::US2_Outside;
                            (v8.clone(), v2.clone())
                        }
                    }
                };
                (v0, v1, v2) = (v4.clone(), v30.clone(), v31.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return (v1.clone(), v2.clone());
            }
        }
    }
}
fn method16(mut v0: Rc<UH0>, mut v1: bool, mut v2: Rc<UH0>) -> (bool, Rc<UH0>) {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                let (mut v19, mut v20): (bool, Rc<UH0>) = if v1 {
                    let v5: bool = &*v3 == "n";
                    let v13: Rc<str> = if v5 {
                        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        v6.clone()
                    } else {
                        let v7: bool = &*v3 == "t";
                        if v7 {
                            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v8.clone()
                        } else {
                            let mut v9: bool = &*v3 == "r";
                            if v9 {
                                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                v10.clone()
                            } else {
                                v3.clone()
                            }
                        }
                    };
                    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v2.clone()));
                    (false, v14.clone())
                } else {
                    let v15: bool = &*v3 == "\\";
                    if v15 {
                        (true, v2.clone())
                    } else {
                        let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v2.clone()));
                        (false, v16.clone())
                    }
                };
                (v0, v1, v2) = (v4.clone(), v19, v20.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return (v1, v2.clone());
            }
        }
    }
}
fn method17(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method18(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<UH0> = v4.clone();
            let v5: Rc<UH1> = method18(v0.clone(), v4, v2.clone());
            let mut v6: Rc<UH0> = split_list_5(v0, v3);
            Rc::new(UH1::UH1_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method19(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<UH0> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> (Rc<UH0>, Rc<UH1>) {
    let (mut v121, mut v122): (Rc<UH0>, Rc<UH1>) = match &*v0 {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<UH0> = v4.clone();
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl columns () = Columns ["); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v3.starts_with(&*v5);
            let (mut v115, mut v116): (Rc<UH0>, Rc<UH1>) = if v6 {
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
                let v8: Option<usize> = v3.find(&*v7);
                let mut v9: Option<usize> = method13(v8.clone());
                let mut v10: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v11: Option<i32> = v9.map(|x| v10(x));
                let v13: i32 = v11.unwrap_or(-1i32);
                let v14: bool = v13 < 0i32;
                let v32: Rc<UH0> = if v14 {
                    { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                } else {
                    let v16: i32 = (v3.clone().len() as i32);
                    let v17: i32 = v16.wrapping_sub(v13);
                    let v18: usize = ((v13) as usize);
                    let v19: i32 = v13.wrapping_add(v17);
                    let v20: usize = ((v19) as usize);
                    let v21: Rc<str> = Rc::<str>::from(&(v3)[v18..v20]);
                    let v22: i32 = (v21.clone().len() as i32);
                    let v23: i32 = v22.wrapping_sub(1i32);
                    let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v25: Rc<UH0> = chars_from_14(v21.clone(), v23, v24.clone());
                    let v26: US2 = US2::US2_Outside;
                    let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let (mut v28, mut v29): (US2, Rc<UH0>) = method15(v25.clone(), v26.clone(), v27.clone());
                    let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    method6(v29.clone(), v30.clone())
                };
                (v32.clone(), v2.clone())
            } else {
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl row_"); } LIT.with(|lit| lit.clone()) };
                let v34: bool = v3.starts_with(&*v33);
                let (mut v113, mut v114): (Rc<UH0>, Rc<UH1>) = if v34 {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
                    let v36: Option<usize> = v3.find(&*v35);
                    let v37: Option<usize> = method13(v36.clone());
                    let mut v38: Rc<dyn Fn((usize)) -> i32> = closure1();
                    let mut v39: Option<i32> = v37.map(|x| v38(x));
                    let v41: i32 = v39.unwrap_or(-1i32);
                    let v42: bool = v41 < 0i32;
                    let v60: Rc<UH0> = if v42 {
                        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                    } else {
                        let v44: i32 = (v3.clone().len() as i32);
                        let v45: i32 = v44.wrapping_sub(v41);
                        let v46: usize = ((v41) as usize);
                        let v47: i32 = v41.wrapping_add(v45);
                        let v48: usize = ((v47) as usize);
                        let v49: Rc<str> = Rc::<str>::from(&(v3)[v46..v48]);
                        let v50: i32 = (v49.clone().len() as i32);
                        let v51: i32 = v50.wrapping_sub(1i32);
                        let mut v52: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v53: Rc<UH0> = chars_from_14(v49.clone(), v51, v52.clone());
                        let v54: US2 = US2::US2_Outside;
                        let v55: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let (mut v56, mut v57): (US2, Rc<UH0>) = method15(v53.clone(), v54.clone(), v55.clone());
                        let v58: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        method6(v57.clone(), v58.clone())
                    };
                    let v61: Rc<UH1> = Rc::new(UH1::UH1_Cons(v60.clone(), v2.clone()));
                    (v1.clone(), v61.clone())
                } else {
                    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl table () = Table \""); } LIT.with(|lit| lit.clone()) };
                    let v63: bool = v3.starts_with(&*v62);
                    let (mut v111, mut v112): (Rc<UH0>, Rc<UH1>) = if v63 {
                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                        let v65: Option<usize> = v3.find(&*v64);
                        let v66: Option<usize> = method13(v65.clone());
                        let mut v67: Rc<dyn Fn((usize)) -> i32> = closure1();
                        let mut v68: Option<i32> = v66.map(|x| v67(x));
                        let v70: i32 = v68.unwrap_or(-1i32);
                        let v71: i32 = v70.wrapping_add(1i32);
                        let v72: i32 = (v3.clone().len() as i32);
                        let v73: i32 = v72.wrapping_sub(v71);
                        let v74: i32 = v73.wrapping_sub(1i32);
                        let v75: usize = ((v71) as usize);
                        let v76: i32 = v71.wrapping_add(v74);
                        let v77: usize = ((v76) as usize);
                        let v78: Rc<str> = Rc::<str>::from(&(v3)[v75..v77]);
                        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v80: i32 = (v78.clone().len() as i32);
                        let v81: i32 = v80.wrapping_sub(1i32);
                        let v82: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v83: Rc<UH0> = chars_from_14(v78.clone(), v81, v82.clone());
                        let v84: bool = false;
                        let mut v85: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let (mut v86, mut v87): (bool, Rc<UH0>) = method16(v83.clone(), v84, v85.clone());
                        let v88: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v89: Rc<UH0> = method6(v87.clone(), v88.clone());
                        let v90: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v91: i32 = 0i32;
                        let v92: i32 = method17(v90.clone(), v89.clone(), v91);
                        let v93: Rc<Vec<Rc<str>>> = Rc::new(v90.borrow().clone());
                        let v94: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v93).as_ref().clone()));
                        let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v96: Rc<str> = Rc::<str>::from((*v94.borrow()).join(&*v95));
                        let v97: Rc<UH0> = split_list_5(v79.clone(), v96.clone());
                        let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                        let v99: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v100: Rc<UH1> = method18(v98.clone(), v97.clone(), v99.clone());
                        let (mut v109, mut v110): (Rc<UH0>, Rc<UH1>) = match &*v100 {
                            UH1::UH1_Cons(v101, v102) => {
                                let v101: Rc<UH0> = v101.clone();
                                let v102: Rc<UH1> = v102.clone();
                                let v103: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                                let v104: Rc<UH1> = method19(v102.clone(), v103.clone());
                                (v101.clone(), v104.clone())
                            }
                            _ => {
                                let (mut v107, mut v108): (Rc<UH0>, Rc<UH1>) = match &*v100 {
                                    UH1::UH1_Nil => {
                                        let v105: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                        let v106: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                                        (v105.clone(), v106.clone())
                                    }
                                    _ => unreachable!(),
                                };
                                (v107.clone(), v108.clone())
                            }
                        };
                        (v109.clone(), v110.clone())
                    } else {
                        (v1.clone(), v2.clone())
                    };
                    (v111.clone(), v112.clone())
                };
                (v113.clone(), v114.clone())
            };
            method12(v4.clone(), v115.clone(), v116.clone())
        }
        _ => {
            let (mut v119, mut v120): (Rc<UH0>, Rc<UH1>) = match &*v0 {
                UH0::UH0_Nil => {
                    (v1.clone(), v2.clone())
                }
                _ => unreachable!(),
            };
            (v119.clone(), v120.clone())
        }
    };
    (v121.clone(), v122.clone())
}
fn method21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let v4: Rc<UH0> = method21(v3, v1.clone());
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4.clone()))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let v4: Rc<UH1> = method22(v3, v1.clone());
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<UH0> = split_list_5(v5, v2);
            Rc::new(UH1::UH1_Cons(v6, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn table_of_tsv_20(mut v0: Rc<str>) -> (Rc<UH0>, Rc<UH1>) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_5(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method21(v5, v6);
    let (mut v18, mut v19): (Rc<UH0>, Rc<UH1>) = match &*v7 {
        UH0::UH0_Cons(v8, v9) => {
            let v8: Rc<str> = v8.clone();
            let mut v9: Rc<UH0> = v9.clone();
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<UH0> = split_list_5(v10, v8);
            let v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v13: Rc<UH1> = method22(v9, v12);
            (v11, v13)
        }
        _ => {
            let (mut v16, mut v17): (Rc<UH0>, Rc<UH1>) = match &*v7 {
                UH0::UH0_Nil => {
                    let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v15: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    (v14, v15)
                }
                _ => unreachable!(),
            };
            (v16, v17)
        }
    };
    (v18, v19)
}
fn method24(mut v0: Rc<UH0>, mut v1: US3) -> US3 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open "); } LIT.with(|lit| lit.clone()) };
                let v5: bool = v2.starts_with(&*v4);
                let v13: US3 = if v5 {
                    let mut v6: i32 = (v2.clone().len() as i32);
                    let v7: i32 = v6.wrapping_sub(5i32);
                    let v8: usize = ((5i32) as usize);
                    let mut v9: i32 = 5i32.wrapping_add(v7);
                    let mut v10: usize = ((v9) as usize);
                    let mut v11: Rc<str> = Rc::<str>::from(&(v2)[v8..v10]);
                    US3::US3_OpenedFrom(v11.clone())
                } else {
                    v1.clone()
                };
                (v0, v1) = (v3.clone(), v13.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn union_of_23(mut v0: Rc<str>) -> US3 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_5(v1, v4);
    let mut v6: US3 = US3::US3_DeclaredHere;
    method24(v5, v6)
}
fn method26(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method28(mut v0: Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_27(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method28(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn cell_plain_30(mut v0: Rc<str>) -> bool {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.contains(&*v1);
    let v3: bool = v2 == false;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v0.contains(&*v4);
        let mut v6: bool = v5 == false;
        v6
    } else {
        false
    }
}
fn method29(mut v0: Rc<UH0>, mut v1: bool) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v5: bool = if v1 {
                    cell_plain_30(v2.clone())
                } else {
                    false
                };
                (v0, v1) = (v3.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method31(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<UH0> = v2.clone();
            let v3: Rc<UH1> = v3.clone();
            let v4: Rc<UH1> = method31(v3, v1.clone());
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method32(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<UH0> = v2.clone();
            let v3: Rc<UH1> = v3.clone();
            let v4: Rc<UH0> = method32(v3, v1.clone());
            let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let mut v6: i32 = 0i32;
            let v7: i32 = method17(v5.clone(), v2, v6);
            let v8: Rc<Vec<Rc<str>>> = Rc::new(v5.borrow().clone());
            let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
            Rc::new(UH0::UH0_Cons(v11, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn appended_to_25(mut v0: US1, mut v1: US3, mut v2: Rc<UH0>, mut v3: Rc<UH1>, mut v4: Rc<UH0>) -> US4 {
    let v5: i32 = 0i32;
    let mut v6: i32 = method26(v2.clone(), v5);
    let v7: i32 = 0i32;
    let v8: i32 = method26(v4.clone(), v7);
    let mut v9: bool = v6 == 0i32;
    if v9 {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the table has no header row"); } LIT.with(|lit| lit.clone()) };
        US4::US4_AppendRefused(v10.clone())
    } else {
        let v12: bool = v8 != v6;
        if v12 {
            let v13: Rc<str> = format_real_27(v8);
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the row has "); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cells but the table has "); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
            let v18: Rc<str> = format_real_27(v6);
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" columns"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
            US4::US4_AppendRefused(v21.clone())
        } else {
            let v23: bool = true;
            let v24: bool = method29(v4.clone(), v23);
            let v25: bool = v24 == false;
            if v25 {
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a cell holds a tab or a newline"); } LIT.with(|lit| lit.clone()) };
                US4::US4_AppendRefused(v26.clone())
            } else {
                let v28: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v29: Rc<UH1> = Rc::new(UH1::UH1_Cons(v4.clone(), v28.clone()));
                let v30: Rc<UH1> = method31(v3.clone(), v29.clone());
                match &v0 {
                    US1::US1_NotAppendable => {
                        let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append writes .spi or .tsv tables"); } LIT.with(|lit| lit.clone()) };
                        US4::US4_AppendRefused(v94.clone())
                    }
                    US1::US1_PlainTsv => {
                        let v76: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v77: Rc<UH0> = method32(v30.clone(), v76.clone());
                        let v78: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v79: i32 = 0i32;
                        let v80: i32 = method17(v78.clone(), v2.clone(), v79);
                        let v81: Rc<Vec<Rc<str>>> = Rc::new(v78.borrow().clone());
                        let v82: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v81).as_ref().clone()));
                        let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                        let v84: Rc<str> = Rc::<str>::from((*v82.borrow()).join(&*v83));
                        let mut v85: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        v85.borrow_mut().push(v84);
                        let v86: i32 = 1i32;
                        let v87: i32 = method17(v85.clone(), v77.clone(), v86);
                        let v88: Rc<Vec<Rc<str>>> = Rc::new(v85.borrow().clone());
                        let v89: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v88).as_ref().clone()));
                        let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v91: Rc<str> = Rc::<str>::from((*v89.borrow()).join(&*v90));
                        let v92: Rc<str> = Rc::<str>::from(format!("{}{}", v91, v90));
                        US4::US4_Appended(v2.clone(), v30.clone(), v92.clone())
                    }
                    US1::US1_WholeSpi => {
                        let v40: Rc<str> = match &v1 {
                            US3::US3_DeclaredHere => {
                                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion table_row = | Columns :: list string -> table_row | Row :: list string -> table_row"); } LIT.with(|lit| lit.clone()) };
                                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" | Table :: string -> table_row\n"); } LIT.with(|lit| lit.clone()) };
                                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion table_row = | Columns :: list string -> table_row | Row :: list string -> table_row | Table :: string -> table_row\n"); } LIT.with(|lit| lit.clone()) };
                                v33.clone()
                            }
                            US3::US3_OpenedFrom(v34) => {
                                let v34: Rc<str> = v34.clone();
                                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nopen "); } LIT.with(|lit| lit.clone()) };
                                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v34));
                                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
                                v38.clone()
                            }
                        };
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl table () = Table \""); } LIT.with(|lit| lit.clone()) };
                        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
                        let v43: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v44: Rc<UH0> = method32(v30.clone(), v43.clone());
                        let v45: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v46: i32 = 0i32;
                        let v47: i32 = method17(v45.clone(), v2.clone(), v46);
                        let v48: Rc<Vec<Rc<str>>> = Rc::new(v45.borrow().clone());
                        let v49: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v48).as_ref().clone()));
                        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                        let v51: Rc<str> = Rc::<str>::from((*v49.borrow()).join(&*v50));
                        let mut v52: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        v52.borrow_mut().push(v51);
                        let v53: i32 = 1i32;
                        let v54: i32 = method17(v52.clone(), v44.clone(), v53);
                        let v55: Rc<Vec<Rc<str>>> = Rc::new(v52.borrow().clone());
                        let v56: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v55).as_ref().clone()));
                        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v58: Rc<str> = Rc::<str>::from((*v56.borrow()).join(&*v57));
                        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                        let v61: Rc<str> = Rc::<str>::from(v58.replace(&*v59, &*v60));
                        let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                        let v64: Rc<str> = Rc::<str>::from(v61.replace(&*v62, &*v63));
                        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                        let v66: Rc<str> = Rc::<str>::from(v64.replace(&*v57, &*v65));
                        let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                        let mut v68: Rc<str> = Rc::<str>::from(v66.replace(&*v50, &*v67));
                        let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                        let v71: Rc<str> = Rc::<str>::from(v68.replace(&*v69, &*v70));
                        let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v71));
                        let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\n"); } LIT.with(|lit| lit.clone()) };
                        let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v73));
                        US4::US4_Appended(v2.clone(), v30.clone(), v74.clone())
                    }
                }
            }
        }
    }
}
fn method33(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<UH0> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn append_run_0() -> i32 {
    let mut v0: i32 = 3i32;
    let v1: Rc<str> = argument_2(v0);
    let v2: i32 = command_line_argument_count_3();
    let v3: i32 = v2.wrapping_add(1i32);
    let v4: i32 = 4i32;
    let v5: Rc<str> = argument_2(v4);
    let mut v6: bool = &*v5 == "--row-tsv";
    let v8: bool = if v6 {
        let v7: bool = v3 == 6i32;
        v7
    } else {
        false
    };
    let v15: Rc<UH0> = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let mut v10: i32 = 5i32;
        let mut v11: Rc<str> = argument_2(v10);
        split_list_5(v9.clone(), v11.clone())
    } else {
        let v13: i32 = 4i32;
        cells_from_8(v13, v3)
    };
    let v16: bool = &*v1 == "";
    let v18: bool = if v16 {
        true
    } else {
        match &*v15 {
            UH0::UH0_Nil => {
                true
            }
            _ => {
                false
            }
        }
    };
    if v18 {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: table append <table.spi|table.tsv> <cell>... | --row-tsv <line>"); } LIT.with(|lit| lit.clone()) };
        refused_1(v19.clone())
    } else {
        let v21: bool = file_exists_9(v1.clone());
        let v22: bool = v21 == false;
        if v22 {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": file not found"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v23));
            refused_1(v24.clone())
        } else {
            let v27: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v1);
            let v29: Vec<u8> = v27.unwrap();
            let v30: Vec<u8> = method10(v29.clone());
            let v32: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v30);
            let v34: std::string::String = v32.unwrap();
            let v36: Rc<str> = Rc::<str>::from(String::as_str(&v34));
            let v37: US1 = target_of_11(v1.clone());
            let mut v38: bool = match &v37 {
                US1::US1_WholeSpi => {
                    true
                }
                _ => {
                    false
                }
            };
            let (mut v52, mut v53): (Rc<UH0>, Rc<UH1>) = if v38 {
                let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v42: Rc<str> = Rc::<str>::from(v36.replace(&*v40, &*v41));
                let v43: Rc<UH0> = split_list_5(v39.clone(), v42.clone());
                let v44: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v45: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let (mut v46, mut v47): (Rc<UH0>, Rc<UH1>) = method12(v43.clone(), v44.clone(), v45.clone());
                let v48: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v49: Rc<UH1> = method19(v47.clone(), v48.clone());
                (v46.clone(), v49.clone())
            } else {
                let (mut v50, mut v51): (Rc<UH0>, Rc<UH1>) = table_of_tsv_20(v36.clone());
                (v50.clone(), v51.clone())
            };
            let v54: US3 = union_of_23(v36.clone());
            let v55: US4 = appended_to_25(v37.clone(), v54.clone(), v52.clone(), v53.clone(), v15.clone());
            match &v55 {
                US4::US4_AppendRefused(v56) => {
                    let v56: Rc<str> = v56.clone();
                    refused_1(v56.clone())
                }
                US4::US4_Appended(v58, v59, v60) => {
                    let v58: Rc<UH0> = v58.clone();
                    let v59: Rc<UH1> = v59.clone();
                    let v60: Rc<str> = v60.clone();
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
                    let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v61));
                    std::fs::write(&*v62, &*v60).unwrap();
                    let v65: Result<(), std::io::Error> = std::fs::rename(&*v62, &*v1);
                    let v66: bool = file_exists_9(v62.clone());
                    if v66 {
                        let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move "); } LIT.with(|lit| lit.clone()) };
                        let mut v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v62));
                        let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" over "); } LIT.with(|lit| lit.clone()) };
                        let v71: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v68, v69)), v1));
                        refused_1(v71.clone())
                    } else {
                        let v73: i32 = 0i32;
                        let v74: i32 = method33(v59.clone(), v73);
                        let v75: Rc<str> = format_real_27(v74);
                        let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table append ok rows="); } LIT.with(|lit| lit.clone()) };
                        let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v75));
                        let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" columns="); } LIT.with(|lit| lit.clone()) };
                        let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v78));
                        let v80: i32 = 0i32;
                        let v81: i32 = method26(v58.clone(), v80);
                        let v82: Rc<str> = format_real_27(v81);
                        let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v82));
                        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
                        let v86: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v83, v84)), v1));
                        println!("{}", v86);
                        0i32
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        append_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_append_run() -> i32 {
    closure0()()
}
