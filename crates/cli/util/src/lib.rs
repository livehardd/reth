//! This crate defines a set of commonly used cli utils.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/// Helper function to load a secret key from a file.
pub mod load_secret_key;
pub use load_secret_key::get_secret_key;

/// Cli parsers functions.
pub mod parsers;
pub use parsers::{hash_or_num_value_parser, parse_duration_from_secs, parse_socket_address};
