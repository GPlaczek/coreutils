// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

// spell-checker:ignore (ToDO) getusername

#[cfg(unix)]
pub use self::unix::get_username;

#[cfg(windows)]
pub use self::windows::get_username;

#[cfg(not(any(unix, windows)))]
mod imp {
    pub unsafe fn getusername() -> std::io::Result<String> {
        Ok("unknown".into())
    }
}

pub use self::imp::getusername;
