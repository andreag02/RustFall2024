mod bank_account;

fn main(){
    let initial_balance = 10.00;
    let mut my_account = bank_account::BankAccount::new(initial_balance);
    println!("Initial balance: ${:.2}", my_account.balance());

    let deposit_amount1 = 100.00;
    my_account.deposit(deposit_amount1);
    println!("Depositing ${:.2}... new balance: ${:.2}", deposit_amount1, my_account.balance());

    let deposit_amount2 = 20.50;
    my_account.deposit(deposit_amount2);
    println!("Depositing ${:.2}... new balance: ${:.2}", deposit_amount2, my_account.balance());

    let withdraw_amount1 = 30.00;
    my_account.withdraw(withdraw_amount1);
    println!("Withdrawing ${:.2}... new balance: ${:.2}", withdraw_amount1, my_account.balance());

    let withdraw_amount2 = 0.50;
    my_account.withdraw(withdraw_amount2);
    println!("Withdrawing ${:.2}... new balance: ${:.2}", withdraw_amount2, my_account.balance());

    let negative_deposit = -1.00;
    my_account.deposit(negative_deposit);
    println!("Attempting to deposit ${:.2}... balance remains ${:.2}", negative_deposit, my_account.balance());

    let insufficient_withdraw = 1000.00;
    my_account.withdraw(insufficient_withdraw);
    println!("Attempting to withdraw ${:.2}... balance remains ${:.2}", insufficient_withdraw, my_account.balance());

    let interest_rate = 5.0;
    my_account.apply_interest(interest_rate);
    println!("Balance after applying {}% interest: ${:.2}", interest_rate, my_account.balance());

}