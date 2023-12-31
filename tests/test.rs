use std::fs::{self};

#[test]
fn map_roundtrip() {
    let original = fs::read("tests/Mindor.Map.Gbx").unwrap();

    let mut compacted = vec![];

    compact_gamebox::compact(original.as_slice(), &mut compacted).unwrap();

    assert!(compacted.len() <= original.len());

    let mut decompacted = Vec::with_capacity(original.len());

    compact_gamebox::decompact(compacted.as_slice(), &mut decompacted).unwrap();

    assert!(decompacted == original);
}
