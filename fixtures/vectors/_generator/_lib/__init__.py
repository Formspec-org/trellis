"""Narrow byte-level helpers shared across vector generators.

This package is deliberately tiny and non-interpretive: it hosts the
pinned numeric constants from RFC 9052 / Core §7.4, the canonical-CBOR
encoder wrapper, the §9.1 domain-separation hash, and the deterministic
ZIP-entry builder from Core §18.1 — all code that every generator
copied verbatim before this extraction landed.

It deliberately does NOT host `cose_sign1`, `load_issuer_key`, or any
other function whose signature or semantics differs between generators;
those stay local so each generator keeps reading like a self-contained
derivation of Core. The G-5 stranger test cares that a new implementor
can read Core and match bytes; dcbor wrapping `cbor2.dumps(x, canonical=
True)` is not spec interpretation, so centralizing it does not hurt the
stranger test.

See `fixtures/vectors/_generator/gen_append_005.py` header comment for
the 3rd-site extraction threshold that this package satisfies.
"""
