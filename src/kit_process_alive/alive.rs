#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn process_alive_0(mut v0: u64) -> bool {
    let v1: bool = v0 == 0u64;
    if v1 {
        true
    } else {
        let v2: u32 = ((v0) as u32);
        let v3: sysinfo::RefreshKind = sysinfo::RefreshKind::nothing().with_processes(Default::default());
        let mut v4: std::rc::Rc<sysinfo::System> = std::rc::Rc::new(sysinfo::System::new_with_specifics(v3));
        let v5: sysinfo::Pid = sysinfo::Pid::from_u32(v2);
        let v6: Option<sysinfo::ProcessStatus> = v4.process(v5).map(|p| p.status());
        let v7: Option<sysinfo::ProcessStatus> = None;
        let v8: bool = v6.eq(&v7);
        let v11: bool = if v8 {
            true
        } else {
            let v9: Option<sysinfo::ProcessStatus> = Some(sysinfo::ProcessStatus::Zombie);
            let v10: bool = v6.eq(&v9);
            v10
        };
        let v14: bool = if v11 {
            true
        } else {
            let v12: Option<sysinfo::ProcessStatus> = Some(sysinfo::ProcessStatus::Dead);
            let v13: bool = v6.eq(&v12);
            v13
        };
        let v15: bool = v14 == false;
        v15
    }
}
fn closure0() -> Rc<dyn Fn(u64) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64) -> bool> = Rc::new(move |mut v0: u64| -> bool {
        process_alive_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_process_alive(v0: u64) -> bool {
    closure0()(v0)
}
