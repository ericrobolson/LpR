#[macro_use]
extern crate lpr;
use lpr::*;

lpr_init!(
    This is a first level header
    ===========================

    This is a second level header
    ----------------------------

    Note: I'm not quite sure what's going on here.
    ### This is the third header
    > This is a blockquote.
    >
    > This is the second paragraph in the block quote.
    > ## This is an H2 in the blockqote. 

    ## This is the list test section.
    ### * symbols
    * Candy.
    * Gum.
    * Booze.
    ### + symbols
    + Candy.
    + Gum.
    + Booze.
    ### - symbols
    - Candy.
    - Gum.
    - Booze.

    fn main() {
        println!("Hello, world!");
    }
);
