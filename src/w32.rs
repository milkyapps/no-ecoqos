use windows::Win32::{
    Foundation::{CloseHandle, GetLastError, FALSE, HANDLE, HMODULE},
    System::{
        ProcessStatus::GetModuleFileNameExA,
        Threading::{
            OpenProcess, SetProcessInformation, PROCESS_POWER_THROTTLING_CURRENT_VERSION,
            PROCESS_POWER_THROTTLING_EXECUTION_SPEED, PROCESS_QUERY_LIMITED_INFORMATION,
            PROCESS_SET_INFORMATION,
        },
    },
};

pub fn enum_processes() -> Vec<usize> {
    let mut pids = [0; 1024 * 100];
    let mut bytes_written = 0;

    unsafe {
        let _ = windows::Win32::System::ProcessStatus::EnumProcesses(
            pids.as_mut_ptr(),
            pids.len() as u32,
            &mut bytes_written,
        );
    }
    unsafe { GetLastError() }.unwrap();

    let qty = bytes_written as usize / std::mem::size_of::<u32>();

    pids[..qty].iter().map(|x| *x as usize).collect()
}

pub fn get_process_name(pid: usize) -> Option<String> {
    let handle =
        unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid as u32) }.ok()?;

    let mut name = [0u8; 1024 * 10];
    let len = unsafe { GetModuleFileNameExA(handle, HMODULE(0), &mut name) };
    let name = std::str::from_utf8(&name[..len as usize]).ok()?;

    Some(name.to_string())
}

pub struct ProcessHandle(HANDLE);

impl Drop for ProcessHandle {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}

pub fn open_process(pid: usize) -> Option<ProcessHandle> {
    unsafe {
        OpenProcess(
            PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_SET_INFORMATION,
            FALSE,
            pid as u32,
        )
    }
    .ok()
    .map(ProcessHandle)
}

pub fn disable_ecoqos(handle: ProcessHandle) -> bool {
    let mut state = windows::Win32::System::Threading::PROCESS_POWER_THROTTLING_STATE {
        Version: PROCESS_POWER_THROTTLING_CURRENT_VERSION,
        ControlMask: PROCESS_POWER_THROTTLING_EXECUTION_SPEED,
        StateMask: 0,
    };

    unsafe {
        SetProcessInformation(
            handle.0,
            windows::Win32::System::Threading::ProcessPowerThrottling,
            &mut state as *mut windows::Win32::System::Threading::PROCESS_POWER_THROTTLING_STATE
                as *mut std::ffi::c_void,
            std::mem::size_of::<windows::Win32::System::Threading::PROCESS_POWER_THROTTLING_STATE>()
                as u32,
        )
    }
    .is_ok()
}
