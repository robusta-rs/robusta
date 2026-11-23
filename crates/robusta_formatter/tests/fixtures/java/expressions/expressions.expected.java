class Account {
  private final String owner;
  private int balance;

  Account(String owner, int balance) {
    this.owner = owner;
    this.balance = balance;
  }

  int deposit(int amount) {
    balance += amount;
    this.log("deposit", amount);
    return balance;
  }

  int signum() {
    return balance < 0 ? -1 : balance == 0 ? 0 : 1;
  }

  boolean canWithdraw(int amount) {
    return 0 < amount && amount <= balance - (balance % 10);
  }

  void log(String action, int amount) {
    Logger.get(this).record(action, amount, true, null);
  }
}
