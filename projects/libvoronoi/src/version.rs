// Copyright (C) 2026  Antonio-Miguel Corbi Bellot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// -- Constants: ----------------------------------------------------------
pub static VERSION_MAJOR: &str = "2026";
pub static VERSION_MINOR: &str = "10";
pub static VERSION_MICRO: &str = "14";

// Función auxiliar const para concatenar y formatear con puntos
const fn build_version(major: &str, minor: &str, micro: &str) -> [u8; 8] {
    let ma = major.as_bytes();
    let mi = minor.as_bytes();
    let mc = micro.as_bytes();

    let mut buf = [0u8; 8];
    let mut i = 0;

    // Copiar major
    let mut j = 0;
    while j < ma.len() {
        buf[i] = ma[j];
        i += 1;
        j += 1;
    }

    // buf[i] = b'.';
    // i += 1;

    // Copiar minor
    j = 0;
    while j < mi.len() {
        buf[i] = mi[j];
        i += 1;
        j += 1;
    }

    // buf[i] = b'.';
    // i += 1;

    // Copiar micro
    j = 0;
    while j < mc.len() {
        buf[i] = mc[j];
        i += 1;
        j += 1;
    }

    buf
}

const VERSION_BYTES: [u8; 8] = build_version(VERSION_MAJOR, VERSION_MINOR, VERSION_MICRO);

// Convertir el array de bytes estático a &str de forma segura en tiempo de compilación
pub const VERSION: &str = match std::str::from_utf8(&VERSION_BYTES) {
    Ok(s) => s,
    Err(_) => panic!("Versión inválida"),
};

//pub const VERSION: &str = "20261014";
