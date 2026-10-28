#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(windows)]
pub fn kit_parented_start(executable: &str, args: &[&str], owner: u32) -> (u64, u64) {
    use std::os::windows::ffi::OsStrExt;
    #[repr(C)]
    struct StartupInfo { cb: u32, reserved: *mut u16, desktop: *mut u16, title: *mut u16, x: u32, y: u32, x_size: u32, y_size: u32, x_chars: u32, y_chars: u32, fill: u32, flags: u32, show: u16, reserved2_len: u16, reserved2: *mut u8, input: *mut std::ffi::c_void, output: *mut std::ffi::c_void, error: *mut std::ffi::c_void }
    #[repr(C)]
    struct StartupInfoEx { startup: StartupInfo, attributes: *mut std::ffi::c_void }
    #[repr(C)]
    struct ProcessInfo { process: *mut std::ffi::c_void, thread: *mut std::ffi::c_void, pid: u32, tid: u32 }
    unsafe extern "system" {
        fn CreateProcessW(app: *const u16, line: *mut u16, process_attributes: *mut std::ffi::c_void, thread_attributes: *mut std::ffi::c_void, inherit: i32, flags: u32, environment: *mut std::ffi::c_void, cwd: *const u16, startup: *mut StartupInfo, info: *mut ProcessInfo) -> i32;
        fn CloseHandle(handle: *mut std::ffi::c_void) -> i32;
        fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void;
        fn InitializeProcThreadAttributeList(list: *mut std::ffi::c_void, count: u32, flags: u32, size: *mut usize) -> i32;
        fn UpdateProcThreadAttribute(list: *mut std::ffi::c_void, flags: u32, attribute: usize, value: *mut std::ffi::c_void, size: usize, previous: *mut std::ffi::c_void, returned: *mut usize) -> i32;
        fn DeleteProcThreadAttributeList(list: *mut std::ffi::c_void);
    }
    #[link(name = "user32")]
    unsafe extern "system" {
        fn GetShellWindow() -> *mut std::ffi::c_void;
        fn GetWindowThreadProcessId(window: *mut std::ffi::c_void, pid: *mut u32) -> u32;
    }
    let quote = char::from(34u8);
    let quoted = |arg: &str| if arg.is_empty() || arg.contains(char::from(32u8)) || arg.contains(quote) { format!("{quote}{}{quote}", arg.replace(quote, &format!("{}{quote}", char::from(92u8)))) } else { arg.to_owned() };
    let line = std::iter::once(quoted(executable)).chain(args.iter().map(|arg| quoted(arg))).collect::<Vec<_>>().join(" ").encode_utf16().chain(std::iter::once(0u16)).collect::<Vec<u16>>();
    let cwd = std::env::temp_dir().as_os_str().encode_wide().chain(std::iter::once(0u16)).collect::<Vec<u16>>();
    let mut shell = owner;
    if shell == 0 {
        let window = unsafe { GetShellWindow() };
        if !window.is_null() { unsafe { GetWindowThreadProcessId(window, &mut shell) }; }
    }
    let parent = if shell == 0 { std::ptr::null_mut() } else { unsafe { OpenProcess(0x0080, 0, shell) } };
    let mut size = 0usize;
    unsafe { InitializeProcThreadAttributeList(std::ptr::null_mut(), 1, 0, &mut size) };
    let mut list = vec![0u8; size.max(64)];
    let list_ptr = list.as_mut_ptr() as *mut std::ffi::c_void;
    let mut value = parent;
    let attributed = !parent.is_null() && unsafe { InitializeProcThreadAttributeList(list_ptr, 1, 0, &mut size) } != 0 && unsafe { UpdateProcThreadAttribute(list_ptr, 0, 0x0002_0000, &mut value as *mut *mut std::ffi::c_void as *mut std::ffi::c_void, std::mem::size_of::<*mut std::ffi::c_void>(), std::ptr::null_mut(), std::ptr::null_mut()) } != 0;
    let start = |flags: u32, with_parent: bool| -> u64 {
        let mut line = line.clone();
        let mut startup: StartupInfoEx = unsafe { std::mem::zeroed() };
        startup.startup.cb = (if with_parent { std::mem::size_of::<StartupInfoEx>() } else { std::mem::size_of::<StartupInfo>() }) as u32;
        if with_parent { startup.attributes = list_ptr; }
        let mut info: ProcessInfo = unsafe { std::mem::zeroed() };
        let flags = if with_parent { flags | 0x0008_0000 } else { flags };
        if unsafe { CreateProcessW(std::ptr::null(), line.as_mut_ptr(), std::ptr::null_mut(), std::ptr::null_mut(), 0, flags, std::ptr::null_mut(), cwd.as_ptr(), &mut startup as *mut StartupInfoEx as *mut StartupInfo, &mut info) } == 0 { 0 } else { unsafe { CloseHandle(info.thread); CloseHandle(info.process); } info.pid as u64 }
    };
    let tries = [(0x0100_0228u32, true), (0x0000_0228u32, true), (0x0100_0228u32, false), (0x0000_0228u32, false)];
    let mut parent_pid = 0u64;
    let mut pid = 0u64;
    for (flags, with_parent) in tries.iter() {
        if *with_parent && !attributed { continue; }
        pid = start(*flags, *with_parent);
        if pid != 0 { parent_pid = if *with_parent { shell as u64 } else { 0 }; break; }
    }
    if attributed { unsafe { DeleteProcThreadAttributeList(list_ptr) }; }
    if !parent.is_null() { unsafe { CloseHandle(parent) }; }
    (pid, parent_pid)
}

#[cfg(windows)]
pub fn kit_shell_parented_start(executable: &str, args: &[&str]) -> (u64, u64) { kit_parented_start(executable, args, 0) }

#[cfg(windows)]
pub fn kit_process_started_ms(pid: u64) -> u64 {
    unsafe extern "system" {
        fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void;
        fn CloseHandle(handle: *mut std::ffi::c_void) -> i32;
        fn GetProcessTimes(process: *mut std::ffi::c_void, creation: *mut u64, exit: *mut u64, kernel: *mut u64, user: *mut u64) -> i32;
        fn GetExitCodeProcess(process: *mut std::ffi::c_void, code: *mut u32) -> i32;
    }
    if pid == 0 { return 0; }
    let handle = unsafe { OpenProcess(0x1000, 0, pid as u32) };
    if handle.is_null() { return 0; }
    let (mut creation, mut exit, mut kernel, mut user, mut code) = (0u64, 0u64, 0u64, 0u64, 0u32);
    let timed = unsafe { GetProcessTimes(handle, &mut creation, &mut exit, &mut kernel, &mut user) } != 0;
    let running = unsafe { GetExitCodeProcess(handle, &mut code) } != 0 && code == 259;
    unsafe { CloseHandle(handle) };
    if timed && running { creation / 10_000 - 11_644_473_600_000 } else { 0 }
}

#[cfg(windows)]
pub fn kit_ancestry_text() -> String {
    #[repr(C)]
    struct Entry { size: u32, usage: u32, pid: u32, heap: usize, module: u32, threads: u32, parent: u32, base_priority: i32, flags: u32, exe: [u16; 260] }
    unsafe extern "system" {
        fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> *mut std::ffi::c_void;
        fn Process32FirstW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32;
        fn Process32NextW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32;
        fn CloseHandle(handle: *mut std::ffi::c_void) -> i32;
    }
    let snapshot = unsafe { CreateToolhelp32Snapshot(2, 0) };
    if snapshot.is_null() || snapshot as isize == -1 { return String::new(); }
    let mut table = std::collections::HashMap::new();
    let mut entry: Entry = unsafe { std::mem::zeroed() };
    entry.size = std::mem::size_of::<Entry>() as u32;
    let mut more = unsafe { Process32FirstW(snapshot, &mut entry) } != 0;
    while more {
        let length = entry.exe.iter().position(|unit| *unit == 0).unwrap_or(entry.exe.len());
        table.insert(entry.pid, (entry.parent, String::from_utf16_lossy(&entry.exe[..length])));
        more = unsafe { Process32NextW(snapshot, &mut entry) } != 0;
    }
    unsafe { CloseHandle(snapshot) };
    let mut rows = Vec::new();
    let mut child = std::process::id();
    let mut child_started = kit_process_started_ms(child as u64);
    for _ in 0..64 {
        let Some((parent, _)) = table.get(&child).cloned() else { break };
        let Some((_, name)) = table.get(&parent).cloned() else { break };
        let started = kit_process_started_ms(parent as u64);
        if started == 0 || (child_started != 0 && started > child_started) { break; }
        rows.push(format!("{parent}|{name}|{started}"));
        child = parent;
        child_started = started;
    }
    rows.join("\n")
}

#[cfg(unix)]
pub fn kit_parented_start(executable: &str, args: &[&str], _owner: u32) -> (u64, u64) { kit_shell_parented_start(executable, args) }

#[cfg(unix)]
pub fn kit_process_started_ms(_pid: u64) -> u64 { 0 }

#[cfg(unix)]
pub fn kit_ancestry_text() -> String { String::new() }

#[cfg(unix)]
pub fn kit_shell_parented_start(executable: &str, args: &[&str]) -> (u64, u64) {
    use std::os::unix::process::CommandExt;
    let mut command = std::process::Command::new(executable);
    command.args(args).current_dir(std::env::temp_dir()).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).process_group(0);
    (command.spawn().map(|child| child.id() as u64).unwrap_or(0), 0)
}

pub fn kit_shell_parented_start_text(executable: &str, args: &str) -> String { let parts: Vec<&str> = args.split(char::from(9u8)).filter(|arg| !arg.is_empty()).collect(); let (pid, parent) = kit_shell_parented_start(executable, &parts); format!("pid={pid} parent={parent}") }

fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_AncestryEnd,
    UH0_Ancestor(u64, Rc<str>, u64, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_OwnedBy(u64, Rc<str>, u64),
    US2_NoOwner,
}
#[derive(Clone)]
enum US3 {
    US3_ShellLauncher,
    US3_KitLauncher,
    US3_OwnerProcess,
}
struct Mut0 { l0: Rc<str> }
fn command_line_argument_count_1() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_2(mut v0: i32) -> Rc<str> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_1();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_3(v0);
        US0::US0_Some(v5.clone())
    } else {
        US0::US0_None
    };
    match &v8 {
        US0::US0_None => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US0::US0_Some(v9) => {
            let v9: Rc<str> = v9.clone();
            v9.clone()
        }
    }
}
fn joined_args_4(mut v0: i32, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: bool = v1 <= v0;
        if v3 {
            return v2.clone();
        } else {
            let v4: i32 = v0.wrapping_add(1i32);
            let v5: Rc<str> = argument_2(v0);
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v5));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            (v0, v1, v2) = (v4, v1, v8.clone());
            continue;
        }
    }
}
fn process_run_0() -> i32 {
    let mut v0: i32 = command_line_argument_count_1();
    let v1: i32 = 2i32;
    let v2: Rc<str> = argument_2(v1);
    let v3: i32 = 1i32;
    let v4: Rc<str> = argument_2(v3);
    let v5: bool = &*v4 != "start-under-shell";
    let v7: bool = if v5 {
        true
    } else {
        let v6: bool = &*v2 == "";
        v6
    };
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: process usage: process start-under-shell <program> [args...] (detached, with the user shell as its live parent, no inherited handles, no caller job)"); } LIT.with(|lit| lit.clone()) };
        println!("{}", v8);
        2i32
    } else {
        let v9: i32 = 3i32;
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = joined_args_4(v9, v0, v10.clone());
        let mut v12: Rc<str> = std::rc::Rc::<str>::from(kit_shell_parented_start_text(&*v2, &*v11).as_str());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid=0 "); } LIT.with(|lit| lit.clone()) };
        let v14: bool = v12.starts_with(&*v13);
        if v14 {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: process start-under-shell: could not start "); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v2));
            println!("{}", v16);
            1i32
        } else {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit process start-under-shell ok "); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v12));
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" program="); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v18, v19)), v2));
            println!("{}", v21);
            0i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        process_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method10(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method11(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn number_of_8(mut v0: Rc<str>) -> u64 {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: i32 = 0i32;
    let v3: i32 = method9(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method10(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let v9: Option<u64> = (v8).trim().parse::<u64>().ok();
    let v10: Option<u64> = method11(v9.clone());
    let mut v11: Rc<dyn Fn((u64)) -> US1> = closure2();
    let mut v12: Option<US1> = v10.map(|x| v11(x));
    let v14: US1 = US1::US1_None;
    let v15: US1 = v14.clone();
    let v16: US1 = v12.unwrap_or(v15);
    match &v16 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v17) => {
            let v17: u64 = *v17;
            v17
        }
    }
}
fn ancestry_from_7(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> Rc<UH0> {
    loop {
        let v2: i32 = (v0.clone().borrow().len() as i32);
        let v3: bool = v1 >= v2;
        if v3 {
            return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_AncestryEnd); } CASE.with(|case| case.clone()) };
        } else {
            let v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v5.split(&*v6).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v8: i32 = (v7.clone().borrow().len() as i32);
            let v9: bool = v8 < 3i32;
            if v9 {
                let v10: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v10);
                continue;
            } else {
                let mut v12: Rc<str> = v7.clone().borrow()[0i32 as usize].clone();
                let v13: u64 = number_of_8(v12.clone());
                let v14: Rc<str> = v7.clone().borrow()[1i32 as usize].clone();
                let v15: Rc<str> = v7.clone().borrow()[2i32 as usize].clone();
                let v16: u64 = number_of_8(v15.clone());
                let v17: i32 = v1.wrapping_add(1i32);
                let v18: Rc<UH0> = ancestry_from_7(v0.clone(), v17);
                return Rc::new(UH0::UH0_Ancestor(v13, v14.clone(), v16, v18.clone()));
            }
        }
    }
}
fn ancestry_of_text_6(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: bool = &*v0 == "";
    if v1 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_AncestryEnd); } CASE.with(|case| case.clone()) }
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v5: i32 = 0i32;
        ancestry_from_7(v4.clone(), v5)
    }
}
fn loop_14(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".exe"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(4i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v5 as i64);
            v0 = v6.clone();
            continue;
        } else {
            return v0.clone();
        }
    }
}
fn launcher_kind_of_13(mut v0: Rc<str>) -> US3 {
    let v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let v2: Rc<str> = loop_14(v1.clone());
    let v3: bool = &*v2 == "cmd";
    let v5: bool = if v3 {
        true
    } else {
        let v4: bool = &*v2 == "pwsh";
        v4
    };
    let v7: bool = if v5 {
        true
    } else {
        let v6: bool = &*v2 == "powershell";
        v6
    };
    let v9: bool = if v7 {
        true
    } else {
        let v8: bool = &*v2 == "sh";
        v8
    };
    let mut v11: bool = if v9 {
        true
    } else {
        let v10: bool = &*v2 == "bash";
        v10
    };
    let v15: bool = if v11 {
        true
    } else {
        let mut v12: bool = &*v2 == "conhost";
        if v12 {
            true
        } else {
            let v13: bool = &*v2 == "openconsole";
            v13
        }
    };
    if v15 {
        US3::US3_ShellLauncher
    } else {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
        let v18: bool = v2.starts_with(&*v17);
        if v18 {
            US3::US3_KitLauncher
        } else {
            US3::US3_OwnerProcess
        }
    }
}
fn owner_of_12(mut v0: Rc<UH0>) -> US2 {
    loop {
        match &*v0 {
            UH0::UH0_Ancestor(v2, v3, v4, v5) => {
                let v2: u64 = *v2;
                let v3: Rc<str> = v3.clone();
                let v4: u64 = *v4;
                let v5: Rc<UH0> = v5.clone();
                let v6: US3 = launcher_kind_of_13(v3.clone());
                match &v6 {
                    US3::US3_OwnerProcess => {
                        return US2::US2_OwnedBy(v2, v3.clone(), v4);
                    }
                    _ => {
                        v0 = v5.clone();
                        continue;
                    }
                }
            }
            UH0::UH0_AncestryEnd => {
                return US2::US2_NoOwner;
            }
        }
    }
}
fn method17(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_16(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method17(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn owner_home_name_15(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_NoOwner => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US2::US2_OwnedBy(v1, v2, v3) => {
            let v1: u64 = *v1;
            let v2: Rc<str> = v2.clone();
            let v3: u64 = *v3;
            let v4: Rc<str> = format_real_16(v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
            let v7: Rc<str> = format_real_16(v3);
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            v8.clone()
        }
    }
}
fn caller_owner_5() -> (Rc<str>, u64, u64) {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(kit_ancestry_text().as_str());
    let v1: Rc<UH0> = ancestry_of_text_6(v0.clone());
    let v2: US2 = owner_of_12(v1.clone());
    match &v2 {
        US2::US2_NoOwner => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v7.clone(), 0u64, 0u64)
        }
        US2::US2_OwnedBy(v3, v4, v5) => {
            let v3: u64 = *v3;
            let v4: Rc<str> = v4.clone();
            let v5: u64 = *v5;
            let v6: Rc<str> = owner_home_name_15(v2.clone());
            (v6.clone(), v3, v5)
        }
    }
}
fn closure1() -> Rc<dyn Fn() -> (Rc<str>, u64, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (Rc<str>, u64, u64)> = Rc::new(move || -> (Rc<str>, u64, u64) {
        caller_owner_5()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn owner_present_18(mut v0: u64, mut v1: u64) -> bool {
    let v2: bool = v0 == 0u64;
    if v2 {
        true
    } else {
        let v3: u64 = kit_process_started_ms(v0);
        let v4: bool = v3 == v1;
        v4
    }
}
fn closure3() -> Rc<dyn Fn(u64, u64) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, u64) -> bool> = Rc::new(move |mut v0: u64, mut v1: u64| -> bool {
        owner_present_18(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_process_launch_run() -> i32 {
    closure0()()
}
pub fn kit_caller_owner() -> (Rc<str>, u64, u64) {
    closure1()()
}
pub fn kit_owner_present(v0: u64, v1: u64) -> bool {
    closure3()(v0, v1)
}
