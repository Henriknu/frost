use curve25519_dalek::constants;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::Identity;
use rand::rngs::ThreadRng;

pub struct BindingValue {
    pub index: u32,
    pub rho_i: Scalar,
}

pub struct KeyGenCommitment {
    pub index: u32,
    pub commitment: Vec<RistrettoPoint>,
}

#[derive(Copy, Clone)]
pub struct SigningResponse {
    pub response: Scalar,
    pub index: u32,
}

#[derive(Copy, Clone)]
pub struct SigningCommitment {
    pub index: u32,
    pub signer_pubkey: RistrettoPoint,
    pub d: RistrettoPoint,
    pub e: RistrettoPoint,
}

impl SigningCommitment {
    pub fn new(
        d: RistrettoPoint,
        e: RistrettoPoint,
        index: u32,
        signer_pubkey: RistrettoPoint,
    ) -> Result<SigningCommitment, &'static str> {
        if d == RistrettoPoint::identity() || e == RistrettoPoint::identity() {
            return Err("Invalid signing commitment");
        }

        Ok(SigningCommitment {
            d,
            e,
            index,
            signer_pubkey,
        })
    }
}

#[derive(Copy, Clone)]
pub struct NoncePair {
    pub d: Nonce,
    pub e: Nonce,
    pub dirty: bool,
}

impl NoncePair {
    pub fn new(rng: &mut ThreadRng) -> Result<NoncePair, &'static str> {
        let d = Scalar::random(rng);
        let e = Scalar::random(rng);
        let d_pub = &constants::RISTRETTO_BASEPOINT_TABLE * &d;
        let e_pub = &constants::RISTRETTO_BASEPOINT_TABLE * &e;

        if d_pub == RistrettoPoint::identity() || e_pub == RistrettoPoint::identity() {
            return Err("Invalid nonce commitment");
        }

        Ok(NoncePair {
            d: Nonce {
                secret: d,
                public: d_pub,
            },
            e: Nonce {
                secret: e,
                public: e_pub,
            },
            dirty: false,
        })
    }

    pub fn mark_as_used(&mut self) {
        self.dirty = true;
    }
}

#[derive(Copy, Clone)]
pub struct Nonce {
    pub secret: Scalar,
    pub public: RistrettoPoint,
}

#[derive(Copy, Clone)]
pub struct Share {
    pub generator_index: u32,
    pub receiver_index: u32,
    pub value: Scalar,
}

pub struct KeyPair {
    pub index: u32,
    pub secret: Scalar,
    pub public: RistrettoPoint,
    pub group_public: RistrettoPoint,
}

pub struct Signature {
    pub r: RistrettoPoint,
    pub z: Scalar,
}
