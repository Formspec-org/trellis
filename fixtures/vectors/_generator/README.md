# Generator Authoring Discipline

## Authoring discipline

A Python generator lives at `fixtures/vectors/_generator/` as an authoring aid. Hand-typing ~50 multi-kilobyte CBOR / COSE structures does not scale, and forbidding tooling would just push authors to ad-hoc scripts outside the repo. The generator is permitted under hard constraints:

- **Allowed imports:** `hashlib`, `cryptography.hazmat.*`, `cbor2`, `pathlib`, `tomllib`, `json`, stdlib only; plus the sibling `_lib` package (see below).
- **Forbidden imports:** any `trellis-*` crate or package, any Trellis-derived abstraction, any high-level spec-interpretive library.
- **Spec-interpretive code** — preimage construction, domain-separation tags, canonical encoding rules, `Sig_structure` assembly, `author_event_hash` / `canonical_event_hash` / `tree_head_hash` preimages — is hand-written in the generator with inline Core § citations.
- **`_lib` package** at `fixtures/vectors/_generator/_lib/` hosts byte-level helpers that were duplicated verbatim across three or more generators: dCBOR wrapper, §9.1 domain-separated SHA-256, §18.1 deterministic `ZipInfo`, and RFC 9052 / Core §7.4 numeric label constants. These are registry-fixed values and stdlib-sugar, not spec interpretation. Generators that need them do `sys.path.insert(0, str(Path(__file__).resolve().parent))` + `from _lib.byte_utils import …`. Anything with a Core § citation or conditional logic belongs in the generator, not in `_lib`.
- **Derivation authority**: `derivation.md` cites Core prose, not generator source. The generator is an authoring aid; it is not normative, not an oracle.
- **G-5 isolation**: the stranger never sees the generator. `_generator/` is excluded from the set of documents read for the stranger test.
- `scripts/check-specs.py` enforces the allowed-import list via AST scan of `fixtures/vectors/_generator/**/*.py`.

The generator doubles as a second hand-written reading of Core (Python), parallel to the Rust reference impl. Two independent hand-written readings of Core are a stronger evidentiary base than one — disagreements between generator output and Rust output during G-4 land as ratification signal, not as bugs in one impl.

See the top-level design spec at `../../../thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md` for rationale; this file is the operative reference authors consult when adding a new generator script.
