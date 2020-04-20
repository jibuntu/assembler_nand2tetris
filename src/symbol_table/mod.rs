// nand2tetris - page 125

use std::collections::HashMap;

/// symbolの名前とアドレスを管理するためのモジュール
#[derive(Debug, PartialEq)]
pub struct SymbolTable {
    table: HashMap<String, usize>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            table: HashMap::new()
        }
    }

    pub fn add_entry(&mut self, symbol: &str, address: usize) {
        self.table.insert(symbol.to_string(), address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Option<&usize> {
        self.table.get(symbol)
    }
}

#[cfg(test)]
mod test {
    use super::SymbolTable;

    #[test]
    fn test_symbol_table_new() {
        SymbolTable::new();
    }

    #[test]
    fn test_symbol_table_add_entry() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_entry("test", 10);
        symbol_table.add_entry("aa", 0);
    }

    #[test]
    fn test_symbol_table_contains() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_entry("test", 10);
        assert_eq!(symbol_table.contains("test"), true);
        assert_eq!(symbol_table.contains("m"), false);
    }

    #[test]
    fn test_symbol_table_get_address() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_entry("test", 10);
        assert_eq!(symbol_table.get_address("test"), Some(&10));
    }
}