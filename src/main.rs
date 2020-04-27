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
#![windows_subsystem = "windows"]
extern crate web_view;
extern crate serde_json;

#[cfg(windows)]
extern crate winrt_notification;
#[cfg(windows)]
extern crate winapi;

mod cert;
mod hosts;
mod platform_utils;

#[cfg(windows)]
use platform_utils::win32 as utils;

#[cfg(unix)]
use platform_utils::nix as utils;

static KEESU_IP: &'static str = r#"206.189.74.123"#;
static MIRROR_IP: &'static str = r#"206.189.74.123"#; // Won't be displayed to user but put in hosts regardless
static CERT_URL: &'static str = r#"https://osu.leu.kr/cert.pem"#;
static RESULT_CERT_NAME: &'static str = r#"keesu.crt"#; // Always needs to end in .crt
static CONTENT: &'static str = include_str!("../resources/index.include.html");

fn main() {
    if !utils::is_root() {
        utils::send_notify("이 실행기를 관리자 권한으로 실행해주세요.");
        std::process::exit(1);
    }

    let user_data = ();
    web_view::run(
        "Keesu server Switcher",
        web_view::Content::Html(CONTENT),
        Some((400, 260)),
        false,
        true,
        move |_web_view| {},
        move |web_view, args, _user_data| {
            let json: serde_json::Value = serde_json::from_str(args).unwrap();
            let cmd = json["cmd"].as_str().unwrap();
            let address = json["address"].as_str().unwrap_or_default();

            match cmd {
                "update" => {
                    let connected = hosts::is_connected();
                    let connect_address = format!("document.getElementById('connect-address').value = '{}';", KEESU_IP);
                    let button_changer = if connected { "toggleConnectButton();" } else { "" };

                    let js = &format!("{}{}", connect_address, button_changer);
                    web_view.eval(js);
                }
                "connect" => {
                    web_view.eval(if hosts::overwrite(address) { "toggleConnectButton();" } else { "displayError();" });
                }
                "disconnect" => {
                    web_view.eval(if hosts::revert() { "toggleConnectButton();" } else { "displayError();" });
                }
                "install" => {
                    cert::install_cert();
                }
                _ => unimplemented!()
            }
        },
        user_data
    );
}
