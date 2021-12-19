# near-raffle

echo "Transfer call raffle contract"
near call $CONTRACT_NAME ft_transfer_call '{"receiver_id": "'$RAFFLE_CONTRACT_NAME'", "amount": "6","msg":"buy_prize"}' --accountId alice.$CONTRACT_NAME --amount 0.000000000000000000000001
