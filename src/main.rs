//! Minimal UOMI agent: copy input to output via runtime FFI.

#[no_mangle]
pub extern "C" fn process() {
    let input = read_input();
    write_output(&input);
}

// === Runtime FFI ===
#[link(wasm_import_module = "env")]
extern "C" {
    fn uomi_read_input(ptr: *mut u8) -> usize;
    fn uomi_save_output(ptr: *const u8, len: usize);
}

fn read_input() -> Vec<u8> {
    // allocate a buffer; runtime will write into it and return actual length
    let mut buf = vec![0u8; 64 * 1024];
    let n = unsafe { uomi_read_input(buf.as_mut_ptr()) };
    buf.truncate(n);
    buf
}

fn write_output(data: &[u8]) {
    unsafe { uomi_save_output(data.as_ptr(), data.len()) }
}
