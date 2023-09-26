// This file is Copyright its original authors, visible in version control
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT> or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

/// A privacy-preserving authenticator that is used for authorization.

use bitcoin::Txid;

pub struct Credentials(pub [u8; 32]);

#[derive(Debug)]
pub enum Proof {
	Txid(Txid),
}
