#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount{
            balance: if initial_balance >= 0.00 && (initial_balance * 100.0).fract() == 0.0{
                initial_balance
            }
            else {
                0.00
            },
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method

        // Amount should be positive, coins should be represented by only 2 decimal digits. Ex: 0.99
        if amount > 0.0 && (amount * 100.0).fract() == 0.0{
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method

        // Amount must be positive, coins should be represented by only 2 decimal digits, and less than the account balance.
        if amount > 0.0 && (amount * 100.0).fract() == 0.0 && amount <= self.balance{
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }

    pub fn apply_interest(&mut self, interest_rate: f64){
        if interest_rate >= 0.0 {
            self.balance += self.balance * (interest_rate/ 100.0);

            // Always round up to 2 decimal places
            self.balance = (self.balance * 100.0).ceil() / 100.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(50.00);
        assert_eq!(account.balance(), 50.00);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(50.00);
        account.deposit(20.00);
        assert_eq!(account.balance(), 70.00);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(50.00);
        account.withdraw(20.00);
        assert_eq!(account.balance(), 30.00);
    }

    #[test]
    fn test_apply_interest(){
        let mut account = BankAccount::new(50.00);
        account.apply_interest(10.99);
        assert!((account.balance() - 55.50).abs() < EPSILON);
    }
    // Add more tests here

    #[test]
    fn test_new_negative_account() {
        // Test for negative initial balance
        let account1 = BankAccount::new(-50.00);
        assert_eq!(account1.balance(), 0.00);

        let account2 = BankAccount::new(-0.99);
        assert_eq!(account2.balance(), 0.00);
    }

    #[test]
    fn test_deposit_coins(){
        // Test for depositing coins
        let mut account = BankAccount::new(50.00);
        account.deposit(0.01);
        assert_eq!(account.balance(), 50.01);
    }

    #[test]
    fn test_deposit_wrong_coins(){
        // Test for depositing coins
        let mut account = BankAccount::new(50.00);
        account.deposit(0.05999);
        assert_eq!(account.balance(), 50.00);
    }

    #[test]
    fn test_negative_deposit(){
        // Tests for depositing negative amounts
        let mut account1 = BankAccount::new(50.00);
        account1.deposit(-20.00);
        assert_eq!(account1.balance(), 50.00);

        let mut account2 = BankAccount::new(50.00);
        account2.deposit(-0.50);
        assert_eq!(account2.balance(), 50.00);
    }

    #[test]
    fn test_withdraw_insufficient(){
        // Tests for withdrawing an amount greater than the balance
        let mut account1 = BankAccount::new(50.00);
        account1.withdraw(100.00);
        assert_eq!(account1.balance(), 50.00);

        let mut account2 = BankAccount::new(50.00);
        account2.withdraw(50.01);
        assert_eq!(account2.balance(), 50.00);
    }

    #[test]
    fn test_negative_withdraw(){
        // Tests for withdrawing a negative ammount
        let mut account1 = BankAccount::new(50.00);
        account1.withdraw(-100.0);
        assert_eq!(account1.balance(), 50.00);

        let mut account2 = BankAccount::new(50.00);
        account2.withdraw(-0.01);
        assert_eq!(account2.balance(), 50.00);
    }

    #[test]
    fn zero_interest(){
        let mut account = BankAccount::new(50.00);
        account.apply_interest(0.0);
        assert_eq!(account.balance(), 50.00);
    }

    #[test]
    fn negative_interest(){
        let mut account = BankAccount::new(50.00);
        account.apply_interest(-10.0);
        assert_eq!(account.balance(), 50.00);
    }
}