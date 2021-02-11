// spell-checker:ignore (ToDO) errno EPERM ENOSYS

pub type Pid = u8;

pub struct ProcessChecker(());

impl ProcessChecker {
    pub fn new(_process_id: self::Pid) -> ProcessChecker {
        ProcessChecker(())
    }

    // Borrowing mutably to be aligned with Windows implementation
    pub fn is_dead(&mut self) -> bool {
        unreachable!()
    }
}

impl Drop for ProcessChecker {
    fn drop(&mut self) {}
}

pub fn supports_pid_checks(_pid: self::Pid) -> bool {
    false
}
