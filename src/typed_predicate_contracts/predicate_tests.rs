#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[cfg(test)] mod typed_predicate_tests { #[test] fn ascii_and_unicode_match_original_rust_predicates() { assert_eq!(super::kit_test_typed_predicates(),0); } }

fn ascii_text_2(mut v0: u64, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::string::String::from_utf8(std::iter::repeat_n(v0 as u8,v1 as usize).collect::<Vec<_>>()).unwrap());
    v2
}
fn require_4(mut v0: bool) -> () {
    if v0 { () } else { std::panic::panic_any("typed predicate differential mismatch") };
    ()
}
fn loop_5(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> bool {
    loop {
        let v3: bool = v2 == v1;
        if v3 {
            return true;
        } else {
            let v4: u64 = v0.as_bytes()[v2 as usize] as u64;
            let v5: bool = v4 < 48u64;
            let v7: bool = if v5 {
                false
            } else {
                let v6: bool = v4 <= 57u64;
                v6
            };
            let v15: bool = if v7 {
                true
            } else {
                let v8: bool = v4 < 65u64;
                let v10: bool = if v8 {
                    false
                } else {
                    let v9: bool = v4 <= 70u64;
                    v9
                };
                if v10 {
                    true
                } else {
                    let v11: bool = v4 < 97u64;
                    if v11 {
                        false
                    } else {
                        let v12: bool = v4 <= 102u64;
                        v12
                    }
                }
            };
            if v15 {
                let v16: u64 = v2.wrapping_add(1u64);
                (v0, v1, v2) = (v0.clone(), v1, v16);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn loop_6(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> bool {
    loop {
        let v3: bool = v2 == v1;
        if v3 {
            return true;
        } else {
            let v4: u64 = v0.as_bytes()[v2 as usize] as u64;
            let v5: bool = v4 < 48u64;
            let v7: bool = if v5 {
                false
            } else {
                let v6: bool = v4 <= 57u64;
                v6
            };
            let v15: bool = if v7 {
                true
            } else {
                let v8: bool = v4 < 65u64;
                let v10: bool = if v8 {
                    false
                } else {
                    let v9: bool = v4 <= 90u64;
                    v9
                };
                if v10 {
                    true
                } else {
                    let v11: bool = v4 < 97u64;
                    if v11 {
                        false
                    } else {
                        let v12: bool = v4 <= 122u64;
                        v12
                    }
                }
            };
            let v23: bool = if v15 {
                true
            } else {
                let v16: bool = v4 < 45u64;
                let v18: bool = if v16 {
                    false
                } else {
                    let v17: bool = v4 <= 47u64;
                    v17
                };
                if v18 {
                    true
                } else {
                    let v19: bool = v4 < 95u64;
                    if v19 {
                        false
                    } else {
                        let v20: bool = v4 <= 95u64;
                        v20
                    }
                }
            };
            if v23 {
                let v24: u64 = v2.wrapping_add(1u64);
                (v0, v1, v2) = (v0.clone(), v1, v24);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn check_3(mut v0: Rc<str>) -> () {
    let v1: bool = v0.len() == 64 && v0.bytes().all(|byte| byte.is_ascii_hexdigit());
    let v2: bool = (v0.len() > 0) && v0.bytes().all(|byte| byte.is_ascii_alphanumeric() || [45u8,46,47,95].contains(&byte));
    let v3: u64 = (v0.clone().len() as u64);
    let v4: bool = v3 < 64u64;
    let v6: bool = if v4 {
        false
    } else {
        let v5: bool = v3 <= 64u64;
        v5
    };
    let v9: bool = if v6 {
        let v7: u64 = 0u64;
        loop_5(v0.clone(), v3, v7)
    } else {
        false
    };
    let v10: bool = 1u64 <= v3;
    let v13: bool = if v10 {
        let v11: u64 = 0u64;
        loop_6(v0.clone(), v3, v11)
    } else {
        false
    };
    let v14: bool = v9 == v1;
    require_4(v14);
    let v15: bool = v13 == v2;
    require_4(v15);
    let v16: bool = kit_handoff::binary_install_hash_text(&*v0) == v1 ;
    require_4(v16);
    let v17: bool = kit_handoff::binary_install_safe_text(&*v0) == v2 ;
    require_4(v17)
}
fn loop_1(mut v0: u64) -> u64 {
    loop {
        let v1: bool = v0 < 128u64;
        if v1 {
            let v2: u64 = 1u64;
            let v3: Rc<str> = ascii_text_2(v0, v2);
            check_3(v3.clone());
            let v4: u64 = 63u64;
            let v5: Rc<str> = ascii_text_2(v0, v4);
            check_3(v5.clone());
            let v6: u64 = 64u64;
            let v7: Rc<str> = ascii_text_2(v0, v6);
            check_3(v7.clone());
            let v8: u64 = 65u64;
            let v9: Rc<str> = ascii_text_2(v0, v8);
            check_3(v9.clone());
            let v10: u64 = v0.wrapping_add(1u64);
            v0 = v10;
            continue;
        } else {
            return v0;
        }
    }
}
fn differential_0() -> i32 {
    let mut v0: u64 = 0u64;
    let v1: u64 = loop_1(v0);
    let v2: bool = v1 == 128u64;
    require_4(v2);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    check_3(v3.clone());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("aB09-._/"); } LIT.with(|lit| lit.clone()) };
    check_3(v4.clone());
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("é"); } LIT.with(|lit| lit.clone()) };
    check_3(v5.clone());
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("路/receipt"); } LIT.with(|lit| lit.clone()) };
    check_3(v6.clone());
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ABCDEFabcdef0123456789ABCDEFabcdef0123456789ABCDEFabcdef0123456789ABCD"); } LIT.with(|lit| lit.clone()) };
    check_3(v7.clone());
    0i32
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        differential_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_test_typed_predicates() -> i32 {
    closure0()()
}
