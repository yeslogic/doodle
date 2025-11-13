use super::util::{FxHash, StableMap};
use super::{
    name::{NameCtxt, PathLabel},
    rust_ast::RustTypeDecl,
};
use crate::Label;

pub struct NameGen {
    pub(super) ctxt: NameCtxt,
    ctr: usize,
    // Reverse mapping from a RustTypeDef to its index in the ad-hoc type inventory and the PathLabel it was first assigned (which cannot be safely mutated)
    pub(super) rev_map: StableMap<RustTypeDecl, (usize, PathLabel), FxHash>,
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

            // NOTE - comment out the cfg attr to re-enable debugging
            #[cfg(false)]
            {
                eprintln!("[RENAME]: {k} -> {v:?} ~ {rename}");
            }
            ret.insert(k.clone(), rename);
        }
        ret
    }

    /// Yields a first-pass name for a [`RustTypeDecl`]
    ///
    /// If the `RustTypeDecl` has not yet been recorded, its entry is populated and it is assigned a first-pass name
    /// based on the current `self.ctxt` path.
    ///
    /// If the `RustTypeDecl` has previously been recorded, the first-pass name it was given originally is preserved, but the name-remap table is updated
    /// to reflect whichever of its currently-held path and the `self.ctxt` path would yield a better second-pass name.
    ///
    /// Returns a nested tuple `(name, (ix, is_new))` where `name` is the first-pass name, `ix` is the index of the type in the adhoc type inventory,
    /// and `is_new` is true iff the `RustTypeDecl` was not previously recorded.
    pub fn get_name(&mut self, decl: &RustTypeDecl) -> (Label, (usize, bool)) {
        match self.rev_map.get(decl) {
            Some((ix, orig_path)) => match self.ctxt.find_name_for(orig_path).ok() {
                Some(name) => {
                    /*
                     * `orig_path`: the PathLabel that `decl` was first assigned in `self.rev_map`
                     * `name`: the first-pass name we are committed to using
                     * `path_here`: the current stack-path stored in `self.ctxt`
                     */
                    let path_here = self.ctxt.register_path();

                    self.name_remap          // with our renaming table (early commit -> final name),
                        .entry(name.clone()) // get the Entry for the `name`, the first-pass name that decl is getting
                        .and_modify(|prior: &mut PathLabel| {
                            let _tmp = prior.clone();
                            // if it is currently associated with a different PathLabel `prior`,
                            let _changed = self.ctxt.refine_path(prior, path_here.clone()); // rebind to whichever of `path_here` or `prior` is better

                            // NOTE - comment out the cfg attr to re-enable debugging
                            #[cfg(false)]
                            {
                                if _changed {
                                    eprintln!(
                                        "[RENAME][OLD] {name} -> {_tmp:?}\n[RENAME][NEW] {name} -> {path_here:?}"
                                    );
                                }
                            }
                        })
                        .or_insert(path_here); // or otherwise insert path_here
                    (name, (*ix, false))
                }
                None => unreachable!("no identifier associated with path, but path is in use"),
            },
            None => {
                let ix = self.ctr;
                self.ctr += 1;
                let (path, ret) = {
                    let loc = self.ctxt.register_path();
                    let name = self.ctxt.find_name_for(&loc).unwrap();
                    (loc, name)
                };

                // NOTE - comment out the cfg attr to re-enable debugging
                #[cfg(false)]
                {
                    eprintln!("[RENAME][INIT] {ret} -> {path:?}");
                }

                self.rev_map.insert(decl.clone(), (ix, path.clone()));
                // ensure deduplication happens by forcing a no-op rename by default
                self.name_remap.insert(ret.clone(), path);

                (ret, (ix, true))
            }
        }
    }
}
