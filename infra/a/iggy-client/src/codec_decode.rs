#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_JobStarted(Rc<str>, Rc<str>),
    US1_JobStep(Rc<str>, Rc<str>, i32, Rc<str>),
    US1_JobDone(Rc<str>, i32),
    US1_JobStop(Rc<str>, Rc<str>),
    US1_LeaseAcquired(Rc<str>, Rc<str>),
    US1_LeaseReleased(Rc<str>, Rc<str>),
    US1_SlotAdmitted(i32, Rc<str>),
    US1_SlotReleased(i32, Rc<str>),
    US1_Health(i32, i32, i32),
    US1_Toolchain(Rc<str>, Rc<str>, Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum US0 {
    US0_Decoded(u64, Rc<str>, US1),
    US0_Truncated,
    US0_UnknownTag(u8),
    US0_NotUtf8,
    US0_TrailingBytes(i32),
}
#[derive(Clone)]
enum US2 {
    US2_RD(US1, i32),
    US2_RTrunc,
    US2_RUtf8,
}
#[derive(Clone)]
enum US3 {
    US3_RD(u64, i32),
    US3_RTrunc,
    US3_RUtf8,
}
#[derive(Clone)]
enum US4 {
    US4_RD(Rc<str>, i32),
    US4_RTrunc,
    US4_RUtf8,
}
struct Mut0 { l0: i32 }
#[derive(Clone)]
enum US5 {
    US5_Some(std::string::String),
    US5_None,
}
#[derive(Clone)]
enum US6 {
    US6_RD(u8, i32),
    US6_RTrunc,
    US6_RUtf8,
}
#[derive(Clone)]
enum US7 {
    US7_RD(i32, i32),
    US7_RTrunc,
    US7_RUtf8,
}
fn finish_3(mut v0: u64, mut v1: Rc<str>, mut v2: US1, mut v3: i32, mut v4: i32) -> US0 {
    let v5: bool = v3 == v4;
    if v5 {
        US0::US0_Decoded(v0, v1, v2)
    } else {
        let v7: i32 = v4.wrapping_sub(v3);
        US0::US0_TrailingBytes(v7)
    }
}
fn complete_2(mut v0: u64, mut v1: Rc<str>, mut v2: i32, mut v3: US2) -> US0 {
    match &v3 {
        US2::US2_RD(v6, v7) => {
            let mut v6: US1 = v6.clone();
            let v7: i32 = *v7;
            finish_3(v0, v1, v6, v7, v2)
        }
        US2::US2_RTrunc => {
            US0::US0_Truncated
        }
        US2::US2_RUtf8 => {
            US0::US0_NotUtf8
        }
    }
}
fn read_u32_6(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> u32 {
    let v2: u8 = v0.clone().borrow()[v1 as usize].clone();
    let v3: u32 = (v2 as u32);
    let mut v4: u32 = v3.wrapping_shl((24i32) as u32);
    let v5: i32 = v1.wrapping_add(1i32);
    let mut v6: u8 = v0.clone().borrow()[v5 as usize].clone();
    let v7: u32 = (v6 as u32);
    let v8: u32 = v7.wrapping_shl((16i32) as u32);
    let v9: u32 = v4 | v8;
    let v10: i32 = v1.wrapping_add(2i32);
    let v11: u8 = v0.clone().borrow()[v10 as usize].clone();
    let v12: u32 = (v11 as u32);
    let v13: u32 = v12.wrapping_shl((8i32) as u32);
    let mut v14: u32 = v9 | v13;
    let v15: i32 = v1.wrapping_add(3i32);
    let v16: u8 = v0.borrow()[v15 as usize].clone();
    let v17: u32 = (v16 as u32);
    let mut v18: u32 = v14 | v17;
    v18
}
fn read_u64_5(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> u64 {
    let v2: u32 = read_u32_6(v0.clone(), v1);
    let v3: u64 = (v2 as u64);
    let mut v4: u64 = v3.wrapping_shl((32i32) as u32);
    let v5: i32 = v1.wrapping_add(4i32);
    let mut v6: u32 = read_u32_6(v0, v5);
    let v7: u64 = (v6 as u64);
    let v8: u64 = v4 | v7;
    v8
}
fn rd_u64_4(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: i32) -> US3 {
    let v3: bool = v1 >= 0i32;
    let v5: bool = if v3 {
        let mut v4: bool = v1 <= v2;
        v4
    } else {
        false
    };
    let v8: bool = if v5 {
        let mut v6: i32 = v2.wrapping_sub(v1);
        let v7: bool = 8i32 <= v6;
        v7
    } else {
        false
    };
    if v8 {
        let v9: u64 = read_u64_5(v0, v1);
        let v10: i32 = v1.wrapping_add(8i32);
        US3::US3_RD(v9, v10)
    } else {
        US3::US3_RTrunc
    }
}
fn method9(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0.clone();
    let v3: bool = v2 < v0;
    v3
}
fn method10(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method11(mut v0: Option<std::string::String>) -> Option<std::string::String> {
    v0
}
fn closure1() -> Rc<dyn Fn((std::string::String)) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((std::string::String)) -> US5> = Rc::new(move |mut v0: (std::string::String)| -> US5 {
        let mut v1: std::string::String = (v0);
        US5::US5_Some(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn rd_str_utf8_8(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: i32) -> US4 {
    let v3: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(vec![<u8>::default(); v2 as usize]));
    let mut v4: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
    while method9(v2, v4.clone()) {
        let mut v6: i32 = v4.borrow().l0.clone();
        let v7: i32 = v1.wrapping_add(4i32);
        let v8: i32 = v7.wrapping_add(v6);
        let v9: u8 = v0.clone().borrow()[v8 as usize].clone();
        v3.clone().borrow_mut()[v6 as usize] = v9;
        let v10: i32 = v6.wrapping_add(1i32);
        v4.borrow_mut().l0 = v10;
        ()
    };
    let v11: Vec<u8> = (v3).borrow().clone();
    let v12: Vec<u8> = method10(v11.clone());
    let mut v14: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v12);
    let v16: Option<std::string::String> = v14.ok();
    let v17: Option<std::string::String> = method11(v16.clone());
    let mut v18: Rc<dyn Fn((std::string::String)) -> US5> = closure1();
    let mut v19: Option<US5> = v17.map(|x| v18(x));
    let v21: US5 = US5::US5_None;
    let v22: US5 = v21.clone();
    let v23: US5 = v19.unwrap_or(v22);
    match &v23 {
        US5::US5_None => {
            US4::US4_RUtf8
        }
        US5::US5_Some(v24) => {
            let v24: std::string::String = v24.clone();
            let v26: Rc<str> = Rc::<str>::from(String::as_str(&v24));
            let v27: i32 = v1.wrapping_add(4i32);
            let v28: i32 = v27.wrapping_add(v2);
            US4::US4_RD(v26.clone(), v28)
        }
    }
}
fn rd_str_7(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: i32) -> US4 {
    let v3: bool = v1 >= 0i32;
    let v5: bool = if v3 {
        let mut v4: bool = v1 <= v2;
        v4
    } else {
        false
    };
    let v8: bool = if v5 {
        let mut v6: i32 = v2.wrapping_sub(v1);
        let v7: bool = 4i32 <= v6;
        v7
    } else {
        false
    };
    let v11: i32 = if v8 {
        let v9: u32 = read_u32_6(v0.clone(), v1);
        let v10: i32 = (v9 as i32);
        v10
    } else {
        0i32
    };
    let v21: bool = if v8 {
        let v12: i32 = v1.wrapping_add(4i32);
        let v13: bool = v12 >= 0i32;
        let v15: bool = if v13 {
            let mut v14: bool = v11 >= 0i32;
            v14
        } else {
            false
        };
        let v17: bool = if v15 {
            let v16: bool = v12 <= v2;
            v16
        } else {
            false
        };
        if v17 {
            let mut v18: i32 = v2.wrapping_sub(v12);
            let mut v19: bool = v11 <= v18;
            v19
        } else {
            false
        }
    } else {
        false
    };
    if v21 {
        rd_str_utf8_8(v0, v1, v11)
    } else {
        US4::US4_RTrunc
    }
}
fn rd_u8_12(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: i32) -> US6 {
    let v3: bool = v1 >= 0i32;
    let v5: bool = if v3 {
        let mut v4: bool = v1 <= v2;
        v4
    } else {
        false
    };
    let v8: bool = if v5 {
        let mut v6: i32 = v2.wrapping_sub(v1);
        let v7: bool = 1i32 <= v6;
        v7
    } else {
        false
    };
    if v8 {
        let v9: u8 = v0.borrow()[v1 as usize].clone();
        let v10: i32 = v1.wrapping_add(1i32);
        US6::US6_RD(v9, v10)
    } else {
        US6::US6_RTrunc
    }
}
fn closure5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v2: i32| -> US2 {
        let v3: US1 = US1::US1_JobStarted(v0.clone(), v1.clone());
        US2::US2_RD(v3.clone(), v2)
    })
}
fn closure4(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v1: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure5(v0.clone(), v1.clone())
    })
}
fn rd_bind_13(mut v0: US4, mut v1: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>>) -> US2 {
    match &v0 {
        US4::US4_RD(v4, v5) => {
            let mut v4: Rc<str> = v4.clone();
            let v5: i32 = *v5;
            let mut v6: Rc<dyn Fn(i32) -> US2> = v1(v4);
            v6(v5)
        }
        US4::US4_RTrunc => {
            US2::US2_RTrunc
        }
        US4::US4_RUtf8 => {
            US2::US2_RUtf8
        }
    }
}
fn closure3(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v3: i32| -> US2 {
        let mut v4: US4 = rd_str_7(v0.clone(), v3, v1);
        let v5: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure4(v2.clone());
        rd_bind_13(v4.clone(), v5.clone())
    })
}
fn closure2(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v2: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure3(v0.clone(), v1, v2.clone())
    })
}
fn read_i32_15(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> i32 {
    let v2: u32 = read_u32_6(v0, v1);
    let v3: i32 = (v2 as i32);
    v3
}
fn rd_i32_14(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: i32) -> US7 {
    let v3: bool = v1 >= 0i32;
    let v5: bool = if v3 {
        let mut v4: bool = v1 <= v2;
        v4
    } else {
        false
    };
    let v8: bool = if v5 {
        let mut v6: i32 = v2.wrapping_sub(v1);
        let v7: bool = 4i32 <= v6;
        v7
    } else {
        false
    };
    if v8 {
        let v9: i32 = read_i32_15(v0, v1);
        let v10: i32 = v1.wrapping_add(4i32);
        US7::US7_RD(v9, v10)
    } else {
        US7::US7_RTrunc
    }
}
fn closure13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v4: i32| -> US2 {
        let v5: US1 = US1::US1_JobStep(v0.clone(), v1.clone(), v2, v3.clone());
        US2::US2_RD(v5.clone(), v4)
    })
}
fn closure12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v3: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure13(v0.clone(), v1.clone(), v2, v3.clone())
    })
}
fn closure11(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v5: i32| -> US2 {
        let mut v6: US4 = rd_str_7(v0.clone(), v5, v1);
        let v7: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure12(v2.clone(), v3.clone(), v4);
        rd_bind_13(v6.clone(), v7.clone())
    })
}
fn closure10(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v4: i32| -> Rc<dyn Fn(i32) -> US2> {
        closure11(v0.clone(), v1, v2.clone(), v3.clone(), v4)
    })
}
fn rd_bind_16(mut v0: US7, mut v1: Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>>) -> US2 {
    match &v0 {
        US7::US7_RD(v4, v5) => {
            let mut v4: i32 = *v4;
            let v5: i32 = *v5;
            let mut v6: Rc<dyn Fn(i32) -> US2> = v1(v4);
            v6(v5)
        }
        US7::US7_RTrunc => {
            US2::US2_RTrunc
        }
        US7::US7_RUtf8 => {
            US2::US2_RUtf8
        }
    }
}
fn closure9(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v4: i32| -> US2 {
        let v5: US7 = rd_i32_14(v0.clone(), v4, v1);
        let mut v6: Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> = closure10(v0.clone(), v1, v2.clone(), v3.clone());
        rd_bind_16(v5.clone(), v6.clone())
    })
}
fn closure8(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v3: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure9(v0.clone(), v1, v2.clone(), v3.clone())
    })
}
fn closure7(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v3: i32| -> US2 {
        let mut v4: US4 = rd_str_7(v0.clone(), v3, v1);
        let v5: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure8(v0.clone(), v1, v2.clone());
        rd_bind_13(v4.clone(), v5.clone())
    })
}
fn closure6(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v2: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure7(v0.clone(), v1, v2.clone())
    })
}
fn closure17(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v2: i32| -> US2 {
        let v3: US1 = US1::US1_JobDone(v0.clone(), v1);
        US2::US2_RD(v3.clone(), v2)
    })
}
fn closure16(mut v0: Rc<str>) -> Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v1: i32| -> Rc<dyn Fn(i32) -> US2> {
        closure17(v0.clone(), v1)
    })
}
fn closure15(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v3: i32| -> US2 {
        let mut v4: US7 = rd_i32_14(v0.clone(), v3, v1);
        let v5: Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> = closure16(v2.clone());
        rd_bind_16(v4.clone(), v5.clone())
    })
}
fn closure14(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v2: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure15(v0.clone(), v1, v2.clone())
    })
}
fn closure21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v2: i32| -> US2 {
        let v3: US1 = US1::US1_JobStop(v0.clone(), v1.clone());
        US2::US2_RD(v3.clone(), v2)
    })
}
fn closure20(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v1: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure21(v0.clone(), v1.clone())
    })
}
fn closure19(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v3: i32| -> US2 {
        let mut v4: US4 = rd_str_7(v0.clone(), v3, v1);
        let v5: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure20(v2.clone());
        rd_bind_13(v4.clone(), v5.clone())
    })
}
fn closure18(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v2: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure19(v0.clone(), v1, v2.clone())
    })
}
fn closure25(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v2: i32| -> US2 {
        let v3: US1 = US1::US1_LeaseAcquired(v0.clone(), v1.clone());
        US2::US2_RD(v3.clone(), v2)
    })
}
fn closure24(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v1: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure25(v0.clone(), v1.clone())
    })
}
fn closure23(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v3: i32| -> US2 {
        let mut v4: US4 = rd_str_7(v0.clone(), v3, v1);
        let v5: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure24(v2.clone());
        rd_bind_13(v4.clone(), v5.clone())
    })
}
fn closure22(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v2: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure23(v0.clone(), v1, v2.clone())
    })
}
fn closure29(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v2: i32| -> US2 {
        let v3: US1 = US1::US1_LeaseReleased(v0.clone(), v1.clone());
        US2::US2_RD(v3.clone(), v2)
    })
}
fn closure28(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v1: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure29(v0.clone(), v1.clone())
    })
}
fn closure27(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v3: i32| -> US2 {
        let mut v4: US4 = rd_str_7(v0.clone(), v3, v1);
        let v5: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure28(v2.clone());
        rd_bind_13(v4.clone(), v5.clone())
    })
}
fn closure26(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v2: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure27(v0.clone(), v1, v2.clone())
    })
}
fn closure33(mut v0: i32, mut v1: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v2: i32| -> US2 {
        let v3: US1 = US1::US1_SlotAdmitted(v0, v1.clone());
        US2::US2_RD(v3.clone(), v2)
    })
}
fn closure32(mut v0: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v1: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure33(v0, v1.clone())
    })
}
fn closure31(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: i32) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v3: i32| -> US2 {
        let mut v4: US4 = rd_str_7(v0.clone(), v3, v1);
        let v5: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure32(v2);
        rd_bind_13(v4.clone(), v5.clone())
    })
}
fn closure30(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v2: i32| -> Rc<dyn Fn(i32) -> US2> {
        closure31(v0.clone(), v1, v2)
    })
}
fn closure37(mut v0: i32, mut v1: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v2: i32| -> US2 {
        let v3: US1 = US1::US1_SlotReleased(v0, v1.clone());
        US2::US2_RD(v3.clone(), v2)
    })
}
fn closure36(mut v0: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v1: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure37(v0, v1.clone())
    })
}
fn closure35(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: i32) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v3: i32| -> US2 {
        let mut v4: US4 = rd_str_7(v0.clone(), v3, v1);
        let v5: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure36(v2);
        rd_bind_13(v4.clone(), v5.clone())
    })
}
fn closure34(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v2: i32| -> Rc<dyn Fn(i32) -> US2> {
        closure35(v0.clone(), v1, v2)
    })
}
fn closure43(mut v0: i32, mut v1: i32, mut v2: i32) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v3: i32| -> US2 {
        let mut v4: US1 = US1::US1_Health(v0, v1, v2);
        US2::US2_RD(v4.clone(), v3)
    })
}
fn closure42(mut v0: i32, mut v1: i32) -> Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v2: i32| -> Rc<dyn Fn(i32) -> US2> {
        closure43(v0, v1, v2)
    })
}
fn closure41(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v4: i32| -> US2 {
        let v5: US7 = rd_i32_14(v0.clone(), v4, v1);
        let mut v6: Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> = closure42(v2, v3);
        rd_bind_16(v5.clone(), v6.clone())
    })
}
fn closure40(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: i32) -> Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v3: i32| -> Rc<dyn Fn(i32) -> US2> {
        closure41(v0.clone(), v1, v2, v3)
    })
}
fn closure39(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: i32) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v3: i32| -> US2 {
        let mut v4: US7 = rd_i32_14(v0.clone(), v3, v1);
        let v5: Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> = closure40(v0.clone(), v1, v2);
        rd_bind_16(v4.clone(), v5.clone())
    })
}
fn closure38(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v2: i32| -> Rc<dyn Fn(i32) -> US2> {
        closure39(v0.clone(), v1, v2)
    })
}
fn closure51(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v4: i32| -> US2 {
        let v5: US1 = US1::US1_Toolchain(v0.clone(), v1.clone(), v2.clone(), v3.clone());
        US2::US2_RD(v5.clone(), v4)
    })
}
fn closure50(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v3: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure51(v0.clone(), v1.clone(), v2.clone(), v3.clone())
    })
}
fn closure49(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v5: i32| -> US2 {
        let mut v6: US4 = rd_str_7(v0.clone(), v5, v1);
        let v7: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure50(v2.clone(), v3.clone(), v4.clone());
        rd_bind_13(v6.clone(), v7.clone())
    })
}
fn closure48(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v4: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure49(v0.clone(), v1, v2.clone(), v3.clone(), v4.clone())
    })
}
fn closure47(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v4: i32| -> US2 {
        let v5: US4 = rd_str_7(v0.clone(), v4, v1);
        let mut v6: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure48(v0.clone(), v1, v2.clone(), v3.clone());
        rd_bind_13(v5.clone(), v6.clone())
    })
}
fn closure46(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v3: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure47(v0.clone(), v1, v2.clone(), v3.clone())
    })
}
fn closure45(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<dyn Fn(i32) -> US2> {
    Rc::new(move |mut v3: i32| -> US2 {
        let mut v4: US4 = rd_str_7(v0.clone(), v3, v1);
        let v5: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure46(v0.clone(), v1, v2.clone());
        rd_bind_13(v4.clone(), v5.clone())
    })
}
fn closure44(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> {
    Rc::new(move |mut v2: Rc<str>| -> Rc<dyn Fn(i32) -> US2> {
        closure45(v0.clone(), v1, v2.clone())
    })
}
fn decode_1(mut v0: Rc<RefCell<Vec<u8>>>) -> US0 {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let v2: i32 = 0i32;
    let v3: US3 = rd_u64_4(v0.clone(), v2, v1);
    match &v3 {
        US3::US3_RD(v6, v7) => {
            let mut v6: u64 = *v6;
            let v7: i32 = *v7;
            let v8: US4 = rd_str_7(v0.clone(), v7, v1);
            match &v8 {
                US4::US4_RD(v11, v12) => {
                    let v11: Rc<str> = v11.clone();
                    let v12: i32 = *v12;
                    let v13: US6 = rd_u8_12(v0.clone(), v12, v1);
                    match &v13 {
                        US6::US6_RD(v16, v17) => {
                            let v16: u8 = *v16;
                            let v17: i32 = *v17;
                            let mut v18: bool = 0u8 == v16;
                            if v18 {
                                let mut v19: US4 = rd_str_7(v0.clone(), v17, v1);
                                let v20: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure2(v0.clone(), v1);
                                let v21: US2 = rd_bind_13(v19, v20);
                                complete_2(v6, v11.clone(), v1, v21)
                            } else {
                                let v23: bool = 1u8 == v16;
                                if v23 {
                                    let v24: US4 = rd_str_7(v0.clone(), v17, v1);
                                    let v25: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure6(v0.clone(), v1);
                                    let v26: US2 = rd_bind_13(v24, v25);
                                    complete_2(v6, v11.clone(), v1, v26)
                                } else {
                                    let v28: bool = 2u8 == v16;
                                    if v28 {
                                        let v29: US4 = rd_str_7(v0.clone(), v17, v1);
                                        let v30: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure14(v0.clone(), v1);
                                        let v31: US2 = rd_bind_13(v29, v30);
                                        complete_2(v6, v11.clone(), v1, v31)
                                    } else {
                                        let v33: bool = 3u8 == v16;
                                        if v33 {
                                            let v34: US4 = rd_str_7(v0.clone(), v17, v1);
                                            let v35: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure18(v0.clone(), v1);
                                            let v36: US2 = rd_bind_13(v34, v35);
                                            complete_2(v6, v11.clone(), v1, v36)
                                        } else {
                                            let v38: bool = 4u8 == v16;
                                            if v38 {
                                                let v39: US4 = rd_str_7(v0.clone(), v17, v1);
                                                let v40: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure22(v0.clone(), v1);
                                                let v41: US2 = rd_bind_13(v39, v40);
                                                complete_2(v6, v11.clone(), v1, v41)
                                            } else {
                                                let v43: bool = 5u8 == v16;
                                                if v43 {
                                                    let v44: US4 = rd_str_7(v0.clone(), v17, v1);
                                                    let v45: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure26(v0.clone(), v1);
                                                    let v46: US2 = rd_bind_13(v44, v45);
                                                    complete_2(v6, v11.clone(), v1, v46)
                                                } else {
                                                    let v48: bool = 6u8 == v16;
                                                    if v48 {
                                                        let v49: US7 = rd_i32_14(v0.clone(), v17, v1);
                                                        let v50: Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> = closure30(v0.clone(), v1);
                                                        let v51: US2 = rd_bind_16(v49, v50);
                                                        complete_2(v6, v11.clone(), v1, v51)
                                                    } else {
                                                        let v53: bool = 7u8 == v16;
                                                        if v53 {
                                                            let v54: US7 = rd_i32_14(v0.clone(), v17, v1);
                                                            let v55: Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> = closure34(v0.clone(), v1);
                                                            let v56: US2 = rd_bind_16(v54, v55);
                                                            complete_2(v6, v11.clone(), v1, v56)
                                                        } else {
                                                            let v58: bool = 8u8 == v16;
                                                            if v58 {
                                                                let v59: US7 = rd_i32_14(v0.clone(), v17, v1);
                                                                let v60: Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> US2>> = closure38(v0.clone(), v1);
                                                                let v61: US2 = rd_bind_16(v59, v60);
                                                                complete_2(v6, v11.clone(), v1, v61)
                                                            } else {
                                                                let v63: bool = 9u8 == v16;
                                                                if v63 {
                                                                    let v64: US4 = rd_str_7(v0.clone(), v17, v1);
                                                                    let v65: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(i32) -> US2>> = closure44(v0, v1);
                                                                    let v66: US2 = rd_bind_13(v64, v65);
                                                                    complete_2(v6, v11, v1, v66)
                                                                } else {
                                                                    US0::US0_UnknownTag(v16)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        US6::US6_RTrunc => {
                            US0::US0_Truncated
                        }
                        US6::US6_RUtf8 => {
                            US0::US0_NotUtf8
                        }
                    }
                }
                US4::US4_RTrunc => {
                    US0::US0_Truncated
                }
                US4::US4_RUtf8 => {
                    US0::US0_NotUtf8
                }
            }
        }
        US3::US3_RTrunc => {
            US0::US0_Truncated
        }
        US3::US3_RUtf8 => {
            US0::US0_NotUtf8
        }
    }
}
fn fields_of_17(mut v0: u64, mut v1: Rc<str>, mut v2: US1) -> (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    let (mut v145, mut v146, mut v147, mut v148, mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v2 {
        US1::US1_JobStarted(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let mut v4: Rc<str> = v4.clone();
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (0i32, 0i32, v0, v1.clone(), 0i32, v3.clone(), v4.clone(), v5.clone(), v5.clone(), 0i32, 0i32, 0i32)
        }
        _ => {
            let (mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143, mut v144): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v2 {
                US1::US1_JobStep(v6, v7, v8, v9) => {
                    let mut v6: Rc<str> = v6.clone();
                    let v7: Rc<str> = v7.clone();
                    let v8: i32 = *v8;
                    let v9: Rc<str> = v9.clone();
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (0i32, 0i32, v0, v1.clone(), 1i32, v6.clone(), v7.clone(), v9.clone(), v10.clone(), v8, 0i32, 0i32)
                }
                _ => {
                    let (mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130, mut v131, mut v132): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v2 {
                        US1::US1_JobDone(v11, v12) => {
                            let v11: Rc<str> = v11.clone();
                            let v12: i32 = *v12;
                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (0i32, 0i32, v0, v1.clone(), 2i32, v11.clone(), v13.clone(), v13.clone(), v13.clone(), v12, 0i32, 0i32)
                        }
                        _ => {
                            let (mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v2 {
                                US1::US1_JobStop(v14, v15) => {
                                    let mut v14: Rc<str> = v14.clone();
                                    let v15: Rc<str> = v15.clone();
                                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    (0i32, 0i32, v0, v1.clone(), 3i32, v14.clone(), v15.clone(), v16.clone(), v16.clone(), 0i32, 0i32, 0i32)
                                }
                                _ => {
                                    let (mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v2 {
                                        US1::US1_LeaseAcquired(v17, v18) => {
                                            let v17: Rc<str> = v17.clone();
                                            let mut v18: Rc<str> = v18.clone();
                                            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            (0i32, 0i32, v0, v1.clone(), 4i32, v17.clone(), v18.clone(), v19.clone(), v19.clone(), 0i32, 0i32, 0i32)
                                        }
                                        _ => {
                                            let (mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v2 {
                                                US1::US1_LeaseReleased(v20, v21) => {
                                                    let v20: Rc<str> = v20.clone();
                                                    let v21: Rc<str> = v21.clone();
                                                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    (0i32, 0i32, v0, v1.clone(), 5i32, v20.clone(), v21.clone(), v22.clone(), v22.clone(), 0i32, 0i32, 0i32)
                                                }
                                                _ => {
                                                    let (mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v2 {
                                                        US1::US1_SlotAdmitted(v23, v24) => {
                                                            let v23: i32 = *v23;
                                                            let v24: Rc<str> = v24.clone();
                                                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                            (0i32, 0i32, v0, v1.clone(), 6i32, v24.clone(), v25.clone(), v25.clone(), v25.clone(), v23, 0i32, 0i32)
                                                        }
                                                        _ => {
                                                            let (mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v2 {
                                                                US1::US1_SlotReleased(v26, v27) => {
                                                                    let v26: i32 = *v26;
                                                                    let v27: Rc<str> = v27.clone();
                                                                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                    (0i32, 0i32, v0, v1.clone(), 7i32, v27.clone(), v28.clone(), v28.clone(), v28.clone(), v26, 0i32, 0i32)
                                                                }
                                                                _ => {
                                                                    let (mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v2 {
                                                                        US1::US1_Health(v29, v30, v31) => {
                                                                            let v29: i32 = *v29;
                                                                            let v30: i32 = *v30;
                                                                            let v31: i32 = *v31;
                                                                            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                            (0i32, 0i32, v0, v1.clone(), 8i32, v32.clone(), v32.clone(), v32.clone(), v32.clone(), v29, v30, v31)
                                                                        }
                                                                        _ => {
                                                                            let (mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v2 {
                                                                                US1::US1_Toolchain(v33, v34, v35, v36) => {
                                                                                    let v33: Rc<str> = v33.clone();
                                                                                    let v34: Rc<str> = v34.clone();
                                                                                    let v35: Rc<str> = v35.clone();
                                                                                    let v36: Rc<str> = v36.clone();
                                                                                    (0i32, 0i32, v0, v1.clone(), 9i32, v33.clone(), v34.clone(), v35.clone(), v36.clone(), 0i32, 0i32, 0i32)
                                                                                }
                                                                                _ => unreachable!(),
                                                                            };
                                                                            (v37, v38, v39, v40.clone(), v41, v42.clone(), v43.clone(), v44.clone(), v45.clone(), v46, v47, v48)
                                                                        }
                                                                    };
                                                                    (v49, v50, v51, v52.clone(), v53, v54.clone(), v55.clone(), v56.clone(), v57.clone(), v58, v59, v60)
                                                                }
                                                            };
                                                            (v61, v62, v63, v64.clone(), v65, v66.clone(), v67.clone(), v68.clone(), v69.clone(), v70, v71, v72)
                                                        }
                                                    };
                                                    (v73, v74, v75, v76.clone(), v77, v78.clone(), v79.clone(), v80.clone(), v81.clone(), v82, v83, v84)
                                                }
                                            };
                                            (v85, v86, v87, v88.clone(), v89, v90.clone(), v91.clone(), v92.clone(), v93.clone(), v94, v95, v96)
                                        }
                                    };
                                    (v97, v98, v99, v100.clone(), v101, v102.clone(), v103.clone(), v104.clone(), v105.clone(), v106, v107, v108)
                                }
                            };
                            (v109, v110, v111, v112.clone(), v113, v114.clone(), v115.clone(), v116.clone(), v117.clone(), v118, v119, v120)
                        }
                    };
                    (v121, v122, v123, v124.clone(), v125, v126.clone(), v127.clone(), v128.clone(), v129.clone(), v130, v131, v132)
                }
            };
            (v133, v134, v135, v136.clone(), v137, v138.clone(), v139.clone(), v140.clone(), v141.clone(), v142, v143, v144)
        }
    };
    (v145, v146, v147, v148.clone(), v149, v150.clone(), v151.clone(), v152.clone(), v153.clone(), v154, v155, v156)
}
fn decoded_0(mut v0: Rc<RefCell<Vec<u8>>>) -> (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    let mut v1: US0 = decode_1(v0.clone());
    let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v1 {
        US0::US0_Decoded(v2, v3, v4) => {
            let v2: u64 = *v2;
            let v3: Rc<str> = v3.clone();
            let mut v4: US1 = v4.clone();
            fields_of_17(v2, v3.clone(), v4.clone())
        }
        _ => {
            let (mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v1 {
                US0::US0_Truncated => {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (1i32, 0i32, 0u64, v17.clone(), -1i32, v17.clone(), v17.clone(), v17.clone(), v17.clone(), 0i32, 0i32, 0i32)
                }
                _ => {
                    let (mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v1 {
                        US0::US0_UnknownTag(v18) => {
                            let mut v18: u8 = *v18;
                            let mut v19: i32 = (v18 as i32);
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (2i32, v19, 0u64, v20.clone(), -1i32, v20.clone(), v20.clone(), v20.clone(), v20.clone(), 0i32, 0i32, 0i32)
                        }
                        _ => {
                            let (mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v1 {
                                US0::US0_NotUtf8 => {
                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    (3i32, 0i32, 0u64, v21.clone(), -1i32, v21.clone(), v21.clone(), v21.clone(), v21.clone(), 0i32, 0i32, 0i32)
                                }
                                _ => {
                                    let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = match &v1 {
                                        US0::US0_TrailingBytes(v22) => {
                                            let v22: i32 = *v22;
                                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            (4i32, v22, 0u64, v23.clone(), -1i32, v23.clone(), v23.clone(), v23.clone(), v23.clone(), 0i32, 0i32, 0i32)
                                        }
                                        _ => unreachable!(),
                                    };
                                    (v24, v25, v26, v27.clone(), v28, v29.clone(), v30.clone(), v31.clone(), v32.clone(), v33, v34, v35)
                                }
                            };
                            (v36, v37, v38, v39.clone(), v40, v41.clone(), v42.clone(), v43.clone(), v44.clone(), v45, v46, v47)
                        }
                    };
                    (v48, v49, v50, v51.clone(), v52, v53.clone(), v54.clone(), v55.clone(), v56.clone(), v57, v58, v59)
                }
            };
            (v60, v61, v62, v63.clone(), v64, v65.clone(), v66.clone(), v67.clone(), v68.clone(), v69, v70, v71)
        }
    };
    (v72, v73, v74, v75.clone(), v76, v77.clone(), v78.clone(), v79.clone(), v80.clone(), v81, v82, v83)
}
fn closure0() -> Rc<dyn Fn(Rc<RefCell<Vec<u8>>>) -> (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<RefCell<Vec<u8>>>) -> (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32)> = Rc::new(move |mut v0: Rc<RefCell<Vec<u8>>>| -> (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
        let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12): (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) = decoded_0(v0.clone());
        (v1, v2, v3, v4.clone(), v5, v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10, v11, v12)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn iggy_decode(v0: Rc<RefCell<Vec<u8>>>) -> (i32, i32, u64, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, i32, i32) {
    closure0()(v0)
}
