# Derivation — `<vector id>`

## Header

**What this vector exercises:** `<one paragraph describing the constructions under test>`

**Core § roadmap (in traversal order):**

1. `<§N, heading>` — `<construction>`
2. `<§N, heading>` — `<construction>`
3. `<§N, heading>` — `<construction>`

## Body

For each construction, one subsection. Template:

### Step N: `<construction name>`

**Core § citation:** `<§N, heading>`.

**Load-bearing sentence:**

> `<verbatim quote of the normative sentence from Core>`

**Input bytes:**

```
<hex dump of the input to this step>
```

**Operation:** `<hash | canonical encode | sign | concat | domain-separate | …>`

**Result:**

```
<hex dump of the output>
```

**Committed as:** `<sibling-filename.ext>`

---

Repeat for each step. Reference inputs from prior steps by filename; do not paraphrase. The goal is that a reader who has only Core + this derivation can reproduce every byte by hand.

## Footer

Full hex dumps of every intermediate and final artifact, one subsection per sibling file, in the order they are produced.

### `<sibling-filename-1.ext>`

```
<full hex dump>
```

### `<sibling-filename-2.ext>`

```
<full hex dump>
```

<!-- Repeat for each committed sibling file. -->

## Authoring notes (delete from copies)

- `derivation.md` MUST cite Core prose only. Do not cite the generator script, the Rust reference impl, or any other implementation. The generator is an authoring aid, not an oracle.
- Every step that produces committed bytes must name the sibling file that holds those bytes. A reader should be able to `xxd <sibling>` and see exactly the hex shown in the step.
- If the operation is a hash, cite both the domain-separation tag (if any) and the preimage construction from Core.
- If the operation is a signature, cite RFC 9052 §4.4 for the `Sig_structure` layout and the Core § that invokes it.
