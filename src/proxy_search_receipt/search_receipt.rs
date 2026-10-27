#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_AllHitsShown,
    US0_HitsTruncated,
}
#[derive(Clone)]
enum US1 {
    US1_SliceFilled,
    US1_SliceReachedEnd,
}
fn search_clock_ms_0() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    v0
}
fn closure0() -> Rc<dyn Fn() -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> u64> = Rc::new(move || -> u64 {
        search_clock_ms_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn parallel_search_receipt_text_2(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: u64, mut v4: u64, mut v5: u64) -> Rc<str> {
    let v6: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy parallel-search ok workers="); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v6));
    let v9: bool = v1 > v2;
    let v12: US0 = if v9 {
        US0::US0_HitsTruncated
    } else {
        US0::US0_AllHitsShown
    };
    let v24: Rc<str> = match &v12 {
        US0::US0_AllHitsShown => {
            let v13: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hits="); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
            v15.clone()
        }
        US0::US0_HitsTruncated => {
            let v16: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hits="); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v16));
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" truncated=true found_at_least="); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
            let v21: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
            v22.clone()
        }
    };
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v24));
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" parity=exact ordering=deterministic excludes=.git,node_modules,build-output,symlinks"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
    let v28: u64 = (v4).saturating_sub(v3);
    let v29: Rc<str> = std::rc::Rc::<str>::from((v28).to_string());
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" serial_ms="); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v29));
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" parallel_ms="); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
    let v34: u64 = (v5).saturating_sub(v4);
    let v35: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v27, Rc::<str>::from(format!("{}{}", v33, v35))));
    let v38: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let v39: Rc<str> = std::rc::Rc::<str>::from((v38).to_string());
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v37, Rc::<str>::from(format!("{}{}", v40, v39))));
    v42.clone()
}
fn parallel_search_receipt_1(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: u64, mut v4: u64, mut v5: u64) -> i32 {
    let v6: Rc<str> = parallel_search_receipt_text_2(v0, v1, v2, v3, v4, v5);
    let v7: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v6.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v7
}
fn closure1() -> Rc<dyn Fn(i32, i32, i32, u64, u64, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32, i32, u64, u64, u64) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32, mut v2: i32, mut v3: u64, mut v4: u64, mut v5: u64| -> i32 {
        parallel_search_receipt_1(v0, v1, v2, v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn fs_search_receipt_text_4(mut v0: i32, mut v1: i32, mut v2: u64) -> Rc<str> {
    let v3: bool = v0 > v1;
    let v6: US0 = if v3 {
        US0::US0_HitsTruncated
    } else {
        US0::US0_AllHitsShown
    };
    let v18: Rc<str> = match &v6 {
        US0::US0_AllHitsShown => {
            let v7: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hits="); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
            v9.clone()
        }
        US0::US0_HitsTruncated => {
            let v10: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hits="); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v10));
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" truncated=true found_at_least="); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
            let v15: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
            v16.clone()
        }
    };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy fs-search ok"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" excludes=.git,node_modules,build-output,symlinks"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let v23: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
    let v24: Rc<str> = std::rc::Rc::<str>::from((v23).to_string());
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v22, Rc::<str>::from(format!("{}{}", v25, v24))));
    v27.clone()
}
fn fs_search_receipt_3(mut v0: i32, mut v1: i32, mut v2: u64) -> i32 {
    let v3: Rc<str> = fs_search_receipt_text_4(v0, v1, v2);
    let v4: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v3.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v4
}
fn closure2() -> Rc<dyn Fn(i32, i32, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32, u64) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32, mut v2: u64| -> i32 {
        fs_search_receipt_3(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn fs_slice_receipt_text_6(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> Rc<str> {
    let v4: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy fs-slice ok start="); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" lines="); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let v9: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" requested="); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    let v13: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let v15: bool = v1 < v2;
    let v18: US1 = if v15 {
        US1::US1_SliceReachedEnd
    } else {
        US1::US1_SliceFilled
    };
    let v22: Rc<str> = match &v18 {
        US1::US1_SliceFilled => {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v19.clone()
        }
        US1::US1_SliceReachedEnd => {
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" end_of_file=true"); } LIT.with(|lit| lit.clone()) };
            v20.clone()
        }
    };
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v22));
    let v24: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let v25: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v23, Rc::<str>::from(format!("{}{}", v26, v25))));
    v28.clone()
}
fn fs_slice_receipt_5(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> i32 {
    let v4: Rc<str> = fs_slice_receipt_text_6(v0, v1, v2, v3);
    let v5: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v4.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v5
}
fn closure3() -> Rc<dyn Fn(u64, u64, u64, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, u64, u64, u64) -> i32> = Rc::new(move |mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64| -> i32 {
        fs_slice_receipt_5(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn fs_context_receipt_text_8(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> Rc<str> {
    let v4: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy fs-context ok offset="); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" window_bytes="); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let v9: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" file_bytes="); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    let v13: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let v15: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let v16: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v14, Rc::<str>::from(format!("{}{}", v17, v16))));
    v19.clone()
}
fn fs_context_receipt_7(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> i32 {
    let v4: Rc<str> = fs_context_receipt_text_8(v0, v1, v2, v3);
    let v5: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v4.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v5
}
fn closure4() -> Rc<dyn Fn(u64, u64, u64, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, u64, u64, u64) -> i32> = Rc::new(move |mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64| -> i32 {
        fs_context_receipt_7(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn hash_receipt_text_10(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: u64 = std::fs::metadata(&*v0).map(|meta| meta.len()).unwrap_or(0);
    let v3: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy hash ok bytes="); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v3));
    let v6: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v1);
    let v7: Rc<str> = std::rc::Rc::<str>::from((v6).to_string());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v5, Rc::<str>::from(format!("{}{}", v8, v7))));
    v10.clone()
}
fn hash_receipt_9(mut v0: Rc<str>, mut v1: u64) -> i32 {
    let v2: Rc<str> = hash_receipt_text_10(v0.clone(), v1);
    let v3: i32 = std::io::Write::write_all(&mut std::io::stderr(), (v2.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v3
}
fn closure5() -> Rc<dyn Fn(Rc<str>, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> i32 {
        hash_receipt_9(v0.clone(), v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn hash_tree_receipt_text_12(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy hash-tree ok relative="); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v0));
    let v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v1);
    let v5: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v3, Rc::<str>::from(format!("{}{}", v6, v5))));
    v8.clone()
}
fn hash_tree_receipt_11(mut v0: Rc<str>, mut v1: u64) -> i32 {
    let v2: Rc<str> = hash_tree_receipt_text_12(v0.clone(), v1);
    let v3: i32 = std::io::Write::write_all(&mut std::io::stderr(), (v2.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v3
}
fn closure6() -> Rc<dyn Fn(Rc<str>, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> i32 {
        hash_tree_receipt_11(v0.clone(), v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn parallel_manifest_receipt_text_14(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64) -> Rc<str> {
    let v6: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy parallel-manifest ok workers="); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v6));
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" entries="); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    let v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" sha256="); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v13)), v2));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" parity=exact ordering=deterministic"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: u64 = (v4).saturating_sub(v3);
    let v19: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" serial_ms="); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" parallel_ms="); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
    let v24: u64 = (v5).saturating_sub(v4);
    let v25: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v17, Rc::<str>::from(format!("{}{}", v23, v25))));
    let v28: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let v29: Rc<str> = std::rc::Rc::<str>::from((v28).to_string());
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v27, Rc::<str>::from(format!("{}{}", v30, v29))));
    v32.clone()
}
fn parallel_manifest_receipt_13(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64) -> i32 {
    let v6: Rc<str> = parallel_manifest_receipt_text_14(v0, v1, v2.clone(), v3, v4, v5);
    let v7: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v6.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v7
}
fn closure7() -> Rc<dyn Fn(i32, i32, Rc<str>, u64, u64, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32, Rc<str>, u64, u64, u64) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64| -> i32 {
        parallel_manifest_receipt_13(v0, v1, v2.clone(), v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_search_clock_ms() -> u64 {
    closure0()()
}
pub fn kit_parallel_search_receipt(v0: i32, v1: i32, v2: i32, v3: u64, v4: u64, v5: u64) -> i32 {
    closure1()(v0, v1, v2, v3, v4, v5)
}
pub fn kit_fs_search_receipt(v0: i32, v1: i32, v2: u64) -> i32 {
    closure2()(v0, v1, v2)
}
pub fn kit_fs_slice_receipt(v0: u64, v1: u64, v2: u64, v3: u64) -> i32 {
    closure3()(v0, v1, v2, v3)
}
pub fn kit_fs_context_receipt(v0: u64, v1: u64, v2: u64, v3: u64) -> i32 {
    closure4()(v0, v1, v2, v3)
}
pub fn kit_hash_receipt(v0: &str, v1: u64) -> i32 {
    closure5()(Rc::<str>::from(v0), v1)
}
pub fn kit_hash_tree_receipt(v0: &str, v1: u64) -> i32 {
    closure6()(Rc::<str>::from(v0), v1)
}
pub fn kit_parallel_manifest_receipt(v0: i32, v1: i32, v2: &str, v3: u64, v4: u64, v5: u64) -> i32 {
    closure7()(v0, v1, Rc::<str>::from(v2), v3, v4, v5)
}
