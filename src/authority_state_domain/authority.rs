#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn authority_state_binding_0(mut v0: i32, mut v1: i32) -> i32 {
    let v2: bool = v1 == 0i32;
    if v2 {
        let v3: bool = v0 == 0i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    } else {
        -1i32
    }
}
fn authority_live_mask_binding_1(mut v0: i32, mut v1: i32) -> i32 {
    let v2: i32 = 1i32 << 0i32;
    let v3: i32 = 1i32 << 1i32;
    let v4: i32 = v2.wrapping_add(v3);
    let v5: i32 = 1i32 << 2i32;
    let v6: i32 = v4.wrapping_add(v5);
    let v7: i32 = 1i32 << 3i32;
    let v8: i32 = v6.wrapping_add(v7);
    let v9: i32 = 1i32 << 4i32;
    let v10: i32 = v8.wrapping_add(v9);
    let v11: bool = v1 == 0i32;
    if v11 {
        let v12: bool = v0 < 0i32;
        if v12 {
            -1i32
        } else {
            let v13: bool = v10 < v0;
            if v13 {
                -1i32
            } else {
                let v14: i32 = v10.wrapping_sub(v0);
                v14
            }
        }
    } else {
        -1i32
    }
}
fn authority_declared_mask_binding_2(mut v0: i32, mut v1: i32) -> i32 {
    let v2: bool = v0 == 0i32;
    if v2 {
        let v3: bool = v1 == 0i32;
        if v3 {
            0i32
        } else {
            -1i32
        }
    } else {
        -1i32
    }
}
fn authority_blocker_bit_binding_3(mut v0: i32, mut v1: i32) -> i32 {
    let v2: bool = v1 == 0i32;
    if v2 {
        let v3: bool = 0i32 == v0;
        if v3 {
            let v4: i32 = 1i32 << 0i32;
            v4
        } else {
            let v5: bool = 1i32 == v0;
            if v5 {
                let v6: i32 = 1i32 << 1i32;
                v6
            } else {
                let v7: bool = 2i32 == v0;
                if v7 {
                    let v8: i32 = 1i32 << 2i32;
                    v8
                } else {
                    let v9: bool = 3i32 == v0;
                    if v9 {
                        let v10: i32 = 1i32 << 3i32;
                        v10
                    } else {
                        let v11: bool = 4i32 == v0;
                        if v11 {
                            let v12: i32 = 1i32 << 4i32;
                            v12
                        } else {
                            -1i32
                        }
                    }
                }
            }
        }
    } else {
        -1i32
    }
}
fn authority_blocker_labels_4(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v1: bool = &*v0 == "v1";
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("semantic-closeout"); } LIT.with(|lit| lit.clone()) };
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cold-proof-v4"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("differential-catalog"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("family-contracts"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cold-rebuild"); } LIT.with(|lit| lit.clone()) };
        (v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v7.clone(), v7.clone(), v7.clone(), v7.clone(), v7.clone())
    }
}
fn closure0() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        authority_state_binding_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        authority_declared_mask_binding_2(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        authority_live_mask_binding_1(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        authority_blocker_bit_binding_3(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
        authority_blocker_labels_4(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_authority_state(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
pub fn kit_authority_declared_mask(v0: i32, v1: i32) -> i32 {
    closure1()(v0, v1)
}
pub fn kit_authority_live_mask(v0: i32, v1: i32) -> i32 {
    closure2()(v0, v1)
}
pub fn kit_authority_blocker_bit(v0: i32, v1: i32) -> i32 {
    closure3()(v0, v1)
}
pub fn kit_authority_blocker_labels(v0: &str) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    closure4()(Rc::<str>::from(v0))
}
