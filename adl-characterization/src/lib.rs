pub mod compare;
pub mod manifest;
pub mod model;
pub mod normalize;
pub mod runner;
pub mod shadow;

pub use compare::{capture_corpus, verify_corpus, VerificationReport};
pub use manifest::{corpus_bundle_sha256, load_corpus};
pub use model::{Corpus, RawObservation};
pub use shadow::{load_shadow_manifest, run_shadow, ShadowInputs, ShadowReport};
