mod support;

use support::TestCluster;
use trellis_store_postgres_async::{
    BundlePublicationError, BundlePublicationIdentity, BundlePublicationRecord,
    get_bundle_publication_by_digest, publish_bundle_publication, reserve_bundle_publication,
    run_migrations,
};

#[tokio::test]
async fn reserve_bundle_publication_is_idempotent_and_rejects_identity_conflicts() {
    let cluster = TestCluster::start_without_migrations();
    let pool = cluster.tls_pool(4).await;
    run_migrations(&pool).await.unwrap();

    let identity = bundle_identity("scope-a", 1, "sha256:checkpoint-a", "sha256:attempt-a");
    reserve_bundle_publication(&pool, &identity)
        .await
        .expect("reserve first identity");
    reserve_bundle_publication(&pool, &identity)
        .await
        .expect("reserve same identity again");

    let seal_conflict = reserve_bundle_publication(
        &pool,
        &bundle_identity("scope-a", 1, "sha256:checkpoint-b", "sha256:attempt-b"),
    )
    .await
    .expect_err("seal version should reject different identity");
    assert!(matches!(seal_conflict, BundlePublicationError::Conflict(_)));

    let checkpoint_conflict = reserve_bundle_publication(
        &pool,
        &bundle_identity("scope-a", 2, "sha256:checkpoint-a", "sha256:attempt-c"),
    )
    .await
    .expect_err("checkpoint digest should reject different identity");
    assert!(matches!(
        checkpoint_conflict,
        BundlePublicationError::Conflict(_)
    ));
}

#[tokio::test]
async fn publish_bundle_publication_records_artifact_ref_once() {
    let cluster = TestCluster::start_without_migrations();
    let pool = cluster.tls_pool(4).await;
    run_migrations(&pool).await.unwrap();

    let identity = bundle_identity("scope-b", 1, "sha256:checkpoint-d", "sha256:attempt-d");
    assert!(
        get_bundle_publication_by_digest(&pool, b"scope-b", "sha256:checkpoint-d")
            .await
            .expect("read before publish")
            .is_none()
    );

    let record = BundlePublicationRecord {
        identity: identity.clone(),
        artifact_ref: "s3://bucket/scope-b/bundles/attempt-d.zip".to_string(),
    };
    publish_bundle_publication(&pool, &record)
        .await
        .expect("publish bundle");
    publish_bundle_publication(&pool, &record)
        .await
        .expect("repeat same bundle publication");

    let loaded = get_bundle_publication_by_digest(&pool, b"scope-b", "sha256:checkpoint-d")
        .await
        .expect("read after publish");
    assert_eq!(loaded, Some(record.clone()));

    let artifact_conflict = publish_bundle_publication(
        &pool,
        &BundlePublicationRecord {
            identity,
            artifact_ref: "s3://bucket/scope-b/bundles/other.zip".to_string(),
        },
    )
    .await
    .expect_err("same identity should reject different artifact ref");
    assert!(matches!(
        artifact_conflict,
        BundlePublicationError::Conflict(_)
    ));
}

fn bundle_identity(
    scope: &str,
    seal_version: u64,
    checkpoint_digest: &str,
    export_attempt_id: &str,
) -> BundlePublicationIdentity {
    BundlePublicationIdentity {
        scope: scope.as_bytes().to_vec(),
        checkpoint_digest: checkpoint_digest.to_string(),
        seal_version,
        export_attempt_id: export_attempt_id.to_string(),
    }
}
