use x25519_dalek::PublicKey as Curve25591PublicKey;

pub(crate) struct SessionKeys {
    pub(crate) identity_key: Curve25591PublicKey,
    pub(crate) base_key: Curve25591PublicKey,
    pub(crate) one_time_key: Curve25591PublicKey,
}

impl SessionKeys {
    pub fn new(
        identity_key: Curve25591PublicKey,
        base_key: Curve25591PublicKey,
        one_time_key: Curve25591PublicKey,
    ) -> Self {
        Self { identity_key, base_key, one_time_key }
    }
}