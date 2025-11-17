use std::any::type_name;

use doodle_gencode::{
    opentype_base_table, opentype_cmap_table, opentype_fvar_table, opentype_gasp_table,
    opentype_gdef_table, opentype_glyf_table, opentype_gpos_table, opentype_gsub_table,
    opentype_gvar_table, opentype_head_table, opentype_hhea_table, opentype_hmtx_table,
    opentype_kern_table, opentype_loca_table, opentype_maxp_table, opentype_name_table,
    opentype_os2_table, opentype_post_table, opentype_stat_table,
};

fn szck<T>() {
    println!("{}\t{}", size_of::<T>(), type_name::<T>())
}

pub fn main() {
    szck::<opentype_cmap_table>();
    szck::<opentype_head_table>();
    szck::<opentype_hhea_table>();
    szck::<opentype_maxp_table>();
    szck::<opentype_hmtx_table>();
    szck::<opentype_name_table>();
    szck::<opentype_os2_table>();
    szck::<opentype_post_table>();
    szck::<opentype_loca_table>();
    szck::<opentype_glyf_table>();
    szck::<opentype_gasp_table>();
    szck::<opentype_base_table>();
    szck::<opentype_gdef_table>();
    szck::<opentype_gpos_table>();
    szck::<opentype_gsub_table>();
    szck::<opentype_fvar_table>();
    szck::<opentype_gvar_table>();
    szck::<opentype_kern_table>();
    szck::<opentype_stat_table>();
}
