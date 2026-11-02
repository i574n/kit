#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_PerRow,
    US0_WholeTable,
}
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
enum US1 {
    US1_DeclaredHere,
    US1_OpenedFrom(Rc<str>),
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_TableQuery,
    US3_TableDiff,
    US3_TableToSpi,
    US3_TableAppend,
    US3_TableUnknown,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(u8, Rc<UH2>),
}
#[derive(Clone)]
enum US4 {
    US4_StepTaken(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool),
    US4_StepRejected(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_Some(u64),
    US5_None,
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
            (v2, v3) = (v5, v7.clone());
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
            (v1, v2) = (v4, v6.clone());
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
                (v1, v2) = (v4.clone(), v5);
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
fn method16(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<UH0> = v2.clone();
            let mut v3: Rc<UH1> = v3.clone();
            let mut v4: Rc<UH0> = method16(v3, v1.clone());
            let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v6: i32 = 0i32;
            let v7: i32 = method13(v5.clone(), v2, v6);
            let v8: Rc<Vec<Rc<str>>> = Rc::new(v5.borrow().clone());
            let v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
            Rc::new(UH0::UH0_Cons(v11, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method17(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
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
fn method18(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn to_spi_run_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US0) -> i32 {
    let mut v4: Rc<str> = kit_table_text::kit_table_problem(&*v0);
    let v5: bool = &*v4 != "";
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v4));
        eprintln!("{}", v7);
        2i32
    } else {
        let v8: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v0).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
        let (mut v9, mut v10): (Rc<UH0>, Rc<UH1>) = typed_table_5(v8.clone());
        let v11: bool = &*v2 == "";
        let v14: US1 = if v11 {
            US1::US1_DeclaredHere
        } else {
            US1::US1_OpenedFrom(v2.clone())
        };
        let v114: Rc<str> = match &v3 {
            US0::US0_PerRow => {
                let v15: i32 = 1i32;
                let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let (mut v18, mut v19, mut v20): (i32, Rc<UH0>, Rc<UH0>) = method9(v10.clone(), v15, v16.clone(), v17.clone());
                let v21: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v22: Rc<UH0> = method12(v9.clone(), v21.clone());
                let v23: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v24: i32 = 0i32;
                let v25: i32 = method13(v23.clone(), v22.clone(), v24);
                let v26: Rc<Vec<Rc<str>>> = Rc::new(v23.borrow().clone());
                let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v26).as_ref().clone()));
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                let v29: Rc<str> = Rc::<str>::from((*v27.borrow()).join(&*v28));
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[ "); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v29));
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ]"); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl columns () = Columns "); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v33));
                let v36: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v37: Rc<UH0> = method14(v19.clone(), v36.clone());
                let mut v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v38.borrow_mut().push(v35);
                let v39: i32 = 1i32;
                let v40: i32 = method13(v38.clone(), v37.clone(), v39);
                let v41: Rc<Vec<Rc<str>>> = Rc::new(v38.borrow().clone());
                let v42: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v41).as_ref().clone()));
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v44: Rc<str> = Rc::<str>::from((*v42.borrow()).join(&*v43));
                let v51: Rc<str> = match &v14 {
                    US1::US1_DeclaredHere => {
                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion table_row = | Columns :: list string -> table_row | Row :: list string -> table_row\n"); } LIT.with(|lit| lit.clone()) };
                        v45.clone()
                    }
                    US1::US1_OpenedFrom(v46) => {
                        let v46: Rc<str> = v46.clone();
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nopen "); } LIT.with(|lit| lit.clone()) };
                        let v49: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v47, v46)), v43));
                        v49.clone()
                    }
                };
                let v53: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v51, v44)), v43));
                let v54: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v55: Rc<UH0> = method14(v20.clone(), v54.clone());
                let v56: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v57: Rc<UH0> = method15(v55.clone(), v56.clone());
                let v58: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v59: i32 = 0i32;
                let v60: i32 = method13(v58.clone(), v57.clone(), v59);
                let v61: Rc<Vec<Rc<str>>> = Rc::new(v58.borrow().clone());
                let v62: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v61).as_ref().clone()));
                let v63: Rc<str> = Rc::<str>::from((*v62.borrow()).join(&*v28));
                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl rows () : list table_row = [ "); } LIT.with(|lit| lit.clone()) };
                let v68: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v53, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v64, v63)), v32)))), v43));
                v68.clone()
            }
            US0::US0_WholeTable => {
                let v78: Rc<str> = match &v14 {
                    US1::US1_DeclaredHere => {
                        let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion table_row = | Columns :: list string -> table_row | Row :: list string -> table_row"); } LIT.with(|lit| lit.clone()) };
                        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" | Table :: string -> table_row\n"); } LIT.with(|lit| lit.clone()) };
                        let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion table_row = | Columns :: list string -> table_row | Row :: list string -> table_row | Table :: string -> table_row\n"); } LIT.with(|lit| lit.clone()) };
                        v71.clone()
                    }
                    US1::US1_OpenedFrom(v72) => {
                        let v72: Rc<str> = v72.clone();
                        let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nopen "); } LIT.with(|lit| lit.clone()) };
                        let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v72));
                        let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v75));
                        v76.clone()
                    }
                };
                let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl table () = Table \""); } LIT.with(|lit| lit.clone()) };
                let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v79));
                let v81: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v82: Rc<UH0> = method16(v10.clone(), v81.clone());
                let v83: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v84: i32 = 0i32;
                let v85: i32 = method13(v83.clone(), v9.clone(), v84);
                let v86: Rc<Vec<Rc<str>>> = Rc::new(v83.borrow().clone());
                let v87: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v86).as_ref().clone()));
                let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v89: Rc<str> = Rc::<str>::from((*v87.borrow()).join(&*v88));
                let mut v90: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v90.borrow_mut().push(v89);
                let v91: i32 = 1i32;
                let v92: i32 = method13(v90.clone(), v82.clone(), v91);
                let v93: Rc<Vec<Rc<str>>> = Rc::new(v90.borrow().clone());
                let v94: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v93).as_ref().clone()));
                let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v96: Rc<str> = Rc::<str>::from((*v94.borrow()).join(&*v95));
                let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                let v99: Rc<str> = Rc::<str>::from(v96.replace(&*v97, &*v98));
                let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                let v102: Rc<str> = Rc::<str>::from(v99.replace(&*v100, &*v101));
                let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                let v104: Rc<str> = Rc::<str>::from(v102.replace(&*v95, &*v103));
                let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                let v106: Rc<str> = Rc::<str>::from(v104.replace(&*v88, &*v105));
                let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                let v109: Rc<str> = Rc::<str>::from(v106.replace(&*v107, &*v108));
                let v110: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v109));
                let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\n"); } LIT.with(|lit| lit.clone()) };
                let v112: Rc<str> = Rc::<str>::from(format!("{}{}", v110, v111));
                v112.clone()
            }
        };
        std::fs::write(&*v1, &*v114).unwrap();
        let v116: i32 = 0i32;
        let v117: i32 = method17(v10.clone(), v116);
        let v118: Rc<str> = std::rc::Rc::<str>::from((v117).to_string().as_str());
        let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table to-spi ok rows="); } LIT.with(|lit| lit.clone()) };
        let v120: Rc<str> = Rc::<str>::from(format!("{}{}", v119, v118));
        let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" columns="); } LIT.with(|lit| lit.clone()) };
        let v122: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v121));
        let v123: i32 = 0i32;
        let v124: i32 = method18(v9.clone(), v123);
        let v125: Rc<str> = std::rc::Rc::<str>::from((v124).to_string().as_str());
        let v126: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v125));
        let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
        let v129: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v126, v127)), v1));
        println!("{}", v129);
        0i32
    }
}
fn command_line_argument_count_20() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_21(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_19(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_20();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US2 = if v5 {
        let v6: Rc<str> = command_line_argument_21(v1);
        US2::US2_Some(v6)
    } else {
        US2::US2_None
    };
    match &v9 {
        US2::US2_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US2::US2_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10
        }
    }
}
fn method25(mut v0: Rc<UH2>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH2::UH2_Cons(v2, v3) => {
                let v2: u8 = *v2;
                let mut v3: Rc<UH2> = v3.clone();
                let mut v4: std::string::String = char::from(v2).to_string();
                let v6: Rc<str> = Rc::<str>::from(String::as_str(&v4));
                let v7: Rc<str> = Rc::<str>::from(v6.to_lowercase());
                let v8: bool = &*v7 != &*v6;
                let v10: bool = if v8 {
                    let v9: bool = &*v1 != "";
                    v9
                } else {
                    false
                };
                let v15: Rc<str> = if v10 {
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v11)), v7));
                    v13.clone()
                } else {
                    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v7));
                    v14.clone()
                };
                (v0, v1) = (v3.clone(), v15.clone());
                continue;
            }
            UH2::UH2_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method26(mut v0: i64, mut v1: Rc<str>, mut v2: i64) -> Rc<UH2> {
    let mut v3: bool = v2 < v0;
    if v3 {
        let mut v4: u8 = v1.clone().as_bytes()[v2 as usize];
        let v5: i64 = v2.wrapping_add(1i64);
        let v6: Rc<UH2> = method26(v0, v1, v5);
        Rc::new(UH2::UH2_Cons(v4, v6))
    } else {
        { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
    }
}
fn kebab_word_24(mut v0: Rc<str>) -> Rc<str> {
    let v1: i64 = (v0.clone().len() as i64);
    let v2: i64 = 0i64;
    let mut v3: Rc<UH2> = method26(v1, v0, v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method25(v3, v4)
}
fn table_command_word_23(mut v0: US3) -> Rc<str> {
    let v6: Rc<str> = match &v0 {
        US3::US3_TableAppend => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Append"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US3::US3_TableDiff => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Diff"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US3::US3_TableQuery => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Query"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US3::US3_TableToSpi => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ToSpi"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US3::US3_TableUnknown => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unknown"); } LIT.with(|lit| lit.clone()) };
            v5
        }
    };
    kebab_word_24(v6)
}
fn table_command_of_22(mut v0: Rc<str>) -> US3 {
    let v1: US3 = US3::US3_TableQuery;
    let v2: Rc<str> = table_command_word_23(v1);
    let mut v3: bool = &*v2 == &*v0;
    if v3 {
        US3::US3_TableQuery
    } else {
        let v5: US3 = US3::US3_TableDiff;
        let v6: Rc<str> = table_command_word_23(v5);
        let v7: bool = &*v6 == &*v0;
        if v7 {
            US3::US3_TableDiff
        } else {
            let v9: US3 = US3::US3_TableToSpi;
            let v10: Rc<str> = table_command_word_23(v9);
            let v11: bool = &*v10 == &*v0;
            if v11 {
                US3::US3_TableToSpi
            } else {
                let v13: US3 = US3::US3_TableAppend;
                let v14: Rc<str> = table_command_word_23(v13);
                let v15: bool = &*v14 == &*v0;
                if v15 {
                    US3::US3_TableAppend
                } else {
                    let v17: US3 = US3::US3_TableUnknown;
                    let v18: Rc<str> = table_command_word_23(v17);
                    US3::US3_TableUnknown
                }
            }
        }
    }
}
fn argument_count_27() -> i32 {
    let mut v0: i32 = command_line_argument_count_20();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn no_request_28() -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v0.clone(), v0.clone(), v0.clone(), 20u64, v0.clone(), v0, false)
}
fn concat5_31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn each_prefixed_32(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).map(|item| format!("{}{}", &*v0, item.trim())).collect::<Vec<_>>().join(",").as_str());
    v2.clone()
}
fn method34(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US5> = Rc::new(move |mut v0: (u64)| -> US5 {
        let v1: u64 = (v0);
        US5::US5_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_33(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method34(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US5> = closure1();
    let mut v4: Option<US5> = v2.map(|x| v3(x));
    let v6: US5 = US5::US5_None;
    let v7: US5 = v6.clone();
    let v8: US5 = v4.unwrap_or(v7);
    match &v8 {
        US5::US5_None => {
            0u64
        }
        US5::US5_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn option_applied_30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: bool, mut v8: Rc<str>, mut v9: Rc<str>) -> US4 {
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
                concat5_31(v0.clone(), v20.clone(), v9.clone(), v21.clone(), v21.clone())
            };
            US4::US4_StepTaken(v23.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
        } else {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--where expects col=value (or col!=value, col>value, col>=value, col<value, col<=value), got "); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = concat5_31(v25.clone(), v9.clone(), v26.clone(), v26.clone(), v26.clone());
            US4::US4_StepRejected(v27.clone())
        }
    } else {
        let v30: bool = &*v8 == "--cols";
        if v30 {
            US4::US4_StepTaken(v0.clone(), v9.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
        } else {
            let v32: bool = &*v8 == "--group";
            if v32 {
                US4::US4_StepTaken(v0.clone(), v1.clone(), v9.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
            } else {
                let v34: bool = &*v8 == "--key";
                if v34 {
                    US4::US4_StepTaken(v0.clone(), v1.clone(), v2.clone(), v9.clone(), v4, v5.clone(), v6.clone(), v7)
                } else {
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--mean|--sd|--median|--sum|--min|--max|--count"); } LIT.with(|lit| lit.clone()) };
                    let v37: bool = v36.split("|").any(|item| item == &*v8);
                    if v37 {
                        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        let v39: Rc<str> = std::rc::Rc::<str>::from((v8).trim_start_matches(&*v38));
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v42: Rc<str> = concat5_31(v39.clone(), v40.clone(), v41.clone(), v41.clone(), v41.clone());
                        let v43: Rc<str> = each_prefixed_32(v42.clone(), v9.clone());
                        let v44: bool = &*v5 == "";
                        let v49: Rc<str> = if v44 {
                            v43.clone()
                        } else {
                            let v45: bool = &*v43 == "";
                            if v45 {
                                v5.clone()
                            } else {
                                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                concat5_31(v5.clone(), v46.clone(), v43.clone(), v41.clone(), v41.clone())
                            }
                        };
                        US4::US4_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v49.clone(), v6.clone(), v7)
                    } else {
                        let v51: bool = &*v8 == "--latest-by";
                        if v51 {
                            US4::US4_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v9.clone(), v7)
                        } else {
                            let v53: bool = &*v8 == "--limit";
                            if v53 {
                                let v54: u64 = decimal_u64_33(v9.clone());
                                let v55: bool = v54 == 0u64;
                                if v55 {
                                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--limit expects a positive integer"); } LIT.with(|lit| lit.clone()) };
                                    US4::US4_StepRejected(v56.clone())
                                } else {
                                    US4::US4_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v54, v5.clone(), v6.clone(), v7)
                                }
                            } else {
                                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v62: Rc<str> = concat5_31(v60.clone(), v8.clone(), v61.clone(), v61.clone(), v61.clone());
                                US4::US4_StepRejected(v62.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
fn options_from_29(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: bool) -> US4 {
    loop {
        let v10: bool = v0 < v1;
        if v10 {
            let v11: Rc<str> = argument_19(v0);
            let v12: bool = &*v11 == "--quiet";
            if v12 {
                let v13: i32 = v0.wrapping_add(1i32);
                let v14: bool = true;
                (v0, v9) = (v13, v14);
                continue;
            } else {
                let v16: i32 = v0.wrapping_add(1i32);
                let v17: bool = v16 < v1;
                if v17 {
                    let v18: Rc<str> = argument_19(v0);
                    let v19: Rc<str> = argument_19(v16);
                    let v20: US4 = option_applied_30(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v9, v18.clone(), v19.clone());
                    match &v20 {
                        US4::US4_StepRejected(v31) => {
                            let v31: Rc<str> = v31.clone();
                            return US4::US4_StepRejected(v31.clone());
                        }
                        US4::US4_StepTaken(v21, v22, v23, v24, v25, v26, v27, v28) => {
                            let v21: Rc<str> = v21.clone();
                            let v22: Rc<str> = v22.clone();
                            let v23: Rc<str> = v23.clone();
                            let v24: Rc<str> = v24.clone();
                            let v25: u64 = *v25;
                            let v26: Rc<str> = v26.clone();
                            let v27: Rc<str> = v27.clone();
                            let v28: bool = *v28;
                            let v29: i32 = v0.wrapping_add(2i32);
                            (v0, v2, v3, v4, v5, v6, v7, v8, v9) = (v29, v21.clone(), v22.clone(), v23.clone(), v24.clone(), v25, v26.clone(), v27.clone(), v28);
                            continue;
                        }
                    }
                } else {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = argument_19(v0);
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) };
                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v39: Rc<str> = concat5_31(v35.clone(), v36.clone(), v37.clone(), v38.clone(), v38.clone());
                    return US4::US4_StepRejected(v39.clone());
                }
            }
        } else {
            return US4::US4_StepTaken(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v9);
        }
    }
}
fn table_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let v1: Rc<str> = argument_19(v0);
    let v2: US3 = table_command_of_22(v1.clone());
    match &v2 {
        US3::US3_TableAppend => {
            let v110: i32 = kit_table_append::kit_table_append_run();
            v110
        }
        US3::US3_TableDiff => {
            let v75: i32 = argument_count_27();
            let v76: bool = v75 < 5i32;
            if v76 {
                let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] [--whole] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index; --whole writes one Table literal of the whole table instead of a Row per line, for tables too big to compile as cell lists) | table append <table.spi|table.tsv> <cell>... | --row-tsv <line> (adds one row, checks its width, rewrites the file in place; a .spi table stays one whole Table literal)"); } LIT.with(|lit| lit.clone()) };
                let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] [--whole] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index; --whole writes one Table literal of the whole table instead of a Row per line, for tables too big to compile as cell lists) | table append <table.spi|table.tsv> <cell>... | --row-tsv <line> (adds one row, checks its width, rewrites the file in place; a .spi table stays one whole Table literal)"); } LIT.with(|lit| lit.clone()) };
                eprintln!("{}", v79);
                2i32
            } else {
                let v80: i32 = 5i32;
                let v81: i32 = argument_count_27();
                let (mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_28();
                let mut v90: US4 = options_from_29(v80, v81, v82.clone(), v83.clone(), v84.clone(), v85.clone(), v86, v87.clone(), v88.clone(), v89);
                match &v90 {
                    US4::US4_StepRejected(v104) => {
                        let v104: Rc<str> = v104.clone();
                        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                        let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v104));
                        eprintln!("{}", v106);
                        2i32
                    }
                    US4::US4_StepTaken(v91, v92, v93, v94, v95, v96, v97, v98) => {
                        let v91: Rc<str> = v91.clone();
                        let v92: Rc<str> = v92.clone();
                        let v93: Rc<str> = v93.clone();
                        let v94: Rc<str> = v94.clone();
                        let v95: u64 = *v95;
                        let v96: Rc<str> = v96.clone();
                        let v97: Rc<str> = v97.clone();
                        let v98: bool = *v98;
                        let v99: i32 = 3i32;
                        let v100: Rc<str> = argument_19(v99);
                        let v101: i32 = 4i32;
                        let v102: Rc<str> = argument_19(v101);
                        let v103: i32 = kit_table_diff::kit_table_diff_run(&*v100, &*v102, &*v94, &*v92, &*v97, v95);
                        v103
                    }
                }
            }
        }
        US3::US3_TableQuery => {
            let mut v3: i32 = argument_count_27();
            let mut v4: bool = v3 < 4i32;
            if v4 {
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] [--whole] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index; --whole writes one Table literal of the whole table instead of a Row per line, for tables too big to compile as cell lists) | table append <table.spi|table.tsv> <cell>... | --row-tsv <line> (adds one row, checks its width, rewrites the file in place; a .spi table stays one whole Table literal)"); } LIT.with(|lit| lit.clone()) };
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] [--whole] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index; --whole writes one Table literal of the whole table instead of a Row per line, for tables too big to compile as cell lists) | table append <table.spi|table.tsv> <cell>... | --row-tsv <line> (adds one row, checks its width, rewrites the file in place; a .spi table stays one whole Table literal)"); } LIT.with(|lit| lit.clone()) };
                eprintln!("{}", v7);
                2i32
            } else {
                let v8: i32 = 4i32;
                let v9: i32 = argument_count_27();
                let (mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_28();
                let v18: US4 = options_from_29(v8, v9, v10.clone(), v11.clone(), v12.clone(), v13.clone(), v14, v15.clone(), v16.clone(), v17);
                match &v18 {
                    US4::US4_StepRejected(v35) => {
                        let v35: Rc<str> = v35.clone();
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                        let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v35));
                        eprintln!("{}", v37);
                        2i32
                    }
                    US4::US4_StepTaken(v19, v20, v21, v22, v23, v24, v25, v26) => {
                        let v19: Rc<str> = v19.clone();
                        let v20: Rc<str> = v20.clone();
                        let v21: Rc<str> = v21.clone();
                        let v22: Rc<str> = v22.clone();
                        let v23: u64 = *v23;
                        let v24: Rc<str> = v24.clone();
                        let v25: Rc<str> = v25.clone();
                        let v26: bool = *v26;
                        let v27: bool = &*v24 != "";
                        if v27 {
                            let v28: i32 = 3i32;
                            let v29: Rc<str> = argument_19(v28);
                            measured_query_1(v29.clone(), v19.clone(), v20.clone(), v21.clone(), v22.clone(), v23, v24.clone(), v25.clone(), v26)
                        } else {
                            let v31: i32 = 3i32;
                            let v32: Rc<str> = argument_19(v31);
                            plain_query_3(v32.clone(), v19.clone(), v20.clone(), v21.clone(), v22.clone(), v23, v24.clone(), v25.clone(), v26)
                        }
                    }
                }
            }
        }
        US3::US3_TableToSpi => {
            let v41: i32 = 5i32;
            let v42: Rc<str> = argument_19(v41);
            let v43: bool = &*v42 == "--open";
            let v53: Rc<str> = if v43 {
                let v44: i32 = 6i32;
                argument_19(v44)
            } else {
                let v46: i32 = 6i32;
                let v47: Rc<str> = argument_19(v46);
                let v48: bool = &*v47 == "--open";
                if v48 {
                    let v49: i32 = 7i32;
                    argument_19(v49)
                } else {
                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v51.clone()
                }
            };
            let v54: i32 = 5i32;
            let v55: Rc<str> = argument_19(v54);
            let v56: bool = &*v55 == "--whole";
            let v60: bool = if v56 {
                true
            } else {
                let v57: i32 = 7i32;
                let v58: Rc<str> = argument_19(v57);
                let v59: bool = &*v58 == "--whole";
                v59
            };
            let v63: US0 = if v60 {
                US0::US0_WholeTable
            } else {
                US0::US0_PerRow
            };
            let v64: i32 = argument_count_27();
            let v65: bool = v64 < 5i32;
            if v65 {
                let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] [--whole] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index; --whole writes one Table literal of the whole table instead of a Row per line, for tables too big to compile as cell lists) | table append <table.spi|table.tsv> <cell>... | --row-tsv <line> (adds one row, checks its width, rewrites the file in place; a .spi table stays one whole Table literal)"); } LIT.with(|lit| lit.clone()) };
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] [--whole] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index; --whole writes one Table literal of the whole table instead of a Row per line, for tables too big to compile as cell lists) | table append <table.spi|table.tsv> <cell>... | --row-tsv <line> (adds one row, checks its width, rewrites the file in place; a .spi table stays one whole Table literal)"); } LIT.with(|lit| lit.clone()) };
                eprintln!("{}", v68);
                2i32
            } else {
                let v69: i32 = 3i32;
                let v70: Rc<str> = argument_19(v69);
                let v71: i32 = 4i32;
                let v72: Rc<str> = argument_19(v71);
                to_spi_run_4(v70.clone(), v72.clone(), v53.clone(), v63.clone())
            }
        }
        US3::US3_TableUnknown => {
            let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] [--whole] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index; --whole writes one Table literal of the whole table instead of a Row per line, for tables too big to compile as cell lists) | table append <table.spi|table.tsv> <cell>... | --row-tsv <line> (adds one row, checks its width, rewrites the file in place; a .spi table stays one whole Table literal)"); } LIT.with(|lit| lit.clone()) };
            let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
            let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] [--whole] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index; --whole writes one Table literal of the whole table instead of a Row per line, for tables too big to compile as cell lists) | table append <table.spi|table.tsv> <cell>... | --row-tsv <line> (adds one row, checks its width, rewrites the file in place; a .spi table stays one whole Table literal)"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v113);
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
