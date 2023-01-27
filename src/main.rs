#[macro_use]
extern crate prettytable;
use prettytable::format::TableFormat;
use prettytable::row::Row;

use std::io::{self, Error, Write};
use std::mem::transmute;

/// An owned printable table
#[derive(Clone, Debug)]
pub struct Table {
    format: Box<TableFormat>,
    titles: Box<Option<Row>>,
    rows: Vec<Row>,
}

#[derive(Clone, Debug)]
pub struct TableSlice<'a> {
    format: &'a TableFormat,
    titles: &'a Option<Row>,
    rows: &'a [Row],
}

impl<'a> TableSlice<'a> {
    /// Compute and return the number of column
    pub fn get_column_num(&self) -> usize {
        let mut cnum = 0;
        for r in self.rows {
            let l = r.len();
            if l > cnum {
                cnum = l;
            }
        }
        cnum
    }

    /// Get the number of rows
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    fn get_column_width(&self, col_idx: usize) -> usize {
        let mut width = match *self.titles {
            Some(ref t) => t.get_cell_width(col_idx),
            None => 0,
        };
        for r in self.rows {
            let l = r.get_cell_width(col_idx);
            if l > width {
                width = l;
            }
        }
        width
    }

    fn get_all_column_width(&self) -> Vec<usize> {
        let colnum = self.get_column_num();
        let mut col_width = vec![0usize; colnum];
        for i in 0..colnum {
            col_width[i] = self.get_column_width(i);
        }
        col_width
    }

    pub fn printstd(&self) {
        let col_width = self.get_all_column_width();
    }
}

impl<'a> AsRef<TableSlice<'a>> for TableSlice<'a> {
    fn as_ref(&self) -> &TableSlice<'a> {
        self
    }
}

impl<'a> AsRef<TableSlice<'a>> for Table {
    fn as_ref(&self) -> &TableSlice<'a> {
        unsafe {
            // All this is a bit hacky. Let's try to find something else
            let s = &mut *((self as *const Table) as *mut Table);
            s.rows.shrink_to_fit();
            transmute(self)
        }
    }
}

impl Table {
    /// Create an empty table
    pub fn new() -> Table {
        Self::init(Vec::new())
    }

    /// Create a table initialized with `rows`
    pub fn init(rows: Vec<Row>) -> Table {
        Table {
            rows: rows,
            titles: Box::new(None),
            format: Box::new(*prettytable::format::consts::FORMAT_DEFAULT),
        }
    }
}

fn check_result<'a>(t1: &Table, t2: &TableSlice<'a>) {
    println!("debug here");
}

fn main() {
    let mut table = Table::new();

    /*
    table.set_titles(row![
        "ServiceID",
        "ServiceName",
        "Vlan ID",
        "Total Ports#",
        "Locked #",
        "Unlocked #",
        "Inconsistent"
    ]);
    table.add_row(row![
        1,
        format!("Service #{}", 1),
        42,
        10000,
        10000,
        9999,
        ""
    ]);
    */

    let table_ref = table.as_ref();
    check_result(&table, table_ref);

    table_ref.printstd();
}
