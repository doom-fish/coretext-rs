#[cfg(feature = "raw-ffi")]
fn main() {
    use core::mem::size_of;

    use coretext::ffi;

    println!(
        "underline={} font_options={} sfnt_dir={} kern_header={}",
        ffi::kCTUnderlineStyleSingle,
        ffi::kCTFontOptionsPreventAutoActivation,
        size_of::<ffi::sfntDirectory>(),
        size_of::<ffi::KernTableHeader>(),
    );
}

#[cfg(not(feature = "raw-ffi"))]
fn main() {
    eprintln!("Run this example with `--features raw-ffi`.");
}
