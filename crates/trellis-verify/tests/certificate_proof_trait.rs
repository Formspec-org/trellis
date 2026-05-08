use trellis_verify::certificate_proof::{
    CertificateResponseProof, ResolverError, ResponseProofResolver,
};

struct StubResolver(Result<Option<CertificateResponseProof>, ResolverError>);

impl ResponseProofResolver for StubResolver {
    fn resolve(
        &self,
        _payload_bytes: &[u8],
    ) -> Result<Option<CertificateResponseProof>, ResolverError> {
        self.0.clone()
    }
}

#[test]
fn core_accepts_neutral_resolver_outcome_some() {
    let resolver = StubResolver(Ok(Some(CertificateResponseProof {
        response_hash: [0xAB; 32],
    })));
    assert_eq!(
        resolver.resolve(b"opaque").unwrap().unwrap().response_hash,
        [0xAB; 32]
    );
}

#[test]
fn core_accepts_neutral_resolver_outcome_none() {
    let resolver = StubResolver(Ok(None));
    assert!(resolver.resolve(b"opaque").unwrap().is_none());
}

#[test]
fn core_propagates_resolver_error() {
    let resolver = StubResolver(Err(ResolverError::MalformedResponseDigest(
        "bad-hex".into(),
    )));
    assert!(matches!(
        resolver.resolve(b"opaque"),
        Err(ResolverError::MalformedResponseDigest(_))
    ));
}
