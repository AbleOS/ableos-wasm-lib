/*! The ableOS provided library for use with the wasm loader and target


`NOTE: All full caps functions are currently incomplete`*/
#![no_std]

mod panic;

extern "C" {
    /// Kill a specific program with PID
    pub fn kill();
    /// A "special" fall back syscall in case things go really wrong
    pub fn empty();
    /// Exit the program cleanly
    pub fn exit();
    /// Reset the console
    pub fn console_reset();
    /// Console Input
    pub fn console_in();
    /// Console output
    pub fn console_out();
    /// Get the console title
    pub fn console_get_title();
    /// Set the console title
    pub fn console_set_title();

    /// Get the proccess ID
    pub fn get_pid();
    /// Get information about the process
    pub fn proccess_info();

    //scheduler Related Syscals
    /// Get scheduler priority
    pub fn get_priority();
    /// Set scheduler priority
    pub fn set_priority();
    //
    pub fn get_hostname();
    pub fn set_hostname();

    //File Related syscalls
    //
    pub fn MAKE_DIRECTORY(); //
    pub fn DELETE_DIRECTORY(); //
    pub fn RENAME_DIRECTORY(); //
    pub fn SET_DIRECTORY_ACCESS(); //
                                   //
    pub fn MAKE_FILE(); //
    pub fn DELETE_FILE(); //
    pub fn RENAME_FILE(); //
    pub fn SET_FILE_ACCESS(); //

    pub fn FILE_READ();
    pub fn FILE_WRITE();

    /// Sleep in milliseconds
    pub fn SLEEP();
    /// Sleep until this time in milliseconds (if this is below the current time return)
    pub fn SLEEP_UNTIL();
    /// Sleep in nanoseconds
    pub fn NANOSLEEP();
    /// Sleep until this time nanoseconds (if this is below the current time return)
    pub fn NANOSLEEP_UNTIL();
    /// Gets the system time (some derivitive of seconds)
    pub fn GET_TIME();
    /// Sets the system time (some derivitive of seconds)
    pub fn SET_TIME();

    // Socket SysCall
    /// Used by servers to lock a port
    pub fn SOCKET_BIND();
    pub fn SOCKET_CONNECT();
    pub fn SOCKET_DISCONNECT();
    pub fn SOCKET_SEND();
    pub fn SOCKET_RECEIVE();

}
