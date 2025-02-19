use rust_i18n::t;

rust_i18n::i18n!("locales");

fn main() {
    println!("{:?}", rust_i18n::available_locales!());
    println!("{}", t!("hello"));
    println!("{}", t!("testing"));
    println!("{}", t!("apple"));
}
