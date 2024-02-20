use rka_attr::*;

fn main() {
    // FIXME: https://github.com/rust-lang/rust/issues/54727
    //#[rka_attr::postfix_block]
    //123
}

#[rka_attr::postfix_block]
fn f() {
    123
}
