struct BankAccount {
    balance: u32,
}

impl BankAccount{
    fn deposit(&mut self, amount: u32){
        self.balance += amount;
    }
}
impl BankAccount{
    fn withdraw(&mut self, amount: u32){
        self.balance -= amount;
    }
}
fn main(){
    let mut account = BankAccount{
        balance: 0,
    };
    account.deposit(100);
    account.withdraw(10);
    println!("{}", account.balance);
}