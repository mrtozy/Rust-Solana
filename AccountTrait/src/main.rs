
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}


struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}


impl Account for BankAccount {
    
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    
    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Yetersiz bakiye!");
        }
    }

   
    fn balance(&self) -> f64 {
        self.balance
    }
}


fn main() {
    
    let mut account1 = BankAccount {
        account_number: String::from("123456"),
        holder_name: String::from("Ahmet Yılmaz"),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: String::from("654321"),
        holder_name: String::from("Ayşe Demir"),
        balance: 500.0,
    };

   
    account1.deposit(200.0);
    println!("Hesap {} bakiye: {:.2}", account1.account_number, account1.balance());

    
    account2.withdraw(100.0);
    println!("Hesap {} bakiye: {:.2}", account2.account_number, account2.balance());
}
