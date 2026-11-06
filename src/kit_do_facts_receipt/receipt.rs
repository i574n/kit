#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn segments_joined_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v0 == "";
    if v2 {
        v1
    } else {
        let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
        let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
        v4
    }
}
fn counted_segment_2(mut v0: bool, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> Rc<str> {
    if v0 {
        let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
        let v5: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
        let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
        segments_joined_3(v1, v6)
    } else {
        v1
    }
}
fn pair_text_4(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: u64, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v5));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v2));
    let v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v4));
    v10
}
fn receipt_text_1(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64, mut v17: u64, mut v18: u64, mut v19: u64, mut v20: u64, mut v21: u64, mut v22: u64, mut v23: u64, mut v24: u64) -> Rc<str> {
    let v25: bool = v24 > 0u64;
    let v31: Rc<str> = if v25 {
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dirs "); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched"); } LIT.with(|lit| lit.clone()) };
        pair_text_4(v26, v22, v27, v23, v28)
    } else {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v30
    };
    let v32: u64 = v3.wrapping_add(v5);
    let v33: u64 = v32.wrapping_add(v7);
    let v34: u64 = v33.wrapping_add(v10);
    let v35: u64 = v34.wrapping_add(v11);
    let v36: bool = v35 > 0u64;
    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
    let v38: Rc<str> = counted_segment_2(v36, v31, v37, v0);
    let v39: u64 = v3.wrapping_add(v11);
    let v40: bool = v39 > 0u64;
    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bytes"); } LIT.with(|lit| lit.clone()) };
    let v42: Rc<str> = counted_segment_2(v40, v38, v41, v1);
    let v43: bool = v3 > 0u64;
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("distinct"); } LIT.with(|lit| lit.clone()) };
    let v45: Rc<str> = counted_segment_2(v43, v42, v44, v2);
    let v46: bool = v5 > 0u64;
    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lookups"); } LIT.with(|lit| lit.clone()) };
    let v48: Rc<str> = counted_segment_2(v46, v45, v47, v4);
    let v49: bool = v7 > 0u64;
    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fields"); } LIT.with(|lit| lit.clone()) };
    let v51: Rc<str> = counted_segment_2(v49, v48, v50, v6);
    let v52: bool = v10 > 0u64;
    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
    let v54: Rc<str> = counted_segment_2(v52, v51, v53, v8);
    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("groups"); } LIT.with(|lit| lit.clone()) };
    let v56: Rc<str> = counted_segment_2(v52, v54, v55.clone(), v9);
    let v57: bool = v14 > 0u64;
    let v63: Rc<str> = if v57 {
        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env "); } LIT.with(|lit| lit.clone()) };
        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" set)"); } LIT.with(|lit| lit.clone()) };
        let v61: Rc<str> = pair_text_4(v58, v12, v59, v13, v60);
        segments_joined_3(v56, v61)
    } else {
        v56
    };
    let v64: bool = v17 > 0u64;
    let v70: Rc<str> = if v64 {
        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("paths "); } LIT.with(|lit| lit.clone()) };
        let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
        let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" present)"); } LIT.with(|lit| lit.clone()) };
        let v68: Rc<str> = pair_text_4(v65, v15, v66, v16, v67);
        segments_joined_3(v63, v68)
    } else {
        v63
    };
    let v71: bool = v21 > 0u64;
    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rows"); } LIT.with(|lit| lit.clone()) };
    let v73: Rc<str> = counted_segment_2(v71, v70, v72, v18);
    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("matched"); } LIT.with(|lit| lit.clone()) };
    let v75: Rc<str> = counted_segment_2(v71, v73, v74, v19);
    counted_segment_2(v71, v75, v55, v20)
}
fn receipt_of_0(mut v0: Rc<RefCell<Vec<(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64)>>>) -> Rc<str> {
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = v0.borrow()[0i32 as usize].clone();
    receipt_text_1(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25)
}
fn closure0() -> Rc<dyn Fn(Rc<RefCell<Vec<(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64)>>>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<RefCell<Vec<(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64)>>>) -> Rc<str>> = Rc::new(move |mut v0: Rc<RefCell<Vec<(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64)>>>| -> Rc<str> {
        receipt_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_facts_receipt(v0: Rc<RefCell<Vec<(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64)>>>) -> Rc<str> {
    closure0()(v0)
}
