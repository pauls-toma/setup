fn main() {
    
let mut account = BankAccount{
    balance: 5.0,
    };

       println!("Balance before deposit: {}", account.balance);
       account.deposit(5.0);
       account.withdraw(9.0);

}
struct BankAccount{
       balance: f64,
}

impl BankAccount {
    fn deposit(&mut self, amount: f64){
        let new_balance: f64 = amount + self.balance;
        self.balance = new_balance;
        println!("after deposit balance: {}", new_balance);
    }
}

impl BankAccount {
    fn withdraw (&mut self, amount: f64){
        let new_balance: f64 = self.balance - amount;
        self.balance = new_balance;
        println!("after withdraw: {}", new_balance);
    }
}

