#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(windows)]
pub fn kit_shell_parented_start(executable: &str, args: &[&str]) -> (u64, u64) {
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
    let window = unsafe { GetShellWindow() };
    let mut shell = 0u32;
    if !window.is_null() { unsafe { GetWindowThreadProcessId(window, &mut shell) }; }
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

#[cfg(unix)]
pub fn kit_shell_parented_start(executable: &str, args: &[&str]) -> (u64, u64) {
    use std::os::unix::process::CommandExt;
    let mut command = std::process::Command::new(executable);
    command.args(args).current_dir(std::env::temp_dir()).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).process_group(0);
    (command.spawn().map(|child| child.id() as u64).unwrap_or(0), 0)
}

pub fn kit_shell_parented_start_text(executable: &str, args: &str) -> String { let parts: Vec<&str> = args.split(char::from(9u8)).filter(|arg| !arg.is_empty()).collect(); let (pid, parent) = kit_shell_parented_start(executable, &parts); format!("pid={pid} parent={parent}") }

#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
fn command_line_argument_count_1() -> i32 {
    let v0: usize = std::env::args().count();
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
    let v0: i32 = command_line_argument_count_1();
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
        let v11: Rc<str> = joined_args_4(v9, v0, v10.clone());
        let v12: Rc<str> = std::rc::Rc::<str>::from(kit_shell_parented_start_text(&*v2, &*v11).as_str());
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
pub fn kit_process_launch_run() -> i32 {
    closure0()()
}
