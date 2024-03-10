use colored::*;
use prettytable::{Cell, Row, Table};
use std::borrow::Cow;

use crate::skatteverket::TestPerson;

// Function to create a colored cell with the specified text and color
// Function to create a colored cell with the specified text and color
pub fn colored_cell<S: Into<Cow<'static, str>>>(text: S, color: Color) -> Cell {
    let text = text.into();
    Cell::new(&format!("{}", text.color(color)))
}
// Function to print the data in a table with colored headers and text
pub fn print_table(data: Vec<TestPerson>) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        colored_cell("Testpersonnummer", Color::BrightCyan),
    ]));

    for person in data {
        table.add_row(Row::new(vec![
            colored_cell(person.testpersonnummer, Color::Green),
        ]));
    }

    table.printstd();
}

pub fn print_logo() {
    println!("{}", "███████ ██████  ███    ██ ██████  ".bright_yellow());
    println!("{}", "██      ██   ██ ████   ██ ██   ██ ".bright_yellow());
    println!("{}", "███████ ██████  ██ ██  ██ ██████  ".bright_yellow());
    println!("{}", "     ██ ██      ██  ██ ██ ██   ██ ".bright_yellow());
    println!("{}", "███████ ██      ██   ████ ██   ██".bright_yellow());
    println!("{}", "                    #####           ".bright_cyan());
    println!("{}", "                  %########         ".bright_cyan());
    println!("{}", "                 ##########         ".bright_cyan());
    println!("{}", "               ######%              ".bright_cyan());
    println!("{}", "               #####  ========     ".bright_cyan());
    println!("{}", "              ####   =========     ".bright_cyan());
    println!("{}", "              ###%=======          ".bright_cyan());
    println!("{}", "              ###*=====  %########".bright_cyan());
    println!("{}", "              ## ====  ###########%".bright_cyan());
    println!("{}", "          #%  ##===  #######   #####".bright_cyan());
    println!("{}", "        ####  ##=== %####         %".bright_cyan());
    println!("{}", "        ##### %%*==####    =*%##%% ".bright_cyan());
    println!("{}", "        #####%  #=+##% ###*===*#####".bright_cyan());
    println!("{}", "      =   ######         ####===== ####".bright_cyan());
    println!("{}", "    ====  #########    %  ####===== #####".bright_cyan());
    println!("{}", "    ======    #########   #### ===== %####".bright_cyan());
    println!("{}", "    ==========      ===== ####  ===== %####".bright_cyan());
    println!("{}", " ##    ===============+#  ####  =====  #####".bright_cyan());
    println!("{}", "#####     =========+#### #####  =====  #####".bright_cyan());
    println!("{}", "########        ####%   %#####  =====  #####".bright_cyan());
    println!("{}", " ##################    ######% ======  #####".bright_cyan());
    println!("{}", "    ###########%      ######   ======  %####".bright_cyan());
}