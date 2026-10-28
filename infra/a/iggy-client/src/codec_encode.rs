#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_JobStarted(Rc<str>, Rc<str>),
    US0_JobStep(Rc<str>, Rc<str>, i32, Rc<str>),
    US0_JobDone(Rc<str>, i32),
    US0_JobStop(Rc<str>, Rc<str>),
    US0_LeaseAcquired(Rc<str>, Rc<str>),
    US0_LeaseReleased(Rc<str>, Rc<str>),
    US0_SlotAdmitted(i32, Rc<str>),
    US0_SlotReleased(i32, Rc<str>),
    US0_Health(i32, i32, i32),
    US0_Toolchain(Rc<str>, Rc<str>, Rc<str>, Rc<str>),
}
struct Mut0 { l0: i32 }
fn method2(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0.clone();
    let v3: bool = v2 < v0;
    v3
}
fn cat_1(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: Rc<RefCell<Vec<u8>>>) -> Rc<RefCell<Vec<u8>>> {
    let v2: i32 = (v0.clone().borrow().len() as i32);
    let v3: i32 = (v1.clone().borrow().len() as i32);
    let v4: i32 = v2.wrapping_add(v3);
    let v5: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(vec![<u8>::default(); v4 as usize]));
    let mut v6: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
    while method2(v4, v6.clone()) {
        let v8: i32 = v6.borrow().l0.clone();
        let v9: bool = v8 < v2;
        let v13: u8 = if v9 {
            let v10: u8 = v0.clone().borrow()[v8 as usize].clone();
            v10
        } else {
            let v11: i32 = v8.wrapping_sub(v2);
            let v12: u8 = v1.clone().borrow()[v11 as usize].clone();
            v12
        };
        v5.clone().borrow_mut()[v8 as usize] = v13;
        let v14: i32 = v8.wrapping_add(1i32);
        v6.borrow_mut().l0 = v14;
        ()
    };
    v5.clone()
}
fn put_u64_3(mut v0: u64) -> Rc<RefCell<Vec<u8>>> {
    let v1: u64 = v0.wrapping_shr((56i32) as u32);
    let v2: u64 = v1 & 255u64;
    let v3: u8 = (v2 as u8);
    let v4: u64 = v0.wrapping_shr((48i32) as u32);
    let v5: u64 = v4 & 255u64;
    let mut v6: u8 = (v5 as u8);
    let v7: u64 = v0.wrapping_shr((40i32) as u32);
    let v8: u64 = v7 & 255u64;
    let v9: u8 = (v8 as u8);
    let v10: u64 = v0.wrapping_shr((32i32) as u32);
    let v11: u64 = v10 & 255u64;
    let v12: u8 = (v11 as u8);
    let v13: u64 = v0.wrapping_shr((24i32) as u32);
    let v14: u64 = v13 & 255u64;
    let v15: u8 = (v14 as u8);
    let v16: u64 = v0.wrapping_shr((16i32) as u32);
    let v17: u64 = v16 & 255u64;
    let v18: u8 = (v17 as u8);
    let v19: u64 = v0.wrapping_shr((8i32) as u32);
    let v20: u64 = v19 & 255u64;
    let v21: u8 = (v20 as u8);
    let v22: u64 = v0 & 255u64;
    let v23: u8 = (v22 as u8);
    let v24: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(vec![v3, v6, v9, v12, v15, v18, v21, v23]));
    v24
}
fn put_u32_5(mut v0: u32) -> Rc<RefCell<Vec<u8>>> {
    let v1: u32 = v0.wrapping_shr((24i32) as u32);
    let v2: u32 = v1 & 255u32;
    let v3: u8 = (v2 as u8);
    let v4: u32 = v0.wrapping_shr((16i32) as u32);
    let v5: u32 = v4 & 255u32;
    let mut v6: u8 = (v5 as u8);
    let v7: u32 = v0.wrapping_shr((8i32) as u32);
    let v8: u32 = v7 & 255u32;
    let v9: u8 = (v8 as u8);
    let v10: u32 = v0 & 255u32;
    let v11: u8 = (v10 as u8);
    let v12: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(vec![v3, v6, v9, v11]));
    v12
}
fn put_str_4(mut v0: Rc<str>) -> Rc<RefCell<Vec<u8>>> {
    let v1: Rc<Vec<u8>> = Rc::new(v0.bytes().collect());
    let v2: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new((v1).as_ref().clone()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: u32 = (v3 as u32);
    let v5: Rc<RefCell<Vec<u8>>> = put_u32_5(v4);
    cat_1(v5, v2)
}
fn body_tag_6(mut v0: US0) -> u8 {
    match &v0 {
        US0::US0_Health(v19, v20, v21) => {
            let v19: i32 = *v19;
            let v20: i32 = *v20;
            let v21: i32 = *v21;
            8u8
        }
        US0::US0_JobDone(v7, v8) => {
            let v7: Rc<str> = v7.clone();
            let v8: i32 = *v8;
            2u8
        }
        US0::US0_JobStarted(v1, v2) => {
            let v1: Rc<str> = v1.clone();
            let v2: Rc<str> = v2.clone();
            0u8
        }
        US0::US0_JobStep(v3, v4, v5, v6) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<str> = v4.clone();
            let v5: i32 = *v5;
            let mut v6: Rc<str> = v6.clone();
            1u8
        }
        US0::US0_JobStop(v9, v10) => {
            let v9: Rc<str> = v9.clone();
            let v10: Rc<str> = v10.clone();
            3u8
        }
        US0::US0_LeaseAcquired(v11, v12) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = v12.clone();
            4u8
        }
        US0::US0_LeaseReleased(v13, v14) => {
            let v13: Rc<str> = v13.clone();
            let v14: Rc<str> = v14.clone();
            5u8
        }
        US0::US0_SlotAdmitted(v15, v16) => {
            let v15: i32 = *v15;
            let v16: Rc<str> = v16.clone();
            6u8
        }
        US0::US0_SlotReleased(v17, v18) => {
            let v17: i32 = *v17;
            let v18: Rc<str> = v18.clone();
            7u8
        }
        US0::US0_Toolchain(v22, v23, v24, v25) => {
            let v22: Rc<str> = v22.clone();
            let v23: Rc<str> = v23.clone();
            let v24: Rc<str> = v24.clone();
            let v25: Rc<str> = v25.clone();
            9u8
        }
    }
}
fn put_i32_8(mut v0: i32) -> Rc<RefCell<Vec<u8>>> {
    let v1: u32 = (v0 as u32);
    put_u32_5(v1)
}
fn body_bytes_7(mut v0: US0) -> Rc<RefCell<Vec<u8>>> {
    match &v0 {
        US0::US0_Health(v47, v48, v49) => {
            let v47: i32 = *v47;
            let v48: i32 = *v48;
            let v49: i32 = *v49;
            let v50: Rc<RefCell<Vec<u8>>> = put_i32_8(v47);
            let v51: Rc<RefCell<Vec<u8>>> = put_i32_8(v48);
            let v52: Rc<RefCell<Vec<u8>>> = put_i32_8(v49);
            let v53: Rc<RefCell<Vec<u8>>> = cat_1(v51, v52);
            cat_1(v50, v53)
        }
        US0::US0_JobDone(v17, v18) => {
            let v17: Rc<str> = v17.clone();
            let v18: i32 = *v18;
            let v19: Rc<RefCell<Vec<u8>>> = put_str_4(v17);
            let v20: Rc<RefCell<Vec<u8>>> = put_i32_8(v18);
            cat_1(v19, v20)
        }
        US0::US0_JobStarted(v1, v2) => {
            let v1: Rc<str> = v1.clone();
            let v2: Rc<str> = v2.clone();
            let v3: Rc<RefCell<Vec<u8>>> = put_str_4(v1);
            let v4: Rc<RefCell<Vec<u8>>> = put_str_4(v2);
            cat_1(v3, v4)
        }
        US0::US0_JobStep(v6, v7, v8, v9) => {
            let mut v6: Rc<str> = v6.clone();
            let v7: Rc<str> = v7.clone();
            let v8: i32 = *v8;
            let v9: Rc<str> = v9.clone();
            let v10: Rc<RefCell<Vec<u8>>> = put_str_4(v6);
            let v11: Rc<RefCell<Vec<u8>>> = put_str_4(v7);
            let v12: Rc<RefCell<Vec<u8>>> = put_i32_8(v8);
            let v13: Rc<RefCell<Vec<u8>>> = put_str_4(v9);
            let v14: Rc<RefCell<Vec<u8>>> = cat_1(v12, v13);
            let v15: Rc<RefCell<Vec<u8>>> = cat_1(v11, v14);
            cat_1(v10, v15)
        }
        US0::US0_JobStop(v22, v23) => {
            let v22: Rc<str> = v22.clone();
            let v23: Rc<str> = v23.clone();
            let v24: Rc<RefCell<Vec<u8>>> = put_str_4(v22);
            let v25: Rc<RefCell<Vec<u8>>> = put_str_4(v23);
            cat_1(v24, v25)
        }
        US0::US0_LeaseAcquired(v27, v28) => {
            let v27: Rc<str> = v27.clone();
            let v28: Rc<str> = v28.clone();
            let v29: Rc<RefCell<Vec<u8>>> = put_str_4(v27);
            let v30: Rc<RefCell<Vec<u8>>> = put_str_4(v28);
            cat_1(v29, v30)
        }
        US0::US0_LeaseReleased(v32, v33) => {
            let v32: Rc<str> = v32.clone();
            let v33: Rc<str> = v33.clone();
            let v34: Rc<RefCell<Vec<u8>>> = put_str_4(v32);
            let v35: Rc<RefCell<Vec<u8>>> = put_str_4(v33);
            cat_1(v34, v35)
        }
        US0::US0_SlotAdmitted(v37, v38) => {
            let v37: i32 = *v37;
            let v38: Rc<str> = v38.clone();
            let v39: Rc<RefCell<Vec<u8>>> = put_i32_8(v37);
            let v40: Rc<RefCell<Vec<u8>>> = put_str_4(v38);
            cat_1(v39, v40)
        }
        US0::US0_SlotReleased(v42, v43) => {
            let v42: i32 = *v42;
            let v43: Rc<str> = v43.clone();
            let v44: Rc<RefCell<Vec<u8>>> = put_i32_8(v42);
            let v45: Rc<RefCell<Vec<u8>>> = put_str_4(v43);
            cat_1(v44, v45)
        }
        US0::US0_Toolchain(v55, v56, v57, v58) => {
            let v55: Rc<str> = v55.clone();
            let v56: Rc<str> = v56.clone();
            let v57: Rc<str> = v57.clone();
            let v58: Rc<str> = v58.clone();
            let v59: Rc<RefCell<Vec<u8>>> = put_str_4(v55);
            let v60: Rc<RefCell<Vec<u8>>> = put_str_4(v56);
            let v61: Rc<RefCell<Vec<u8>>> = put_str_4(v57);
            let v62: Rc<RefCell<Vec<u8>>> = put_str_4(v58);
            let v63: Rc<RefCell<Vec<u8>>> = cat_1(v61, v62);
            let v64: Rc<RefCell<Vec<u8>>> = cat_1(v60, v63);
            cat_1(v59, v64)
        }
    }
}
fn encode_0(mut v0: u64, mut v1: Rc<str>, mut v2: US0) -> Rc<RefCell<Vec<u8>>> {
    let v3: Rc<RefCell<Vec<u8>>> = put_u64_3(v0);
    let v4: Rc<RefCell<Vec<u8>>> = put_str_4(v1);
    let v5: u8 = body_tag_6(v2.clone());
    let mut v6: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(vec![v5]));
    let v7: Rc<RefCell<Vec<u8>>> = body_bytes_7(v2);
    let v8: Rc<RefCell<Vec<u8>>> = cat_1(v6, v7);
    let v9: Rc<RefCell<Vec<u8>>> = cat_1(v4, v8);
    cat_1(v3, v9)
}
fn closure0() -> Rc<dyn Fn(u64, Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<u8>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<u8>>>> = Rc::new(move |mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<RefCell<Vec<u8>>> {
        let v4: US0 = US0::US0_JobStarted(v2.clone(), v3.clone());
        encode_0(v0, v1.clone(), v4.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(u64, Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>) -> Rc<RefCell<Vec<u8>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>) -> Rc<RefCell<Vec<u8>>>> = Rc::new(move |mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>| -> Rc<RefCell<Vec<u8>>> {
        let mut v6: US0 = US0::US0_JobStep(v2.clone(), v3.clone(), v4, v5.clone());
        encode_0(v0, v1.clone(), v6.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(u64, Rc<str>, Rc<str>, i32) -> Rc<RefCell<Vec<u8>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, Rc<str>, Rc<str>, i32) -> Rc<RefCell<Vec<u8>>>> = Rc::new(move |mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32| -> Rc<RefCell<Vec<u8>>> {
        let v4: US0 = US0::US0_JobDone(v2.clone(), v3);
        encode_0(v0, v1.clone(), v4.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(u64, Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<u8>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<u8>>>> = Rc::new(move |mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<RefCell<Vec<u8>>> {
        let v4: US0 = US0::US0_JobStop(v2.clone(), v3.clone());
        encode_0(v0, v1.clone(), v4.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(u64, Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<u8>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<u8>>>> = Rc::new(move |mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<RefCell<Vec<u8>>> {
        let v4: US0 = US0::US0_LeaseAcquired(v2.clone(), v3.clone());
        encode_0(v0, v1.clone(), v4.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn(u64, Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<u8>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<u8>>>> = Rc::new(move |mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<RefCell<Vec<u8>>> {
        let v4: US0 = US0::US0_LeaseReleased(v2.clone(), v3.clone());
        encode_0(v0, v1.clone(), v4.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn(u64, Rc<str>, i32, Rc<str>) -> Rc<RefCell<Vec<u8>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, Rc<str>, i32, Rc<str>) -> Rc<RefCell<Vec<u8>>>> = Rc::new(move |mut v0: u64, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>| -> Rc<RefCell<Vec<u8>>> {
        let v4: US0 = US0::US0_SlotAdmitted(v2, v3.clone());
        encode_0(v0, v1.clone(), v4.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(u64, Rc<str>, i32, Rc<str>) -> Rc<RefCell<Vec<u8>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, Rc<str>, i32, Rc<str>) -> Rc<RefCell<Vec<u8>>>> = Rc::new(move |mut v0: u64, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>| -> Rc<RefCell<Vec<u8>>> {
        let v4: US0 = US0::US0_SlotReleased(v2, v3.clone());
        encode_0(v0, v1.clone(), v4.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(u64, Rc<str>, i32, i32, i32) -> Rc<RefCell<Vec<u8>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, Rc<str>, i32, i32, i32) -> Rc<RefCell<Vec<u8>>>> = Rc::new(move |mut v0: u64, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32| -> Rc<RefCell<Vec<u8>>> {
        let v5: US0 = US0::US0_Health(v2, v3, v4);
        encode_0(v0, v1.clone(), v5.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn(u64, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<u8>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<u8>>>> = Rc::new(move |mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>| -> Rc<RefCell<Vec<u8>>> {
        let mut v6: US0 = US0::US0_Toolchain(v2.clone(), v3.clone(), v4.clone(), v5.clone());
        encode_0(v0, v1.clone(), v6.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn iggy_job_started(v0: u64, v1: &str, v2: &str, v3: &str) -> Rc<RefCell<Vec<u8>>> {
    closure0()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub fn iggy_job_step(v0: u64, v1: &str, v2: &str, v3: &str, v4: i32, v5: &str) -> Rc<RefCell<Vec<u8>>> {
    closure1()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4, Rc::<str>::from(v5))
}
pub fn iggy_job_done(v0: u64, v1: &str, v2: &str, v3: i32) -> Rc<RefCell<Vec<u8>>> {
    closure2()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), v3)
}
pub fn iggy_job_stop(v0: u64, v1: &str, v2: &str, v3: &str) -> Rc<RefCell<Vec<u8>>> {
    closure3()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub fn iggy_lease_acquired(v0: u64, v1: &str, v2: &str, v3: &str) -> Rc<RefCell<Vec<u8>>> {
    closure4()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub fn iggy_lease_released(v0: u64, v1: &str, v2: &str, v3: &str) -> Rc<RefCell<Vec<u8>>> {
    closure5()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub fn iggy_slot_admitted(v0: u64, v1: &str, v2: i32, v3: &str) -> Rc<RefCell<Vec<u8>>> {
    closure6()(v0, Rc::<str>::from(v1), v2, Rc::<str>::from(v3))
}
pub fn iggy_slot_released(v0: u64, v1: &str, v2: i32, v3: &str) -> Rc<RefCell<Vec<u8>>> {
    closure7()(v0, Rc::<str>::from(v1), v2, Rc::<str>::from(v3))
}
pub fn iggy_health(v0: u64, v1: &str, v2: i32, v3: i32, v4: i32) -> Rc<RefCell<Vec<u8>>> {
    closure8()(v0, Rc::<str>::from(v1), v2, v3, v4)
}
pub fn iggy_toolchain(v0: u64, v1: &str, v2: &str, v3: &str, v4: &str, v5: &str) -> Rc<RefCell<Vec<u8>>> {
    closure9()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), Rc::<str>::from(v5))
}
