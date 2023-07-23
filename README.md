# GameAccountToken

#open the UI to deploy smart contract in the test network and call the contract's function:
use command:    cd substrate-contracts-node
                cargo build --release
                cd contracts-ui
                yarn 
                yarn start 

# open another terminal to run substrate node on local 
use command:
   cd substrate-contracts-node/target/release 
   cargo build --release       
   ./substrate-contracts-node 
   
# test 
use command: cargo test

# deploy the smart contract
use command: cargo contract build

