use criterion::{criterion_group, criterion_main, Benchmark, BenchmarkId, Criterion};
use itertools::iproduct;
use rand::Rng;

use concrete_core::crypto::bootstrap::BootstrapKey;
use concrete_core::crypto::cross::{bootstrap, cmux, constant_sample_extract, external_product};
use concrete_core::crypto::encoding::{Plaintext, PlaintextList};
use concrete_core::crypto::glwe::{GlweCiphertext, GlweList};
use concrete_core::crypto::lwe::{LweCiphertext, LweKeyswitchKey};
use concrete_core::crypto::secret::{GlweSecretKey, LweSecretKey};
use concrete_core::crypto::{
    CiphertextCount, GlweDimension, LweDimension, LweSize, PlaintextCount,
};
use concrete_core::math::dispersion::{DispersionParameter, LogStandardDev, Variance};
use concrete_core::math::fft::{Complex64, Fft, FourierPolynomial};
use concrete_core::math::polynomial::PolynomialSize;
use concrete_core::math::random::{
    fill_with_random_uniform, fill_with_random_uniform_boolean, random_uniform_n_msb,
    RandomGenerable, UniformMsb,
};
use concrete_core::math::tensor::{
    AsMutSlice, AsMutTensor, AsRefSlice, AsRefTensor, IntoTensor, Tensor,
};
use concrete_core::numeric::{CastFrom, CastInto, Numeric};

mod bootstrap;
mod bsk_generation;
mod keyswitch;
mod lwe_encrypt;
mod mac;
mod multisum;
mod random;
mod rlwe_encrypt;

criterion_group!(bootstrap_b, bootstrap::bench_32, bootstrap::bench_64);
criterion_group!(
    bsk_generation_b,
    bsk_generation::bench_32,
    bsk_generation::bench_64
);
criterion_group!(keyswitch_b, keyswitch::bench_32, keyswitch::bench_64);
criterion_group!(
    random_b,
    random::bench_8,
    random::bench_16,
    random::bench_32,
    random::bench_64,
    random::bench_128
);
criterion_group!(multisum_b, multisum::bench_32, multisum::bench_64);
criterion_group!(mac_b, mac::bench);
criterion_group!(
    rlwe_encrypt_b,
    rlwe_encrypt::bench_32,
    rlwe_encrypt::bench_64
);
criterion_group!(lwe_encrypt_b, lwe_encrypt::bench_32, lwe_encrypt::bench_64);

criterion_main!(
    bootstrap_b,
    bsk_generation_b,
    keyswitch_b,
    random_b,
    multisum_b,
    mac_b,
    rlwe_encrypt_b,
    lwe_encrypt_b,
);
