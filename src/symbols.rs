use std::collections::BTreeMap;

use addr2line::object::{SymbolIndex, ElfFile, Object};

pub struct SymbolTable {
    map: BTreeMap<u64, SymbolIndex>,
}

impl SymbolTable {
    pub fn new(elf_file: &ElfFile) -> Self {
        let mut map = BTreeMap::new();

        for (idx, sym) in elf_file.symbols() {
            map.insert(sym.address(), idx);
        }

        SymbolTable { map }
    }

    pub fn lookup_symbol_index(&self, addr: u64) -> Option<SymbolIndex> {
        self.map.range(0..=addr).next_back().map(|(_, idx)| *idx)
    }
}
