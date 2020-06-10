use winapi::um::winnt;

bitflags! {
    pub struct ProtectFlag: u32 {
        const PAGE_EXECUTE = winnt::PAGE_EXECUTE;
        const PAGE_EXECUTE_READ = winnt::PAGE_EXECUTE_READ;
        const PAGE_EXECUTE_READWRITE = winnt::PAGE_EXECUTE_READWRITE;
        const PAGE_EXECUTE_WRITECOPY = winnt::PAGE_EXECUTE_WRITECOPY;
        const PAGE_NOACCESS = winnt::PAGE_NOACCESS;
        const PAGE_READONLY = winnt::PAGE_READONLY;
        const PAGE_READWRITE = winnt::PAGE_READWRITE;
        const PAGE_WRITECOPY = winnt::PAGE_WRITECOPY;
        const PAGE_TARGETS_INVALID = winnt::PAGE_TARGETS_INVALID;
        const PAGE_TARGETS_NO_UPDATE = winnt::PAGE_TARGETS_NO_UPDATE;
        const PAGE_GUARD = winnt::PAGE_GUARD;
        const PAGE_NOCACHE = winnt::PAGE_NOCACHE;
        const PAGE_WRITECOMBINE = winnt::PAGE_WRITECOMBINE;
        const PAGE_ENCLAVE_THREAD_CONTROL = winnt::PAGE_ENCLAVE_THREAD_CONTROL;
        const PAGE_ENCLAVE_UNVALIDATED = winnt::PAGE_ENCLAVE_UNVALIDATED;
    }
}
