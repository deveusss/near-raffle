
Alice trying to buy prize, sending ft_transfer_call to prize ft contract:
```bash
near call $prize ft_transfer_call '{"receiver_id": "'$raffle'", "amount": "6","msg":"buy_prize"}' --accountId alice.$prize --amount 0.000000000000000000000001 --gas 200000000000000
```

Reset: will recreate all tickets again:
```bash
near call $raffle reset --accountId $raffle
```

Check balance of alice
```bash
near view $prize ft_balance_of '{"account_id": "'alice.$prize'"}'
```

Available tickets
```bash
near view $raffle total_tickets 
```

