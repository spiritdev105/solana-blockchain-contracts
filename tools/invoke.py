from solana.rpc.api import Client
from solders.pubkey import Pubkey

def main():
    client = Client("http://localhost:8899")
    pub_key = Pubkey.from_string("CHDriqdpVGcYGR9hKbxGtBUiUh7umMEa4gViotcYgsnN")
    balance = client.get_balance(pub_key)
    print(balance)

if __name__ == "__main__":
    main()
