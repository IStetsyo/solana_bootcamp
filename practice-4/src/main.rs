use dotenv::dotenv;
mod check_balance;
mod generate_keypair;
mod load_keypair;
mod parse_key;

fn main() {
    dotenv().ok();

    generate_keypair::generate_by_start("anza");
    load_keypair::load_keypair();
    check_balance::check_balance(false)
}
