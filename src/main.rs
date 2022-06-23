mod colors;
use std::array;

use colors::*;
use comfy_table::Table;
use comfy_table::TableComponent::*;

fn main() {
    println!("{RED}{}@{BLUE}{}", whoami::username(), whoami::hostname());
    let mut table = Table::new();
    table
        .add_row(vec!["Insert ascii art here", "hostname etc"])
        .add_row(vec!["", "system info here"]);
    let components = [
        LeftBorder,
        RightBorder,
        TopBorder,
        BottomBorder,
        LeftHeaderIntersection,
        HeaderLines,
        MiddleHeaderIntersections,
        RightHeaderIntersection,
        VerticalLines,
        HorizontalLines,
        MiddleIntersections,
        LeftBorderIntersections,
        RightBorderIntersections,
        TopBorderIntersections,
        BottomBorderIntersections,
        TopLeftCorner,
        TopRightCorner,
        BottomLeftCorner,
        BottomRightCorner,
    ];

    for component in components {
        table.set_style(component, ' ');
    }

    println!("{table}")
}
