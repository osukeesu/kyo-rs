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
extern crate hyper;
extern crate reqwest;

use std::io::Read;

pub fn install_cert() {
    std::thread::spawn(move || {
        if download_cert() {
            super::utils::send_notify("인증서가 성공적으로 설치되었습니다.");
        }
    });
}

fn download_cert() -> bool {
    let mut response = reqwest::get(super::CERT_URL).unwrap();

    if !response.status().is_success() {
        super::utils::send_notify(format!(
            "인증서를 다운로드 할 수 없습니다. 서버가 {}를 반환했습니다.", response.status()
        ).as_str());

        return false;
    }

    let mut content = String::new();
    response.read_to_string(&mut content).expect("서버로부터 응답을 받을 수 없습니다.");

    super::utils::install_cert(content.as_str());

    return true;
}
