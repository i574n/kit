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
pub struct Mut0 { pub l0: Rc<str> }
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
            return v3;
        } else {
            let v5: i32 = v2.wrapping_sub(1i32);
            let v6: Rc<str> = std::rc::Rc::<str>::from((v0).get(v1 as usize).and_then(|cells| cells.get(v2 as usize)).map(|cell| &**cell).unwrap_or(""));
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v3));
            (v2, v3) = (v5, v7);
            continue;
        }
    }
}
fn cells_in_row_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<UH0> {
    let v2: i32 = (v0).get(v1 as usize).map_or(0, |r| r.len()) as i32;
    let mut v3: i32 = v2.wrapping_sub(1i32);
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    row_cells_7(v0, v1, v3, v4)
}
fn data_rows_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v3: bool = v1 < 1i32;
        if v3 {
            return v2;
        } else {
            let mut v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<UH0> = cells_in_row_6(v0.clone(), v1);
            let v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v2));
            (v1, v2) = (v4, v6);
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
fn method11(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_10(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method11(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method12(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(v2.replace(&*v5, &*v6));
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v9));
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v10));
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
            Rc::new(UH0::UH0_Cons(v12, v4))
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
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
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
fn method9(mut v0: Rc<UH1>, mut v1: i32, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> (i32, Rc<UH0>, Rc<UH0>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v4, v5) => {
                let mut v4: Rc<UH0> = v4;
                let v5: Rc<UH1> = v5;
                let v6: Rc<str> = format_real_10(v1);
                let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("row_"); } LIT.with(|lit| lit.clone()) }, v6));
                let v8: i32 = v1.wrapping_add(1i32);
                let v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) }, v7.clone()));
                let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Row "); } LIT.with(|lit| lit.clone()) }));
                let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v12: Rc<UH0> = method12(v4, v11);
                let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v14: i32 = 0i32;
                let v15: i32 = method13(v13.clone(), v12, v14);
                let v16: Rc<Vec<Rc<str>>> = Rc::new(v13.borrow().clone());
                let v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v16).as_ref().clone()));
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = Rc::<str>::from((*v17.borrow()).join(&*v18));
                let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[ "); } LIT.with(|lit| lit.clone()) }, v19));
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ]"); } LIT.with(|lit| lit.clone()) }));
                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v21));
                let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v22, v2));
                let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v3));
                (v0, v1, v2, v3) = (v5, v8, v23, v24);
                continue;
            }
            UH1::UH1_Nil => {
                return (v1, v2, v3);
            }
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method15(v3, v1);
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ()"); } LIT.with(|lit| lit.clone()) }));
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method16(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<UH0> = v2;
            let mut v3: Rc<UH1> = v3;
            let mut v4: Rc<UH0> = method16(v3, v1);
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
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<UH0> = v2;
                let mut v3: Rc<UH1> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
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
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
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
        let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v4));
        eprintln!("{}", v6);
        2i32
    } else {
        let v7: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v0).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
        let (mut v8, mut v9): (Rc<UH0>, Rc<UH1>) = typed_table_5(v7);
        let v10: bool = &*v2 == "";
        let v13: US1 = if v10 {
            US1::US1_DeclaredHere
        } else {
            US1::US1_OpenedFrom(v2)
        };
        let v102: Rc<str> = match v3 {
            US0::US0_PerRow => {
                let v14: i32 = 1i32;
                let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let (mut v17, mut v18, mut v19): (i32, Rc<UH0>, Rc<UH0>) = method9(v9.clone(), v14, v15, v16);
                let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v21: Rc<UH0> = method12(v8.clone(), v20);
                let v22: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v23: i32 = 0i32;
                let v24: i32 = method13(v22.clone(), v21, v23);
                let v25: Rc<Vec<Rc<str>>> = Rc::new(v22.borrow().clone());
                let v26: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v25).as_ref().clone()));
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = Rc::<str>::from((*v26.borrow()).join(&*v27));
                let v29: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[ "); } LIT.with(|lit| lit.clone()) }, v28));
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ]"); } LIT.with(|lit| lit.clone()) }));
                let v31: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl columns () = Columns "); } LIT.with(|lit| lit.clone()) }, v30));
                let v32: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v33: Rc<UH0> = method14(v18, v32);
                let mut v34: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v34.borrow_mut().push(v31);
                let v35: i32 = 1i32;
                let v36: i32 = method13(v34.clone(), v33, v35);
                let v37: Rc<Vec<Rc<str>>> = Rc::new(v34.borrow().clone());
                let v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v37).as_ref().clone()));
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v40: Rc<str> = Rc::<str>::from((*v38.borrow()).join(&*v39));
                let v46: Rc<str> = match v13 {
                    US1::US1_DeclaredHere => {
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion table_row = | Columns :: list string -> table_row | Row :: list string -> table_row\n"); } LIT.with(|lit| lit.clone()) };
                        v41
                    }
                    US1::US1_OpenedFrom(v42) => {
                        let v42: Rc<str> = v42;
                        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nopen "); } LIT.with(|lit| lit.clone()) }, v42));
                        let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                        v44
                    }
                };
                let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v40));
                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                let v49: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v50: Rc<UH0> = method14(v19, v49);
                let v51: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v52: Rc<UH0> = method15(v50, v51);
                let v53: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v54: i32 = 0i32;
                let v55: i32 = method13(v53.clone(), v52, v54);
                let v56: Rc<Vec<Rc<str>>> = Rc::new(v53.borrow().clone());
                let v57: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v56).as_ref().clone()));
                let v58: Rc<str> = Rc::<str>::from((*v57.borrow()).join(&*v27));
                let v59: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl rows () : list table_row = [ "); } LIT.with(|lit| lit.clone()) }, v58));
                let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ]"); } LIT.with(|lit| lit.clone()) }));
                let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v60));
                let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                v62
            }
            US0::US0_WholeTable => {
                let v68: Rc<str> = match v13 {
                    US1::US1_DeclaredHere => {
                        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion table_row = | Columns :: list string -> table_row | Row :: list string -> table_row | Table :: string -> table_row\n"); } LIT.with(|lit| lit.clone()) };
                        v63
                    }
                    US1::US1_OpenedFrom(v64) => {
                        let v64: Rc<str> = v64;
                        let v65: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nopen "); } LIT.with(|lit| lit.clone()) }, v64));
                        let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                        v66
                    }
                };
                let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl table () = Table \""); } LIT.with(|lit| lit.clone()) }));
                let v70: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v71: Rc<UH0> = method16(v9.clone(), v70);
                let v72: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v73: i32 = 0i32;
                let v74: i32 = method13(v72.clone(), v8.clone(), v73);
                let v75: Rc<Vec<Rc<str>>> = Rc::new(v72.borrow().clone());
                let v76: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v75).as_ref().clone()));
                let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v78: Rc<str> = Rc::<str>::from((*v76.borrow()).join(&*v77));
                let mut v79: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v79.borrow_mut().push(v78);
                let v80: i32 = 1i32;
                let v81: i32 = method13(v79.clone(), v71, v80);
                let v82: Rc<Vec<Rc<str>>> = Rc::new(v79.borrow().clone());
                let v83: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v82).as_ref().clone()));
                let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v85: Rc<str> = Rc::<str>::from((*v83.borrow()).join(&*v84));
                let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                let v88: Rc<str> = Rc::<str>::from(v85.replace(&*v86, &*v87));
                let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                let v91: Rc<str> = Rc::<str>::from(v88.replace(&*v89, &*v90));
                let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                let v93: Rc<str> = Rc::<str>::from(v91.replace(&*v84, &*v92));
                let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                let v95: Rc<str> = Rc::<str>::from(v93.replace(&*v77, &*v94));
                let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                let v98: Rc<str> = Rc::<str>::from(v95.replace(&*v96, &*v97));
                let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v98));
                let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v99, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\n"); } LIT.with(|lit| lit.clone()) }));
                v100
            }
        };
        std::fs::write(&*v1, &*v102).unwrap();
        let v104: i32 = 0i32;
        let v105: i32 = method17(v9, v104);
        let v106: Rc<str> = std::rc::Rc::<str>::from((v105).to_string().as_str());
        let v107: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table to-spi ok rows="); } LIT.with(|lit| lit.clone()) }, v106));
        let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v107, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" columns="); } LIT.with(|lit| lit.clone()) }));
        let v109: i32 = 0i32;
        let v110: i32 = method18(v8, v109);
        let v111: Rc<str> = std::rc::Rc::<str>::from((v110).to_string().as_str());
        let v112: Rc<str> = Rc::<str>::from(format!("{}{}", v108, v111));
        let v113: Rc<str> = Rc::<str>::from(format!("{}{}", v112, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) }));
        let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v113, v1));
        println!("{}", v114);
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
    match v9 {
        US2::US2_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US2::US2_Some(v10) => {
            let v10: Rc<str> = v10;
            v10
        }
    }
}
fn method25(mut v0: Rc<UH2>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: u8 = v2;
                let mut v3: Rc<UH2> = v3;
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
                let v14: Rc<str> = if v10 {
                    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
                    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v7));
                    v12
                } else {
                    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v7));
                    v13
                };
                (v0, v1) = (v3, v14);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
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
    let v6: Rc<str> = match v0 {
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
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn each_prefixed_32(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).map(|item| format!("{}{}", &*v0, item.trim())).collect::<Vec<_>>().join(",").as_str());
    v2
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
    let v2: Option<u64> = method34(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US5> = closure1();
    let mut v4: Option<US5> = v2.map(|x| v3(x));
    let v6: US5 = US5::US5_None;
    let v7: US5 = v6.clone();
    let v8: US5 = v4.unwrap_or(v7);
    match v8 {
        US5::US5_None => {
            0u64
        }
        US5::US5_Some(v9) => {
            let v9: u64 = v9;
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
                v9
            } else {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                concat5_31(v0, v20, v9, v21.clone(), v21.clone())
            };
            US4::US4_StepTaken(v23, v1, v2, v3, v4, v5, v6, v7)
        } else {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--where expects col=value (or col!=value, col>value, col>=value, col<value, col<=value), got "); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = concat5_31(v25, v9, v26.clone(), v26.clone(), v26.clone());
            US4::US4_StepRejected(v27)
        }
    } else {
        let v30: bool = &*v8 == "--cols";
        if v30 {
            US4::US4_StepTaken(v0, v9, v2, v3, v4, v5, v6, v7)
        } else {
            let v32: bool = &*v8 == "--group";
            if v32 {
                US4::US4_StepTaken(v0, v1, v9, v3, v4, v5, v6, v7)
            } else {
                let mut v34: bool = &*v8 == "--key";
                if v34 {
                    US4::US4_StepTaken(v0, v1, v2, v9, v4, v5, v6, v7)
                } else {
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--mean|--sd|--median|--sum|--min|--max|--count"); } LIT.with(|lit| lit.clone()) };
                    let v37: bool = v36.split("|").any(|item| item == &*v8);
                    if v37 {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        let v39: Rc<str> = std::rc::Rc::<str>::from((v8).trim_start_matches(&*v38));
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v42: Rc<str> = concat5_31(v39, v40, v41.clone(), v41.clone(), v41.clone());
                        let v43: Rc<str> = each_prefixed_32(v42, v9);
                        let v44: bool = &*v5 == "";
                        let v49: Rc<str> = if v44 {
                            v43
                        } else {
                            let v45: bool = &*v43 == "";
                            if v45 {
                                v5
                            } else {
                                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                concat5_31(v5, v46, v43, v41.clone(), v41.clone())
                            }
                        };
                        US4::US4_StepTaken(v0, v1, v2, v3, v4, v49, v6, v7)
                    } else {
                        let v51: bool = &*v8 == "--latest-by";
                        if v51 {
                            US4::US4_StepTaken(v0, v1, v2, v3, v4, v5, v9, v7)
                        } else {
                            let v53: bool = &*v8 == "--limit";
                            if v53 {
                                let v54: u64 = decimal_u64_33(v9);
                                let v55: bool = v54 == 0u64;
                                if v55 {
                                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--limit expects a positive integer"); } LIT.with(|lit| lit.clone()) };
                                    US4::US4_StepRejected(v56)
                                } else {
                                    US4::US4_StepTaken(v0, v1, v2, v3, v54, v5, v6, v7)
                                }
                            } else {
                                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v62: Rc<str> = concat5_31(v60, v8, v61.clone(), v61.clone(), v61.clone());
                                US4::US4_StepRejected(v62)
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
                    let v20: US4 = option_applied_30(v2, v3, v4, v5, v6, v7, v8, v9, v18, v19);
                    match v20 {
                        US4::US4_StepRejected(v31) => {
                            let v31: Rc<str> = v31;
                            return US4::US4_StepRejected(v31);
                        }
                        US4::US4_StepTaken(v21, v22, v23, v24, v25, v26, v27, v28) => {
                            let v21: Rc<str> = v21;
                            let v22: Rc<str> = v22;
                            let v23: Rc<str> = v23;
                            let v24: Rc<str> = v24;
                            let v25: u64 = v25;
                            let v26: Rc<str> = v26;
                            let v27: Rc<str> = v27;
                            let v28: bool = v28;
                            let v29: i32 = v0.wrapping_add(2i32);
                            (v0, v2, v3, v4, v5, v6, v7, v8, v9) = (v29, v21, v22, v23, v24, v25, v26, v27, v28);
                            continue;
                        }
                    }
                } else {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = argument_19(v0);
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) };
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v39: Rc<str> = concat5_31(v35, v36, v37, v38.clone(), v38.clone());
                    return US4::US4_StepRejected(v39);
                }
            }
        } else {
            return US4::US4_StepTaken(v2, v3, v4, v5, v6, v7, v8, v9);
        }
    }
}
fn table_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let v1: Rc<str> = argument_19(v0);
    let v2: US3 = table_command_of_22(v1);
    match v2 {
        US3::US3_TableAppend => {
            let v105: i32 = kit_table_append::kit_table_append_run();
            v105
        }
        US3::US3_TableDiff => {
            let v72: i32 = argument_count_27();
            let v73: bool = v72 < 5i32;
            if v73 {
                let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] [--whole] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index; --whole writes one Table literal of the whole table instead of a Row per line, for tables too big to compile as cell lists) | table append <table.spi|table.tsv> <cell>... | --row-tsv <line> (adds one row, checks its width, rewrites the file in place; a .spi table stays one whole Table literal)"); } LIT.with(|lit| lit.clone()) };
                let v75: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v74));
                eprintln!("{}", v75);
                2i32
            } else {
                let v76: i32 = 5i32;
                let v77: i32 = argument_count_27();
                let (mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_28();
                let v86: US4 = options_from_29(v76, v77, v78, v79, v80, v81, v82, v83, v84, v85);
                match v86 {
                    US4::US4_StepRejected(v100) => {
                        let v100: Rc<str> = v100;
                        let v101: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v100));
                        eprintln!("{}", v101);
                        2i32
                    }
                    US4::US4_StepTaken(v87, v88, v89, v90, v91, v92, v93, v94) => {
                        let v87: Rc<str> = v87;
                        let v88: Rc<str> = v88;
                        let v89: Rc<str> = v89;
                        let v90: Rc<str> = v90;
                        let v91: u64 = v91;
                        let v92: Rc<str> = v92;
                        let v93: Rc<str> = v93;
                        let v94: bool = v94;
                        let v95: i32 = 3i32;
                        let v96: Rc<str> = argument_19(v95);
                        let v97: i32 = 4i32;
                        let v98: Rc<str> = argument_19(v97);
                        let v99: i32 = kit_table_diff::kit_table_diff_run(&*v96, &*v98, &*v90, &*v88, &*v93, v91);
                        v99
                    }
                }
            }
        }
        US3::US3_TableQuery => {
            let mut v3: i32 = argument_count_27();
            let mut v4: bool = v3 < 4i32;
            if v4 {
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] [--whole] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index; --whole writes one Table literal of the whole table instead of a Row per line, for tables too big to compile as cell lists) | table append <table.spi|table.tsv> <cell>... | --row-tsv <line> (adds one row, checks its width, rewrites the file in place; a .spi table stays one whole Table literal)"); } LIT.with(|lit| lit.clone()) };
                let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v5));
                eprintln!("{}", v6);
                2i32
            } else {
                let v7: i32 = 4i32;
                let v8: i32 = argument_count_27();
                let (mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_28();
                let v17: US4 = options_from_29(v7, v8, v9, v10, v11, v12, v13, v14, v15, v16);
                match v17 {
                    US4::US4_StepRejected(v34) => {
                        let mut v34: Rc<str> = v34;
                        let v35: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v34));
                        eprintln!("{}", v35);
                        2i32
                    }
                    US4::US4_StepTaken(v18, v19, v20, v21, v22, v23, v24, v25) => {
                        let v18: Rc<str> = v18;
                        let v19: Rc<str> = v19;
                        let v20: Rc<str> = v20;
                        let v21: Rc<str> = v21;
                        let v22: u64 = v22;
                        let v23: Rc<str> = v23;
                        let v24: Rc<str> = v24;
                        let v25: bool = v25;
                        let v26: bool = &*v23 != "";
                        if v26 {
                            let v27: i32 = 3i32;
                            let v28: Rc<str> = argument_19(v27);
                            measured_query_1(v28, v18, v19, v20, v21, v22, v23, v24, v25)
                        } else {
                            let v30: i32 = 3i32;
                            let v31: Rc<str> = argument_19(v30);
                            plain_query_3(v31, v18, v19, v20, v21, v22, v23, v24, v25)
                        }
                    }
                }
            }
        }
        US3::US3_TableToSpi => {
            let v39: i32 = 5i32;
            let v40: Rc<str> = argument_19(v39);
            let v41: bool = &*v40 == "--open";
            let v51: Rc<str> = if v41 {
                let v42: i32 = 6i32;
                argument_19(v42)
            } else {
                let v44: i32 = 6i32;
                let v45: Rc<str> = argument_19(v44);
                let v46: bool = &*v45 == "--open";
                if v46 {
                    let v47: i32 = 7i32;
                    argument_19(v47)
                } else {
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v49
                }
            };
            let v52: i32 = 5i32;
            let v53: Rc<str> = argument_19(v52);
            let v54: bool = &*v53 == "--whole";
            let v58: bool = if v54 {
                true
            } else {
                let v55: i32 = 7i32;
                let v56: Rc<str> = argument_19(v55);
                let v57: bool = &*v56 == "--whole";
                v57
            };
            let v61: US0 = if v58 {
                US0::US0_WholeTable
            } else {
                US0::US0_PerRow
            };
            let v62: i32 = argument_count_27();
            let v63: bool = v62 < 5i32;
            if v63 {
                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] [--whole] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index; --whole writes one Table literal of the whole table instead of a Row per line, for tables too big to compile as cell lists) | table append <table.spi|table.tsv> <cell>... | --row-tsv <line> (adds one row, checks its width, rewrites the file in place; a .spi table stays one whole Table literal)"); } LIT.with(|lit| lit.clone()) };
                let v65: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v64));
                eprintln!("{}", v65);
                2i32
            } else {
                let v66: i32 = 3i32;
                let v67: Rc<str> = argument_19(v66);
                let v68: i32 = 4i32;
                let v69: Rc<str> = argument_19(v68);
                to_spi_run_4(v67, v69, v51, v61)
            }
        }
        US3::US3_TableUnknown => {
            let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--median|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | table to-spi <file.tsv> <out.spi> [--open <module>] [--whole] (typed row module; --open uses that module's table_row union instead of declaring one, plus an inl rows () index; --whole writes one Table literal of the whole table instead of a Row per line, for tables too big to compile as cell lists) | table append <table.spi|table.tsv> <cell>... | --row-tsv <line> (adds one row, checks its width, rewrites the file in place; a .spi table stays one whole Table literal)"); } LIT.with(|lit| lit.clone()) };
            let v107: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v106));
            eprintln!("{}", v107);
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
        rows_kept_2(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_run() -> i32 {
    closure0()()
}
pub fn kit_table_rows_kept(v0: &str, v1: &str) -> (bool, Rc<str>) {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
