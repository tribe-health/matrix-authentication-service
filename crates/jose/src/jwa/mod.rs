// Copyright 2022 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use mas_iana::jose::JsonWebSignatureAlg;
use sha2::{Sha256, Sha384, Sha512};

mod asymmetric;
pub(crate) mod hmac;
pub(self) mod signature;
mod symmetric;

pub use self::{
    asymmetric::{AsymmetricKeyFromJwkError, AsymmetricSigningKey, AsymmetricVerifyingKey},
    symmetric::{InvalidAlgorithm, SymmetricKey},
};

pub type Hs256Key = self::hmac::Hmac<Sha256>;
pub type Hs384Key = self::hmac::Hmac<Sha384>;
pub type Hs512Key = self::hmac::Hmac<Sha512>;

pub type Rs256SigningKey = rsa::pkcs1v15::SigningKey<Sha256>;
pub type Rs256VerifyingKey = rsa::pkcs1v15::VerifyingKey<Sha256>;
pub type Rs384SigningKey = rsa::pkcs1v15::SigningKey<Sha384>;
pub type Rs384VerifyingKey = rsa::pkcs1v15::VerifyingKey<Sha384>;
pub type Rs512SigningKey = rsa::pkcs1v15::SigningKey<Sha512>;
pub type Rs512VerifyingKey = rsa::pkcs1v15::VerifyingKey<Sha512>;

pub type Ps256SigningKey = rsa::pss::SigningKey<Sha256>;
pub type Ps256VerifyingKey = rsa::pss::VerifyingKey<Sha256>;
pub type Ps384SigningKey = rsa::pss::SigningKey<Sha384>;
pub type Ps384VerifyingKey = rsa::pss::VerifyingKey<Sha384>;
pub type Ps512SigningKey = rsa::pss::SigningKey<Sha512>;
pub type Ps512VerifyingKey = rsa::pss::VerifyingKey<Sha512>;

pub type Es256SigningKey = ecdsa::SigningKey<p256::NistP256>;
pub type Es256VerifyingKey = ecdsa::VerifyingKey<p256::NistP256>;
pub type Es384SigningKey = ecdsa::SigningKey<p384::NistP384>;
pub type Es384VerifyingKey = ecdsa::VerifyingKey<p384::NistP384>;
pub type Es256KSigningKey = ecdsa::SigningKey<k256::Secp256k1>;
pub type Es256KVerifyingKey = ecdsa::VerifyingKey<k256::Secp256k1>;

/// All the signing algorithms supported by this crate.
pub const SUPPORTED_SIGNING_ALGORITHMS: [JsonWebSignatureAlg; 12] = [
    JsonWebSignatureAlg::Hs256,
    JsonWebSignatureAlg::Hs384,
    JsonWebSignatureAlg::Hs512,
    JsonWebSignatureAlg::Rs256,
    JsonWebSignatureAlg::Rs384,
    JsonWebSignatureAlg::Rs512,
    JsonWebSignatureAlg::Ps256,
    JsonWebSignatureAlg::Ps384,
    JsonWebSignatureAlg::Ps512,
    JsonWebSignatureAlg::Es256,
    JsonWebSignatureAlg::Es384,
    JsonWebSignatureAlg::Es256K,
];
