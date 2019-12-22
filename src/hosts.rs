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
extern crate reqwest;

use std::io::BufRead;

#[cfg(windows)]
static HOSTS_PATH: &'static str = r#"C:\Windows\System32\drivers\etc\hosts"#;
#[cfg(windows)]
static NEW_LINE: &'static str = "\r\n";

#[cfg(unix)]
static HOSTS_PATH: &'static str = r#"/etc/hosts"#;
#[cfg(unix)]
static NEW_LINE: &'static str = "\n";

#[cfg(windows)]
use platform_utils::win32 as utils;

#[cfg(unix)]
use platform_utils::nix as utils;

pub fn overwrite(address: &str) -> bool {
    let mut changed_perms = false;

    if address.is_empty() {
        utils::send_notify("주소가 입력되어 있지 않습니다!");
        return false;
    }

    if is_connected() {
        return true;
    }

    if is_read_only(HOSTS_PATH) {
        set_read_only(HOSTS_PATH, false);
        changed_perms = true;
    }

    // 1. Remove all other entries for ppy.sh
    // 2. Add new entry for ppy.sh
    let lines = read_file_lines(HOSTS_PATH);
    let mut hosts = lines.clone();

    for (i, line) in lines.iter().enumerate() {
        if (!line.starts_with("#") && line.contains("ppy.sh")) {
            hosts[i] = "#".to_owned() + line;
        }
    }

    hosts.push(format!("{} osu.ppy.sh", address));
    hosts.push(format!("{} c.ppy.sh", address));
    hosts.push(format!("{} ce.ppy.sh", address));

    for i in 1..7 {
        if i == 2 {
            continue;
        }

        hosts.push(format!("{} c{}.ppy.sh", address, i));
    }

    hosts.push(format!("{} a.ppy.sh", address));
    hosts.push(format!("{} i.ppy.sh", address));



    hosts.push(NEW_LINE.to_owned());

    let result = hosts.join(NEW_LINE);

    let file = std::fs::File::create(HOSTS_PATH).unwrap();
    file.set_len(0).unwrap();

    std::fs::write(HOSTS_PATH, result).unwrap();

    if changed_perms {
        set_read_only(HOSTS_PATH, true);
    }

    return true;
}

pub fn revert() -> bool {
    let mut changed_perms = false;

    if !is_connected() {
        return true;
    }

    if is_read_only(HOSTS_PATH) {
        set_read_only(HOSTS_PATH, false);
        changed_perms = true;
    }

    let lines = read_file_lines(HOSTS_PATH);
    let mut hosts = lines.clone();
    let mut fline = 0;
    let mut fline2 = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.contains("ppy.sh") {
            fline = fline + 1;
        }
    }
    println!("Found {} lines to edit", fline);
    let mut done = false;
    while !done {
        for (i, line) in lines.iter().enumerate() {
            if line.contains("ppy.sh") {
                if (i>0){
                    hosts.remove(i-fline2);
                } else {
                    hosts.remove(i);
                }
                fline = fline - 1;
                fline2 = fline2 + 1;
                if (fline == 0){
                    done = true;
                }
            }
        }
    }

    

    hosts.retain(|s| s != "removed by kyo-rs (osukeesu)");

    let result = hosts.join(NEW_LINE);

    let file = std::fs::File::create(HOSTS_PATH).unwrap();
    file.set_len(0).unwrap();

    std::fs::write(HOSTS_PATH, result).unwrap();

    if changed_perms {
        set_read_only(HOSTS_PATH, true);
    }

    return true;
}

pub fn is_connected() -> bool {
    let file = std::fs::File::open(HOSTS_PATH).unwrap();

    for content in std::io::BufReader::new(file).lines() {
        let unwrapped = content.unwrap();
        let line = unwrapped.as_str();

        if line.starts_with("#") || !line.contains("ppy.sh") {
            continue;
        }

        if line.contains(super::KEESU_IP) || line.contains(super::MIRROR_IP) {
            return true;
        }
    }

    return false;
}

fn is_read_only(file: &str) -> bool {
    let perms = std::fs::metadata(file).unwrap().permissions();
    return perms.readonly();
}

fn set_read_only(file: &str, read_only: bool) {
    let mut perms = std::fs::metadata(file).unwrap().permissions();
    perms.set_readonly(read_only);
    std::fs::set_permissions(file, perms).unwrap();
}

fn read_file_lines<P>(filename: P) -> Vec<String> where P: AsRef<std::path::Path>, {
    let file = std::fs::File::open(filename).unwrap();
    let buf = std::io::BufReader::new(file);
    buf.lines()
        .map(|l| l.unwrap())
        .collect()
}
