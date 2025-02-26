use super::util::{FxHash, StableMap};
use super::{
    name::{pick_best_path, NameCtxt, PathLabel},
    rust_ast::RustTypeDef,
};
use crate::Label;

pub struct NameGen {
    pub(super) ctxt: NameCtxt,
    ctr: usize,
    // Reverse mapping from a RustTypeDef to its index in the ad-hoc type inventory and the PathLabel it is to be assigned
    pub(super) rev_map: StableMap<RustTypeDef, (usize, PathLabel), FxHash>,
    // Reassociation table for converting first-pass name selections into their ideal seed-PathLabel
    pub(super) name_remap: StableMap<Label, PathLabel, FxHash>,
}

impl NameGen {
    pub fn new() -> Self {
        Self {
            ctxt: NameCtxt::new(),
            ctr: 0,
            rev_map: Default::default(),
            name_remap: Default::default(),
        }
    }

    pub fn manifest_renaming_table(&mut self) -> StableMap<Label, Label, FxHash> {
        let mut ret = StableMap::<Label, Label, FxHash>::default();
        for (k, v) in self.name_remap.iter() {
            let rename = self.ctxt.find_name_for(v).unwrap();
            ret.insert(k.clone(), rename);
        }
        ret
    }

    /// Finds an existing name, or generates a new name, for a [`RustTypeDef`]
    ///
    /// Returns `(old, (ix, false))` if the RustTypeDef was already given a name `old`, where `ix` is the index of the definition in the overall order of ad-hoc types that were defined thus-far.
    ///
    /// Returns `(new, (ix, true))` otherwise, where `ix` is the uniquely-identifying index of the newly defined type at time-of-invocation, and `new` is a fresh path-based name for the type.
    pub fn get_name(&mut self, def: &RustTypeDef) -> (Label, (usize, bool)) {
        match self.rev_map.get(def) {
            Some((ix, path)) => match self.ctxt.find_name_for(path).ok() {
                Some(name) => {
                    let path = self.ctxt.produce_name();
                    self.name_remap
                        .entry(name.clone())
                        .and_modify(|prior| pick_best_path(prior, path.clone()))
                        .or_insert(path);
                    (name, (*ix, false))
                }
                None => unreachable!("no identifier associated with path, but path is in use"),
            },
            None => {
                let ix = self.ctr;
                self.ctr += 1;
                let (path, ret) = {
                    let loc = self.ctxt.produce_name();
                    let name = self.ctxt.find_name_for(&loc).unwrap();
                    (loc, name)
                };
                self.rev_map.insert(def.clone(), (ix, path.clone()));
                // ensure deduplication happens by forcing a no-op rename by default
                self.name_remap.insert(ret.clone(), path);
                (ret, (ix, true))
            }
        }
    }
}
