use prettytable::{Table, row};

use crate::etherscan::account::AccTxn;

pub fn display_table(mut data: Vec<AccTxn>) {
    let mut table = Table::new();

    table.add_row(row!["Block Number", "TimeStamp", "From", "To", "Value"]);

    loop {
        match data.pop() {
            Some(txn) => {
                table.add_row(row![
                    txn.block_number,
                    txn.time_stamp,
                    txn.from,
                    txn.to,
                    txn.value,
                ]);
            },
            None => break,
        }
    }
    table.printstd();
}