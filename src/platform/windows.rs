use advapi32::AdjustTokenPrivileges;
use kernel32::GetCurrentProcess;

fn drop_token_privileges() -> bool {
    let pr = unsafe { GetCurrentProcess() };
    unsafe { AdjustTokenPrivileges(pr, true, std::ptr::null_mut(), 0, std::ptr::null_mut(), 0) }
}
