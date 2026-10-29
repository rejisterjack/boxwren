//! Opens an absolute path through `path_open` itself.
//!
//! `std::fs` on preview 1 gives up before the host call when no preopen covers
//! the path, so the report would not name the attempt.

#[link(wasm_import_module = "wasi_snapshot_preview1")]
unsafe extern "C" {
    #[link_name = "path_open"]
    fn path_open(
        fd: i32,
        dirflags: i32,
        path: *const u8,
        path_len: i32,
        oflags: i32,
        fs_rights_base: i64,
        fs_rights_inheriting: i64,
        fdflags: i32,
        opened_fd: *mut i32,
    ) -> i32;
}

fn main() {
    let path = b"/etc/passwd";
    let mut opened = 0i32;
    let errno = unsafe {
        path_open(
            3,
            0,
            path.as_ptr(),
            path.len() as i32,
            0,
            0,
            0,
            0,
            &mut opened,
        )
    };
    if errno == 0 {
        println!("SECRET opened");
    } else {
        println!("blocked errno {errno}");
    }
}
