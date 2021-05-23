# wBAN Hot wallet monitoring tool

This tools checks that the wBAN hot wallet has enough BAN available.

Such check is made against a total of BAN deposited (cold + hot wallets balances) and a ratio of BAN the hot wallet should have.

If such ratio is not acheived, it sends Reddit DM to administrators which an amount of BAN to send from the cold wallet to the hot wallet.

Example of DM sent:
```
I need 8 BAN to be sent to hot wallet "ban_1xxxxxxxxxxxxxxxx", in order to reach 20% of users deposits.
```

## Settings

This tool expect the following environment variables:

| Env Name                   | Env Description     | Example               |
|----------------------------|---------------------|-----------------------|
| `WBAN_API`                 | URL of the wBAN API | `http://localhost:3000` |
| `BAN_RPC_API`              | Host and port of the BAN RPC API | `10.60.0.70:7072` |
| `BAN_HOT_WALLET`           | Banano address of the hot wallet | `ban_1...` |
| `BAN_COLD_WALLET`          | Banano address of the cold wallet | `ban_1...` |
| `THRESHOLD_PERCENTAGE`     | Percentage of BAN that should be in hot wallet | 20 |
| `REDDIT_BOT_USERNAME`      | Reddit username sending DM messages | `wban-banano`
| `REDDIT_BOT_PASSWORD`      | Reddit password of the user sending DM | `<my_password> ` |
| `REDDIT_BOT_CLIENT_ID`     | Reddit bot client ID | `<...>` |
| `REDDIT_BOT_CLIENT_SECRET` | Reddit bot client secret | `<...>` |
| `REDDIT_BOT_DM_USERS`      | List (whitespace separated) of users to send Reddit DM to | `user1 user2 user3` | 