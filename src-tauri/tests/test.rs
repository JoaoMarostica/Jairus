#[cfg(test)]
mod tests {
    use seed_manager_lib::read_spreadsheet;

    #[test]
    fn read_spreadsheet_test() {
        read_spreadsheet("./tests/planilha.xlsx");
    }

}