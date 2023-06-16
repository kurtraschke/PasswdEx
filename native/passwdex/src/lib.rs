use rustler::{Encoder, Env, Error::Term as ErrorTerm, NifResult, NifStruct, Term};
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

mod atoms {
    rustler::atoms! {
        ok,
        error,
        unsupported,
        not_found
    }
}

#[rustler::nif]
fn getuid(env: Env) -> NifResult<Term> {
    #[cfg(unix)]
    {
        unsafe { Ok((atoms::ok(), libc::getuid()).encode(env)) }
    }

    #[cfg(not(unix))]
    {
        Err(rustler::Error::Term(Box::new(atoms::unsupported())))
    }
}

#[rustler::nif]
fn getgid(env: Env) -> NifResult<Term> {
    #[cfg(unix)]
    {
        unsafe { Ok((atoms::ok(), libc::getgid()).encode(env)) }
    }

    #[cfg(not(unix))]
    {
        Err(rustler::Error::Term(Box::new(atoms::unsupported())))
    }
}

#[rustler::nif]
fn geteuid(env: Env) -> NifResult<Term> {
    #[cfg(unix)]
    {
        unsafe { Ok((atoms::ok(), libc::geteuid()).encode(env)) }
    }

    #[cfg(not(unix))]
    {
        Err(rustler::Error::Term(Box::new(atoms::unsupported())))
    }
}

#[rustler::nif]
fn getegid(env: Env) -> NifResult<Term> {
    #[cfg(unix)]
    {
        unsafe { Ok((atoms::ok(), libc::getegid()).encode(env)) }
    }

    #[cfg(not(unix))]
    {
        Err(rustler::Error::Term(Box::new(atoms::unsupported())))
    }
}

#[rustler::nif]
fn getpwuid(env: Env, uid: u32) -> NifResult<Term> {
    #[cfg(unix)]
    {
        unsafe {
            let p = libc::getpwuid(uid);

            passwd_to_term(env, p)
        }
    }

    #[cfg(not(unix))]
    {
        Err(rustler::Error::Term(Box::new(atoms::unsupported())))
    }
}

#[rustler::nif]
fn getpwnam(env: Env, name: String) -> NifResult<Term> {
    #[cfg(unix)]
    {
        unsafe {
            let n = CString::new(name).unwrap();

            let p = libc::getpwnam(n.as_ptr());

            passwd_to_term(env, p)
        }
    }

    #[cfg(not(unix))]
    {
        Err(rustler::Error::Term(Box::new(atoms::unsupported())))
    }
}

#[cfg(unix)]
unsafe fn passwd_to_term(env: Env, p: *mut libc::passwd) -> NifResult<Term> {
    if !p.is_null() {
        Ok((
            atoms::ok(),
            Passwd {
                name: c_char_to_string((*p).pw_name),
                passwd: c_char_to_string((*p).pw_passwd),
                uid: (*p).pw_uid,
                gid: (*p).pw_gid,
                gecos: c_char_to_string((*p).pw_gecos),
                dir: c_char_to_string((*p).pw_dir),
                shell: c_char_to_string((*p).pw_shell),
            },
        )
            .encode(env))
    } else {
        Err(ErrorTerm(Box::new(atoms::not_found())))
    }
}

#[cfg(unix)]
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
