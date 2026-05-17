# Derivation — `export/007-signature-admission-failed-inline`

This fixture realizes the rejected-signature branch of the WOS/Formspec signing
projection contract.

It packages a single readable WOS `SignatureAdmissionFailed` payload in the
Trellis export. No `062-signature-affirmations.cbor` catalog is present because
there is no successful `SignatureAffirmation` source record. The export still
derives `066-signed-acts.cbor`, and that catalog contains one rejected act whose
source reference points at the signed WOS admission-failure event.

The rejected row is privacy-minimized: signer reference, signed payload digest,
signature id, signing intent, and stable reason code are present; consent,
document placement, provider ceremony, primitive verification, and raw failed
content are null.
