#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_LightLoad,
    US1_MediumLoad,
    US1_HeavyLoad,
}
#[derive(Clone)]
enum US0 {
    US0_KindField(Rc<str>),
    US0_LoadField(US1),
    US0_EstimateMinutes(i32),
    US0_AssigneeField(Rc<str>),
    US0_PriorityField(Rc<str>),
    US0_UnknownField(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(US0, Rc<UH0>),
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_Some(US0),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US1),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Some(i32),
    US4_None,
}
fn method5(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_4(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn field_json_3(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_AssigneeField(v16) => {
            let v16: Rc<str> = v16;
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"who\":\""); } LIT.with(|lit| lit.clone()) }, v16));
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
            v18
        }
        US0::US0_EstimateMinutes(v13) => {
            let mut v13: i32 = v13;
            let v14: Rc<str> = format_real_4(v13);
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"est\":"); } LIT.with(|lit| lit.clone()) }, v14));
            v15
        }
        US0::US0_KindField(v1) => {
            let v1: Rc<str> = v1;
            let v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"kind\":\""); } LIT.with(|lit| lit.clone()) }, v1));
            let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
            v3
        }
        US0::US0_LoadField(v4) => {
            let mut v4: US1 = v4;
            let v10: Rc<str> = match v4 {
                US1::US1_HeavyLoad => {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy"); } LIT.with(|lit| lit.clone()) };
                    v7
                }
                US1::US1_LightLoad => {
                    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("light"); } LIT.with(|lit| lit.clone()) };
                    v5
                }
                US1::US1_MediumLoad => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("medium"); } LIT.with(|lit| lit.clone()) };
                    v6
                }
            };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"load\":\""); } LIT.with(|lit| lit.clone()) }, v10));
            let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
            v12
        }
        US0::US0_PriorityField(v19) => {
            let v19: Rc<str> = v19;
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"prio\":\""); } LIT.with(|lit| lit.clone()) }, v19));
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
            v21
        }
        US0::US0_UnknownField(v22) => {
            let v22: Rc<str> = v22;
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = Rc::<str>::from(v22.replace(&*v23, &*v24));
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = Rc::<str>::from(v25.replace(&*v26, &*v27));
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"unknown\":\""); } LIT.with(|lit| lit.clone()) }, v28));
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
            v30
        }
    }
}
fn joined_2(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: US0 = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v1 == "";
                let v9: Rc<str> = if v4 {
                    field_json_3(v2)
                } else {
                    let mut v6: Rc<str> = field_json_3(v2);
                    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
                    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
                    v8
                };
                (v0, v1) = (v3, v9);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn fields_json_1(mut v0: Rc<UH0>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = joined_2(v0, v1);
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) }, v2));
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) }));
    v4
}
fn method7(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn tail_after_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
    let v3: Option<usize> = v1.find(&*v2);
    let mut v4: Option<usize> = method7(v3);
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let v8: i32 = v6.unwrap_or(-1i32);
    let v9: i32 = (v0.len() as i32);
    let v10: i32 = v8.wrapping_add(v9);
    let v11: i32 = v10.wrapping_add(1i32);
    let mut v12: bool = v8 < 0i32;
    if v12 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    } else {
        let v14: i32 = (v1.clone().len() as i32);
        let v15: i32 = v14.wrapping_sub(v11);
        let v16: usize = ((v11) as usize);
        let v17: i32 = v11.wrapping_add(v15);
        let v18: usize = ((v17) as usize);
        let v19: Rc<str> = Rc::<str>::from(&(v1)[v16..v18]);
        v19
    }
}
fn load_of_12(mut v0: Rc<str>) -> US3 {
    let v1: bool = &*v0 == "light";
    if v1 {
        let v2: US1 = US1::US1_LightLoad;
        US3::US3_Some(v2)
    } else {
        let mut v4: bool = &*v0 == "medium";
        if v4 {
            let mut v5: US1 = US1::US1_MediumLoad;
            US3::US3_Some(v5)
        } else {
            let v7: bool = &*v0 == "heavy";
            if v7 {
                let v8: US1 = US1::US1_HeavyLoad;
                US3::US3_Some(v8)
            } else {
                US3::US3_None
            }
        }
    }
}
fn method14(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure2() -> Rc<dyn Fn((i32)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US4> = Rc::new(move |mut v0: (i32)| -> US4 {
        let v1: i32 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn minutes_of_13(mut v0: Rc<str>) -> US4 {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: bool = v1 > 1i32;
    let mut v5: bool = if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
        let mut v4: bool = v0.ends_with(&*v3);
        v4
    } else {
        false
    };
    if v5 {
        let mut v6: i32 = v1.wrapping_sub(1i32);
        let v7: usize = ((0i32) as usize);
        let v8: usize = ((v6) as usize);
        let v9: Rc<str> = Rc::<str>::from(&(v0)[v7..v8]);
        let v10: Option<i32> = (v9).trim().parse::<i32>().ok();
        let v11: Option<i32> = method14(v10);
        let mut v12: Rc<dyn Fn((i32)) -> US4> = closure2();
        let mut v13: Option<US4> = v11.map(|x| v12(x));
        let v15: US4 = US4::US4_None;
        let v16: US4 = v15.clone();
        let v17: US4 = v13.unwrap_or(v16);
        v17
    } else {
        US4::US4_None
    }
}
fn typed_field_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let v3: bool = &*v0 == "kind";
    let v10: bool = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic|Feature|Story|TaskKind"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v1.clone()));
        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }));
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v4));
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }));
        let v9: bool = v8.contains(&*v6);
        v9
    } else {
        false
    };
    if v10 {
        US0::US0_KindField(v1)
    } else {
        let mut v12: bool = &*v0 == "who";
        let v19: bool = if v12 {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agent|pair|user"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v1.clone()));
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }));
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v13));
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }));
            let v18: bool = v17.contains(&*v15);
            v18
        } else {
            false
        };
        if v19 {
            US0::US0_AssigneeField(v1)
        } else {
            let v21: bool = &*v0 == "prio";
            let v28: bool = if v21 {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0|P1|P2|P3|P4|P5"); } LIT.with(|lit| lit.clone()) };
                let v23: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v1.clone()));
                let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }));
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v22));
                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }));
                let v27: bool = v26.contains(&*v24);
                v27
            } else {
                false
            };
            if v28 {
                US0::US0_PriorityField(v1)
            } else {
                let v30: bool = &*v0 == "load";
                if v30 {
                    let v31: US3 = load_of_12(v1);
                    match v31 {
                        US3::US3_None => {
                            US0::US0_UnknownField(v2)
                        }
                        US3::US3_Some(v32) => {
                            let v32: US1 = v32;
                            US0::US0_LoadField(v32)
                        }
                    }
                } else {
                    let v37: bool = &*v0 == "est";
                    if v37 {
                        let v38: US4 = minutes_of_13(v1);
                        match v38 {
                            US4::US4_None => {
                                US0::US0_UnknownField(v2)
                            }
                            US4::US4_Some(v39) => {
                                let v39: i32 = v39;
                                US0::US0_EstimateMinutes(v39)
                            }
                        }
                    } else {
                        US0::US0_UnknownField(v2)
                    }
                }
            }
        }
    }
}
fn field_of_10(mut v0: Rc<str>) -> US2 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method7(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: i32 = (v0.clone().len() as i32);
    let v9: bool = v7 < 1i32;
    if v9 {
        US2::US2_None
    } else {
        let v11: usize = ((0i32) as usize);
        let mut v12: usize = ((v7) as usize);
        let mut v13: Rc<str> = Rc::<str>::from(&(v0)[v11..v12]);
        let v14: i32 = v7.wrapping_add(1i32);
        let v15: i32 = v8.wrapping_sub(v7);
        let v16: i32 = v15.wrapping_sub(1i32);
        let v17: usize = ((v14) as usize);
        let v18: i32 = v14.wrapping_add(v16);
        let v19: usize = ((v18) as usize);
        let v20: Rc<str> = Rc::<str>::from(&(v0)[v17..v19]);
        let v21: US0 = typed_field_11(v13, v20, v0);
        US2::US2_Some(v21)
    }
}
fn walk_9(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: bool = v2 >= v1;
        if v4 {
            return v3;
        } else {
            let mut v5: Rc<str> = v0.borrow()[v2 as usize].clone();
            let mut v6: US2 = field_of_10(v5);
            match v6 {
                US2::US2_None => {
                    return v3;
                }
                US2::US2_Some(v7) => {
                    let v7: US0 = v7;
                    let v8: i32 = v2.wrapping_add(1i32);
                    let v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v3));
                    (v2, v3) = (v8, v9);
                    continue;
                }
            }
        }
    }
}
fn marker_fields_8(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: i32 = 0i32;
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    walk_9(v2, v3, v4, v5)
}
fn fields_of_marker_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = tail_after_6(v0, v1);
    let v3: Rc<UH0> = marker_fields_8(v2);
    fields_json_1(v3)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        fields_of_marker_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_todo_fields(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
