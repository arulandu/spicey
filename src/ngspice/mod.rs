mod structs;
use structs::{NgVecInfoAll, NgVecValuesAll};

use libc::{c_char, c_int, c_void};

#[cfg(unix)]
use libloading::os::unix::Symbol;
#[cfg(windows)]
use libloading::os::windows::Symbol;
use libloading::{Library, library_filename};

use std::ffi::OsStr;


pub struct NgSpice {
    lib: Library,
}


type NgSpice_Init = extern "C" fn(
    Option<unsafe extern "C" fn(*const c_char, c_int, *const c_void) -> c_int>, 
    Option<unsafe extern "C" fn(*const c_char, c_int, *const c_void) -> c_int>, 
    Option<unsafe extern "C" fn(c_int, bool, bool, c_int, *const c_void) -> c_int>, 
    Option<unsafe extern "C" fn(*const NgVecValuesAll, c_int, c_int, *const c_void) -> c_int>, 
    Option<unsafe extern "C" fn(*const NgVecInfoAll, c_int, *const c_void) -> c_int>, 
    Option<unsafe extern "C" fn(bool, c_int, *const c_void) -> c_int>, 
    *const c_void, 
) -> c_int;
type NgSpice_Command = extern "C" fn(*const c_char) -> c_int;

impl NgSpice {
    pub fn new(libpath: Option<String>) -> Result<Self, libloading::Error> {
        let path = libpath.unwrap_or("./lib/libngspice.dylib".to_string());

        unsafe {
            let lib = Library::new(OsStr::new(&path))?;
            Ok(Self { lib })
        }
    }

    unsafe fn get_symbol<T>(&self, name: &[u8]) -> Symbol<T> {
        unsafe {
            let symbol = self.lib.get(name).unwrap();
            libloading::Symbol::<T>::into_raw(symbol)
        }
    }

    pub fn init<T: NgSpiceManager>(&self, manager: Option<T>) -> Result<(), String> {
        unsafe extern "C" fn cbw_send_char<T:NgSpiceManager>(msg: *const c_char, id: c_int, user: *const c_void) -> c_int{
            unsafe {
                <T as NgSpiceManager>::send_char(&mut *(user as *mut T), std::ffi::CStr::from_ptr(msg).to_str().unwrap().to_owned(), id);
            }
            0
        }

        unsafe extern "C" fn cbw_send_stat<T:NgSpiceManager>(msg: *const c_char, id: c_int, user: *const c_void) -> c_int {
            unsafe {
                <T as NgSpiceManager>::send_stat(&mut *(user as *mut T), std::ffi::CStr::from_ptr(msg).to_str().unwrap().to_owned(), id);
            }
            0
        }
        unsafe extern "C" fn cbw_controlled_exit<T:NgSpiceManager>(status: c_int, immediate: bool, exit_on_quit: bool, id: c_int, user: *const c_void) -> c_int {
            unsafe {
                <T as NgSpiceManager>::controlled_exit(&mut *(user as *mut T), status, immediate, exit_on_quit, id);
            }
            0
        }
        unsafe extern "C" fn cbw_send_data<T:NgSpiceManager>(pvecvaluesall: *const NgVecValuesAll, count: c_int, id: c_int, user: *const c_void) -> c_int {
            unsafe {
                <T as NgSpiceManager>::send_data(&mut *(user as *mut T), pvecvaluesall, count, id);
            }
            0
        }
        unsafe extern "C" fn cbw_send_init_data<T:NgSpiceManager>(pvecinfoall: *const NgVecInfoAll, id: c_int, user: *const c_void) -> c_int {
            unsafe {
                <T as NgSpiceManager>::send_init_data(&mut *(user as *mut T), pvecinfoall, id);
            }
            0
        }
        unsafe extern "C" fn cbw_bgthread_running<T:NgSpiceManager>(finished: bool, id: c_int, user: *const c_void) -> c_int {
            unsafe {
                <T as NgSpiceManager>::bgthread_running(&mut *(user as *mut T), finished, id);
            }
            0
        }
        
        let ret = unsafe {
            let ngSpice_Init = self.get_symbol::<NgSpice_Init>(b"ngSpice_Init\0");
            match manager {
                Some(m) => ngSpice_Init(
                    Some(cbw_send_char::<T>), 
                    Some(cbw_send_stat::<T>), 
                    Some(cbw_controlled_exit::<T>), 
                    Some(cbw_send_data::<T>), 
                    Some(cbw_send_init_data::<T>), 
                    Some(cbw_bgthread_running::<T>), 
                    (&m as *const T) as *const c_void
                ),
                None => ngSpice_Init(None, None, None, None, None, None, std::ptr::null()), // clear control structures
            }
        };
        match ret {
            0 => Ok(()),
            _ => Err(format!("Init failed: {}", ret)),
        }
    }

    pub fn command(&self, s: &str) -> Result<(), String> {
        let ret = unsafe {
            let ngSpice_Command =
                self.get_symbol::<NgSpice_Command>(b"ngSpice_Command\0");
            if s.find("bg_") == Some(0) {
                eprintln!(
                    "Warning: Background commands are not supported. Use Rust threads instead."
                );
                0;
            }

            let s2 = std::ffi::CString::new(s).unwrap();
            let ps2: *const c_char = if s.is_empty() {
                std::ptr::null() // release control structures
            } else {
                s2.as_ptr()
            };

            ngSpice_Command(ps2) 
        };

        match ret{
            0 => Ok(()),
            _ => Err(format!("Command failed: {}", s)),
        }
    }
}

pub trait NgSpiceManager where Self : Sized  {
    fn send_char(&mut self, msg: String, id: i32);
    fn send_stat(&mut self, msg: String, id: i32) {}
    fn controlled_exit(&mut self, status: i32, is_immediate: bool, exit_on_quit: bool, id: i32) {}
    fn send_data(&mut self, pvecvaluesall: *const NgVecValuesAll, count: i32, id: i32) {}
    fn send_init_data(&mut self, pvecinfoall: *const NgVecInfoAll, id: i32) {}
    fn bgthread_running(&mut self, finished: bool, id: i32) {}
}



#[test]
fn ngspice_test() {
    struct Manager {
        message: String,
    }
    
    impl NgSpiceManager for Manager {
        fn send_char(&mut self, msg: String, id: i32) {
            self.message = msg;
        }
    }
    let ng = NgSpice::new(None).unwrap();
    let manager = Manager { message: String::from("Hello, world!") };
    ng.init(Some(manager)).unwrap();
    // ng.command("source ./netlists/rcrcac.sp").unwrap();
    ng.command("source ./netlists/vdiv.sp").unwrap();
    ng.command("op").unwrap();
    ng.command("print out").unwrap();
    println!("Library: {:?}", ng.lib);
}
