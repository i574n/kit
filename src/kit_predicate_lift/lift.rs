#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Rejected(Rc<str>),
    US0_Candidate(Rc<str>),
}
fn command_line_argument_count_1() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn count_0() -> i32 {
    let v0: i32 = command_line_argument_count_1();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn rewrite_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let (mut v3, mut v4): (bool, Rc<str>) = predicate_rewrite_domain::kit_predicate_rewrite(&*v0,&*v1,&*v2);
    if v3 {
        US0::US0_Candidate(v4.clone())
    } else {
        US0::US0_Rejected(v4)
    }
}
fn spiral_main() -> i32 {
    let v0: i32 = count_0();
    let v1: bool = v0 == 5i32 ;
    if v1 {
        let v2: Rc<str> = std::env::args().nth(1i32 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
        let v3: Rc<str> = std::env::args().nth(2i32 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
        let v4: Rc<str> = std::env::args().nth(3i32 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
        let v5: Rc<str> = std::env::args().nth(4i32 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v7: bool = v4.ends_with(&*v6);
        let v9: bool = if v7 {
            let v8: bool = v5.ends_with(&*v6);
            v8
        } else {
            false
        };
        if v9 {
            let v10: Rc<str> = std::rc::Rc::<str>::from(String::from_utf8(kit_rust_std_fs::read_regular_limited(std::path::Path::new(&*v4),1024*1024).expect("read input")).expect("input must be UTF-8"));
            let v11: US0 = rewrite_2(v10.clone(), v3.clone(), v2.clone());
            match &v11 {
                US0::US0_Candidate(v15) => {
                    let v15: Rc<str> = v15.clone();
                    let v16: i32 = std::fs::OpenOptions::new().write(true).create_new(true).open(&*v5).and_then(|mut file| std::io::Write::write_all(&mut file,v15.as_bytes())).map_or(2,|()|0);
                    let v17: bool = v16 == 0i32 ;
                    if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Spiral candidate written; input unchanged. Regenerate and test before publication.\n"); } LIT.with(|lit| lit.clone()) };
                        std::io::Write::write_all(&mut std::io::stdout(),v18.as_bytes()).expect("write output");
                        ()
                    } else {
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Cannot create candidate; output must not exist.\n"); } LIT.with(|lit| lit.clone()) };
                        std::io::Write::write_all(&mut std::io::stdout(),v19.as_bytes()).expect("write output");
                        ()
                    };
                    v16
                }
                US0::US0_Rejected(v12) => {
                    let v12: Rc<str> = v12.clone();
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
                    std::io::Write::write_all(&mut std::io::stdout(),v14.as_bytes()).expect("write output");
                    2i32
                }
            }
        } else {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Input and output must be .spi files.\n"); } LIT.with(|lit| lit.clone()) };
            std::io::Write::write_all(&mut std::io::stdout(),v22.as_bytes()).expect("write output");
            2i32
        }
    } else {
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-lift-predicate <sha256|receipt-text> <function> <input.spi> <new-output.spi>\nAdd typed_predicate to the owning package before regenerating.\n"); } LIT.with(|lit| lit.clone()) };
        std::io::Write::write_all(&mut std::io::stdout(),v24.as_bytes()).expect("write output");
        let v25: i32 = count_0();
        let v26: bool = v25 == 2i32 ;
        let v29: bool = if v26 {
            let v27: Rc<str> = std::env::args().nth(1i32 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
            let v28: bool = &*v27 == "--help";
            v28
        } else {
            false
        };
        if v29 {
            0i32
        } else {
            2i32
        }
    }
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
