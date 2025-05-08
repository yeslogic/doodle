use std::mem::size_of;

use crate::format::BaseModule;
use doodle::bounds::Bounds;
use doodle::{helper::*, IntoLabel, Label};
use doodle::{BaseType, Expr, Format, FormatModule, FormatRef, Pattern, ValueType};

const ISBE_ARG: (Label, ValueType) = (Label::Borrowed("is_be"), ValueType::Base(BaseType::Bool));
const CLASS_ARG: (Label, ValueType) = (Label::Borrowed("class"), ValueType::Base(BaseType::U8));

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    // SECTION - common byte-oriented types

    fn define_format_endian_aligned<T>(
        module: &mut FormatModule,
        name: impl IntoLabel,
        be_version: Format,
        le_version: Format,
    ) -> FormatRef {
        module.define_format_args(
            name,
            vec![ISBE_ARG],
            aligned(
                if_then_else(var("is_be"), be_version, le_version),
                size_of::<T>(),
            ),
        )
    }

    // SECTION - 32-bit ELF types

    // Unsigned Program Address (32-bit)
    let elf32_addr_endian = define_format_endian_aligned::<u32>(
        module,
        "elf.types.elf32-addr",
        base.u32be(),
        base.u32le(),
    );

    // Unsigned File Offset (32-bit)
    let elf32_off_endian = define_format_endian_aligned::<u32>(
        module,
        "elf.types.elf32-off",
        base.u32be(),
        base.u32le(),
    );
    // !SECTION

    // SECTION - Common 32-bit/64-bit ELF type definitions
    // Unsigned medium integer (16 bits)
    let elf_half_endian = define_format_endian_aligned::<u16>(
        module,
        "elf.types.elf-half",
        base.u16be(),
        base.u16le(),
    );

    // Unsigned integer (32-bit)
    let elf_word_endian = define_format_endian_aligned::<u32>(
        module,
        "elf.types.elf-word",
        base.u32be(),
        base.u32le(),
    );
    // !SECTION

    // SECTION - 64-bit ELF types
    let elf64_addr_endian = define_format_endian_aligned::<u64>(
        module,
        "elf.types.elf64-addr",
        base.u64be(),
        base.u64le(),
    );

    let elf64_off_endian = define_format_endian_aligned::<u64>(
        module,
        "elf.types.elf64-off",
        base.u64be(),
        base.u64le(),
    );

    let elf64_xword_endian = define_format_endian_aligned::<u64>(
        module,
        "elf.types.elf64-Xword",
        base.u64be(),
        base.u64le(),
    );
    // !SECTION

    // !SECTION

    // SECTION - Elf Header - Identifier

    // Invalid Class
    const ELF_CLASS_NONE: u8 = 0;
    // 32-bit Objects
    const ELF_CLASS_32: u8 = 1;
    // 64-bit objects
    const ELF_CLASS_64: u8 = 2;

    // REVIEW - const-enum construction to promote magic numbers to proper variants?
    let ei_class = module.define_format(
        "elf.header.ident.class",
        where_between_u8(base.u8(), ELF_CLASS_NONE, ELF_CLASS_64),
    );

    // Invalid Data Encoding
    const ELF_DATA_NONE: u8 = 0;
    // 2's Complement, Little Endian
    #[allow(dead_code)]
    const ELF_DATA_2LSB: u8 = 1;
    // 2's Complement, Big Endian
    const ELF_DATA_2MSB: u8 = 2;

    // direct `BE -> true`, `LE -> false` mapping on an arbitrary expression
    fn is_be(x: Expr) -> Expr {
        expr_eq(x, Expr::U8(ELF_DATA_2MSB))
    }

    let ei_data = module.define_format(
        "elf.header.ident.data",
        where_between_u8(base.u8(), ELF_DATA_NONE, ELF_DATA_2MSB),
    );

    // Invalid Version
    const EV_NONE: u8 = 0;
    // Current Version
    // REVIEW - this is specified as being able to change but unsure how likely that is in practice (libelf on github has := 1, so we should be fine?)
    const EV_CURRENT: u8 = 1;

    let ei_version = module.define_format(
        "elf.header.ident.version",
        where_between_u8(base.u8(), EV_NONE, EV_CURRENT),
    );

    // NOTE: the possible values and interpretations thereof are machine-specific
    let ei_osabi = module.define_format("elf.header.ident.os-abi", base.u8());

    // NOTE: this value distinguishes between different versions of the abi specified by `elf_osabi`, and cannot be interpreted or validated in isolation
    let ei_abiversion = module.define_format("elf.header.ident.abi-version", base.u8());

    const EI_NIDENT: u32 = 16;

    // ELF Identification (e_ident[])
    let elf_ident = module.define_format(
        "elf.header.ident",
        record_auto([
            ("__magic", is_bytes(b"\x7fELF")), // Magic bytes
            ("class", ei_class.call()),        // Class (32-bit or 64-bit)
            ("data", ei_data.call()),          // Data Encoding (endianness)
            ("version", ei_version.call()),    // Elf Version (should be EV_CURRENT (:= 1 ?))
            ("os_abi", ei_osabi.call()),
            ("abi_version", ei_abiversion.call()),
            ("__pad", repeat(is_byte(0x00))), // Zero-byte padding to end of Ident section
        ]),
    );
    // !SECTION

    // SECTION - ELF Header - (File) Type

    const ET_NONE: u16 = 0; // no file type
    #[allow(dead_code)]
    const ET_REL: u16 = 1; // relocatable file
    #[allow(dead_code)]
    const ET_EXEC: u16 = 2; // executable file
    #[allow(dead_code)]
    const ET_DYN: u16 = 3; // shared object file
    const ET_CORE: u16 = 4; // core file
    const ET_LOOS: u16 = 0xfe00; // OS-specific
    const ET_HIOS: u16 = 0xfeff; // OS-specific
    const ET_LOPROC: u16 = 0xff00; // processor-specific
    const ET_HIPROC: u16 = 0xffff; // processor-specific

    // ELF File Type
    let e_type = module.define_format_args(
        "elf.header.type",
        vec![(Label::Borrowed("is_be"), ValueType::Base(BaseType::Bool))],
        where_lambda(
            elf_half_endian.call_args(vec![var("is_be")]),
            "type",
            expr_match(
                var("type"),
                [
                    (
                        Pattern::Int(Bounds::new(ET_NONE as usize, ET_CORE as usize)),
                        Expr::Bool(true),
                    ), // non-processor-specific
                    (
                        Pattern::Int(Bounds::new(ET_LOOS as usize, ET_HIOS as usize)),
                        Expr::Bool(true),
                    ), // OS-specific
                    (
                        Pattern::Int(Bounds::new(ET_LOPROC as usize, ET_HIPROC as usize)),
                        Expr::Bool(true),
                    ), // processor-specific
                    (Pattern::Wildcard, Expr::Bool(false)), // not a recognized object filetype
                ],
            ),
        ),
    );
    // !SECTION

    // ELF Machine (Architecture) identifier
    // TODO - machine variants are an open class that has no fixed definition, and enough distinct values that it would be difficult to exhaustively list them
    let e_machine = module.define_format_args(
        "elf.header.machine",
        vec![(Label::Borrowed("is_be"), ValueType::Base(BaseType::Bool))],
        elf_half_endian.call_args(vec![var("is_be")]),
    );

    // 4-byte version that should match
    let e_version = module.define_format_args(
        "elf.header.version",
        vec![(Label::Borrowed("is_be"), ValueType::Base(BaseType::Bool))],
        where_between_u32(
            elf_word_endian.call_args(vec![var("is_be")]),
            EV_NONE as u32,
            EV_CURRENT as u32,
        ),
    );

    let var_is_be = is_be(record_proj(var("ident"), "data"));

    let elf_addr = module.define_format_args(
        "elf.types.elf-addr",
        vec![
            (Label::Borrowed("is_be"), ValueType::BOOL),
            (Label::Borrowed("class"), ValueType::Base(BaseType::U8)),
        ],
        match_variant(
            var("class"),
            vec![
                (
                    Pattern::U8(ELF_CLASS_32),
                    "Addr32",
                    elf32_addr_endian.call_args(vec![var("is_be")]),
                ), // 32-bit addr
                (
                    Pattern::U8(ELF_CLASS_64),
                    "Addr64",
                    elf64_addr_endian.call_args(vec![var("is_be")]),
                ), // 64-bit addr
            ],
        ),
    );

    let elf_off = module.define_format_args(
        "elf.types.elf-off",
        vec![
            (Label::Borrowed("is_be"), ValueType::BOOL),
            (Label::Borrowed("class"), ValueType::Base(BaseType::U8)),
        ],
        match_variant(
            var("class"),
            vec![
                (
                    Pattern::U8(ELF_CLASS_32),
                    "Off32",
                    elf32_off_endian.call_args(vec![var("is_be")]),
                ), // 32-bit offset
                (
                    Pattern::U8(ELF_CLASS_64),
                    "Off64",
                    elf64_off_endian.call_args(vec![var("is_be")]),
                ), // 64-bit foffset
            ],
        ),
    );

    // Value that is Elf32_Word in 32-bit and Elf64_Xword in 64-bit ELF files
    let elf_full_endian = module.define_format_args(
        "elf.types.elf-full",
        vec![
            (Label::Borrowed("is_be"), ValueType::BOOL),
            (Label::Borrowed("class"), ValueType::Base(BaseType::U8)),
        ],
        match_variant(
            var("class"),
            vec![
                (
                    Pattern::U8(ELF_CLASS_32),
                    "Full32",
                    elf_word_endian.call_args(vec![var("is_be")]),
                ), // 32-bit full-width unsigned int
                (
                    Pattern::U8(ELF_CLASS_64),
                    "Full64",
                    elf64_xword_endian.call_args(vec![var("is_be")]),
                ), // 64-bit full-width unsigned int
            ],
        ),
    );

    let elf_header = module.define_format(
        "elf.header",
        record([
            ("ident", slice(Expr::U32(EI_NIDENT), elf_ident.call())), // machine-independent ELF identification array (byte-oriented)
            ("type", e_type.call_args(vec![var_is_be.clone()])),      // file-type identifier
            ("machine", e_machine.call_args(vec![var_is_be.clone()])), // identifier for the architecture required by the ELF image
            ("version", e_version.call_args(vec![var_is_be.clone()])), // ELF version (should agree with the equivalent field in `ident`)
            (
                "entry",
                elf_addr.call_args(vec![var_is_be.clone(), record_proj(var("ident"), "class")]),
            ), // virtual address of the entry-point into the ELF image
            (
                "phoff",
                elf_off.call_args(vec![var_is_be.clone(), record_proj(var("ident"), "class")]),
            ), // file-offset of the program header table, in bytes (0 if not present)
            (
                "shoff",
                elf_off.call_args(vec![var_is_be.clone(), record_proj(var("ident"), "class")]),
            ), // file-offset of the section header table, in bytes (0 if not present)
            ("flags", elf_word_endian.call_args(vec![var_is_be.clone()])), // processor-specific flags
            ("ehsize", elf_half_endian.call_args(vec![var_is_be.clone()])), // size of the ELF header in bytes
            (
                "phentsize",
                elf_half_endian.call_args(vec![var_is_be.clone()]),
            ), // (consistent) size of each entry in the program header table
            ("phnum", elf_half_endian.call_args(vec![var_is_be.clone()])), // number of entries in the program header table
            (
                "shentsize",
                elf_half_endian.call_args(vec![var_is_be.clone()]),
            ), // (consistent) size of each entry in the section header table
            ("shnum", elf_half_endian.call_args(vec![var_is_be.clone()])), // number of entries in the section header table
            (
                "shstrndx",
                elf_half_endian.call_args(vec![var_is_be.clone()]),
            ), // section header table index of the entry associated with the section name string table
        ]),
    );

    // SECTION - Section Header Type

    const SHT_NULL: u32 = 0; // no associated section
    #[allow(dead_code)]
    const SHT_PROGBITS: u32 = 1; // program-specific data
    #[allow(dead_code)]
    const SHT_SYMTAB: u32 = 2; // symbols for link editing (multiple disallowed)
    #[allow(dead_code)]
    const SHT_STRTAB: u32 = 3; // string table
    #[allow(dead_code)]
    const SHT_RELA: u32 = 4; // relocation entries with addends (multiple allowed)
    #[allow(dead_code)]
    const SHT_HASH: u32 = 5; // symbol hash table (multiple disallowed)
    #[allow(dead_code)]
    const SHT_DYNAMIC: u32 = 6; // information for dynamic linking (multiple disallowed)
    #[allow(dead_code)]
    const SHT_NOTE: u32 = 7; // notes section
    #[allow(dead_code)]
    const SHT_NOBITS: u32 = 8; // like PROGBITS, but occupying no space in the file (sh_offset indicates conceptual file offset)
    #[allow(dead_code)]
    const SHT_REL: u32 = 9; // relocation entries (multiple allowed)
    #[allow(dead_code)]
    const SHT_SHLIB: u32 = 10; // reserved, unspecified semantics
    #[allow(dead_code)]
    const SHT_DYNSYM: u32 = 11; // symbol hash table (multiple disallowed)
                                // NOTE - range-gap for [12,13]
    #[allow(dead_code)]
    const SHT_INIT_ARRAY: u32 = 14; // array of pointers to initialization functions
    #[allow(dead_code)]
    const SHT_FINI_ARRAY: u32 = 15; // array of pointers to termination functions
    #[allow(dead_code)]
    const SHT_PREINIT_ARRAY: u32 = 16; // array of pointers to pre-initialization functions
    #[allow(dead_code)]
    const SHT_GROUP: u32 = 17; // section group (may only appear in ET_REL ELF files, and must precede all other entries of the given group)
    const SHT_SYMTAB_SHNDX: u32 = 18; // Extended section-header indexes associated with an SHT_SYMTAB section, indicating a symbol table entry (and occurring in the same order)

    // NOTE - the following constants are range endpoints and not implicit singletons
    const SHT_LOOS: u32 = 0x60000000; // OS-specific range (lower bounds)
    #[allow(dead_code)]
    const SHT_HIOS: u32 = 0x6fffffff; // OS-specific range (upper bounds)

    #[allow(dead_code)]
    const SHT_LOPROC: u32 = 0x70000000; // processor-specific range (lower bounds)
    #[allow(dead_code)]
    const SHT_HIPROC: u32 = 0x7fffffff; // processor-specific range (upper bounds)

    #[allow(dead_code)]
    const SHT_LOUSER: u32 = 0x80000000; // application-specific range (lower bounds)
    const SHT_HIUSER: u32 = 0xffffffff; // application-specific range (upper bounds)

    // Section Header Type
    let elf_sh_type = module.define_format_args(
        "elf.shdr.sh-type",
        vec![ISBE_ARG],
        where_lambda(
            elf_word_endian.call_args(vec![var("is_be")]),
            "sh-type",
            expr_match(
                var("sh-type"),
                [
                    // values in range SHT_NULL..=SHT_DYNSYM (or 0..=11)
                    (
                        Pattern::Int(Bounds::new(SHT_NULL as usize, SHT_DYNSYM as usize)),
                        Expr::Bool(true),
                    ),
                    (
                        Pattern::Int(Bounds::new(
                            SHT_INIT_ARRAY as usize,
                            SHT_SYMTAB_SHNDX as usize,
                        )),
                        Expr::Bool(true),
                    ),
                    (
                        Pattern::Int(Bounds::new(SHT_LOOS as usize, SHT_HIUSER as usize)),
                        Expr::Bool(true),
                    ),
                    (Pattern::Wildcard, Expr::Bool(false)),
                ],
            ),
        ),
    );

    // !SECTION

    /* Section Header Flags - 1-bit flags in ELF-file specific byte-order and bit-class
     *
     * Flag Values:
     *   0x1 - SHF_WRITE : whether the section data should be writable during execution
     *   0x2 - SHF_ALLOC : whether the section occupies memory during execution
     *   0x4 - SHF_EXECINSTR : whether the section contains executable machine instructions
     *  0x10 - SHF_MERGE :  whether the section-data may be merged to eliminate duplication
     *  0x20 - SHF_STRINGS : whether the section-data consists of null-terminated char-strings (character size indicated by `entsize` field`)
     *  0x40 - SHF_INFO_LINK : whether the `info` field in section header contains a section header table index
     *  0x80 - SHF_LINK_ORDER : used to add special ordering requirements for link editors
     * 0x100 - SHF_OS_NONCONFORMING : whether the section must be processed in a special, OS-specific way by the linker to avoid incorrect behavior
     * 0x200 - SHF_GROUP : whether the section is a member (even singleton) of a section group
     * 0x0ff0_00000 - SHF_MASKOS : mask of bits reserved for operating system-specific semantics
     * 0xf000_00000 - SHF_MASKPROC : mask of bits reserved for processor-specific semantics
     */
    // TODO - add ease-of-interpretation for any flags that may dictate or inform the layout of the ELF file, as well as for external transparency in output
    // FIXME - for now, using an uninterpreted, raw full-width uint for section-header flags...
    let elf_sh_flags = elf_full_endian;

    /* `sh_info` field of shdr
     *
     * Interpretation and semantics depend on the elf_sh_type value:
     *  - SHT_REL[A]  => section header index for section to which relocation applies
     *  - SHT_SYMTAB,
     *    SHT_DYNSYM  => one more than the symbol table index of last local symbol (STB_LOCAL binding)
     *  - SHT_GROUP   => symbol table index of entry in associated symbol table
     *  - SHT_SYMTAB_SHNDX => 0
     *  - SHT_DYNAMIC => 0
     *  - SHT_HASH    => 0
     */
    let elf_sh_info = module.define_format_args(
        "elf.shdr.sh-info",
        vec![ISBE_ARG],
        elf_word_endian.call_args(vec![var("is_be")]),
    );

    // Elf Section Header
    let elf_shdr = module.define_format_args(
        "elf.shdr",
        vec![ISBE_ARG, CLASS_ARG],
        record([
            ("name", elf_word_endian.call_args(vec![var("is_be")])), // specifier for the section-name, given as an index into the string-header section table
            ("type", elf_sh_type.call_args(vec![var("is_be")])), // section type, which dictates its contents and semantics
            (
                "flags",
                elf_sh_flags.call_args(vec![var("is_be"), var("class")]),
            ), // sequence of 1-bit flags dictating various attributes
            ("addr", elf_addr.call_args(vec![var("is_be"), var("class")])), // virtual address of (the first byte of) this section in memory, when it will appear in the memory image of the process during execution (0 otherwise)
            (
                "offset",
                elf_off.call_args(vec![var("is_be"), var("class")]),
            ), // file-offset of the first byte in this section
            (
                "size",
                elf_full_endian.call_args(vec![var("is_be"), var("class")]),
            ), // number of bytes in this section
            ("link", elf_word_endian.call_args(vec![var("is_be")])), // section header table index link (depends on section type)
            ("info", elf_sh_info.call_args(vec![var("is_be")])), // extra information that the section type dictates the interpretation of
            (
                "addralign",
                elf_full_endian.call_args(vec![var("is_be"), var("class")]),
            ), // section alignment
            (
                "entsize",
                elf_full_endian.call_args(vec![var("is_be"), var("class")]),
            ), // section entry size
        ]),
    );

    // FIXME - if the number of sections is large enough, a 0 will be recorded in header.shnum and the number of headers will be indicated by the first header
    // REVIEW - this is not correct for all cases
    let elf_shdr_table = module.define_format_args(
        "elf.shdr-table",
        vec![
            ISBE_ARG,
            CLASS_ARG,
            (Label::Borrowed("shnum"), ValueType::Base(BaseType::U16)),
        ],
        repeat_count(
            var("shnum"),
            elf_shdr.call_args(vec![var("is_be"), var("class")]),
        ),
    );

    // Program Header Flags (32-bit and 64-bit)

    // 64-bit selective Option
    let elf_ph_flags64 = module.define_format_args(
        "elf.phdr.p-flags64",
        vec![ISBE_ARG, CLASS_ARG],
        cond_maybe(
            expr_eq(var("class"), Expr::U8(ELF_CLASS_64)),
            elf_word_endian.call_args(vec![var("is_be")]),
        ),
    );

    // 32-bit selective Option
    let elf_ph_flags32 = module.define_format_args(
        "elf.phdr.p-flags32",
        vec![ISBE_ARG, CLASS_ARG],
        cond_maybe(
            expr_eq(var("class"), Expr::U8(ELF_CLASS_32)),
            elf_word_endian.call_args(vec![var("is_be")]),
        ),
    );

    let elf_phdr = module.define_format_args(
        "elf.phdr",
        vec![ISBE_ARG, CLASS_ARG],
        record([
            ("type", elf_word_endian.call_args(vec![var("is_be")])),
            (
                "flags64",
                elf_ph_flags64.call_args(vec![var("is_be"), var("class")]),
            ),
            (
                "offset",
                elf_off.call_args(vec![var("is_be"), var("class")]),
            ),
            (
                "vaddr",
                elf_addr.call_args(vec![var("is_be"), var("class")]),
            ),
            (
                "paddr",
                elf_addr.call_args(vec![var("is_be"), var("class")]),
            ),
            (
                "filesz",
                elf_full_endian.call_args(vec![var("is_be"), var("class")]),
            ),
            (
                "memsz",
                elf_full_endian.call_args(vec![var("is_be"), var("class")]),
            ),
            (
                "flags32",
                elf_ph_flags32.call_args(vec![var("is_be"), var("class")]),
            ),
            (
                "align",
                elf_full_endian.call_args(vec![var("is_be"), var("class")]),
            ),
        ]),
    );

    let elf_phdr_table = module.define_format_args(
        "elf.phdr-table",
        vec![
            ISBE_ARG,
            CLASS_ARG,
            (Label::Borrowed("phnum"), ValueType::Base(BaseType::U16)),
        ],
        repeat_count(
            var("phnum"),
            elf_phdr.call_args(vec![var("is_be"), var("class")]),
        ),
    );

    let elf_section = module.define_format_args(
        "elf.section",
        vec![
            (Label::Borrowed("type"), ValueType::Base(BaseType::U32)),
            (Label::Borrowed("size"), ValueType::Base(BaseType::U64)),
        ],
        // FIXME - we can refine this a lot more based on the type passed in
        Format::Match(
            Box::new(var("type")),
            vec![
                (Pattern::Wildcard, repeat_count(var("size"), base.u8())), // abstract (unrefined) section
            ],
        ),
    );

    let full_as_64 = |e: Expr| -> Expr {
        expr_match(
            e,
            [
                (
                    Pattern::Variant(Label::Borrowed("Full32"), Box::new(Pattern::binding("x32"))),
                    Expr::AsU64(Box::new(var("x32"))),
                ),
                (
                    Pattern::Variant(Label::Borrowed("Full64"), Box::new(Pattern::binding("x64"))),
                    var("x64"),
                ),
            ],
        )
    };

    let off_as_64 = |e: Expr| -> Expr {
        expr_match(
            e,
            [
                (
                    Pattern::Variant(Label::Borrowed("Off32"), Box::new(Pattern::binding("x32"))),
                    Expr::AsU64(Box::new(var("x32"))),
                ),
                (
                    Pattern::Variant(Label::Borrowed("Off64"), Box::new(Pattern::binding("x64"))),
                    var("x64"),
                ),
            ],
        )
    };

    let eoh_offset_none0 = |offset_file: Expr, f: Format| {
        cond_maybe(
            expr_match(
                offset_file.clone(),
                [
                    (
                        Pattern::Variant(Label::Borrowed("Off32"), Box::new(Pattern::U32(0))),
                        Expr::Bool(false),
                    ),
                    (
                        Pattern::Variant(Label::Borrowed("Off64"), Box::new(Pattern::U64(0))),
                        Expr::Bool(false),
                    ),
                    (Pattern::Wildcard, Expr::Bool(true)),
                ],
            ),
            Format::WithRelativeOffset(
                Box::new(Expr::U64(0)),
                Box::new(off_as_64(offset_file)),
                Box::new(f),
            ),
        )
    };

    let elf_ph = eoh_offset_none0(
        record_proj(var("header"), "phoff"),
        elf_phdr_table.call_args(vec![
            is_be(record_lens(var("header"), &["ident", "data"])),
            record_lens(var("header"), &["ident", "class"]),
            record_proj(var("header"), "phnum"),
        ]),
    );

    let elf_sh = eoh_offset_none0(
        record_proj(var("header"), "shoff"),
        elf_shdr_table.call_args(vec![
            is_be(record_lens(var("header"), &["ident", "data"])),
            record_lens(var("header"), &["ident", "class"]),
            record_proj(var("header"), "shnum"),
        ]),
    );

    module.define_format(
        "elf.main",
        record_auto([
            ("header", elf_header.call()),
            ("__eoh", Format::Pos),
            ("program_headers", elf_ph),
            ("section_headers", elf_sh),
            (
                "sections",
                // FIXME: this suggests the definition of a Format-level Option::map helper
                Format::Match(
                    Box::new(var("section_headers")),
                    vec![
                        (
                            pat_some(Pattern::binding("shdrs")),
                            fmt_some(for_each(
                                var("shdrs"),
                                "shdr",
                                cond_maybe(
                                    and(
                                        expr_ne(
                                            record_proj(var("shdr"), "type"),
                                            Expr::U32(SHT_NOBITS),
                                        ),
                                        expr_ne(
                                            record_proj(var("shdr"), "type"),
                                            Expr::U32(SHT_NULL),
                                        ),
                                    ),
                                    Format::WithRelativeOffset(
                                        Box::new(Expr::U64(0)),
                                        Box::new(off_as_64(record_proj(var("shdr"), "offset"))),
                                        Box::new(elf_section.call_args(vec![
                                            record_proj(var("shdr"), "type"),
                                            full_as_64(record_proj(var("shdr"), "size")),
                                        ])),
                                    ),
                                ),
                            )),
                        ),
                        (pat_none(), fmt_none()),
                    ],
                ),
            ),
            ("__skip", Format::SkipRemainder),
        ]),
    )
}
