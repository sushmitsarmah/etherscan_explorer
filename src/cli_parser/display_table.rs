use prettytable::{Table, row};

// use crate::etherscan::account::AccTxn;

pub struct TableRow {
    block_number: String,
    time_stamp: String,
    from: String,
    to: String,
    value: String,
}

impl TableRow {
    pub fn new(block_number: String, time_stamp: String, from: String, to: String, value: String) -> Self {
        TableRow { block_number, time_stamp, from, to, value }
    }

    pub fn add_table_row(&self, table: &mut Table) {
        table.add_row(row![
            &self.block_number,
            &self.time_stamp,
            &self.from,
            &self.to,
            &self.value,
        ]);
    }
}

pub fn display_table_new(data: Vec<TableRow>) {
    let mut table = Table::new();

    table.add_row(row!["Block Number", "TimeStamp", "From", "To", "Value"]);

    for i in 0..data.len() {
        data[i].add_table_row(&mut table);
    }

    table.printstd();
}
