#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn native_codemod_coverage_finish(code: i32) -> i32 {
    #[cfg(all(windows, kit_coverage))]
    {
        unsafe extern "C" { fn __llvm_profile_write_file() -> i32; }
        if unsafe { __llvm_profile_write_file() } != 0 {
            eprintln!("native codemod coverage profile flush failed");
            return if code == 0 { 1 } else { code };
        }
    }
    code
}

#[derive(Clone)]
enum US0 {
    US0_Rejected(Rc<str>),
    US0_Candidate(Rc<str>),
}
fn source_1(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = predicate_literal_domain::kit_predicate_encode(&*v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl rust_global (x : string) : () = !!!!Global(x)\ninl emit () : () = rust_global "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    v5
}
fn rewrite_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let (mut v3, mut v4): (bool, Rc<str>) = predicate_rewrite_domain::kit_predicate_rewrite(&*v0,&*v1,&*v2);
    if v3 {
        US0::US0_Candidate(v4.clone())
    } else {
        US0::US0_Rejected(v4)
    }
}
fn check_3(mut v0: bool, mut v1: Rc<str>) -> i32 {
    if v0 {
        0i32
    } else {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failed\n"); } LIT.with(|lit| lit.clone()) };
        let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v2));
        std::io::Write::write_all(&mut std::io::stdout(),v3.as_bytes()).expect("write output");
        1i32
    }
}
fn run_0() -> i32 {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn digest(v:&str)->bool { v.len()==64 && v.bytes().all(|b|b.is_ascii_hexdigit()) }"); } LIT.with(|lit| lit.clone()) };
    let v1: Rc<str> = source_1(v0.clone());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("digest"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sha256"); } LIT.with(|lit| lit.clone()) };
    let v4: US0 = rewrite_2(v1.clone(), v2.clone(), v3.clone());
    let v9: Rc<str> = match &v4 {
        US0::US0_Candidate(v5) => {
            let v5: Rc<str> = v5.clone();
            v5.clone()
        }
        US0::US0_Rejected(v6) => {
            let v6: Rc<str> = v6.clone();
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
    };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lift_sha256 \"digest\""); } LIT.with(|lit| lit.clone()) };
    let v11: bool = v9.contains(&*v10);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hash lift"); } LIT.with(|lit| lit.clone()) };
    let v13: i32 = check_3(v11, v12.clone());
    let v14: US0 = rewrite_2(v9.clone(), v2.clone(), v3.clone());
    let v16: bool = match &v14 {
        US0::US0_Rejected(v15) => {
            let v15: Rc<str> = v15.clone();
            true
        }
        _ => {
            false
        }
    };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("idempotent rejection"); } LIT.with(|lit| lit.clone()) };
    let v18: i32 = check_3(v16, v17.clone());
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn digest(v:&str)->bool{v.len()==63&&v.bytes().all(|b|b.is_ascii_hexdigit())}"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = source_1(v19.clone());
    let v21: US0 = rewrite_2(v20.clone(), v2.clone(), v3.clone());
    let v23: bool = match &v21 {
        US0::US0_Rejected(v22) => {
            let v22: Rc<str> = v22.clone();
            true
        }
        _ => {
            false
        }
    };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("near miss"); } LIT.with(|lit| lit.clone()) };
    let v25: i32 = check_3(v23, v24.clone());
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pub fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())}"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = source_1(v26.clone());
    let v28: US0 = rewrite_2(v27.clone(), v2.clone(), v3.clone());
    let v30: bool = match &v28 {
        US0::US0_Rejected(v29) => {
            let v29: Rc<str> = v29.clone();
            true
        }
        _ => {
            false
        }
    };
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("visibility preservation"); } LIT.with(|lit| lit.clone()) };
    let v32: i32 = check_3(v30, v31.clone());
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v1));
    let v34: US0 = rewrite_2(v33.clone(), v2.clone(), v3.clone());
    let v36: bool = match &v34 {
        US0::US0_Rejected(v35) => {
            let v35: Rc<str> = v35.clone();
            true
        }
        _ => {
            false
        }
    };
    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ambiguity"); } LIT.with(|lit| lit.clone()) };
    let v38: i32 = check_3(v36, v37.clone());
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown"); } LIT.with(|lit| lit.clone()) };
    let v40: US0 = rewrite_2(v1.clone(), v2.clone(), v39.clone());
    let v42: bool = match &v40 {
        US0::US0_Rejected(v41) => {
            let v41: Rc<str> = v41.clone();
            true
        }
        _ => {
            false
        }
    };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown family"); } LIT.with(|lit| lit.clone()) };
    let v44: i32 = check_3(v42, v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())} fn safe(v:&str)->bool{!v.is_empty()&&v.bytes().all(|b|b.is_ascii_alphanumeric()||matches!(b,b'.'|b'_'|b'-'|b'/'))}"); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<str> = source_1(v45.clone());
    let v47: US0 = rewrite_2(v46.clone(), v2.clone(), v3.clone());
    let v52: Rc<str> = match &v47 {
        US0::US0_Candidate(v48) => {
            let v48: Rc<str> = v48.clone();
            v48.clone()
        }
        US0::US0_Rejected(v49) => {
            let v49: Rc<str> = v49.clone();
            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v50.clone()
        }
    };
    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("safe"); } LIT.with(|lit| lit.clone()) };
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("receipt-text"); } LIT.with(|lit| lit.clone()) };
    let v55: US0 = rewrite_2(v52.clone(), v53.clone(), v54.clone());
    let v60: Rc<str> = match &v55 {
        US0::US0_Candidate(v56) => {
            let v56: Rc<str> = v56.clone();
            v56.clone()
        }
        US0::US0_Rejected(v57) => {
            let v57: Rc<str> = v57.clone();
            let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v58.clone()
        }
    };
    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lift_receipt_text \"safe\""); } LIT.with(|lit| lit.clone()) };
    let v62: bool = v60.contains(&*v61);
    let v64: bool = if v62 {
        let v63: bool = v60.contains(&*v10);
        v63
    } else {
        false
    };
    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nested lifts"); } LIT.with(|lit| lit.clone()) };
    let v66: i32 = check_3(v64, v65.clone());
    let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("// rust_global \"fake function\"\n"); } LIT.with(|lit| lit.clone()) };
    let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v1));
    let v69: US0 = rewrite_2(v68.clone(), v2.clone(), v3.clone());
    let v74: Rc<str> = match &v69 {
        US0::US0_Candidate(v70) => {
            let v70: Rc<str> = v70.clone();
            v70.clone()
        }
        US0::US0_Rejected(v71) => {
            let v71: Rc<str> = v71.clone();
            let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v72.clone()
        }
    };
    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("// rust_global"); } LIT.with(|lit| lit.clone()) };
    let v76: bool = v74.contains(&*v75);
    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Spiral comments"); } LIT.with(|lit| lit.clone()) };
    let v78: i32 = check_3(v76, v77.clone());
    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("const TEXT:&str=r###\"fn digest(v:&str)->bool{false}\"###;"); } LIT.with(|lit| lit.clone()) };
    let v80: Rc<str> = source_1(v79.clone());
    let v81: US0 = rewrite_2(v80.clone(), v2.clone(), v3.clone());
    let v83: bool = match &v81 {
        US0::US0_Rejected(v82) => {
            let v82: Rc<str> = v82.clone();
            true
        }
        _ => {
            false
        }
    };
    let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("raw string isolation"); } LIT.with(|lit| lit.clone()) };
    let v85: i32 = check_3(v83, v84.clone());
    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("// ü 路\n"); } LIT.with(|lit| lit.clone()) };
    let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v1));
    let v88: US0 = rewrite_2(v87.clone(), v2.clone(), v3.clone());
    let v93: Rc<str> = match &v88 {
        US0::US0_Candidate(v89) => {
            let v89: Rc<str> = v89.clone();
            v89.clone()
        }
        US0::US0_Rejected(v90) => {
            let v90: Rc<str> = v90.clone();
            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v91.clone()
        }
    };
    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ü 路"); } LIT.with(|lit| lit.clone()) };
    let v95: bool = v93.contains(&*v94);
    let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("UTF-8 preservation"); } LIT.with(|lit| lit.clone()) };
    let v97: i32 = check_3(v95, v96.clone());
    let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#[cfg(test)] fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())}"); } LIT.with(|lit| lit.clone()) };
    let v99: Rc<str> = source_1(v98.clone());
    let v100: US0 = rewrite_2(v99.clone(), v2.clone(), v3.clone());
    let v102: bool = match &v100 {
        US0::US0_Rejected(v101) => {
            let v101: Rc<str> = v101.clone();
            true
        }
        _ => {
            false
        }
    };
    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("attribute preservation"); } LIT.with(|lit| lit.clone()) };
    let v104: i32 = check_3(v102, v103.clone());
    let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust_global (typed_predicate.model.lift_sha256 /*"); } LIT.with(|lit| lit.clone()) };
    let v106: US0 = rewrite_2(v105.clone(), v2.clone(), v3.clone());
    let v108: bool = match &v106 {
        US0::US0_Rejected(v107) => {
            let v107: Rc<str> = v107.clone();
            true
        }
        _ => {
            false
        }
    };
    let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed wrapper comment"); } LIT.with(|lit| lit.clone()) };
    let v110: i32 = check_3(v108, v109.clone());
    let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("const MSG:&str=\"ü 路\"; /* nested /* fn digest() {} */ comment */ fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())}"); } LIT.with(|lit| lit.clone()) };
    let v112: Rc<str> = source_1(v111.clone());
    let v113: US0 = rewrite_2(v112.clone(), v2.clone(), v3.clone());
    let v118: Rc<str> = match &v113 {
        US0::US0_Candidate(v114) => {
            let v114: Rc<str> = v114.clone();
            v114.clone()
        }
        US0::US0_Rejected(v115) => {
            let v115: Rc<str> = v115.clone();
            let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v116.clone()
        }
    };
    let v119: bool = v118.contains(&*v94);
    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Rust UTF-8 and nested comments"); } LIT.with(|lit| lit.clone()) };
    let v121: i32 = check_3(v119, v120.clone());
    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn outer(){fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())}}"); } LIT.with(|lit| lit.clone()) };
    let v123: Rc<str> = source_1(v122.clone());
    let v124: US0 = rewrite_2(v123.clone(), v2.clone(), v3.clone());
    let v126: bool = match &v124 {
        US0::US0_Rejected(v125) => {
            let v125: Rc<str> = v125.clone();
            true
        }
        _ => {
            false
        }
    };
    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nested function isolation"); } LIT.with(|lit| lit.clone()) };
    let v128: i32 = check_3(v126, v127.clone());
    let v129: i32 = v97.wrapping_add(v104);
    let v130: i32 = v85.wrapping_add(v129);
    let v131: i32 = v78.wrapping_add(v130);
    let v132: i32 = v66.wrapping_add(v131);
    let v133: i32 = v44.wrapping_add(v132);
    let v134: i32 = v38.wrapping_add(v133);
    let v135: i32 = v32.wrapping_add(v134);
    let v136: i32 = v25.wrapping_add(v135);
    let v137: i32 = v18.wrapping_add(v136);
    let v138: i32 = v13.wrapping_add(v137);
    let v139: i32 = v121.wrapping_add(v128);
    let v140: i32 = v110.wrapping_add(v139);
    let v141: i32 = v138.wrapping_add(v140);
    let v142: bool = v141 == 0i32 ;
    if v142 {
        let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("14 native Spiral codemod checks passed\n"); } LIT.with(|lit| lit.clone()) };
        std::io::Write::write_all(&mut std::io::stdout(),v143.as_bytes()).expect("write output");
        ()
    };
    v141
}
fn spiral_main() -> i32 {
    let v0: i32 = run_0();
    let v1: i32 = native_codemod_coverage_finish(v0);
    v1
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
