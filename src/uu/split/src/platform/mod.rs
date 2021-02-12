// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

#[cfg_attr(unix, path = "unix.rs")]
#[cfg_attr(not(unix), path = "generic.rs")]
mod imp;

pub use imp::{instantiate_current_writer, paths_refer_to_same_file};
