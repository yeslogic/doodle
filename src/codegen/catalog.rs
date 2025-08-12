use super::{DecoderFn, GTExpr};
use intmap::IntMap;
use vec_collections::VecSet2;

/// Map from adhoc type declarations (keys, represented by their uniquely
/// identifying index) to sets of decoders which produce output of that type
/// (values, represented by the index they are found at in the sourcemap).
pub(crate) type CrossIndex = IntMap<usize, VecSet2<usize>>;

/// Constructs a `CrossIndex` from the given `type_decls` and `decoder_skels`.
pub(crate) fn make_index<A, B>(
    type_decls: &[(&A, &(usize, B))],
    decoder_skels: &[DecoderFn<GTExpr>],
) -> CrossIndex {
    let mut index = IntMap::new();
    for (_, (ix, _)) in type_decls {
        index.insert(*ix, VecSet2::empty());
    }
    for (ix_value, skel) in decoder_skels.iter().enumerate() {
        let Some(ix_key) = skel.ret_type.as_decl_index() else {
            continue;
        };
        index.entry(ix_key).or_default().insert(ix_value);
    }
    index
}
