pub trait Account{
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}
pub struct BankAccount{
    pub account_number: String,
    pub holder_name: String,
    pub balance: f64,
}


impl Account for BankAccount {

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}
 fn main(){
     let mut  bank_account1 = BankAccount{
         account_number: String::from("30"),
         holder_name: String::from("nyakio main"),
         balance: 10000.0,
     };
     
     
    let mut bank_account2 = BankAccount{
         account_number: String::from("50"),
         holder_name: String::from("Joanne muthoni"),
         balance: 20000.0,
     };
     
     bank_account1.deposit(5000.0);
     bank_account2.withdraw(300.0);
     
     println!("this it the balance of accout 1: {}",  bank_account1.balance());
     
     println!("this it the balance of accout 2: {}",  bank_account2.balance());
     
 }
