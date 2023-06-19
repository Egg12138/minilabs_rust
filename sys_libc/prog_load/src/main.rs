#![feature(register_tool)]
#![register_tool(cilppy)]
#![allow(cilppy::unwrap_used)]
use std::env;
use std::ffi::CString;
use std::num::NonZeroUsize;
use libc::{self, printf, memcmp, Elf64_Phdr};
use nix::fcntl::{ open, OFlag};
use nix::sys::mman::MapFlags;
use nix::sys::{ mman, stat::Mode};
use nix::Result;


fn main() {
      
      let mut argv = env::args();
      let _ = argv.next();

      // open file and get the file size
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
          libc::close(fd);
        } else {
          eprintln!("Error, failed to parse the argument");
        }
      }

      // TODO: practice: open file and mmap to a Elf64_Ehdr
       // Using rust-like nix ffi.
       /*
        pub unsafe fn mmap(
        addr: Option<NonZeroUsize>,
        length: NonZeroUsize,
        prot: ProtFlags,
        flags: MapFlags,
        fd: RawFd,
        offset: off_t
        ) -> Result<*mut c_void> 
        */
        println!("open again...");
        
        execve_(env::args());

       // TODO: Elf64_Ehdr parser, 
       // TODO: Check magic



}

pub(crate) fn execve_(mut args: std::env::Args) -> Result<()>{
        let _  = args.next();
        if let Some(path)  = args.next() {
          // let nixpath = nix::NixPath::
          let oflag = OFlag::O_RDONLY;
          // let olag = OFlag::O_RDONLY | OFlag::O_TRUNC | OFlag::O_CREAT;
          let mode = Mode::empty();
          let nixfd = open(path.as_str(), oflag, mode).unwrap();
           
          unsafe {
            let mflag = mman::MapFlags::MAP_PRIVATE;
            let mprot = mman::ProtFlags::PROT_READ;
            let addr = Some(NonZeroUsize::MIN);
            let length = NonZeroUsize::new(4096).unwrap(); 
            // definitely success, so unwarp it directly.
            let mmap_res = mman::mmap(addr, length, mprot, mflag, nixfd, 0)  ;
            match mmap_res {
              Ok(hdr_ref) => {
                assert_ne!(hdr_ref , libc::MAP_FAILED);
                let hdr = hdr_ref as *mut libc::Elf64_Ehdr;
                assert_eq!((*hdr).e_type, libc::ET_EXEC);
                // check is a valid ELF 
                assert_eq!((*hdr).e_machine, libc::EM_X86_64);

                assert_eq!((*hdr).e_ident[libc::EI_MAG0],  libc::ELFMAG0);
                assert_eq!((*hdr).e_ident[libc::EI_MAG1],  libc::ELFMAG1);
                assert_eq!((*hdr).e_ident[libc::EI_MAG2],  libc::ELFMAG2);
                assert_eq!((*hdr).e_ident[libc::EI_MAG3],  libc::ELFMAG3);

                let pht: *mut libc::Elf64_Phdr = (hdr as u64 + (*hdr).e_phoff) as *mut libc::Elf64_Phdr;

                for i in 0..((*hdr).e_phnum) {
                  let pht_slice = std::slice::from_raw_parts_mut(pht.as_mut().unwrap(),  (*hdr).e_phnum as usize);
                  // NOTICE: encountered some tackles in Type convertion ;
                  let p = pht_slice.get(i as usize).unwrap() as *mut libc::Elf64_Phdr;
                  // let p: *mut libc::Elf64_Phdr = pht_slice.get(i as usize).unwrap();

                } 
              },
              Err(e) => { return Err(e); }
            }            
          }

        };

      
        Ok(())

}