use winapi::um::winnt;

bitflags! {
    pub struct ProcessAccess: u32 {
        const DELETE = winnt::DELETE;
        const READ_CONTROL = winnt::READ_CONTROL;
        const SYNCHRONIZE = winnt::SYNCHRONIZE;
        const WRITE_DAC = winnt::WRITE_DAC;
        const WRITE_OWNER = winnt::WRITE_OWNER;
        const PROCESS_ALL_ACCESS = winnt::PROCESS_ALL_ACCESS;
        const PROCESS_CREATE_PROCESS = winnt::PROCESS_CREATE_PROCESS;
        const PROCESS_CREATE_THREAD = winnt::PROCESS_CREATE_THREAD;
        const PROCESS_DUP_HANDLE = winnt::PROCESS_DUP_HANDLE;
        const PROCESS_QUERY_INFORMATION = winnt::PROCESS_QUERY_INFORMATION;
        const PROCESS_QUERY_LIMITED_INFORMATION = winnt::PROCESS_QUERY_LIMITED_INFORMATION;
        const PROCESS_SET_INFORMATION = winnt::PROCESS_SET_INFORMATION;
        const PROCESS_SET_QUOTA = winnt::PROCESS_SET_QUOTA;
        const PROCESS_SUSPEND_RESUME = winnt::PROCESS_SUSPEND_RESUME;
        const PROCESS_TERMINATE = winnt::PROCESS_TERMINATE;
        const PROCESS_VM_OPERATION = winnt::PROCESS_VM_OPERATION;
        const PROCESS_VM_READ = winnt::PROCESS_VM_READ;
        const PROCESS_VM_WRITE = winnt::PROCESS_VM_WRITE;
    }
}
