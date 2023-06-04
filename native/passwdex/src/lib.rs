use rustler::{types::atom::nil, Encoder, Env, NifStruct, Term};
use std::ffi::{CStr, CString};

#[derive(Debug, NifStruct)]
#[module = "PasswdEx.Passwd"]
struct Passwd {
    name: String,
    passwd: String,
    uid: u32,
    gid: u32,
    gecos: String,
    dir: String,
    shell: String,
}

#[rustler::nif]
fn getuid() -> u32 {
    unsafe { libc::getuid() }
}

#[rustler::nif]
fn getgid() -> u32 {
    unsafe { libc::getgid() }
}

#[rustler::nif]
fn geteuid() -> u32 {
    unsafe { libc::geteuid() }
}

#[rustler::nif]
fn getegid() -> u32 {
    unsafe { libc::getegid() }
}

#[rustler::nif]
fn getpwuid(env: Env<'_>, uid: u32) -> Term<'_> {
    unsafe {
        let p = libc::getpwuid(uid);

        passwd_to_term(p, env)
    }
}

#[rustler::nif]
fn getpwnam(env: Env<'_>, name: String) -> Term<'_> {
    unsafe {
        let n = CString::new(name).unwrap();

        let p = libc::getpwnam(n.as_ptr());

        passwd_to_term(p, env)
    }
}

unsafe fn passwd_to_term(p: *mut libc::passwd, env: Env) -> Term<'_> {
    if !p.is_null() {
        Passwd {
            name: c_char_to_string((*p).pw_name),
            passwd: c_char_to_string((*p).pw_passwd),
            uid: (*p).pw_uid,
            gid: (*p).pw_gid,
            gecos: c_char_to_string((*p).pw_gecos),
            dir: c_char_to_string((*p).pw_dir),
            shell: c_char_to_string((*p).pw_shell),
        }
        .encode(env)
    } else {
        nil().encode(env)
    }
}

unsafe fn c_char_to_string(ptr: *const libc::c_char) -> String {
    let cstr = CStr::from_ptr(ptr);
    let str_slice = cstr.to_str().unwrap();
    let str_buf = str_slice.to_owned();

    str_buf
}

rustler::init!(
    "Elixir.PasswdEx",
    [getuid, getgid, geteuid, getegid, getpwuid, getpwnam]
);
