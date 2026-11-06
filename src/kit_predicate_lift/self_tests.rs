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
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl rust_global (x : string) : () = !!!!Global(x)\ninl emit () : () = rust_global "); } LIT.with(|lit| lit.clone()) }, v1));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
    v3
}
fn rewrite_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let (mut v3, mut v4): (bool, Rc<str>) = predicate_rewrite_domain::kit_predicate_rewrite(&*v0,&*v1,&*v2);
    if v3 {
        US0::US0_Candidate(v4)
    } else {
        US0::US0_Rejected(v4)
    }
}
fn check_3(mut v0: bool, mut v1: Rc<str>) -> i32 {
    if v0 {
        0i32
    } else {
        let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failed\n"); } LIT.with(|lit| lit.clone()) }));
        std::io::Write::write_all(&mut std::io::stdout(),v2.as_bytes()).expect("write output");
        1i32
    }
}
fn run_0() -> i32 {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn digest(v:&str)->bool { v.len()==64 && v.bytes().all(|b|b.is_ascii_hexdigit()) }"); } LIT.with(|lit| lit.clone()) };
    let v1: Rc<str> = source_1(v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("digest"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sha256"); } LIT.with(|lit| lit.clone()) };
    let v4: US0 = rewrite_2(v1.clone(), v2.clone(), v3.clone());
    let v9: Rc<str> = match v4 {
        US0::US0_Candidate(v5) => {
            let v5: Rc<str> = v5;
            v5
        }
        US0::US0_Rejected(v6) => {
            let v6: Rc<str> = v6;
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7
        }
    };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lift_sha256 \"digest\""); } LIT.with(|lit| lit.clone()) };
    let v11: bool = v9.contains(&*v10);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hash lift"); } LIT.with(|lit| lit.clone()) };
    let v13: i32 = check_3(v11, v12);
    let v14: US0 = rewrite_2(v9, v2.clone(), v3.clone());
    let v16: bool = match v14 {
        US0::US0_Rejected(v15) => {
            let v15: Rc<str> = v15;
            true
        }
        _ => {
            false
        }
    };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("idempotent rejection"); } LIT.with(|lit| lit.clone()) };
    let v18: i32 = check_3(v16, v17);
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn digest(v:&str)->bool{v.len()==63&&v.bytes().all(|b|b.is_ascii_hexdigit())}"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = source_1(v19);
    let v21: US0 = rewrite_2(v20, v2.clone(), v3.clone());
    let v23: bool = match v21 {
        US0::US0_Rejected(v22) => {
            let v22: Rc<str> = v22;
            true
        }
        _ => {
            false
        }
    };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("near miss"); } LIT.with(|lit| lit.clone()) };
    let v25: i32 = check_3(v23, v24);
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pub fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())}"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = source_1(v26);
    let v28: US0 = rewrite_2(v27, v2.clone(), v3.clone());
    let v30: bool = match v28 {
        US0::US0_Rejected(v29) => {
            let v29: Rc<str> = v29;
            true
        }
        _ => {
            false
        }
    };
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("visibility preservation"); } LIT.with(|lit| lit.clone()) };
    let v32: i32 = check_3(v30, v31);
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v1.clone(), v1.clone()));
    let v34: US0 = rewrite_2(v33, v2.clone(), v3.clone());
    let v36: bool = match v34 {
        US0::US0_Rejected(v35) => {
            let v35: Rc<str> = v35;
            true
        }
        _ => {
            false
        }
    };
    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ambiguity"); } LIT.with(|lit| lit.clone()) };
    let v38: i32 = check_3(v36, v37);
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown"); } LIT.with(|lit| lit.clone()) };
    let v40: US0 = rewrite_2(v1.clone(), v2.clone(), v39);
    let v42: bool = match v40 {
        US0::US0_Rejected(v41) => {
            let v41: Rc<str> = v41;
            true
        }
        _ => {
            false
        }
    };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown family"); } LIT.with(|lit| lit.clone()) };
    let v44: i32 = check_3(v42, v43);
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())} fn safe(v:&str)->bool{!v.is_empty()&&v.bytes().all(|b|b.is_ascii_alphanumeric()||matches!(b,b'.'|b'_'|b'-'|b'/'))}"); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<str> = source_1(v45);
    let v47: US0 = rewrite_2(v46, v2.clone(), v3.clone());
    let v52: Rc<str> = match v47 {
        US0::US0_Candidate(v48) => {
            let v48: Rc<str> = v48;
            v48
        }
        US0::US0_Rejected(v49) => {
            let v49: Rc<str> = v49;
            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v50
        }
    };
    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("safe"); } LIT.with(|lit| lit.clone()) };
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("receipt-text"); } LIT.with(|lit| lit.clone()) };
    let v55: US0 = rewrite_2(v52, v53, v54);
    let v60: Rc<str> = match v55 {
        US0::US0_Candidate(v56) => {
            let v56: Rc<str> = v56;
            v56
        }
        US0::US0_Rejected(v57) => {
            let v57: Rc<str> = v57;
            let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v58
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
    let v66: i32 = check_3(v64, v65);
    let v67: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("// rust_global \"fake function\"\n"); } LIT.with(|lit| lit.clone()) }, v1.clone()));
    let v68: US0 = rewrite_2(v67, v2.clone(), v3.clone());
    let v73: Rc<str> = match v68 {
        US0::US0_Candidate(v69) => {
            let v69: Rc<str> = v69;
            v69
        }
        US0::US0_Rejected(v70) => {
            let v70: Rc<str> = v70;
            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v71
        }
    };
    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("// rust_global"); } LIT.with(|lit| lit.clone()) };
    let v75: bool = v73.contains(&*v74);
    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Spiral comments"); } LIT.with(|lit| lit.clone()) };
    let v77: i32 = check_3(v75, v76);
    let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("const TEXT:&str=r###\"fn digest(v:&str)->bool{false}\"###;"); } LIT.with(|lit| lit.clone()) };
    let v79: Rc<str> = source_1(v78);
    let v80: US0 = rewrite_2(v79, v2.clone(), v3.clone());
    let v82: bool = match v80 {
        US0::US0_Rejected(v81) => {
            let v81: Rc<str> = v81;
            true
        }
        _ => {
            false
        }
    };
    let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("raw string isolation"); } LIT.with(|lit| lit.clone()) };
    let v84: i32 = check_3(v82, v83);
    let v85: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("// ü 路\n"); } LIT.with(|lit| lit.clone()) }, v1));
    let v86: US0 = rewrite_2(v85, v2.clone(), v3.clone());
    let v91: Rc<str> = match v86 {
        US0::US0_Candidate(v87) => {
            let v87: Rc<str> = v87;
            v87
        }
        US0::US0_Rejected(v88) => {
            let v88: Rc<str> = v88;
            let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v89
        }
    };
    let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ü 路"); } LIT.with(|lit| lit.clone()) };
    let v93: bool = v91.contains(&*v92);
    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("UTF-8 preservation"); } LIT.with(|lit| lit.clone()) };
    let v95: i32 = check_3(v93, v94);
    let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#[cfg(test)] fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())}"); } LIT.with(|lit| lit.clone()) };
    let v97: Rc<str> = source_1(v96);
    let v98: US0 = rewrite_2(v97, v2.clone(), v3.clone());
    let v100: bool = match v98 {
        US0::US0_Rejected(v99) => {
            let v99: Rc<str> = v99;
            true
        }
        _ => {
            false
        }
    };
    let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("attribute preservation"); } LIT.with(|lit| lit.clone()) };
    let v102: i32 = check_3(v100, v101);
    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust_global (typed_predicate.model.lift_sha256 /*"); } LIT.with(|lit| lit.clone()) };
    let v104: US0 = rewrite_2(v103, v2.clone(), v3.clone());
    let v106: bool = match v104 {
        US0::US0_Rejected(v105) => {
            let v105: Rc<str> = v105;
            true
        }
        _ => {
            false
        }
    };
    let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed wrapper comment"); } LIT.with(|lit| lit.clone()) };
    let v108: i32 = check_3(v106, v107);
    let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("const MSG:&str=\"ü 路\"; /* nested /* fn digest() {} */ comment */ fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())}"); } LIT.with(|lit| lit.clone()) };
    let v110: Rc<str> = source_1(v109);
    let v111: US0 = rewrite_2(v110, v2.clone(), v3.clone());
    let v116: Rc<str> = match v111 {
        US0::US0_Candidate(v112) => {
            let v112: Rc<str> = v112;
            v112
        }
        US0::US0_Rejected(v113) => {
            let v113: Rc<str> = v113;
            let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v114
        }
    };
    let v117: bool = v116.contains(&*v92);
    let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Rust UTF-8 and nested comments"); } LIT.with(|lit| lit.clone()) };
    let v119: i32 = check_3(v117, v118);
    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn outer(){fn digest(v:&str)->bool{v.len()==64&&v.bytes().all(|b|b.is_ascii_hexdigit())}}"); } LIT.with(|lit| lit.clone()) };
    let v121: Rc<str> = source_1(v120);
    let v122: US0 = rewrite_2(v121, v2, v3);
    let v124: bool = match v122 {
        US0::US0_Rejected(v123) => {
            let v123: Rc<str> = v123;
            true
        }
        _ => {
            false
        }
    };
    let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nested function isolation"); } LIT.with(|lit| lit.clone()) };
    let v126: i32 = check_3(v124, v125);
    let v127: i32 = v95.wrapping_add(v102);
    let v128: i32 = v84.wrapping_add(v127);
    let v129: i32 = v77.wrapping_add(v128);
    let v130: i32 = v66.wrapping_add(v129);
    let v131: i32 = v44.wrapping_add(v130);
    let v132: i32 = v38.wrapping_add(v131);
    let v133: i32 = v32.wrapping_add(v132);
    let v134: i32 = v25.wrapping_add(v133);
    let v135: i32 = v18.wrapping_add(v134);
    let v136: i32 = v13.wrapping_add(v135);
    let v137: i32 = v119.wrapping_add(v126);
    let v138: i32 = v108.wrapping_add(v137);
    let v139: i32 = v136.wrapping_add(v138);
    let v140: bool = v139 == 0i32 ;
    if v140 {
        let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("14 native Spiral codemod checks passed\n"); } LIT.with(|lit| lit.clone()) };
        std::io::Write::write_all(&mut std::io::stdout(),v141.as_bytes()).expect("write output");
        ()
    };
    v139
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
