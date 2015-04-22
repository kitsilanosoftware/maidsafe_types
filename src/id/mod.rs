// Copyright 2015 MaidSafe.net limited
//
// This MaidSafe Software is licensed to you under (1) the MaidSafe.net Commercial License, version
// 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which licence you
// accepted on initial access to the Software (the "Licences").
//
// By contributing code to the MaidSafe Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
// directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also available at
// http://maidsafe.net/licenses
//
// Unless required by applicable law or agreed to in writing, the MaidSafe Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// See the Licences for the specific language governing permissions and limitations relating to use
// of the MaidSafe Software.

/// AnMaid
pub mod an_maid;
/// AnMpid
pub mod an_mpid;
/// Maid
pub mod maid;
/// Mpid
pub mod mpid;
/// PublicMaid
pub mod public_maid;
/// PublicMpid
pub mod public_mpid;
/// PublicAnMaid
pub mod public_an_maid;

pub use self::an_maid::*;
pub use self::an_mpid::*;
pub use self::maid::*;
pub use self::mpid::*;
pub use self::public_maid::*;
pub use self::public_mpid::*;
pub use self::public_an_maid::*;
#[test]
fn dummy()  {
}
