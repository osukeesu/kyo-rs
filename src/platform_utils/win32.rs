/*
 * kyo-rs - Rust rewrite of kyo, a modern osu! server switcher
 * Copyright (C) 2018 Marc3842h
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

extern crate std;
extern crate user32;
extern crate winapi;
extern crate winrt;
extern crate winrt_notification;


use std::io::Error;

use winapi::um::wincrypt::*;
use winrt_notification::{Duration, Toast};

fn to_wstring(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;

    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub fn is_root() -> bool {
    /*unsafe {
        winapi::vc::shell::IsUserAnAdmin()
    }*/

    // The current method above is not implemented in the winapi crate
    // Workaround this by *requiring* the program to be started
    // with admin permissions by specifying it in the manifest.
    true
}

pub fn install_cert(cert: &str) {
    /*unsafe {
        let root_str = std::ffi::CString::new("ROOT").unwrap();
        let cert_str = std::ffi::CString::new(cert).unwrap();

        let context_ptr: *mut PCCERT_CONTEXT = std::ptr::null_mut();
        let cert_store: HCERTSTORE = CertOpenSystemStoreA(0, root_str.as_ptr());

        CertAddEncodedCertificateToStore(
            cert_store,
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            cert_str.as_ptr() as *const _,
            cert.len() as u32,
            CERT_STORE_ADD_USE_EXISTING,
            context_ptr
        );

        CertCloseStore(cert_store, 0);
    }*/

    let mut file = std::env::temp_dir();
    file.push(super::RESULT_CERT_NAME);

    let path = file.as_path();
    let path_str = path.to_str().unwrap();

    std::fs::write(path, cert).expect("디스크에 인증서를 설치할 수 없습니다.");

    std::process::Command::new("certutil.exe")
        .arg("-addstore")
        .arg("Root")
        .arg(path_str)
        .status()
        .expect("루트 인증소에 설치 할 수 없습니다.");
}

// https://github.com/pachi/rust_winapi_examples/blob/master/src/bin/01_helloworld.rs
pub fn send_notify(msg: &str) {
    use std::ptr::null_mut;
    use winapi::um::winuser::{MessageBoxW, MB_ICONINFORMATION, MB_OK};

    let lp_text = to_wstring(msg);
    let lp_caption = to_wstring("Keesu Server Switcher");
    let ret = unsafe {
        MessageBoxW(
            null_mut(),          // hWnd
            lp_text.as_ptr(),    // text
            lp_caption.as_ptr(), // caption (dialog box title)
            MB_OK | MB_ICONINFORMATION,
        )
    };
    if ret == 0 {
        println!("{}", Error::last_os_error());
    }
}
