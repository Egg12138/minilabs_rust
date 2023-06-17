#![feature(register_tool)]
#![register_tool(cilppy)]
#![allow(cilppy::unwrap_used)]
use std::env;
use std::ffi::CString;


fn main() {
      
      let mut argv = env::args();
      let _ = argv.next();

      unsafe {
        let flag = libc::O_RDONLY;
        if let Some(path_cs) = argv.next() {
          let pathraw = CString::new(path_cs.as_str())
            .unwrap();
          let path : *const libc::c_char = pathraw.as_ptr();
          let fd = libc::open(path, flag);
          if fd < 0 { 
            eprintln!("Error, fd = {fd}");
            std::process::exit(-libc::ENOENT);
          } else {
            println!("opened file {path_cs}, fd = {fd}");
          }
          let fsize = libc::lseek(fd, 0, libc::SEEK_END);
          println!("file size is {fsize}");
        } else {
          eprintln!("Error, failed to parse the argument");
        }

      }
}
