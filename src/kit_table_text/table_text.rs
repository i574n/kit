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
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<UH0>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_Outside,
    US0_InCell(Rc<str>),
    US0_AfterEscape(Rc<str>),
}
fn file_exists_1(mut v0: Rc<str>) -> bool {
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
fn table_problem_0(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = file_exists_1(v0.clone());
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": file not found"); } LIT.with(|lit| lit.clone()) }));
        v3
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        table_problem_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn quoted_as_tsv_6(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: bool = v1 >= v2;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v1.wrapping_add(1i32);
            let mut v6: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v7: i32 = v1.wrapping_rem(2i32);
            let v8: bool = v7 == 1i32;
            let v29: Rc<str> = if v8 {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v11: Rc<str> = Rc::<str>::from(v6.replace(&*v9, &*v10));
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v10));
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = Rc::<str>::from(v13.replace(&*v14, &*v10));
                v15
            } else {
                let v16: bool = &*v6 == "";
                let v18: bool = if v16 {
                    let v17: bool = v1 > 0i32;
                    v17
                } else {
                    false
                };
                let v21: bool = if v18 {
                    let v19: i32 = v2.wrapping_sub(1i32);
                    let v20: bool = v1 < v19;
                    v20
                } else {
                    false
                };
                if v21 {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    v22
                } else {
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v25: Rc<str> = Rc::<str>::from(v6.replace(&*v23, &*v24));
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = Rc::<str>::from(v25.replace(&*v26, &*v23));
                    v27
                }
            };
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v29));
            (v1, v3) = (v5, v30);
            continue;
        }
    }
}
fn csv_line_as_tsv_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = 0i32;
    let v4: i32 = (v2.clone().borrow().len() as i32);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    quoted_as_tsv_6(v2, v3, v4, v5)
}
fn tsv_lines_4(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v7));
            let mut v9: Rc<str> = csv_line_as_tsv_5(v8);
            let mut v10: i32 = v1.wrapping_add(1i32);
            let mut v11: bool = v1 == 0i32;
            let v14: Rc<str> = if v11 {
                v9
            } else {
                let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v9));
                v13
            };
            (v1, v2) = (v10, v14);
            continue;
        }
    }
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn method9(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method9(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method8(v16, v17)
    }
}
fn method11(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn chars_from_12(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: usize = ((v1) as usize);
            let mut v6: i32 = v1.wrapping_add(1i32);
            let v7: usize = ((v6) as usize);
            let v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
            let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v2));
            (v1, v2) = (v4, v9);
            continue;
        }
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: US0, mut v2: Rc<UH0>) -> (US0, Rc<UH0>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let (mut v30, mut v31): (US0, Rc<UH0>) = match v1 {
                    US0::US0_AfterEscape(v23) => {
                        let v23: Rc<str> = v23;
                        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v3));
                        let v25: US0 = US0::US0_InCell(v24);
                        (v25, v2)
                    }
                    US0::US0_InCell(v11) => {
                        let mut v11: Rc<str> = v11;
                        let mut v12: bool = &*v3 == "\\";
                        if v12 {
                            let v13: US0 = US0::US0_AfterEscape(v11);
                            (v13, v2)
                        } else {
                            let v14: bool = &*v3 == "\"";
                            if v14 {
                                let v15: US0 = US0::US0_Outside;
                                let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v2));
                                (v15, v16)
                            } else {
                                let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v3));
                                let v18: US0 = US0::US0_InCell(v17);
                                (v18, v2)
                            }
                        }
                    }
                    US0::US0_Outside => {
                        let v5: bool = &*v3 == "\"";
                        if v5 {
                            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v7: US0 = US0::US0_InCell(v6);
                            (v7, v2)
                        } else {
                            let v8: US0 = US0::US0_Outside;
                            (v8, v2)
                        }
                    }
                };
                (v0, v1, v2) = (v4, v30, v31);
                continue;
            }
            UH0::UH0_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: bool, mut v2: Rc<UH0>) -> (bool, Rc<UH0>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let (mut v19, mut v20): (bool, Rc<UH0>) = if v1 {
                    let v5: bool = &*v3 == "n";
                    let v13: Rc<str> = if v5 {
                        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        v6
                    } else {
                        let v7: bool = &*v3 == "t";
                        if v7 {
                            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v8
                        } else {
                            let mut v9: bool = &*v3 == "r";
                            if v9 {
                                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                v10
                            } else {
                                v3
                            }
                        }
                    };
                    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v2));
                    (false, v14)
                } else {
                    let v15: bool = &*v3 == "\\";
                    if v15 {
                        (true, v2)
                    } else {
                        let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v2));
                        (false, v16)
                    }
                };
                (v0, v1, v2) = (v4, v19, v20);
                continue;
            }
            UH0::UH0_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn method15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn method16(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH1> = method16(v0.clone(), v4, v2);
            let mut v6: Rc<UH0> = split_list_7(v0, v3);
            Rc::new(UH1::UH1_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method17(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<UH0> = v2;
                let v3: Rc<UH1> = v3;
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
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> (Rc<UH0>, Rc<UH1>) {
    let (mut v121, mut v122): (Rc<UH0>, Rc<UH1>) = match &*v0 {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<UH0> = v4.clone();
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl columns () = Columns ["); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v3.starts_with(&*v5);
            let (mut v115, mut v116): (Rc<UH0>, Rc<UH1>) = if v6 {
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
                let v8: Option<usize> = v3.find(&*v7);
                let mut v9: Option<usize> = method11(v8);
                let mut v10: Rc<dyn Fn((usize)) -> i32> = closure2();
                let mut v11: Option<i32> = v9.map(|x| v10(x));
                let v13: i32 = v11.unwrap_or(-1i32);
                let v14: bool = v13 < 0i32;
                let mut v32: Rc<UH0> = if v14 {
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
                    let v25: Rc<UH0> = chars_from_12(v21, v23, v24);
                    let v26: US0 = US0::US0_Outside;
                    let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let (mut v28, mut v29): (US0, Rc<UH0>) = method13(v25, v26, v27);
                    let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    method8(v29, v30)
                };
                (v32, v2)
            } else {
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl row_"); } LIT.with(|lit| lit.clone()) };
                let v34: bool = v3.starts_with(&*v33);
                let (mut v113, mut v114): (Rc<UH0>, Rc<UH1>) = if v34 {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
                    let v36: Option<usize> = v3.find(&*v35);
                    let v37: Option<usize> = method11(v36);
                    let mut v38: Rc<dyn Fn((usize)) -> i32> = closure2();
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
                        let v52: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v53: Rc<UH0> = chars_from_12(v49, v51, v52);
                        let v54: US0 = US0::US0_Outside;
                        let v55: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let (mut v56, mut v57): (US0, Rc<UH0>) = method13(v53, v54, v55);
                        let v58: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        method8(v57, v58)
                    };
                    let v61: Rc<UH1> = Rc::new(UH1::UH1_Cons(v60, v2));
                    (v1, v61)
                } else {
                    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl table () = Table \""); } LIT.with(|lit| lit.clone()) };
                    let v63: bool = v3.starts_with(&*v62);
                    let (mut v111, mut v112): (Rc<UH0>, Rc<UH1>) = if v63 {
                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                        let v65: Option<usize> = v3.find(&*v64);
                        let v66: Option<usize> = method11(v65);
                        let mut v67: Rc<dyn Fn((usize)) -> i32> = closure2();
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
                        let v83: Rc<UH0> = chars_from_12(v78, v81, v82);
                        let v84: bool = false;
                        let v85: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let (mut v86, mut v87): (bool, Rc<UH0>) = method14(v83, v84, v85);
                        let v88: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v89: Rc<UH0> = method8(v87, v88);
                        let v90: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v91: i32 = 0i32;
                        let v92: i32 = method15(v90.clone(), v89, v91);
                        let v93: Rc<Vec<Rc<str>>> = Rc::new(v90.borrow().clone());
                        let v94: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v93).as_ref().clone()));
                        let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v96: Rc<str> = Rc::<str>::from((*v94.borrow()).join(&*v95));
                        let v97: Rc<UH0> = split_list_7(v79, v96);
                        let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                        let v99: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v100: Rc<UH1> = method16(v98, v97, v99);
                        let (mut v109, mut v110): (Rc<UH0>, Rc<UH1>) = match &*v100 {
                            UH1::UH1_Cons(v101, v102) => {
                                let v101: Rc<UH0> = v101.clone();
                                let v102: Rc<UH1> = v102.clone();
                                let v103: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                                let v104: Rc<UH1> = method17(v102, v103);
                                (v101, v104)
                            }
                            _ => {
                                let (mut v107, mut v108): (Rc<UH0>, Rc<UH1>) = match Rc::unwrap_or_clone(v100) {
                                    UH1::UH1_Nil => {
                                        let v105: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                        let v106: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                                        (v105, v106)
                                    }
                                    _ => unreachable!(),
                                };
                                (v107, v108)
                            }
                        };
                        (v109, v110)
                    } else {
                        (v1, v2)
                    };
                    (v111, v112)
                };
                (v113, v114)
            };
            method10(v4, v115, v116)
        }
        _ => {
            let (mut v119, mut v120): (Rc<UH0>, Rc<UH1>) = match Rc::unwrap_or_clone(v0) {
                UH0::UH0_Nil => {
                    (v1, v2)
                }
                _ => unreachable!(),
            };
            (v119, v120)
        }
    };
    (v121, v122)
}
fn method18(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<UH0> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH0> = method18(v3, v1);
            let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let mut v6: i32 = 0i32;
            let v7: i32 = method15(v5.clone(), v2, v6);
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
fn tsv_of_text_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".csv"); } LIT.with(|lit| lit.clone()) };
    let v4: bool = v2.ends_with(&*v3);
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = 0i32;
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        tsv_lines_4(v6, v7, v8)
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let mut v11: bool = v2.ends_with(&*v10);
        if v11 {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = Rc::<str>::from(v1.replace(&*v13, &*v14));
            let v16: Rc<UH0> = split_list_7(v12.clone(), v15);
            let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v18: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let (mut v19, mut v20): (Rc<UH0>, Rc<UH1>) = method10(v16, v17, v18);
            let v21: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v22: Rc<UH1> = method17(v20, v21);
            let v23: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v24: Rc<UH0> = method18(v22, v23);
            let v25: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v26: i32 = 0i32;
            let v27: i32 = method15(v25.clone(), v19, v26);
            let v28: Rc<Vec<Rc<str>>> = Rc::new(v25.borrow().clone());
            let v29: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v28).as_ref().clone()));
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = Rc::<str>::from((*v29.borrow()).join(&*v30));
            let mut v32: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v32.borrow_mut().push(v31);
            let v33: i32 = 1i32;
            let v34: i32 = method15(v32.clone(), v24, v33);
            let v35: Rc<Vec<Rc<str>>> = Rc::new(v32.borrow().clone());
            let v36: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v35).as_ref().clone()));
            let v37: Rc<str> = Rc::<str>::from((*v36.borrow()).join(&*v12));
            v37
        } else {
            v1
        }
    }
}
fn method19(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn table_tsv_2(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = file_exists_1(v0.clone());
    if v1 {
        let v3: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let v5: Vec<u8> = v3.unwrap();
        let mut v6: Vec<u8> = method19(v5);
        let v8: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v6);
        let mut v10: std::string::String = v8.unwrap();
        let mut v12: Rc<str> = Rc::<str>::from(String::as_str(&v10));
        tsv_of_text_3(v0, v12)
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v14
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        table_tsv_2(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method22(v3, v1);
            let v5: bool = &*v2 != "";
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
fn method23(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH1> = method23(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<UH0> = split_list_7(v5, v2);
            Rc::new(UH1::UH1_Cons(v6, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn table_of_tsv_21(mut v0: Rc<str>) -> (Rc<UH0>, Rc<UH1>) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_7(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method22(v5, v6);
    let (mut v18, mut v19): (Rc<UH0>, Rc<UH1>) = match &*v7 {
        UH0::UH0_Cons(v8, v9) => {
            let v8: Rc<str> = v8.clone();
            let mut v9: Rc<UH0> = v9.clone();
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<UH0> = split_list_7(v10, v8);
            let mut v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v13: Rc<UH1> = method23(v9, v12);
            (v11, v13)
        }
        _ => {
            let (mut v16, mut v17): (Rc<UH0>, Rc<UH1>) = match Rc::unwrap_or_clone(v7) {
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
fn spi_of_tsv_20(mut v0: Rc<str>) -> Rc<str> {
    let (mut v1, mut v2): (Rc<UH0>, Rc<UH1>) = table_of_tsv_21(v0);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH0> = method18(v2, v3);
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v6: i32 = 0i32;
    let v7: i32 = method15(v5.clone(), v1, v6);
    let v8: Rc<Vec<Rc<str>>> = Rc::new(v5.borrow().clone());
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
    let mut v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v12.borrow_mut().push(v11);
    let v13: i32 = 1i32;
    let v14: i32 = method15(v12.clone(), v4, v13);
    let v15: Rc<Vec<Rc<str>>> = Rc::new(v12.borrow().clone());
    let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(v18.replace(&*v19, &*v20));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = Rc::<str>::from(v21.replace(&*v22, &*v23));
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(v24.replace(&*v17, &*v25));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = Rc::<str>::from(v26.replace(&*v10, &*v27));
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = Rc::<str>::from(v28.replace(&*v29, &*v30));
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nopen table_rows\ninl table () = Table \""); } LIT.with(|lit| lit.clone()) }, v31));
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\n"); } LIT.with(|lit| lit.clone()) }));
    v33
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        spi_of_tsv_20(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_problem(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_table_tsv(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
pub fn kit_table_spi_of_tsv(v0: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0))
}
pub type SpiralShape_652accfc6a8d = Mut0;
