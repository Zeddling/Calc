/// Symbol Table Struct
/// Data structure to store information about the scope and binding information of
/// the variable names.
#[derive(Debug)]
pub struct SymbolTable {
    /// The data stored about the variables
    entries: Vec<(String, f64)>,
}

impl SymbolTable {
    /// Returns a new symbol table data structure
    ///
    /// #   Examples
    ///
    /// ```
    /// use symbol_table::SymbolTable;
    /// let table = SymbolTable::new();
    /// ```
    pub fn new() -> SymbolTable {
        SymbolTable {
            entries: Vec::<(String, f64)>::new(),
        }
    }

    /// Returns a Result struct holding the symbol table's size
    ///
    /// #   Examples
    /// ```
    /// use symbol_table::SymbolTable;
    /// let table = SymbolTable::new();
    /// assert_eq!(table.insert_symbol("a"), 0);
    /// ```
    pub fn insert_symbol(&mut self, identifier: &str) -> Result<usize, String> {
        if self
            .entries
            .iter()
            .find(|item| item.0 == identifier)
            .is_some()
        {
            Err(format!(
                "Error: Identifier '{}' declared several times.",
                identifier
            ))
        } else {
            self.entries.push((identifier.to_string(), 0.));
            Ok(self.entries.len() - 1)
        }
    }

    /// Returns a Reuslt struct containing the position of the identifier
    ///
    /// #   Examples
    /// ```
    /// use symbol_table::SymbolTable;
    /// let table = SymbolTable::new();
    /// table::insert_symbol("a");
    /// assert_eq!(table.find_symbol("a"), 0);
    /// ```
    pub fn find_symbol(&self, identifier: &str) -> Result<usize, String> {
        if let Some(position) = self.entries.iter().position(|item| item.0 == identifier) {
            Ok(position)
        } else {
            Err(format!(
                "Error: Identifier '{}' used before having been declared.",
                identifier
            ))
        }
    }
}
