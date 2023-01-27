#[macro_use]
extern crate prettytable;
use prettytable::Table;

fn main() {
    use prettytable::format;

    let mut table = Table::new();

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
    table.printstd();
}
