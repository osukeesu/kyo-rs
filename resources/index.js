/*
 * kyo-rs - Rust rewrite of kyo, a modern osu! server switcher
 * Copyright (C) 2018 Marc3842h, czapek
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

// Invoked by frontend

function updateData() {
    invokeBackend(
        {
            cmd: "update"
        }
    );
}

function connectKeesu() {
    invokeBackend(
        {
            cmd: "connect",
            address: document.getElementById("connect-address").value
        }
    );
}

function connectBancho() {
    invokeBackend(
        {
            cmd: "disconnect"
        }
    );
}

function installCertificate() {
    invokeBackend(
        {
            cmd: "install"
        }
    );
}

function invokeBackend(args) {
    window.external.invoke(JSON.stringify(args));
}

// Invoked by backend

function toggleConnectButton() {
    let connectButton = document.getElementById("btn-connect");

    connectButton.classList.toggle("pink");
    connectButton.classList.toggle("blue");

    // We just switched to Keesu
    if (connectButton.classList.contains("blue")) {
        document.getElementById("text-connect").innerHTML = "<b>Bancho</b>에 접속하기";
        document.getElementById("icon-switch").className = "fas fa-unlink";
        connectButton.onclick = connectBancho;
    }

    // We just switched back to Bancho
    if (connectButton.classList.contains("pink")) {
        document.getElementById("text-connect").innerHTML = "<b>Keesu</b>에 접속하기";
        document.getElementById("icon-switch").className = "fas fa-sync-alt";
        connectButton.onclick = connectKeesu;
    }
}

function displayError() {
    let input = document.getElementById("input-address");
    input.classList.add("shake");

    setTimeout(function() {
        input.classList.remove("shake");
    }, 820);
}


// Init

updateData();
var connectaddr = document.getElementById("connect-address");
connectaddr.focus();

// For known issues

connectaddr.readOnly = true;