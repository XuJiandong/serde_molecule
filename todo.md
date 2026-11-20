# Code review TODOs

Reviewed commit `ecf4799`. Each item includes a regression check.

## P1

- [x] **Decode array elements with the fixed layout.** Named and tuple structs now decode within each array element’s byte boundary. Regression tests cover top-level arrays, an array in a table field, and rejection of oversized elements. [Array reader](serde_molecule/src/de.rs#L393) · [Regression tests](tests/src/test_serde.rs#L332)
- [x] **Decode optional fixed structs at the table field boundary.** The `struct_serde` adapter decodes an empty field as `None` and a present payload with the fixed layout. `test_table1` checks both cases against bytes from the generated Molecule builder. [Adapter](serde_molecule/src/struct_serde.rs#L27) · [Regression test](tests/src/simple.rs#L35)
- [ ] **Require fixed-layout values to consume their entire slice.** A fixed struct can decode successfully with trailing bytes that disappear on reserialization. Check cursor exhaustion at each complete value boundary, including annotated table fields and fixvec elements. Nested fields that share a cursor should leave it positioned for the next field. Verify that an oversized value is rejected at each boundary. [Top-level reader](serde_molecule/src/de.rs#L25) · [Generated size check](tests/src/old/test1.rs#L2609)
- [ ] **Reject payloads on empty fixvecs.** The zero-count branch returns before checking the remaining length. Require exactly four bytes for a zero count. Cover both a valid empty vector and a zero count followed by an extra byte. [Fixvec decoder](serde_molecule/src/molecule.rs#L92)

## P2

- [ ] **Pass fixed-layout characters to Serde’s character visitor.** The reader calls `visit_u32`, which rejects valid `char` values. Use `char::from_u32` and `visit_char`, returning `InvalidChar` for an invalid scalar. Cover a character in a fixed struct and in `Vec<char>`. [Character decoder](serde_molecule/src/struct_serde.rs#L219)
- [ ] **Validate UTF-8 and support borrowed strings.** Lossy conversion changes invalid input, and the owned-string visit prevents `from_slice::<&str>` from borrowing valid input. Validate with `core::str::from_utf8` and call `visit_borrowed_str`. Cover invalid UTF-8 and valid borrowed and owned strings. [String decoder](serde_molecule/src/de.rs#L199)
- [ ] **Return `None` at the end of a tuple struct.** The sequence reader always asks for another field, so a visitor that reads until `None` can reach an assertion. Stop at the declared field count. Cover a custom visitor that reads a valid tuple struct to exhaustion. [Tuple struct sequence reader](serde_molecule/src/de.rs#L487)

## P3

- [ ] **Register the saved fuzz regression with the test harness.** Add `#[test]` to `test_oom` so CI exercises the checked-in `oom-dump` input during `cargo test`. [Regression test](tests/src/test_fuzzing.rs#L28)
