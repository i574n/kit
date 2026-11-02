#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<UH0>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_DeclaredHere,
    US0_OpenedFrom(Rc<str>),
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_TableQuery,
    US2_TableDiff,
    US2_TableToSpi,
    US2_TableUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_StepTaken(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool),
    US3_StepRejected(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(u64),
    US4_None,
}
fn rows_kept_2(mut v0: Rc<str>, mut v1: Rc<str>) -> (bool, Rc<str>) {
    let (mut v2, mut v3): (bool, Rc<str>) = kit_table_filter::kit_table_filter_kept(&*v0, &*v1);
    (v2, v3)
}
fn measured_query_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> i32 {
    let (mut v9, mut v10): (bool, Rc<str>) = rows_kept_2(v0.clone(), v1);
    let v11: i32 = kit_table_measures::kit_table_measured(&*v0, &*v3, &*v6, v5, v8, v9, &*v10);
    v11
}
fn plain_query_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> i32 {
    let v9: i32 = kit_table_plain::kit_table_plain(&*v0, &*v1, &*v2, &*v3, v5, v8);
    v9
}
fn row_cells_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3.clone();
        } else {
            let v5: i32 = v2.wrapping_sub(1i32);
            let v6: Rc<str> = std::rc::Rc::<str>::from((v0).get(v1 as usize).and_then(|cells| cells.get(v2 as usize)).map(|cell| &**cell).unwrap_or(""));
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6.clone(), v3.clone()));
            (v0, v1, v2, v3) = (v0.clone(), v1, v5, v7.clone());
            continue;
        }
    }
}
fn cells_in_row_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<UH0> {
    let v2: i32 = (v0).get(v1 as usize).map_or(0, |r| r.len()) as i32;
    let mut v3: i32 = v2.wrapping_sub(1i32);
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    row_cells_7(v0.clone(), v1, v3, v4.clone())
}
fn data_rows_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v3: bool = v1 < 1i32;
        if v3 {
            return v2.clone();
        } else {
            let mut v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<UH0> = cells_in_row_6(v0.clone(), v1);
            let v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5.clone(), v2.clone()));
            (v0, v1, v2) = (v0.clone(), v4, v6.clone());
            continue;
        }
    }
}
fn typed_table_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>) -> (Rc<UH0>, Rc<UH1>) {
    let v1: i32 = 0i32;
    let v2: Rc<UH0> = cells_in_row_6(v0.clone(), v1);
    let mut v3: i32 = ((v0).len().max(1) - 1) as i32;
    let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH1> = data_rows_8(v0, v3, v4);
    (v2, v5)
}
fn method11(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_10(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method11(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method12(v3, v1.clone());
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(v2.replace(&*v5, &*v6));
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v9));
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v8, v10)), v8));
            Rc::new(UH0::UH0_Cons(v12, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method13(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v4.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method9(mut v0: Rc<UH1>, mut v1: i32, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> (i32, Rc<UH0>, Rc<UH0>) {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v4, v5) => {
                let mut v4: Rc<UH0> = v4.clone();
                let v5: Rc<UH1> = v5.clone();
                let v6: Rc<str> = format_real_10(v1);
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("row_"); } LIT.with(|lit| lit.clone()) };
                let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v6));
                let v9: i32 = v1.wrapping_add(1i32);
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v8));
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Row "); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
                let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v15: Rc<UH0> = method12(v4.clone(), v14.clone());
                let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v17: i32 = 0i32;
                let v18: i32 = method13(v16.clone(), v15.clone(), v17);
                let v19: Rc<Vec<Rc<str>>> = Rc::new(v16.borrow().clone());
                let v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v19).as_ref().clone()));
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                let v22: Rc<str> = Rc::<str>::from((*v20.borrow()).join(&*v21));
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[ "); } LIT.with(|lit| lit.clone()) };
                let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v22));
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ]"); } LIT.with(|lit| lit.clone()) };
                let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v13, Rc::<str>::from(format!("{}{}", v24, v25))));
                let v28: Rc<UH0> = Rc::new(UH0::UH0_Cons(v27.clone(), v2.clone()));
                let v29: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8.clone(), v3.clone()));
                (v0, v1, v2, v3) = (v5.clone(), v9, v28.clone(), v29.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return (v1, v2.clone(), v3.clone());
            }
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method15(v3, v1.clone());
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ()"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v5));
            Rc::new(UH0::UH0_Cons(v6, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method16(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<UH0> = v2.clone();
                let mut v3: Rc<UH1> = v3.clone();
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method17(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn to_spi_run_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let mut v3: Rc<str> = kit_table_text::kit_table_problem(&*v0);
    let mut v4: bool = &*v3 != "";
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v3));
        eprintln!("{}", v6);
        2i32
    } else {
        let v7: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v0).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
        let (mut v8, mut v9): (Rc<UH0>, Rc<UH1>) = typed_table_5(v7.clone());
        let v10: bool = &*v2 == "";
        let v13: US0 = if v10 {
            US0::US0_DeclaredHere
        } else {
            US0::US0_OpenedFrom(v2.clone())
        };
        let v14: i32 = 1i32;
        let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let (mut v17, mut v18, mut v19): (i32, Rc<UH0>, Rc<UH0>) = method9(v9.clone(), v14, v15.clone(), v16.clone());
        let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v21: Rc<UH0> = method12(v8.clone(), v20.clone());
        let v22: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v23: i32 = 0i32;
        let v24: i32 = method13(v22.clone(), v21.clone(), v23);
        let v25: Rc<Vec<Rc<str>>> = Rc::new(v22.borrow().clone());
        let v26: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v25).as_ref().clone()));
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = Rc::<str>::from((*v26.borrow()).join(&*v27));
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[ "); } LIT.with(|lit| lit.clone()) };
        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v28));
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ]"); } LIT.with(|lit| lit.clone()) };
        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl columns () = Columns "); } LIT.with(|lit| lit.clone()) };
        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v32));
        let v35: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v36: Rc<UH0> = method14(v18.clone(), v35.clone());
        let mut v37: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        v37.borrow_mut().push(v34);
        let v38: i32 = 1i32;
        let v39: i32 = method13(v37.clone(), v36.clone(), v38);
        let v40: Rc<Vec<Rc<str>>> = Rc::new(v37.borrow().clone());
        let v41: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v40).as_ref().clone()));
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v43: Rc<str> = Rc::<str>::from((*v41.borrow()).join(&*v42));
        let v50: Rc<str> = match &v13 {
            US0::US0_DeclaredHere => {
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion table_row = | Columns :: list string -> table_row | Row :: list string -> table_row\n"); } LIT.with(|lit| lit.clone()) };
                v44.clone()
            }
            US0::US0_OpenedFrom(v45) => {
                let v45: Rc<str> = v45.clone();
                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nopen "); } LIT.with(|lit| lit.clone()) };
                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v46, v45)), v42));
                v48.clone()
            }
        };
        let v52: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v50, v43)), v42));
        let v53: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v54: Rc<UH0> = method14(v19.clone(), v53.clone());
        let v55: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v56: Rc<UH0> = method15(v54.clone(), v55.clone());
        let v57: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v58: i32 = 0i32;
        let v59: i32 = method13(v57.clone(), v56.clone(), v58);
        let v60: Rc<Vec<Rc<str>>> = Rc::new(v57.borrow().clone());
        let v61: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v60).as_ref().clone()));
        let v62: Rc<str> = Rc::<str>::from((*v61.borrow()).join(&*v27));
        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl rows () : list table_row = [ "); } LIT.with(|lit| lit.clone()) };
        let v67: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v52, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v63, v62)), v31)))), v42));
        std::fs::write(&*v1, &*v67).unwrap();
        let v69: i32 = 0i32;
        let v70: i32 = method16(v9.clone(), v69);
        let v71: Rc<str> = std::rc::Rc::<str>::from((v70).to_string().as_str());
        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table to-spi ok rows="); } LIT.with(|lit| lit.clone()) };
        let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v71));
        let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" columns="); } LIT.with(|lit| lit.clone()) };
        let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v74));
        let v76: i32 = 0i32;
        let v77: i32 = method17(v8.clone(), v76);
        let v78: Rc<str> = std::rc::Rc::<str>::from((v77).to_string().as_str());
        let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v78));
        let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
        let v82: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v79, v80)), v1));
        println!("{}", v82);
        0i32
    }
}
fn command_line_argument_count_19() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_20(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_18(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_19();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US1 = if v5 {
        let v6: Rc<str> = command_line_argument_20(v1);
        US1::US1_Some(v6)
    } else {
        US1::US1_None
    };
    match &v9 {
        US1::US1_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US1::US1_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10
        }
    }
}
fn argument_count_21() -> i32 {
    let mut v0: i32 = command_line_argument_count_19();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn no_request_22() -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v0.clone(), v0.clone(), v0.clone(), 20u64, v0.clone(), v0, false)
}
fn each_prefixed_25(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).map(|item| format!("{}{}", &*v0, item.trim())).collect::<Vec<_>>().join(",").as_str());
    v2.clone()
}
fn method27(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_26(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method27(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US4> = closure1();
    let mut v4: Option<US4> = v2.map(|x| v3(x));
    let v6: US4 = US4::US4_None;
    let v7: US4 = v6.clone();
    let v8: US4 = v4.unwrap_or(v7);
    match &v8 {
        US4::US4_None => {
            0u64
        }
        US4::US4_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn option_applied_24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: bool, mut v8: Rc<str>, mut v9: Rc<str>) -> US3 {
    let v10: bool = &*v8 == "--where";
    if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
        let v12: bool = v9.contains(&*v11);
        let v15: bool = if v12 {
            true
        } else {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
            let v14: bool = v9.contains(&*v13);
            v14
        };
        let v18: bool = if v15 {
            true
        } else {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
            let v17: bool = v9.contains(&*v16);
            v17
        };
        if v18 {
            let v19: bool = &*v0 == "";
            let v23: Rc<str> = if v19 {
                v9.clone()
            } else {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v22: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v20, &*v9, &*v21, &*v21].concat());
                v22.clone()
            };
            US3::US3_StepTaken(v23.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
        } else {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--where expects col=value (or col!=value, col>value, col>=value, col<value, col<=value), got "); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v9, &*v26, &*v26, &*v26].concat());
            US3::US3_StepRejected(v27.clone())
        }
    } else {
        let v30: bool = &*v8 == "--cols";
        if v30 {
            US3::US3_StepTaken(v0.clone(), v9.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
        } else {
            let v32: bool = &*v8 == "--group";
            if v32 {
                US3::US3_StepTaken(v0.clone(), v1.clone(), v9.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
            } else {
                let v34: bool = &*v8 == "--key";
                if v34 {
                    US3::US3_StepTaken(v0.clone(), v1.clone(), v2.clone(), v9.clone(), v4, v5.clone(), v6.clone(), v7)
                } else {
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--mean|--sd|--median|--sum|--min|--max|--count"); } LIT.with(|lit| lit.clone()) };
                    let mut v37: bool = v36.split("|").any(|item| item == &*v8);
                    if v37 {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        let v39: Rc<str> = std::rc::Rc::<str>::from((v8).trim_start_matches(&*v38));
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v42: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v40, &*v41, &*v41, &*v41].concat());
                        let v43: Rc<str> = each_prefixed_25(v42.clone(), v9.clone());
                        let v44: bool = &*v5 == "";
                        let v49: Rc<str> = if v44 {
                            v43.clone()
                        } else {
                            let v45: bool = &*v43 == "";
                            if v45 {
                                v5.clone()
                            } else {
                                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                let v47: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v46, &*v43, &*v41, &*v41].concat());
                                v47.clone()
                            }
                        };
                        US3::US3_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v49.clone(), v6.clone(), v7)
                    } else {
                        let v51: bool = &*v8 == "--latest-by";
                        if v51 {
                            US3::US3_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v9.clone(), v7)
                        } else {
                            let v53: bool = &*v8 == "--limit";
                            if v53 {
                                let v54: u64 = decimal_u64_26(v9.clone());
                                let v55: bool = v54 == 0u64;
                                if v55 {
                                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--limit expects a positive integer"); } LIT.with(|lit| lit.clone()) };
                                    US3::US3_StepRejected(v56.clone())
                                } else {
                                    US3::US3_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v54, v5.clone(), v6.clone(), v7)
                                }
                            } else {
                                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v62: Rc<str> = std::rc::Rc::<str>::from([&*v60, &*v8, &*v61, &*v61, &*v61].concat());
                                US3::US3_StepRejected(v62.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
fn options_from_23(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: bool) -> US3 {
    loop {
        let v10: bool = v0 < v1;
        if v10 {
            let v11: Rc<str> = argument_18(v0);
            let v12: bool = &*v11 == "--quiet";
            if v12 {
                let v13: i32 = v0.wrapping_add(1i32);
                let v14: bool = true;
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v13, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v14);
                continue;
            } else {
                let v16: i32 = v0.wrapping_add(1i32);
                let v17: bool = v16 < v1;
                if v17 {
                    let v18: Rc<str> = argument_18(v0);
                    let v19: Rc<str> = argument_18(v16);
                    let v20: US3 = option_applied_24(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v9, v18.clone(), v19.clone());
                    match &v20 {
                        US3::US3_StepRejected(v31) => {
                            let v31: Rc<str> = v31.clone();
                            return US3::US3_StepRejected(v31.clone());
                        }
                        US3::US3_StepTaken(v21, v22, v23, v24, v25, v26, v27, v28) => {
                            let v21: Rc<str> = v21.clone();
                            let v22: Rc<str> = v22.clone();
                            let v23: Rc<str> = v23.clone();
                            let v24: Rc<str> = v24.clone();
                            let v25: u64 = *v25;
                            let v26: Rc<str> = v26.clone();
                            let v27: Rc<str> = v27.clone();
                            let v28: bool = *v28;
                            let v29: i32 = v0.wrapping_add(2i32);
                            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v29, v1, v21.clone(), v22.clone(), v23.clone(), v24.clone(), v25, v26.clone(), v27.clone(), v28);
                            continue;
                        }
                    }
                } else {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = argument_18(v0);
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) };
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v39: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v36, &*v37, &*v38, &*v38].concat());
                    return US3::US3_StepRejected(v39.clone());
                }
            }
        } else {
            return US3::US3_StepTaken(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v9);
        }
    }
}
fn table_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let v1: Rc<str> = argument_18(v0);
    let v2: bool = &*v1 == "query";
    let v11: US2 = if v2 {
        US2::US2_TableQuery
    } else {
        let mut v4: bool = &*v1 == "diff";
        if v4 {
            US2::US2_TableDiff
        } else {
            let v6: bool = &*v1 == "to-spi";
            if v6 {
                US2::US2_TableToSpi
            } else {
                US2::US2_TableUnknown
            }
        }
    };
    match &v11 {
        US2::US2_TableDiff => {
            let v68: i32 = argument_count_21();
            let v69: bool = v68 < 5i32;
            if v69 {
                let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index)"); } LIT.with(|lit| lit.clone()) };
                let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index)"); } LIT.with(|lit| lit.clone()) };
                eprintln!("{}", v72);
                2i32
            } else {
                let v73: i32 = 5i32;
                let v74: i32 = argument_count_21();
                let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_22();
                let v83: US3 = options_from_23(v73, v74, v75.clone(), v76.clone(), v77.clone(), v78.clone(), v79, v80.clone(), v81.clone(), v82);
                match &v83 {
                    US3::US3_StepRejected(v97) => {
                        let v97: Rc<str> = v97.clone();
                        let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                        let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v97));
                        eprintln!("{}", v99);
                        2i32
                    }
                    US3::US3_StepTaken(v84, v85, v86, v87, v88, v89, v90, v91) => {
                        let v84: Rc<str> = v84.clone();
                        let v85: Rc<str> = v85.clone();
                        let v86: Rc<str> = v86.clone();
                        let v87: Rc<str> = v87.clone();
                        let v88: u64 = *v88;
                        let v89: Rc<str> = v89.clone();
                        let v90: Rc<str> = v90.clone();
                        let v91: bool = *v91;
                        let v92: i32 = 3i32;
                        let v93: Rc<str> = argument_18(v92);
                        let v94: i32 = 4i32;
                        let v95: Rc<str> = argument_18(v94);
                        let v96: i32 = kit_table_diff::kit_table_diff_run(&*v93, &*v95, &*v87, &*v85, &*v90, v88);
                        v96
                    }
                }
            }
        }
        US2::US2_TableQuery => {
            let v12: i32 = argument_count_21();
            let v13: bool = v12 < 4i32;
            if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index)"); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index)"); } LIT.with(|lit| lit.clone()) };
                eprintln!("{}", v16);
                2i32
            } else {
                let v17: i32 = 4i32;
                let v18: i32 = argument_count_21();
                let (mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_22();
                let v27: US3 = options_from_23(v17, v18, v19.clone(), v20.clone(), v21.clone(), v22.clone(), v23, v24.clone(), v25.clone(), v26);
                match &v27 {
                    US3::US3_StepRejected(v44) => {
                        let v44: Rc<str> = v44.clone();
                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                        let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v44));
                        eprintln!("{}", v46);
                        2i32
                    }
                    US3::US3_StepTaken(v28, v29, v30, v31, v32, v33, v34, v35) => {
                        let v28: Rc<str> = v28.clone();
                        let v29: Rc<str> = v29.clone();
                        let v30: Rc<str> = v30.clone();
                        let v31: Rc<str> = v31.clone();
                        let v32: u64 = *v32;
                        let v33: Rc<str> = v33.clone();
                        let v34: Rc<str> = v34.clone();
                        let v35: bool = *v35;
                        let v36: bool = &*v33 != "";
                        if v36 {
                            let mut v37: i32 = 3i32;
                            let v38: Rc<str> = argument_18(v37);
                            measured_query_1(v38.clone(), v28.clone(), v29.clone(), v30.clone(), v31.clone(), v32, v33.clone(), v34.clone(), v35)
                        } else {
                            let v40: i32 = 3i32;
                            let v41: Rc<str> = argument_18(v40);
                            plain_query_3(v41.clone(), v28.clone(), v29.clone(), v30.clone(), v31.clone(), v32, v33.clone(), v34.clone(), v35)
                        }
                    }
                }
            }
        }
        US2::US2_TableToSpi => {
            let v50: i32 = 5i32;
            let v51: Rc<str> = argument_18(v50);
            let v52: bool = &*v51 == "--open";
            let v56: Rc<str> = if v52 {
                let v53: i32 = 6i32;
                argument_18(v53)
            } else {
                let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v55.clone()
            };
            let v57: i32 = argument_count_21();
            let v58: bool = v57 < 5i32;
            if v58 {
                let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index)"); } LIT.with(|lit| lit.clone()) };
                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index)"); } LIT.with(|lit| lit.clone()) };
                eprintln!("{}", v61);
                2i32
            } else {
                let v62: i32 = 3i32;
                let v63: Rc<str> = argument_18(v62);
                let v64: i32 = 4i32;
                let v65: Rc<str> = argument_18(v64);
                to_spi_run_4(v63.clone(), v65.clone(), v56.clone())
            }
        }
        US2::US2_TableUnknown => {
            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index)"); } LIT.with(|lit| lit.clone()) };
            let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index)"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v105);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        table_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (bool, Rc<str>) {
        rows_kept_2(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_run() -> i32 {
    closure0()()
}
pub fn kit_table_rows_kept(v0: &str, v1: &str) -> (bool, Rc<str>) {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
