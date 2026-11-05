#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US4 {
    US4_Granted,
    US4_ReleasedWhileWaiting,
    US4_WaitExpired,
    US4_KeepWaiting,
}
#[derive(Clone)]
enum US5 {
    US5_Held,
    US5_Waiting,
}
#[derive(Clone)]
enum US6 {
    US6_ClassKnown(US0),
    US6_ClassUnknown,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US7 {
    US7_Some(Rc<str>),
    US7_None,
}
fn concat5_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn stderr_line_4(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn refused_2(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: window "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = concat5_3(v1, v0, v2.clone(), v2.clone(), v2);
    stderr_line_4(v3);
    2i32
}
fn windows_root_6() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn lock_taken_7(mut v0: Rc<str>) -> bool {
    let mut v1: bool = { let lock = std::path::Path::new(&*v0).join("lock"); let _ = std::fs::create_dir_all(&*v0); let mut taken = false; for _ in 0..400 { if std::fs::create_dir(&lock).is_ok() { taken = true; break; } let stale = std::fs::metadata(&lock).and_then(|meta| meta.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age.as_secs() > 10).unwrap_or(false); if stale { let _ = std::fs::remove_dir(&lock); } else { std::thread::sleep(std::time::Duration::from_millis(50)); } } taken };
    v1
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
    let mut v6: US1 = US1::US1_None;
    let mut v7: US1 = v6.clone();
    let mut v8: US1 = v4.unwrap_or(v7);
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
        let mut v6: Rc<str> = tab_field_13(v0, v5);
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
        let mut v8: bool = v6.ends_with(&*v7);
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
            let mut v6: bool = v5 == false;
            v6
        }
    }
}
fn joined_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn window_file_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = concat5_3(v1, v2, v3.clone(), v3.clone(), v3);
    joined_21(v0, v4)
}
fn file_removed_22(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn method26(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_25(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method26(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn number_text_24(mut v0: u64) -> Rc<str> {
    format_real_25(v0)
}
fn window_event_word_27(mut v0: US3) -> Rc<str> {
    match &v0 {
        US3::US3_WindowDropped => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
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
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pruned"); } LIT.with(|lit| lit.clone()) };
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
fn event_logged_23(mut v0: Rc<str>, mut v1: US3, mut v2: Rc<str>) -> () {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("events.log"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = joined_21(v0, v3);
    let v5: u64 = now_ms_8();
    let mut v6: Rc<str> = number_text_24(v5);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = window_event_word_27(v1);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = concat5_3(v2, v9, v10.clone(), v10.clone(), v10);
    let v12: Rc<str> = concat5_3(v6, v7.clone(), v8, v7, v11);
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
fn other_rows_28(mut v0: Rc<str>) -> Rc<str> {
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
            let mut v6: bool = row_expired_12(v5.clone(), v2);
            let mut v8: bool = if v6 {
                true
            } else {
                orphaned_window_16(v5.clone())
            };
            if v8 {
                let mut v9: i32 = 0i32;
                let v10: Rc<str> = tab_field_13(v5.clone(), v9);
                let v11: Rc<str> = window_file_20(v0.clone(), v10.clone());
                let v12: bool = file_removed_22(v11.clone());
                let mut v13: US3 = US3::US3_WindowPruned;
                event_logged_23(v0.clone(), v13.clone(), v5.clone());
                let v14: Rc<str> = other_rows_28(v1.clone());
                v1 = v14.clone();
                continue;
            } else {
                let mut v16: Rc<str> = other_rows_28(v1.clone());
                let mut v17: bool = &*v3 == "";
                let v21: Rc<str> = if v17 {
                    v5.clone()
                } else {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_3(v3.clone(), v18.clone(), v5.clone(), v19.clone(), v19.clone())
                };
                (v1, v3) = (v16.clone(), v21.clone());
                continue;
            }
        }
    }
}
fn free_mb_29() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn free() -> u64 { #[repr(C)] struct Status { length: u32, load: u32, total: u64, available: u64, total_page: u64, available_page: u64, total_virtual: u64, available_virtual: u64, extended: u64 } unsafe extern "system" { fn GlobalMemoryStatusEx(status: *mut Status) -> i32; } unsafe { let mut status: Status = std::mem::zeroed(); status.length = std::mem::size_of::<Status>() as u32; if GlobalMemoryStatusEx(&mut status) == 0 { u64::MAX } else { status.available / 1048576 } } } #[cfg(not(windows))] fn free() -> u64 { std::fs::read_to_string("/proc/meminfo").ok().and_then(|text| text.lines().find(|line| line.starts_with("MemAvailable:")).and_then(|line| line.split_whitespace().nth(1)).and_then(|kb| kb.parse::<u64>().ok())).map(|kb| kb / 1024).unwrap_or(u64::MAX) } std::env::var("KIT_WINDOWS_FREE_MB").ok().and_then(|text| text.trim().parse::<u64>().ok()).unwrap_or_else(free) };
    v0
}
fn class_word_31(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_Bench => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US0::US0_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_CompileHold => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
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
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US0::US0_TreeWrite => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-write"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    }
}
fn verdict_text_30(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: Rc<str>, mut v6: u64, mut v7: u64, mut v8: u64) -> Rc<str> {
    let mut v9: Rc<str> = class_word_31(v0);
    let v10: Rc<str> = kit_window_admission::kit_window_verdict(&*v9, &*v1, &*v2, v3, v4, &*v5, v6, v7, v8);
    v10
}
fn released_mark_32(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".released"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = concat5_3(v1, v2, v3.clone(), v3.clone(), v3);
    joined_21(v0, v4)
}
fn file_exists_33(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn wait_step_of_34(mut v0: bool, mut v1: bool, mut v2: u64, mut v3: u64) -> US4 {
    if v1 {
        US4::US4_ReleasedWhileWaiting
    } else {
        if v0 {
            US4::US4_Granted
        } else {
            let mut v6: bool = v2 >= v3;
            if v6 {
                US4::US4_WaitExpired
            } else {
                US4::US4_KeepWaiting
            }
        }
    }
}
fn one_line_37(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>());
    v1.clone()
}
fn row_text_36(mut v0: Rc<str>, mut v1: US0, mut v2: US5, mut v3: Rc<str>, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = class_word_31(v1);
    let v11: Rc<str> = match &v2 {
        US5::US5_Held => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US5::US5_Waiting => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    };
    let v12: Rc<str> = concat5_3(v0, v6.clone(), v7, v6.clone(), v11);
    let mut v13: Rc<str> = one_line_37(v3);
    let v14: Rc<str> = number_text_24(v4);
    let v15: Rc<str> = number_text_24(v5);
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = concat5_3(v14, v6.clone(), v15, v16.clone(), v16);
    concat5_3(v12, v6.clone(), v13, v6, v17)
}
fn request_row_35(mut v0: US0, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: US5, mut v8: u64) -> Rc<str> {
    row_text_36(v3, v0, v7, v2, v4, v8)
}
fn text_written_38(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn lock_released_39(mut v0: Rc<str>) -> () {
    { let _ = std::fs::remove_dir(std::path::Path::new(&*v0).join("lock")); };
    ()
}
fn request_head_40(mut v0: US0, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window request "); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" id="); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" class="); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = class_word_31(v0);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = concat5_3(v10, v11, v12.clone(), v12.clone(), v12);
    concat5_3(v8, v7, v9, v3, v13)
}
fn stdout_line_41(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn requested_5(mut v0: US0, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: bool) -> i32 {
    loop {
        let mut v8: Rc<str> = windows_root_6();
        let mut v9: bool = lock_taken_7(v8.clone());
        let v10: bool = v9 == false;
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: the registry lock stayed taken for 20 s"); } LIT.with(|lit| lit.clone()) };
            return refused_2(v11.clone());
        } else {
            let mut v13: u64 = now_ms_8();
            let v14: Rc<str> = stored_rows_9(v8.clone());
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = live_rows_10(v8.clone(), v14.clone(), v13, v15.clone());
            let mut v17: u64 = free_mb_29();
            let v18: Rc<str> = verdict_text_30(v0.clone(), v2.clone(), v3.clone(), v4, v1, v16.clone(), v17, v13, v6);
            let v19: bool = &*v18 == "";
            let v20: Rc<str> = released_mark_32(v8.clone(), v3.clone());
            let v21: bool = file_exists_33(v20.clone());
            let v22: US4 = wait_step_of_34(v19, v21, v13, v5);
            match &v22 {
                US4::US4_Granted => {
                    let v36: u64 = v1.wrapping_mul(60000u64);
                    let v37: u64 = v13.wrapping_add(v36);
                    let v38: Rc<str> = window_file_20(v8.clone(), v3.clone());
                    let v39: US5 = US5::US5_Held;
                    let v40: Rc<str> = request_row_35(v0.clone(), v1, v2.clone(), v3.clone(), v13, v5, v6, v39.clone(), v37);
                    let v41: Rc<str> = text_written_38(v38.clone(), v40.clone());
                    let v42: US3 = US3::US3_WindowGranted;
                    let v43: US5 = US5::US5_Held;
                    let v44: Rc<str> = request_row_35(v0.clone(), v1, v2.clone(), v3.clone(), v13, v5, v6, v43.clone(), v37);
                    event_logged_23(v8.clone(), v42.clone(), v44.clone());
                    lock_released_39(v8.clone());
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                    let v46: Rc<str> = request_head_40(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v45.clone());
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" minutes="); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<str> = number_text_24(v1);
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free_mb="); } LIT.with(|lit| lit.clone()) };
                    let v50: Rc<str> = number_text_24(v17);
                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let v52: u64 = v13.wrapping_sub(v4);
                    let v53: Rc<str> = number_text_24(v52);
                    let v54: Rc<str> = concat5_3(v50.clone(), v51.clone(), v53.clone(), v15.clone(), v15.clone());
                    let v55: Rc<str> = concat5_3(v46.clone(), v47.clone(), v48.clone(), v49.clone(), v54.clone());
                    stdout_line_41(v55.clone());
                    return 0i32;
                }
                US4::US4_KeepWaiting => {
                    let v65: Rc<str> = window_file_20(v8.clone(), v3.clone());
                    let v66: US5 = US5::US5_Waiting;
                    let v67: u64 = v13.wrapping_add(30000u64);
                    let v68: Rc<str> = request_row_35(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v66.clone(), v67);
                    let v69: Rc<str> = text_written_38(v65.clone(), v68.clone());
                    if v7 {
                        ()
                    } else {
                        let v70: US3 = US3::US3_WindowQueued;
                        let v71: US5 = US5::US5_Waiting;
                        let v72: Rc<str> = request_row_35(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v71.clone(), v67);
                        event_logged_23(v8.clone(), v70.clone(), v72.clone())
                    };
                    lock_released_39(v8.clone());
                    if v7 {
                        ()
                    } else {
                        let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
                        let v74: Rc<str> = request_head_40(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v73.clone());
                        let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v76: Rc<str> = concat5_3(v74.clone(), v75.clone(), v18.clone(), v15.clone(), v15.clone());
                        stdout_line_41(v76.clone())
                    };
                    let v77: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v77));
                    let v78: bool = true;
                    v7 = v78;
                    continue;
                }
                US4::US4_ReleasedWhileWaiting => {
                    let v23: Rc<str> = released_mark_32(v8.clone(), v3.clone());
                    let v24: bool = file_removed_22(v23.clone());
                    let v25: Rc<str> = window_file_20(v8.clone(), v3.clone());
                    let v26: bool = file_removed_22(v25.clone());
                    let v27: US3 = US3::US3_WindowReleasedWaiting;
                    let v28: US5 = US5::US5_Waiting;
                    let v29: Rc<str> = request_row_35(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v28.clone(), v13);
                    event_logged_23(v8.clone(), v27.clone(), v29.clone());
                    lock_released_39(v8.clone());
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
                    let v31: Rc<str> = request_head_40(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v30.clone());
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" while waiting waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let v33: u64 = v13.wrapping_sub(v4);
                    let v34: Rc<str> = number_text_24(v33);
                    let v35: Rc<str> = concat5_3(v31.clone(), v32.clone(), v34.clone(), v15.clone(), v15.clone());
                    stdout_line_41(v35.clone());
                    return 1i32;
                }
                US4::US4_WaitExpired => {
                    let v56: Rc<str> = window_file_20(v8.clone(), v3.clone());
                    let v57: bool = file_removed_22(v56.clone());
                    let v58: US3 = US3::US3_WindowGaveUp;
                    let v59: US5 = US5::US5_Waiting;
                    let v60: Rc<str> = request_row_35(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v59.clone(), v13);
                    event_logged_23(v8.clone(), v58.clone(), v60.clone());
                    lock_released_39(v8.clone());
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked"); } LIT.with(|lit| lit.clone()) };
                    let v62: Rc<str> = request_head_40(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v61.clone());
                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v64: Rc<str> = concat5_3(v62.clone(), v63.clone(), v18.clone(), v15.clone(), v15.clone());
                    stdout_line_41(v64.clone());
                    return 1i32;
                }
            }
        }
    }
}
fn class_of_42(mut v0: Rc<str>) -> US6 {
    let mut v1: US0 = US0::US0_LiveGui;
    let v2: Rc<str> = class_word_31(v1);
    let mut v3: bool = &*v0 == &*v2;
    if v3 {
        let mut v4: US0 = US0::US0_LiveGui;
        US6::US6_ClassKnown(v4)
    } else {
        let mut v6: US0 = US0::US0_TestSuite;
        let mut v7: Rc<str> = class_word_31(v6);
        let mut v8: bool = &*v0 == &*v7;
        if v8 {
            let mut v9: US0 = US0::US0_TestSuite;
            US6::US6_ClassKnown(v9)
        } else {
            let v11: US0 = US0::US0_HeavyBuild;
            let v12: Rc<str> = class_word_31(v11);
            let mut v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US0 = US0::US0_HeavyBuild;
                US6::US6_ClassKnown(v14)
            } else {
                let mut v16: US0 = US0::US0_Compile;
                let mut v17: Rc<str> = class_word_31(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US0 = US0::US0_Compile;
                    US6::US6_ClassKnown(v19)
                } else {
                    let v21: US0 = US0::US0_CompilerInstall;
                    let v22: Rc<str> = class_word_31(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: US0 = US0::US0_CompilerInstall;
                        US6::US6_ClassKnown(v24)
                    } else {
                        let v26: US0 = US0::US0_TreeRead;
                        let v27: Rc<str> = class_word_31(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let v29: US0 = US0::US0_TreeRead;
                            US6::US6_ClassKnown(v29)
                        } else {
                            let v31: US0 = US0::US0_Bench;
                            let v32: Rc<str> = class_word_31(v31);
                            let v33: bool = &*v0 == &*v32;
                            if v33 {
                                let v34: US0 = US0::US0_Bench;
                                US6::US6_ClassKnown(v34)
                            } else {
                                let v36: US0 = US0::US0_CompileHold;
                                let v37: Rc<str> = class_word_31(v36);
                                let v38: bool = &*v0 == &*v37;
                                if v38 {
                                    let v39: US0 = US0::US0_CompileHold;
                                    US6::US6_ClassKnown(v39)
                                } else {
                                    let v41: US0 = US0::US0_TreeWrite;
                                    let v42: Rc<str> = class_word_31(v41);
                                    let v43: bool = &*v0 == &*v42;
                                    if v43 {
                                        let v44: US0 = US0::US0_TreeWrite;
                                        US6::US6_ClassKnown(v44)
                                    } else {
                                        US6::US6_ClassUnknown
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
fn announce_refusal_43(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = windows_root_6();
    let mut v3: Rc<str> = kit_window_announce_check::kit_window_announced_refusal(&*v2, &*v0, v1);
    v3
}
fn option_value_44(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1.clone()
}
fn tree_access_45(mut v0: US0) -> bool {
    match &v0 {
        US0::US0_TreeRead => {
            true
        }
        US0::US0_TreeWrite => {
            true
        }
        _ => {
            false
        }
    }
}
fn method47(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn rooted_path_46(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: bool = v3.starts_with(&*v2);
    if v4 {
        true
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":/"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Option<usize> = v3.find(&*v5);
        let mut v7: Option<usize> = method47(v6.clone());
        let mut v8: Rc<dyn Fn((usize)) -> i32> = closure2();
        let mut v9: Option<i32> = v7.map(|x| v8(x));
        let v11: i32 = v9.unwrap_or(-1i32);
        let v12: bool = v11 == 1i32;
        v12
    }
}
fn process_number_48() -> u64 {
    let mut v0: u64 = std::process::id() as u64;
    v0
}
fn window_requested_1(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: US6 = class_of_42(v0.clone());
    match &v2 {
        US6::US6_ClassKnown(v46) => {
            let v46: US0 = v46.clone();
            let v47: u64 = decimal_of_14(v1.clone());
            let v48: bool = v47 == 0u64;
            if v48 {
                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: minutes must be a whole number above 0, received '"); } LIT.with(|lit| lit.clone()) };
                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] | window remote <ssh-alias> [--bin <remote kit path>]; classes "); } LIT.with(|lit| lit.clone()) };
                let v52: US0 = US0::US0_LiveGui;
                let v53: Rc<str> = class_word_31(v52.clone());
                let v54: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v53.clone()));
                let v55: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v54.clone()));
                let v56: US0 = US0::US0_TestSuite;
                let v57: Rc<str> = class_word_31(v56.clone());
                let v58: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v57.clone()));
                let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v55.clone(), v58.clone()));
                let v60: US0 = US0::US0_HeavyBuild;
                let v61: Rc<str> = class_word_31(v60.clone());
                let v62: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v61.clone()));
                let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v59.clone(), v62.clone()));
                let v64: US0 = US0::US0_Compile;
                let v65: Rc<str> = class_word_31(v64.clone());
                let v66: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v65.clone()));
                let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v63.clone(), v66.clone()));
                let v68: US0 = US0::US0_CompilerInstall;
                let v69: Rc<str> = class_word_31(v68.clone());
                let v70: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v69.clone()));
                let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v67.clone(), v70.clone()));
                let v72: US0 = US0::US0_TreeRead;
                let v73: Rc<str> = class_word_31(v72.clone());
                let v74: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v73.clone()));
                let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v71.clone(), v74.clone()));
                let v76: US0 = US0::US0_Bench;
                let v77: Rc<str> = class_word_31(v76.clone());
                let v78: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v77.clone()));
                let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v75.clone(), v78.clone()));
                let v80: US0 = US0::US0_CompileHold;
                let v81: Rc<str> = class_word_31(v80.clone());
                let v82: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v81.clone()));
                let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v79.clone(), v82.clone()));
                let v84: US0 = US0::US0_TreeWrite;
                let v85: Rc<str> = class_word_31(v84.clone());
                let v86: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v85.clone()));
                let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v83.clone(), v86.clone()));
                let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v89: Rc<str> = concat5_3(v51.clone(), v87.clone(), v88.clone(), v88.clone(), v88.clone());
                let v90: Rc<str> = concat5_3(v49.clone(), v1.clone(), v50.clone(), v89.clone(), v88.clone());
                refused_2(v90.clone())
            } else {
                let v92: Rc<str> = announce_refusal_43(v0.clone(), v47);
                let v93: bool = &*v92 != "";
                if v93 {
                    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: "); } LIT.with(|lit| lit.clone()) };
                    let v95: Rc<str> = announce_refusal_43(v0.clone(), v47);
                    let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v97: Rc<str> = concat5_3(v94.clone(), v95.clone(), v96.clone(), v96.clone(), v96.clone());
                    refused_2(v97.clone())
                } else {
                    let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--label"); } LIT.with(|lit| lit.clone()) };
                    let v100: Rc<str> = option_value_44(v99.clone());
                    let v101: bool = tree_access_45(v46.clone());
                    let v104: bool = if v101 {
                        let v102: bool = rooted_path_46(v100.clone());
                        let v103: bool = v102 == false;
                        v103
                    } else {
                        false
                    };
                    let v107: Rc<str> = if v104 {
                        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a tree window guards the path in its label; give the rooted path it reads or writes, e.g. C:/home/git/spiral/lib"); } LIT.with(|lit| lit.clone()) };
                        v105.clone()
                    } else {
                        let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v106.clone()
                    };
                    let v108: bool = &*v107 != "";
                    if v108 {
                        let v109: Rc<str> = option_value_44(v99.clone());
                        let v110: bool = tree_access_45(v46.clone());
                        let v113: bool = if v110 {
                            let v111: bool = rooted_path_46(v109.clone());
                            let v112: bool = v111 == false;
                            v112
                        } else {
                            false
                        };
                        let v116: Rc<str> = if v113 {
                            let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a tree window guards the path in its label; give the rooted path it reads or writes, e.g. C:/home/git/spiral/lib"); } LIT.with(|lit| lit.clone()) };
                            v114.clone()
                        } else {
                            let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v115.clone()
                        };
                        let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: "); } LIT.with(|lit| lit.clone()) };
                        let v118: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v116));
                        refused_2(v118.clone())
                    } else {
                        let v120: u64 = now_ms_8();
                        let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("win-"); } LIT.with(|lit| lit.clone()) };
                        let v122: Rc<str> = number_text_24(v120);
                        let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        let v124: u64 = process_number_48();
                        let v125: Rc<str> = number_text_24(v124);
                        let v126: Rc<str> = class_word_31(v46.clone());
                        let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v128: Rc<str> = concat5_3(v123.clone(), v126.clone(), v127.clone(), v127.clone(), v127.clone());
                        let v129: Rc<str> = concat5_3(v121.clone(), v122.clone(), v123.clone(), v125.clone(), v128.clone());
                        let v130: Rc<str> = option_value_44(v99.clone());
                        let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--wait"); } LIT.with(|lit| lit.clone()) };
                        let v132: Rc<str> = option_value_44(v131.clone());
                        let v133: bool = &*v132 == "until-granted";
                        let v138: u64 = if v133 {
                            9223372036854775807u64
                        } else {
                            let v134: Rc<str> = option_value_44(v131.clone());
                            let v135: u64 = decimal_of_14(v134.clone());
                            let v136: u64 = v135.wrapping_mul(60000u64);
                            let v137: u64 = v120.wrapping_add(v136);
                            v137
                        };
                        let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb"); } LIT.with(|lit| lit.clone()) };
                        let v140: Rc<str> = option_value_44(v139.clone());
                        let v141: u64 = decimal_of_14(v140.clone());
                        let v142: bool = false;
                        requested_5(v46.clone(), v47, v130.clone(), v129.clone(), v120, v138, v141, v142)
                    }
                }
            }
        }
        US6::US6_ClassUnknown => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: unknown class '"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] | window remote <ssh-alias> [--bin <remote kit path>]; classes "); } LIT.with(|lit| lit.clone()) };
            let mut v6: US0 = US0::US0_LiveGui;
            let mut v7: Rc<str> = class_word_31(v6.clone());
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v7.clone()));
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v8.clone()));
            let v10: US0 = US0::US0_TestSuite;
            let v11: Rc<str> = class_word_31(v10.clone());
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v11.clone()));
            let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v9.clone(), v12.clone()));
            let v14: US0 = US0::US0_HeavyBuild;
            let v15: Rc<str> = class_word_31(v14.clone());
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v15.clone()));
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), v16.clone()));
            let v18: US0 = US0::US0_Compile;
            let v19: Rc<str> = class_word_31(v18.clone());
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v19.clone()));
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v17.clone(), v20.clone()));
            let v22: US0 = US0::US0_CompilerInstall;
            let v23: Rc<str> = class_word_31(v22.clone());
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v23.clone()));
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v21.clone(), v24.clone()));
            let v26: US0 = US0::US0_TreeRead;
            let v27: Rc<str> = class_word_31(v26.clone());
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v27.clone()));
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v25.clone(), v28.clone()));
            let v30: US0 = US0::US0_Bench;
            let v31: Rc<str> = class_word_31(v30.clone());
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v31.clone()));
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v29.clone(), v32.clone()));
            let v34: US0 = US0::US0_CompileHold;
            let v35: Rc<str> = class_word_31(v34.clone());
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v35.clone()));
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v33.clone(), v36.clone()));
            let v38: US0 = US0::US0_TreeWrite;
            let v39: Rc<str> = class_word_31(v38.clone());
            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v39.clone()));
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v37.clone(), v40.clone()));
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = concat5_3(v5.clone(), v41.clone(), v42.clone(), v42.clone(), v42.clone());
            let v44: Rc<str> = concat5_3(v3.clone(), v0.clone(), v4.clone(), v43.clone(), v42.clone());
            refused_2(v44.clone())
        }
    }
}
fn command_line_argument_count_50() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_51(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_49(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_50();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US7 = if v4 {
        let v5: Rc<str> = command_line_argument_51(v0);
        US7::US7_Some(v5)
    } else {
        US7::US7_None
    };
    match &v8 {
        US7::US7_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US7::US7_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_49(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn request_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_49(v0);
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
                            let mut v6: Rc<str> = v6.clone();
                            let mut v7: Rc<UH0> = v7.clone();
                            match &*v7 {
                                UH0::UH0_Cons(v8, v9) => {
                                    let mut v8: Rc<str> = v8.clone();
                                    let mut v9: Rc<UH0> = v9.clone();
                                    window_requested_1(v6.clone(), v8)
                                }
                                UH0::UH0_Nil => {
                                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    window_requested_1(v6, v11)
                                }
                            }
                        }
                        _ => {
                            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            window_requested_1(v15.clone(), v15)
                        }
                    }
                }
                _ => {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    window_requested_1(v18.clone(), v18)
                }
            }
        }
        _ => {
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            window_requested_1(v21.clone(), v21)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        request_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_window_request_run() -> i32 {
    closure0()()
}
