#[macro_use]
extern crate dotenv_codegen;

pub mod fiat;
pub mod shared;

use futures::executor;

use cosmrs::{
    bip32::secp256k1::sha2::Sha256,
    cosmwasm::MsgExecuteContract,
    proto::cosmos::tx::v1beta1::Tx as ProtoTx,
    rpc::{Client, HttpClient},
    tendermint::{block::Height, hash::Algorithm, Hash},
    tx::{MessageExt, Msg, Raw},
    Tx,
};
use dotenv::dotenv;
use localmoney_protocol::trade::ExecuteMsg as TradeExecuteMsg;
use sha256::digest as sha256_digest;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let rpc_url = dotenv!("RPC");
    let client = HttpClient::new(rpc_url).unwrap();

    // let start_block = 7470327;
    let start_block = 8578393;
    let status = client.status().await.unwrap();
    // get the latest block height and compare it to the start block, see how many blocks we need to process
    let latest_block_height = status.sync_info.latest_block_height.value();
    let block_difference = latest_block_height - start_block;
    // Fetch the blocks and process them
    for i in 0..block_difference {
        // Fetch the block
        let block = client
            .block(Height::try_from(start_block + i).unwrap())
            .await
            .unwrap();
        println!("Processing block: {:?}", block.block.header.height);
        // Process the block and extract the transactions
        // Print block data (i.e. transactions)
        block.block.data().iter().for_each(|tx| {
            let parsed_tx = Tx::from_bytes(tx).unwrap();
            // Print the execution log of the parsed_tx
            parsed_tx.body.messages.iter().for_each(|msg| {
                if msg.type_url == "/cosmwasm.wasm.v1.MsgExecuteContract" {
                    let msg: MsgExecuteContract = Msg::from_any(msg).unwrap();
                    if msg.contract.to_string().eq(&String::from(
                        "kujira1hdydzhe7dfhw2vmfsrzu6dcw2aeuff4ja8wr5puqhsg8rlf44gvq6car5s",
                    )) {
                        // Parse the msg.msg bytes into a JSON String.
                        let msg_json = String::from_utf8(msg.msg.clone()).unwrap();
                        // print!("Message: {:?}", msg_json);
                        let trade_msg: TradeExecuteMsg = serde_json::from_str(&msg_json).unwrap();
                        println!("Trade Message: {:?}", trade_msg);
                        // Check the TradeExecuteMsg enum variant and print the relevant data
                        match trade_msg {
                            TradeExecuteMsg::Create(create_msg) => {
                                println!("Create Message: {:?}", create_msg);
                                executor::block_on(async {
                                    let hash = sha256_digest(tx.as_slice());
                                    let hash = Hash::from_hex_upper(Algorithm::Sha256, hash.to_uppercase().as_str()).unwrap();
                                    let tx_response = client.tx(hash, false).await.unwrap();
                                    tx_response.tx_result.events.iter().for_each(|event| {
                                        event.attributes.iter().for_each(|attribute| {
                                            if attribute.key.eq(&String::from("trade_id")) {
                                                println!("TradeID: {:?}", attribute.value);
                                            }
                                        });
                                    });
                                });
                            }
                            TradeExecuteMsg::AcceptRequest {
                                trade_id,
                                maker_contact,
                            } => {
                                println!("Accept Request Message: {:?}", trade_id);
                            }
                            TradeExecuteMsg::FundEscrow {
                                trade_id,
                                maker_contact,
                            } => {
                                println!("Fund Escrow Message: {:?}", trade_id);
                            }
                            TradeExecuteMsg::RefundEscrow { trade_id } => {
                                println!("Refund Escrow Message: {:?}", trade_id);
                            }
                            TradeExecuteMsg::ReleaseEscrow { trade_id } => {
                                println!("Release Escrow Message: {:?}", trade_id);
                            }
                            TradeExecuteMsg::DisputeEscrow {
                                trade_id,
                                buyer_contact,
                                seller_contact,
                            } => {
                                println!("Dispute Escrow Message: {:?}", trade_id);
                            }
                            TradeExecuteMsg::FiatDeposited { trade_id } => {
                                println!("Fiat Deposited Message: {:?}", trade_id);
                            }
                            TradeExecuteMsg::CancelRequest { trade_id } => {
                                println!("Cancel Request Message: {:?}", trade_id);
                            }
                            TradeExecuteMsg::NewArbitrator {
                                arbitrator,
                                fiat,
                                encryption_key,
                            } => {
                                println!("New Arbitrator Message: {:?}", arbitrator);
                            }
                            TradeExecuteMsg::DeleteArbitrator { arbitrator, fiat } => {
                                println!("Delete Arbitrator Message: {:?}", arbitrator);
                            }
                            TradeExecuteMsg::SettleDispute { trade_id, winner } => {
                                println!("Settle Dispute Message: {:?}", trade_id);
                            }
                            TradeExecuteMsg::RegisterHub {} => {
                                println!("Register Hub Message");
                            }
                            TradeExecuteMsg::RegisterConversionRouteForDenom { denom, route } => {
                                println!(
                                    "Register Conversion Route For Denom Message: {:?}",
                                    denom
                                );
                            }
                        };
                    }
                }
            });
        });
    }
}

async fn load_trade_id(hash: Hash) {}
