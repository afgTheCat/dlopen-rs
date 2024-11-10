use dlopen_rs::ELFLibrary;
use std::path::Path;

fn main() {
    let path = Path::new("./target/release/libexample.so");

    let libc = ELFLibrary::sys_load("libc.so.6").unwrap();

    let libexample = ELFLibrary::from_file(path)
        .unwrap()
        .relocate(&[libc])
        .finish()
        .unwrap();

    let f: dlopen_rs::Symbol<fn(i32, i32) -> i32> = unsafe { libexample.get("add").unwrap() };
    println!("{}", f(1, 1));

    let g: dlopen_rs::Symbol<fn(&str)> = unsafe { libexample.get("print").unwrap() };
    g("dlopen-rs: hello world");

    let f: dlopen_rs::Symbol<fn()> = unsafe { libexample.get("thread_local").unwrap() };
    f();

    let f: dlopen_rs::Symbol<fn()> = unsafe { libexample.get("panic").unwrap() };
    f();
}
