fn main(){
    let account = new_account(String::from("Pauls Toma"), 500000000000000);
    println!("Owner: {}\nBalance: ${}", account.owner, account.balance);
}

struct Account{
    owner: String,
    balance: u64,
    active: bool,
}

fn new_account(owner: String, balance: u64) -> Account {
    Account {
        owner,
        balance,
        active: true,
    }
}