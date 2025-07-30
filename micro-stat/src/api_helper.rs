use super::*;

pub type StatTable<'a> = opentype_stat_table<'a>;
pub type OpenType<'a> = opentype_main<'a>;
pub type OpenTypeDir<'a> = opentype_main_directory<'a>;
pub type TableDir<'a> = opentype_table_directory<'a>;
pub type TTCHeader<'a> = opentype_ttc_header_header<'a>;
pub type HeaderV1<'a> = opentype_ttc_header_header_Version1<'a>;
pub type HeaderV2<'a> = opentype_ttc_header_header_Version2<'a>;

pub fn parse_otf<'input>(buf: &'input [u8]) -> PResult<OpenType<'input>> {
    let mut input = Parser::new(buf);
    Decoder_opentype_main(&mut input)
}

pub fn dump_axis_value_offsets(filename: &str) {
    let buffer = std::fs::read(std::path::Path::new(filename)).expect("failed to read");
    let otf = parse_otf(&buffer).expect("failed to parse");
    if let Some(stat) = otf.get_stat() {
        println!("== {filename}: stat table ==");
        for (ix, offs) in stat
            .axis_value_offsets
            .axis_value_offsets
            .iter()
            .enumerate()
        {
            println!("{}: {}", ix, offs);
        }
    } else {
        println!("{filename}: no stat table");
    }
}

impl<'a> OpenType<'a> {
    pub fn get_stat(&self) -> Option<StatTable<'a>> {
        self.directory.get_stat()
    }
}

impl<'a> OpenTypeDir<'a> {
    pub fn get_stat(&self) -> Option<StatTable<'a>> {
        match self {
            OpenTypeDir::TTCHeader(header) => match &header.header {
                TTCHeader::Version1(HeaderV1 {
                    table_directories, ..
                })
                | TTCHeader::Version2(HeaderV2 {
                    table_directories, ..
                }) => {
                    for ofs_dir in table_directories.iter() {
                        if let Some(dir) = ofs_dir.link.as_ref() {
                            if let Some(stat) = dir.get_stat() {
                                return Some(stat);
                            }
                        }
                    }
                    None
                }
                TTCHeader::UnknownVersion(bad_ver) => {
                    eprintln!("unknown ttc header version ({bad_ver})");
                    None
                }
            },
            OpenTypeDir::TableDirectory(dir) => dir.get_stat(),
        }
    }
}

impl<'a> TableDir<'a> {
    pub fn get_stat(&self) -> Option<StatTable<'a>> {
        self.table_links.stat
    }
}
