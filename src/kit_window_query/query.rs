#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
struct Mut0 { l0: i32, l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_LiveGui,
    US0_TestSuite,
    US0_HeavyBuild,
    US0_Compile,
    US0_CompilerInstall,
    US0_TreeRead,
    US0_Bench,
    US0_CompileHold,
    US0_TreeWrite,
}
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_ProcessHolder,
    US2_LeaseHolder,
}
#[derive(Clone)]
enum US3 {
    US3_WindowQueued,
    US3_WindowGranted,
    US3_WindowReleased,
    US3_WindowReleasedWaiting,
    US3_WindowGaveUp,
    US3_WindowPruned,
    US3_WindowDropped,
}
struct Mut1 { l0: Rc<str> }
#[derive(Clone)]
enum US4 {
    US4_ClassKnown(US0),
    US4_ClassUnknown,
}
#[derive(Clone)]
enum US5 {
    US5_Some(Rc<str>),
    US5_None,
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method3(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method3(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method2(v16.clone(), v17.clone())
    }
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method4(v3, v1.clone());
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4.clone()))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method5(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn windows_root_7() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn now_ms_8() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn stored_rows_9(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn first_row_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1
}
fn tab_field_13(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn method15(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let mut v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_14(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method15(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US1> = closure1();
    let mut v4: Option<US1> = v2.map(|x| v3(x));
    let v6: US1 = US1::US1_None;
    let v7: US1 = v6.clone();
    let v8: US1 = v4.unwrap_or(v7);
    match &v8 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v9) => {
            let mut v9: u64 = *v9;
            v9
        }
    }
}
fn row_expired_12(mut v0: Rc<str>, mut v1: u64) -> bool {
    let v2: i32 = 5i32;
    let mut v3: Rc<str> = tab_field_13(v0, v2);
    let mut v4: u64 = decimal_of_14(v3);
    let v5: bool = v4 < v1;
    v5
}
fn holder_of_17(mut v0: Rc<str>) -> US2 {
    let mut v1: i32 = 3i32;
    let v2: Rc<str> = tab_field_13(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(kit job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let mut v9: bool = if v4 {
        true
    } else {
        let v5: i32 = 0i32;
        let v6: Rc<str> = tab_field_13(v0, v5);
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v6.ends_with(&*v7);
        v8
    };
    if v9 {
        US2::US2_ProcessHolder
    } else {
        US2::US2_LeaseHolder
    }
}
fn window_pid_18(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).split(char::from(45u8)).nth(2).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_19(mut v0: u64) -> bool {
    let mut v1: bool = kit_process_alive::kit_process_alive(v0);
    v1
}
fn orphaned_window_16(mut v0: Rc<str>) -> bool {
    let mut v1: US2 = holder_of_17(v0.clone());
    match &v1 {
        US2::US2_LeaseHolder => {
            false
        }
        US2::US2_ProcessHolder => {
            let v2: i32 = 0i32;
            let mut v3: Rc<str> = tab_field_13(v0, v2);
            let mut v4: u64 = window_pid_18(v3);
            let v5: bool = process_alive_19(v4);
            let v6: bool = v5 == false;
            v6
        }
    }
}
fn joined_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn concat5_22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn window_file_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = concat5_22(v1, v2, v3.clone(), v3.clone(), v3);
    joined_21(v0, v4)
}
fn file_removed_23(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn method27(mut v0: Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_26(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method27(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn number_text_25(mut v0: u64) -> Rc<str> {
    format_real_26(v0)
}
fn window_event_word_28(mut v0: US3) -> Rc<str> {
    match &v0 {
        US3::US3_WindowDropped => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US3::US3_WindowGaveUp => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gave-up"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US3::US3_WindowGranted => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US3::US3_WindowPruned => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pruned"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US3::US3_WindowQueued => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US3::US3_WindowReleased => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US3::US3_WindowReleasedWaiting => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released-waiting"); } LIT.with(|lit| lit.clone()) };
            v4
        }
    }
}
fn event_logged_24(mut v0: Rc<str>, mut v1: US3, mut v2: Rc<str>) -> () {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("events.log"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = joined_21(v0, v3);
    let v5: u64 = now_ms_8();
    let v6: Rc<str> = number_text_25(v5);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = window_event_word_28(v1);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = concat5_22(v2, v9, v10.clone(), v10.clone(), v10);
    let v12: Rc<str> = concat5_22(v6, v7.clone(), v8, v7, v11);
    let mut v13: std::io::Result<std::fs::File> = std::fs::OpenOptions::new().create(true).append(true).open(&*v4);
    let v14: bool = v13.is_ok();
    let v15: bool = v14 == false;
    if v15 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the file could not be opened"); } LIT.with(|lit| lit.clone()) }))
    };
    let mut v16: std::fs::File = v13.unwrap();
    let mut v17: std::io::Result<()> = std::io::Write::write_all(&mut v16, (v12).as_bytes());
    let v18: bool = v17.is_ok();
    let v19: bool = v18 == false;
    if v19 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the write failed"); } LIT.with(|lit| lit.clone()) }))
    }
}
fn other_rows_29(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1
}
fn live_rows_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = &*v1 == "";
        if v4 {
            return v3.clone();
        } else {
            let v5: Rc<str> = first_row_11(v1.clone());
            let v6: bool = row_expired_12(v5.clone(), v2);
            let v8: bool = if v6 {
                true
            } else {
                orphaned_window_16(v5.clone())
            };
            if v8 {
                let mut v9: i32 = 0i32;
                let v10: Rc<str> = tab_field_13(v5.clone(), v9);
                let v11: Rc<str> = window_file_20(v0.clone(), v10.clone());
                let v12: bool = file_removed_23(v11.clone());
                let mut v13: US3 = US3::US3_WindowPruned;
                event_logged_24(v0.clone(), v13.clone(), v5.clone());
                let v14: Rc<str> = other_rows_29(v1.clone());
                v1 = v14.clone();
                continue;
            } else {
                let mut v16: Rc<str> = other_rows_29(v1.clone());
                let mut v17: bool = &*v3 == "";
                let v21: Rc<str> = if v17 {
                    v5.clone()
                } else {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_22(v3.clone(), v18.clone(), v5.clone(), v19.clone(), v19.clone())
                };
                (v1, v3) = (v16.clone(), v21.clone());
                continue;
            }
        }
    }
}
fn free_mb_30() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn free() -> u64 { #[repr(C)] struct Status { length: u32, load: u32, total: u64, available: u64, total_page: u64, available_page: u64, total_virtual: u64, available_virtual: u64, extended: u64 } unsafe extern "system" { fn GlobalMemoryStatusEx(status: *mut Status) -> i32; } unsafe { let mut status: Status = std::mem::zeroed(); status.length = std::mem::size_of::<Status>() as u32; if GlobalMemoryStatusEx(&mut status) == 0 { u64::MAX } else { status.available / 1048576 } } } #[cfg(not(windows))] fn free() -> u64 { std::fs::read_to_string("/proc/meminfo").ok().and_then(|text| text.lines().find(|line| line.starts_with("MemAvailable:")).and_then(|line| line.split_whitespace().nth(1)).and_then(|kb| kb.parse::<u64>().ok())).map(|kb| kb / 1024).unwrap_or(u64::MAX) } std::env::var("KIT_WINDOWS_FREE_MB").ok().and_then(|text| text.trim().parse::<u64>().ok()).unwrap_or_else(free) };
    v0
}
fn class_word_32(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_Bench => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US0::US0_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_CompileHold => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US0::US0_CompilerInstall => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US0::US0_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_LiveGui => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_TestSuite => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_TreeRead => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US0::US0_TreeWrite => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-write"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    }
}
fn verdict_text_31(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: Rc<str>, mut v6: u64, mut v7: u64, mut v8: u64) -> Rc<str> {
    let mut v9: Rc<str> = class_word_32(v0);
    let v10: Rc<str> = kit_window_admission::kit_window_verdict(&*v9, &*v1, &*v2, v3, v4, &*v5, v6, v7, v8);
    v10
}
fn class_admitted_6(mut v0: US0) -> bool {
    let mut v1: Rc<str> = windows_root_7();
    let v2: u64 = now_ms_8();
    let mut v3: Rc<str> = stored_rows_9(v1.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = live_rows_10(v1, v3, v2, v4.clone());
    let v6: u64 = 0u64;
    let v7: u64 = free_mb_30();
    let v8: u64 = 0u64;
    let mut v9: Rc<str> = verdict_text_31(v0, v4.clone(), v4, v2, v6, v5, v7, v2, v8);
    let v10: bool = &*v9 == "";
    v10
}
fn fitting_loads_0() -> Rc<str> {
    let mut v0: Rc<str> = kit_window_run::kit_heavy_holders_text();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH0> = split_list_1(v1, v0);
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = method4(v2, v3);
    let v5: i32 = 0i32;
    let v6: i32 = method5(v4, v5);
    let v7: US0 = US0::US0_Compile;
    let v8: bool = class_admitted_6(v7);
    let v11: Rc<str> = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",medium"); } LIT.with(|lit| lit.clone()) };
        v9
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v10
    };
    let v12: bool = v6 < 2i32;
    let v15: bool = if v12 {
        let mut v13: US0 = US0::US0_HeavyBuild;
        class_admitted_6(v13)
    } else {
        false
    };
    let v18: Rc<str> = if v15 {
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",heavy"); } LIT.with(|lit| lit.clone()) };
        v16
    } else {
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v17
    };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("light,none"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v11)), v18));
    v21
}
fn closure0() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        fitting_loads_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn minutes_left_35(mut v0: u64, mut v1: u64) -> u64 {
    let v2: bool = v0 <= v1;
    if v2 {
        0u64
    } else {
        let mut v3: u64 = v0.wrapping_sub(v1);
        let mut v4: u64 = v3.wrapping_add(59999u64);
        let v5: u64 = v4.wrapping_div(60000u64);
        v5
    }
}
fn stdout_line_36(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn shown_rows_34(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> u64 {
    loop {
        let mut v3: bool = &*v0 == "";
        if v3 {
            return v2;
        } else {
            let mut v4: Rc<str> = first_row_11(v0.clone());
            let v5: i32 = 2i32;
            let v6: Rc<str> = tab_field_13(v4.clone(), v5);
            let v7: bool = &*v6 == "waiting";
            let v25: Rc<str> = if v7 {
                let v8: i32 = 4i32;
                let mut v9: Rc<str> = tab_field_13(v4.clone(), v8);
                let v10: u64 = decimal_of_14(v9.clone());
                let v11: u64 = v1.wrapping_sub(v10);
                let v12: u64 = v11.wrapping_div(60000u64);
                let mut v13: Rc<str> = number_text_25(v12);
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waited "); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                v17.clone()
            } else {
                let v18: i32 = 5i32;
                let v19: Rc<str> = tab_field_13(v4.clone(), v18);
                let v20: u64 = decimal_of_14(v19.clone());
                let v21: u64 = minutes_left_35(v20, v1);
                let v22: Rc<str> = number_text_25(v21);
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m left"); } LIT.with(|lit| lit.clone()) };
                let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
                v24.clone()
            };
            let v26: i32 = 0i32;
            let v27: Rc<str> = tab_field_13(v4.clone(), v26);
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
            let v30: i32 = 1i32;
            let v31: Rc<str> = tab_field_13(v4.clone(), v30);
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v29, v31)), v28));
            let v34: i32 = 2i32;
            let v35: Rc<str> = tab_field_13(v4.clone(), v34);
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v35, v28)), v25)), v28));
            let v39: i32 = 3i32;
            let v40: Rc<str> = tab_field_13(v4.clone(), v39);
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v33, Rc::<str>::from(format!("{}{}", v38, v40))));
            stdout_line_36(v42.clone());
            let v43: Rc<str> = other_rows_29(v0.clone());
            let v44: u64 = v2.wrapping_add(1u64);
            (v0, v2) = (v43.clone(), v44);
            continue;
        }
    }
}
fn window_listed_33() -> i32 {
    let mut v0: Rc<str> = windows_root_7();
    let mut v1: u64 = now_ms_8();
    let v2: Rc<str> = stored_rows_9(v0.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = live_rows_10(v0, v2, v1, v3);
    let v5: u64 = 0u64;
    let v6: u64 = shown_rows_34(v4, v1, v5);
    let v7: Rc<str> = kit_window_run::kit_heavy_holders_text();
    let v8: bool = &*v7 == "";
    if v8 {
        ()
    } else {
        stdout_line_36(v7.clone())
    };
    let mut v9: u64 = (v7).lines().count() as u64;
    let v10: Rc<str> = number_text_25(v6);
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window list ok windows="); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v10));
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free_mb="); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let v15: u64 = free_mb_30();
    let mut v16: Rc<str> = number_text_25(v15);
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v16));
    let v18: Rc<str> = number_text_25(v9);
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" heavy_slots="); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v17, Rc::<str>::from(format!("{}{}", v19, v18))));
    stdout_line_36(v21);
    0i32
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        window_listed_33()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn stderr_line_40(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn refused_39(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: window "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = concat5_22(v1, v0, v2.clone(), v2.clone(), v2);
    stderr_line_40(v3);
    2i32
}
fn class_of_41(mut v0: Rc<str>) -> US4 {
    let mut v1: US0 = US0::US0_LiveGui;
    let v2: Rc<str> = class_word_32(v1);
    let mut v3: bool = &*v0 == &*v2;
    if v3 {
        let mut v4: US0 = US0::US0_LiveGui;
        US4::US4_ClassKnown(v4)
    } else {
        let v6: US0 = US0::US0_TestSuite;
        let v7: Rc<str> = class_word_32(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let mut v9: US0 = US0::US0_TestSuite;
            US4::US4_ClassKnown(v9)
        } else {
            let v11: US0 = US0::US0_HeavyBuild;
            let v12: Rc<str> = class_word_32(v11);
            let mut v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US0 = US0::US0_HeavyBuild;
                US4::US4_ClassKnown(v14)
            } else {
                let mut v16: US0 = US0::US0_Compile;
                let mut v17: Rc<str> = class_word_32(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US0 = US0::US0_Compile;
                    US4::US4_ClassKnown(v19)
                } else {
                    let v21: US0 = US0::US0_CompilerInstall;
                    let v22: Rc<str> = class_word_32(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: US0 = US0::US0_CompilerInstall;
                        US4::US4_ClassKnown(v24)
                    } else {
                        let v26: US0 = US0::US0_TreeRead;
                        let v27: Rc<str> = class_word_32(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let v29: US0 = US0::US0_TreeRead;
                            US4::US4_ClassKnown(v29)
                        } else {
                            let v31: US0 = US0::US0_Bench;
                            let v32: Rc<str> = class_word_32(v31);
                            let v33: bool = &*v0 == &*v32;
                            if v33 {
                                let v34: US0 = US0::US0_Bench;
                                US4::US4_ClassKnown(v34)
                            } else {
                                let v36: US0 = US0::US0_CompileHold;
                                let v37: Rc<str> = class_word_32(v36);
                                let v38: bool = &*v0 == &*v37;
                                if v38 {
                                    let v39: US0 = US0::US0_CompileHold;
                                    US4::US4_ClassKnown(v39)
                                } else {
                                    let v41: US0 = US0::US0_TreeWrite;
                                    let v42: Rc<str> = class_word_32(v41);
                                    let v43: bool = &*v0 == &*v42;
                                    if v43 {
                                        let v44: US0 = US0::US0_TreeWrite;
                                        US4::US4_ClassKnown(v44)
                                    } else {
                                        US4::US4_ClassUnknown
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
fn shared_with_42(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        let v2: bool = &*v0 == "";
        if v2 {
            return v1.clone();
        } else {
            let mut v3: Rc<str> = first_row_11(v0.clone());
            let mut v4: i32 = 1i32;
            let v5: Rc<str> = tab_field_13(v3.clone(), v4);
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
            let v8: i32 = 3i32;
            let mut v9: Rc<str> = tab_field_13(v3.clone(), v8);
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v9));
            let v11: i32 = 2i32;
            let v12: Rc<str> = tab_field_13(v3.clone(), v11);
            let mut v13: bool = &*v12 == "held";
            let v14: Rc<str> = other_rows_29(v0.clone());
            let mut v16: bool = if v13 {
                let v15: bool = &*v1 != "";
                v15
            } else {
                false
            };
            let v21: Rc<str> = if v16 {
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v17)), v10));
                v19.clone()
            } else {
                if v13 {
                    v10.clone()
                } else {
                    v1.clone()
                }
            };
            (v0, v1) = (v14.clone(), v21.clone());
            continue;
        }
    }
}
fn window_checked_38(mut v0: Rc<str>) -> i32 {
    let mut v1: US4 = class_of_41(v0.clone());
    match &v1 {
        US4::US4_ClassKnown(v47) => {
            let v47: US0 = v47.clone();
            let v48: Rc<str> = windows_root_7();
            let v49: u64 = now_ms_8();
            let v50: Rc<str> = stored_rows_9(v48.clone());
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v52: Rc<str> = live_rows_10(v48.clone(), v50.clone(), v49, v51.clone());
            let v53: u64 = 0u64;
            let v54: u64 = free_mb_30();
            let v55: u64 = 0u64;
            let v56: Rc<str> = verdict_text_31(v47.clone(), v51.clone(), v51.clone(), v49, v53, v52.clone(), v54, v49, v55);
            let v57: bool = &*v56 == "";
            if v57 {
                let v58: Rc<str> = shared_with_42(v52.clone(), v51.clone());
                let v59: bool = &*v58 == "";
                let v64: Rc<str> = if v59 {
                    v51.clone()
                } else {
                    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (shares with "); } LIT.with(|lit| lit.clone()) };
                    let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v58));
                    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                    let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v62));
                    v63.clone()
                };
                let v65: Rc<str> = class_word_32(v47.clone());
                let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window check ok class="); } LIT.with(|lit| lit.clone()) };
                let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v65));
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free"); } LIT.with(|lit| lit.clone()) };
                let v70: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v67, v68)), v64));
                stdout_line_36(v70.clone());
                0i32
            } else {
                let v71: Rc<str> = class_word_32(v47.clone());
                let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window check blocked class="); } LIT.with(|lit| lit.clone()) };
                let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v71));
                let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v76: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v73, v74)), v56));
                stdout_line_36(v76.clone());
                1i32
            }
        }
        US4::US4_ClassUnknown => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check: unknown class '"); } LIT.with(|lit| lit.clone()) };
            let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v0));
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] | window remote <ssh-alias> [--bin <remote kit path>]; classes "); } LIT.with(|lit| lit.clone()) };
            let v7: US0 = US0::US0_LiveGui;
            let v8: Rc<str> = class_word_32(v7.clone());
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v8.clone()));
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v9.clone()));
            let v11: US0 = US0::US0_TestSuite;
            let v12: Rc<str> = class_word_32(v11.clone());
            let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v12.clone()));
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v10.clone(), v13.clone()));
            let v15: US0 = US0::US0_HeavyBuild;
            let mut v16: Rc<str> = class_word_32(v15.clone());
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v16.clone()));
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v14.clone(), v17.clone()));
            let v19: US0 = US0::US0_Compile;
            let v20: Rc<str> = class_word_32(v19.clone());
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v20.clone()));
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v18.clone(), v21.clone()));
            let v23: US0 = US0::US0_CompilerInstall;
            let v24: Rc<str> = class_word_32(v23.clone());
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v24.clone()));
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v22.clone(), v25.clone()));
            let v27: US0 = US0::US0_TreeRead;
            let v28: Rc<str> = class_word_32(v27.clone());
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v28.clone()));
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v26.clone(), v29.clone()));
            let v31: US0 = US0::US0_Bench;
            let v32: Rc<str> = class_word_32(v31.clone());
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v32.clone()));
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v30.clone(), v33.clone()));
            let v35: US0 = US0::US0_CompileHold;
            let v36: Rc<str> = class_word_32(v35.clone());
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v36.clone()));
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v34.clone(), v37.clone()));
            let v39: US0 = US0::US0_TreeWrite;
            let v40: Rc<str> = class_word_32(v39.clone());
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v40.clone()));
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v38.clone(), v41.clone()));
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v44: Rc<str> = concat5_22(v6.clone(), v42.clone(), v43.clone(), v43.clone(), v43.clone());
            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v44));
            refused_39(v45.clone())
        }
    }
}
fn command_line_argument_count_44() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_45(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_43(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_44();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US5 = if v4 {
        let v5: Rc<str> = command_line_argument_45(v0);
        US5::US5_Some(v5)
    } else {
        US5::US5_None
    };
    match &v8 {
        US5::US5_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US5::US5_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_43(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn check_run_37() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_43(v0);
    match &*v1 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            match &*v3 {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4.clone();
                    let v5: Rc<UH0> = v5.clone();
                    match &*v5 {
                        UH0::UH0_Cons(v6, v7) => {
                            let v6: Rc<str> = v6.clone();
                            let v7: Rc<UH0> = v7.clone();
                            window_checked_38(v6)
                        }
                        _ => {
                            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            window_checked_38(v9)
                        }
                    }
                }
                _ => {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    window_checked_38(v12)
                }
            }
        }
        _ => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            window_checked_38(v15)
        }
    }
}
fn closure3() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        check_run_37()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_fitting_loads() -> Rc<str> {
    closure0()()
}
pub fn kit_window_list_run() -> i32 {
    closure2()()
}
pub fn kit_window_check_run() -> i32 {
    closure3()()
}
