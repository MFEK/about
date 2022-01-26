mod svgs;
pub(crate) use svgs::*;

pub(crate) static WIDTH: u32 = 1800;
pub(crate) static HEIGHT: u32 = 900;

#[cfg(target_family = "unix")]
pub(crate) static MFEK: &str = "\x1B[2J\x1B[38;5;5m\x1B[1mModular Font Editor K (MFEK)\x1B[0m\n";
#[cfg(target_family = "windows")]
pub(crate) static MFEK: &str = "Modular Font Editor K (MFEK)\n";

#[cfg(target_family = "unix")]
pub(crate) static NG: &str = "\x1B[38;5;1m\x1B[1mNG\x1B[0m";
#[cfg(target_family = "unix")]
pub(crate) static OK: &str = "\x1B[38;5;2m\x1B[1mOK\x1B[0m";
#[cfg(target_family = "windows")]
pub(crate) static NG: &str = "NG";
#[cfg(target_family = "windows")]
pub(crate) static OK: &str = "OK";

pub(crate) static INFO: &str = "(c) 2020-2022 Fredrick R. Brennan
(c) 2021–2022 Eli Heuer
(c) 2021 Matthew Blanchard
(c) 2020–2022 MFEK Authors

MFEK is modular software. For other authors, see AUTHORS / LICENSE file in each module's GitHub repository.
Note: Your MFEK distribution may contain non-official modules not listed below.

Modules found in your $PATH:";
