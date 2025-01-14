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

//! Enums from the "JSON Object Signing and Encryption" IANA registry
//! See <https://www.iana.org/assignments/oauth-parameters/oauth-parameters.xhtml>

// Do not edit this file manually

use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde_with::{DeserializeFromStr, SerializeDisplay};

/// JSON Web Signature "alg" parameter
///
/// Source: <https://www.iana.org/assignments/jose/web-signature-encryption-algorithms.csv>
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    FromStr,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
)]
#[non_exhaustive]
pub enum JsonWebSignatureAlg {
    /// HMAC using SHA-256
    #[schemars(rename = "HS256")]
    #[display("HS256")]
    Hs256,

    /// HMAC using SHA-384
    #[schemars(rename = "HS384")]
    #[display("HS384")]
    Hs384,

    /// HMAC using SHA-512
    #[schemars(rename = "HS512")]
    #[display("HS512")]
    Hs512,

    /// RSASSA-PKCS1-v1_5 using SHA-256
    #[schemars(rename = "RS256")]
    #[display("RS256")]
    Rs256,

    /// RSASSA-PKCS1-v1_5 using SHA-384
    #[schemars(rename = "RS384")]
    #[display("RS384")]
    Rs384,

    /// RSASSA-PKCS1-v1_5 using SHA-512
    #[schemars(rename = "RS512")]
    #[display("RS512")]
    Rs512,

    /// ECDSA using P-256 and SHA-256
    #[schemars(rename = "ES256")]
    #[display("ES256")]
    Es256,

    /// ECDSA using P-384 and SHA-384
    #[schemars(rename = "ES384")]
    #[display("ES384")]
    Es384,

    /// ECDSA using P-521 and SHA-512
    #[schemars(rename = "ES512")]
    #[display("ES512")]
    Es512,

    /// RSASSA-PSS using SHA-256 and MGF1 with SHA-256
    #[schemars(rename = "PS256")]
    #[display("PS256")]
    Ps256,

    /// RSASSA-PSS using SHA-384 and MGF1 with SHA-384
    #[schemars(rename = "PS384")]
    #[display("PS384")]
    Ps384,

    /// RSASSA-PSS using SHA-512 and MGF1 with SHA-512
    #[schemars(rename = "PS512")]
    #[display("PS512")]
    Ps512,

    /// No digital signature or MAC performed
    #[schemars(rename = "none")]
    #[display("none")]
    None,

    /// EdDSA signature algorithms
    #[schemars(rename = "EdDSA")]
    #[display("EdDSA")]
    EdDsa,

    /// ECDSA using secp256k1 curve and SHA-256
    #[schemars(rename = "ES256K")]
    #[display("ES256K")]
    Es256K,

    /// An unknown value.
    #[display("{0}")]
    #[schemars(skip)]
    Unknown(String),
}

/// JSON Web Encryption "alg" parameter
///
/// Source: <https://www.iana.org/assignments/jose/web-signature-encryption-algorithms.csv>
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    FromStr,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
)]
#[non_exhaustive]
pub enum JsonWebEncryptionAlg {
    /// RSAES-PKCS1-v1_5
    #[schemars(rename = "RSA1_5")]
    #[display("RSA1_5")]
    Rsa15,

    /// RSAES OAEP using default parameters
    #[schemars(rename = "RSA-OAEP")]
    #[display("RSA-OAEP")]
    RsaOaep,

    /// RSAES OAEP using SHA-256 and MGF1 with SHA-256
    #[schemars(rename = "RSA-OAEP-256")]
    #[display("RSA-OAEP-256")]
    RsaOaep256,

    /// AES Key Wrap using 128-bit key
    #[schemars(rename = "A128KW")]
    #[display("A128KW")]
    A128Kw,

    /// AES Key Wrap using 192-bit key
    #[schemars(rename = "A192KW")]
    #[display("A192KW")]
    A192Kw,

    /// AES Key Wrap using 256-bit key
    #[schemars(rename = "A256KW")]
    #[display("A256KW")]
    A256Kw,

    /// Direct use of a shared symmetric key
    #[schemars(rename = "dir")]
    #[display("dir")]
    Dir,

    /// ECDH-ES using Concat KDF
    #[schemars(rename = "ECDH-ES")]
    #[display("ECDH-ES")]
    EcdhEs,

    /// ECDH-ES using Concat KDF and "A128KW" wrapping
    #[schemars(rename = "ECDH-ES+A128KW")]
    #[display("ECDH-ES+A128KW")]
    EcdhEsA128Kw,

    /// ECDH-ES using Concat KDF and "A192KW" wrapping
    #[schemars(rename = "ECDH-ES+A192KW")]
    #[display("ECDH-ES+A192KW")]
    EcdhEsA192Kw,

    /// ECDH-ES using Concat KDF and "A256KW" wrapping
    #[schemars(rename = "ECDH-ES+A256KW")]
    #[display("ECDH-ES+A256KW")]
    EcdhEsA256Kw,

    /// Key wrapping with AES GCM using 128-bit key
    #[schemars(rename = "A128GCMKW")]
    #[display("A128GCMKW")]
    A128Gcmkw,

    /// Key wrapping with AES GCM using 192-bit key
    #[schemars(rename = "A192GCMKW")]
    #[display("A192GCMKW")]
    A192Gcmkw,

    /// Key wrapping with AES GCM using 256-bit key
    #[schemars(rename = "A256GCMKW")]
    #[display("A256GCMKW")]
    A256Gcmkw,

    /// PBES2 with HMAC SHA-256 and "A128KW" wrapping
    #[schemars(rename = "PBES2-HS256+A128KW")]
    #[display("PBES2-HS256+A128KW")]
    Pbes2Hs256A128Kw,

    /// PBES2 with HMAC SHA-384 and "A192KW" wrapping
    #[schemars(rename = "PBES2-HS384+A192KW")]
    #[display("PBES2-HS384+A192KW")]
    Pbes2Hs384A192Kw,

    /// PBES2 with HMAC SHA-512 and "A256KW" wrapping
    #[schemars(rename = "PBES2-HS512+A256KW")]
    #[display("PBES2-HS512+A256KW")]
    Pbes2Hs512A256Kw,

    /// RSA-OAEP using SHA-384 and MGF1 with SHA-384
    #[schemars(rename = "RSA-OAEP-384")]
    #[display("RSA-OAEP-384")]
    RsaOaep384,

    /// RSA-OAEP using SHA-512 and MGF1 with SHA-512
    #[schemars(rename = "RSA-OAEP-512")]
    #[display("RSA-OAEP-512")]
    RsaOaep512,

    /// An unknown value.
    #[display("{0}")]
    #[schemars(skip)]
    Unknown(String),
}

/// JSON Web Encryption "enc" parameter
///
/// Source: <https://www.iana.org/assignments/jose/web-signature-encryption-algorithms.csv>
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    FromStr,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
)]
#[non_exhaustive]
pub enum JsonWebEncryptionEnc {
    /// AES_128_CBC_HMAC_SHA_256 authenticated encryption algorithm
    #[schemars(rename = "A128CBC-HS256")]
    #[display("A128CBC-HS256")]
    A128CbcHs256,

    /// AES_192_CBC_HMAC_SHA_384 authenticated encryption algorithm
    #[schemars(rename = "A192CBC-HS384")]
    #[display("A192CBC-HS384")]
    A192CbcHs384,

    /// AES_256_CBC_HMAC_SHA_512 authenticated encryption algorithm
    #[schemars(rename = "A256CBC-HS512")]
    #[display("A256CBC-HS512")]
    A256CbcHs512,

    /// AES GCM using 128-bit key
    #[schemars(rename = "A128GCM")]
    #[display("A128GCM")]
    A128Gcm,

    /// AES GCM using 192-bit key
    #[schemars(rename = "A192GCM")]
    #[display("A192GCM")]
    A192Gcm,

    /// AES GCM using 256-bit key
    #[schemars(rename = "A256GCM")]
    #[display("A256GCM")]
    A256Gcm,

    /// An unknown value.
    #[display("{0}")]
    #[schemars(skip)]
    Unknown(String),
}

/// JSON Web Encryption Compression Algorithm
///
/// Source: <https://www.iana.org/assignments/jose/web-encryption-compression-algorithms.csv>
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    FromStr,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
)]
#[non_exhaustive]
pub enum JsonWebEncryptionCompressionAlgorithm {
    /// DEFLATE
    #[schemars(rename = "DEF")]
    #[display("DEF")]
    Def,

    /// An unknown value.
    #[display("{0}")]
    #[schemars(skip)]
    Unknown(String),
}

/// JSON Web Key Type
///
/// Source: <https://www.iana.org/assignments/jose/web-key-types.csv>
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    FromStr,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
)]
#[non_exhaustive]
pub enum JsonWebKeyType {
    /// Elliptic Curve
    #[schemars(rename = "EC")]
    #[display("EC")]
    Ec,

    /// RSA
    #[schemars(rename = "RSA")]
    #[display("RSA")]
    Rsa,

    /// Octet sequence
    #[schemars(rename = "oct")]
    #[display("oct")]
    Oct,

    /// Octet string key pairs
    #[schemars(rename = "OKP")]
    #[display("OKP")]
    Okp,

    /// An unknown value.
    #[display("{0}")]
    #[schemars(skip)]
    Unknown(String),
}

/// JSON Web Key EC Elliptic Curve
///
/// Source: <https://www.iana.org/assignments/jose/web-key-elliptic-curve.csv>
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    FromStr,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
)]
#[non_exhaustive]
pub enum JsonWebKeyEcEllipticCurve {
    /// P-256 Curve
    #[schemars(rename = "P-256")]
    #[display("P-256")]
    P256,

    /// P-384 Curve
    #[schemars(rename = "P-384")]
    #[display("P-384")]
    P384,

    /// P-521 Curve
    #[schemars(rename = "P-521")]
    #[display("P-521")]
    P521,

    /// SECG secp256k1 curve
    #[schemars(rename = "secp256k1")]
    #[display("secp256k1")]
    Secp256K1,

    /// An unknown value.
    #[display("{0}")]
    #[schemars(skip)]
    Unknown(String),
}

/// JSON Web Key OKP Elliptic Curve
///
/// Source: <https://www.iana.org/assignments/jose/web-key-elliptic-curve.csv>
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    FromStr,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
)]
#[non_exhaustive]
pub enum JsonWebKeyOkpEllipticCurve {
    /// Ed25519 signature algorithm key pairs
    #[schemars(rename = "Ed25519")]
    #[display("Ed25519")]
    Ed25519,

    /// Ed448 signature algorithm key pairs
    #[schemars(rename = "Ed448")]
    #[display("Ed448")]
    Ed448,

    /// X25519 function key pairs
    #[schemars(rename = "X25519")]
    #[display("X25519")]
    X25519,

    /// X448 function key pairs
    #[schemars(rename = "X448")]
    #[display("X448")]
    X448,

    /// An unknown value.
    #[display("{0}")]
    #[schemars(skip)]
    Unknown(String),
}

/// JSON Web Key Use
///
/// Source: <https://www.iana.org/assignments/jose/web-key-use.csv>
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    FromStr,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
)]
#[non_exhaustive]
pub enum JsonWebKeyUse {
    /// Digital Signature or MAC
    #[schemars(rename = "sig")]
    #[display("sig")]
    Sig,

    /// Encryption
    #[schemars(rename = "enc")]
    #[display("enc")]
    Enc,

    /// An unknown value.
    #[display("{0}")]
    #[schemars(skip)]
    Unknown(String),
}

/// JSON Web Key Operation
///
/// Source: <https://www.iana.org/assignments/jose/web-key-operations.csv>
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    FromStr,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
)]
#[non_exhaustive]
pub enum JsonWebKeyOperation {
    /// Compute digital signature or MAC
    #[schemars(rename = "sign")]
    #[display("sign")]
    Sign,

    /// Verify digital signature or MAC
    #[schemars(rename = "verify")]
    #[display("verify")]
    Verify,

    /// Encrypt content
    #[schemars(rename = "encrypt")]
    #[display("encrypt")]
    Encrypt,

    /// Decrypt content and validate decryption, if applicable
    #[schemars(rename = "decrypt")]
    #[display("decrypt")]
    Decrypt,

    /// Encrypt key
    #[schemars(rename = "wrapKey")]
    #[display("wrapKey")]
    WrapKey,

    /// Decrypt key and validate decryption, if applicable
    #[schemars(rename = "unwrapKey")]
    #[display("unwrapKey")]
    UnwrapKey,

    /// Derive key
    #[schemars(rename = "deriveKey")]
    #[display("deriveKey")]
    DeriveKey,

    /// Derive bits not to be used as a key
    #[schemars(rename = "deriveBits")]
    #[display("deriveBits")]
    DeriveBits,

    /// An unknown value.
    #[display("{0}")]
    #[schemars(skip)]
    Unknown(String),
}
