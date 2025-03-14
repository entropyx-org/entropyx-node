use crate::imports::*;
use convert_case::{Case, Casing};
use entropyx_consensus_core::subnets::SubnetworkId;
use entropyx_rpc_core::api::ops::RpcApiOps;

#[derive(Default, Handler)]
#[help("Execute RPC commands against the connected EntropyX node")]
pub struct Rpc;

impl Rpc {
    fn println<T>(&self, ctx: &Arc<EntropyXCli>, v: T)
    where
        T: core::fmt::Debug,
    {
        ctx.term().writeln(format!("{v:#?}").crlf());
    }

    async fn main(self: Arc<Self>, ctx: &Arc<dyn Context>, mut argv: Vec<String>, cmd: &str) -> Result<()> {
        let ctx = ctx.clone().downcast_arc::<EntropyXCli>()?;
        let rpc = ctx.wallet().rpc_api().clone();
        // tprintln!(ctx, "{response}");

        if argv.is_empty() {
            return self.display_help(ctx, argv).await;
        }

        let op_str = argv.remove(0);

        let sanitize = regex::Regex::new(r"\s*rpc\s+\S+\s+").unwrap();
        let _args = sanitize.replace(cmd, "").trim().to_string();
        let op_str_uc = op_str.to_case(Case::UpperCamel).to_string();
        // tprintln!(ctx, "uc: '{op_str_uc}'");

        let op = RpcApiOps::from_str(op_str_uc.as_str()).ok_or(Error::custom(format!("No such rpc method: '{op_str}'")))?;

        match op {
            RpcApiOps::Ping => {
                rpc.ping().await?;
                tprintln!(ctx, "ok");
            }
            RpcApiOps::GetMetrics => {
                let result = rpc.get_metrics(true, true, true, true, true, true).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetSystemInfo => {
                let result = rpc.get_system_info().await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetConnections => {
                let result = rpc.get_connections(true).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetServerInfo => {
                let result = rpc.get_server_info_call(None, GetServerInfoRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetSyncStatus => {
                let result = rpc.get_sync_status_call(None, GetSyncStatusRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetCurrentNetwork => {
                let result = rpc.get_current_network_call(None, GetCurrentNetworkRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::SubmitBlock => {
                if argv.is_empty() {
                    return Err(Error::custom("Usage: rpc submitblock <block_hex> [allow_non_daa_blocks]"));
                }
                
                // Parse block from hex string
                let block_hex = argv.remove(0);
                let block_bytes = hex::decode(&block_hex)
                    .map_err(|_| Error::custom("block must be a valid hex string"))?;
                let block: RpcRawBlock = serde_json::from_slice(&block_bytes)
                    .map_err(|_| Error::custom("failed to deserialize block data"))?;
                
                // Parse optional allow_non_daa_blocks parameter, defaults to false
                let allow_non_daa_blocks = if !argv.is_empty() {
                    argv.remove(0).parse::<bool>()
                        .map_err(|_| Error::custom("allow_non_daa_blocks must be 'true' or 'false'"))?
                } else {
                    false
                };
                
                let result = rpc.submit_block_call(None, SubmitBlockRequest { block, allow_non_daa_blocks }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetBlockTemplate => {
                if argv.is_empty() {
                    return Err(Error::custom("Usage: rpc getblocktemplate <pay_address> [extra_data]"));
                }
                
                // Parse pay address from first argument
                let pay_address = RpcAddress::try_from(argv.remove(0).as_str())?;
                
                // Parse optional extra data from second argument, or use empty vec if not provided
                let extra_data = if !argv.is_empty() {
                    // Try to parse as hex string
                    let hex_str = argv.remove(0);
                    RpcExtraData::from_hex(&hex_str).map_err(|_| Error::custom("extra_data must be a valid hex string"))?
                } else {
                    vec![] // Empty extra data if not provided
                };
                let result = rpc.get_block_template_call(None, GetBlockTemplateRequest { pay_address, extra_data }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetPeerAddresses => {
                let result = rpc.get_peer_addresses_call(None, GetPeerAddressesRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetSink => {
                let result = rpc.get_sink_call(None, GetSinkRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetMempoolEntry => {
                if argv.is_empty() {
                    return Err(Error::custom("Usage: rpc getmempoolentry <transaction_id>"));
                }
            
                // Parse transaction_id from hex string
                let transaction_id = RpcHash::from_hex(argv.remove(0).as_str())
                    .map_err(|_| Error::custom("transaction_id must be a valid hex string"))?;

                let include_orphan_pool = if !argv.is_empty() {
                    argv.remove(0).parse::<bool>()
                        .map_err(|_| Error::custom("include_orphan_pool must be 'true' or 'false'"))?
                } else {
                    true
                };

                let filter_transaction_pool = if !argv.is_empty() {
                    argv.remove(0).parse::<bool>()
                        .map_err(|_| Error::custom("filter_transaction_pool must be 'true' or 'false'"))?
                } else {
                    true
                };

                let result = rpc.get_mempool_entry_call(None, GetMempoolEntryRequest { transaction_id, include_orphan_pool, filter_transaction_pool }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetMempoolEntries => {
                // TODO
                let result = rpc
                    .get_mempool_entries_call(
                        None,
                        GetMempoolEntriesRequest { include_orphan_pool: true, filter_transaction_pool: true },
                    )
                    .await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetConnectedPeerInfo => {
                let result = rpc.get_connected_peer_info_call(None, GetConnectedPeerInfoRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::AddPeer => {
                if argv.is_empty() {
                    return Err(Error::custom("Usage: rpc addpeer <ip:port> [true|false for 'is_permanent']"));
                }
                let peer_address = argv.remove(0).parse::<RpcContextualPeerAddress>()?;
                let is_permanent = argv.remove(0).parse::<bool>().unwrap_or(false);
                let result = rpc.add_peer_call(None, AddPeerRequest { peer_address, is_permanent }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::SubmitTransaction => {
                if argv.is_empty() {
                    return Err(Error::custom("Usage: rpc submittransaction <transaction_hex> [allow_orphan]"));
                }
                
                // Parse transaction from hex string
                let tx_hex = argv.remove(0);
                let tx_bytes = hex::decode(&tx_hex)
                    .map_err(|_| Error::custom("transaction must be a valid hex string"))?;
                let transaction: RpcTransaction = serde_json::from_slice(&tx_bytes)
                    .map_err(|_| Error::custom("failed to deserialize transaction data"))?;
            
                // Parse optional allow_orphan parameter
                let allow_orphan = if !argv.is_empty() {
                    argv.remove(0).parse::<bool>()
                        .map_err(|_| Error::custom("allow_orphan must be 'true' or 'false'"))?
                } else {
                    false
                };

                let result = rpc.submit_transaction_call(None, SubmitTransactionRequest { transaction, allow_orphan }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetBlock => {
                if argv.is_empty() {
                    return Err(Error::custom("Missing block hash argument"));
                }
                let hash = argv.remove(0);
                let hash = RpcHash::from_hex(hash.as_str())?;
                let include_transactions = argv.first().and_then(|x| x.parse::<bool>().ok()).unwrap_or(true);
                let result = rpc.get_block_call(None, GetBlockRequest { hash, include_transactions }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetSubnetwork => {
                if argv.is_empty() {
                    return Err(Error::custom("Usage: rpc getsubnetwork <subnetwork_id>"));
                }
            
                // Parse subnetwork_id from hex string
                let subnetwork_id = SubnetworkId::from_hex(argv.remove(0).as_str())
                    .map_err(|_| Error::custom("subnetwork_id must be a valid hex string"))?;
            
                let result = rpc.get_subnetwork_call(None, GetSubnetworkRequest { subnetwork_id }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetVirtualChainFromBlock => {
                if argv.is_empty() {
                    return Err(Error::custom("Missing startHash argument"));
                };
                let start_hash = RpcHash::from_hex(argv.remove(0).as_str())?;
                let include_accepted_transaction_ids = argv.first().and_then(|x| x.parse::<bool>().ok()).unwrap_or_default();
                let result = rpc
                    .get_virtual_chain_from_block_call(
                        None,
                        GetVirtualChainFromBlockRequest { start_hash, include_accepted_transaction_ids },
                    )
                    .await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetBlocks => {
                if argv.is_empty() {
                    return Err(Error::custom("Usage: rpc getheaders <start_hash> [limit] [is_ascending]"));
                }
            
                // Parse required start_hash parameter
                let low_hash = Some(RpcHash::from_hex(argv.remove(0).as_str())
                    .map_err(|_| Error::custom("low_hash must be a valid hex string"))?);
            
                // Parse optional include_blocks parameter, default to true
                let include_blocks = if !argv.is_empty() {
                    argv.remove(0).parse::<bool>()
                        .map_err(|_| Error::custom("include_blocks must be 'true' or 'false'"))?
                } else {
                    true
                };
            
                // Convert include_transactions to bool
                let include_transactions = if !argv.is_empty() {
                    argv.remove(0).parse::<bool>()
                        .map_err(|_| Error::custom("include_transactions must be 'true' or 'false'"))?
                } else {
                    true
                };
                let result = rpc.get_blocks_call(None, GetBlocksRequest { low_hash, include_blocks, include_transactions }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetBlockCount => {
                let result = rpc.get_block_count_call(None, GetBlockCountRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetBlockDagInfo => {
                let result = rpc.get_block_dag_info_call(None, GetBlockDagInfoRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::ResolveFinalityConflict => {
                if argv.is_empty() {
                    return Err(Error::custom("Usage: rpc resolvefinalityconflict <finality_block_hash>"));
                }
            
                // Parse finality_block_hash from hex string
                let finality_block_hash = RpcHash::from_hex(argv.remove(0).as_str())
                    .map_err(|_| Error::custom("finality_block_hash must be a valid hex string"))?;
                
                let result = rpc.resolve_finality_conflict_call(None, ResolveFinalityConflictRequest { finality_block_hash }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::Shutdown => {
                let result = rpc.shutdown_call(None, ShutdownRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetHeaders => {
                if argv.is_empty() {
                    return Err(Error::custom("Usage: rpc getheaders <start_hash> [limit] [is_ascending]"));
                }
            
                // Parse required start_hash parameter
                let start_hash = RpcHash::from_hex(argv.remove(0).as_str())
                    .map_err(|_| Error::custom("start_hash must be a valid hex string"))?;
            
                // Parse optional limit parameter, default to 1000
                let limit = if !argv.is_empty() {
                    argv.remove(0).parse::<u64>()
                        .map_err(|_| Error::custom("limit must be a valid number"))?
                } else {
                    1000
                };
            
                // Convert is_ascending to bool
                let is_ascending = if !argv.is_empty() {
                    argv.remove(0).parse::<bool>()
                        .map_err(|_| Error::custom("is_ascending must be 'true' or 'false'"))?
                } else {
                    true
                };

                let result = rpc.get_headers_call(None,GetHeadersRequest { start_hash, limit, is_ascending }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetUtxosByAddresses => {
                if argv.is_empty() {
                    return Err(Error::custom("Please specify at least one address"));
                }
                let addresses = argv.iter().map(|s| Address::try_from(s.as_str())).collect::<std::result::Result<Vec<_>, _>>()?;
                let result = rpc.get_utxos_by_addresses_call(None, GetUtxosByAddressesRequest { addresses }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetBalanceByAddress => {
                if argv.is_empty() {
                    return Err(Error::custom("Please specify at least one address"));
                }
                let addresses = argv.iter().map(|s| Address::try_from(s.as_str())).collect::<std::result::Result<Vec<_>, _>>()?;
                for address in addresses {
                    let result = rpc.get_balance_by_address_call(None, GetBalanceByAddressRequest { address }).await?;
                    self.println(&ctx, sompi_to_entropyx(result.balance));
                }
            }
            RpcApiOps::GetBalancesByAddresses => {
                if argv.is_empty() {
                    return Err(Error::custom("Please specify at least one address"));
                }
                let addresses = argv.iter().map(|s| Address::try_from(s.as_str())).collect::<std::result::Result<Vec<_>, _>>()?;
                let result = rpc.get_balances_by_addresses_call(None, GetBalancesByAddressesRequest { addresses }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetSinkBlueScore => {
                let result = rpc.get_sink_blue_score_call(None, GetSinkBlueScoreRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::Ban => {
                if argv.is_empty() {
                    return Err(Error::custom("Please specify peer IP address"));
                }
                let ip: RpcIpAddress = argv.remove(0).parse()?;
                let result = rpc.ban_call(None, BanRequest { ip }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::Unban => {
                if argv.is_empty() {
                    return Err(Error::custom("Please specify peer IP address"));
                }
                let ip: RpcIpAddress = argv.remove(0).parse()?;
                let result = rpc.unban_call(None, UnbanRequest { ip }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetInfo => {
                let result = rpc.get_info_call(None, GetInfoRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::EstimateNetworkHashesPerSecond => {
                let window_size = if !argv.is_empty() {
                    argv.remove(0).parse::<u32>()
                        .map_err(|_| Error::custom("window_size must be a valid number"))?
                } else {
                    10
                };
            
                // convert start_hash to RpcHash
                let start_hash = if !argv.is_empty() {
                    Some(RpcHash::from_hex(argv.remove(0).as_str())
                        .map_err(|_| Error::custom("start_hash must be a valid hex string"))?)
                } else {
                    None
                };
                let result = rpc.estimate_network_hashes_per_second_call(None, EstimateNetworkHashesPerSecondRequest { window_size, start_hash }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetMempoolEntriesByAddresses => {
                if argv.is_empty() {
                    return Err(Error::custom("Please specify at least one address"));
                }
                let addresses = argv.iter().map(|s| Address::try_from(s.as_str())).collect::<std::result::Result<Vec<_>, _>>()?;
                let include_orphan_pool = true;
                let filter_transaction_pool = true;
                let result = rpc
                    .get_mempool_entries_by_addresses_call(
                        None,
                        GetMempoolEntriesByAddressesRequest { addresses, include_orphan_pool, filter_transaction_pool },
                    )
                    .await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetCoinSupply => {
                let result = rpc.get_coin_supply_call(None, GetCoinSupplyRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetDaaScoreTimestampEstimate => {
                if argv.is_empty() {
                    return Err(Error::custom("Please specify a daa_score"));
                }
                let daa_score_result = argv.iter().map(|s| s.parse::<u64>()).collect::<std::result::Result<Vec<_>, _>>();

                match daa_score_result {
                    Ok(daa_scores) => {
                        let result = rpc
                            .get_daa_score_timestamp_estimate_call(None, GetDaaScoreTimestampEstimateRequest { daa_scores })
                            .await?;
                        self.println(&ctx, result);
                    }
                    Err(_err) => {
                        return Err(Error::custom("Could not parse daa_scores to u64"));
                    }
                }
            }
            RpcApiOps::GetFeeEstimate => {
                let result = rpc.get_fee_estimate_call(None, GetFeeEstimateRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetFeeEstimateExperimental => {
                let verbose = if argv.is_empty() { false } else { argv.remove(0).parse().unwrap_or(false) };
                let result = rpc.get_fee_estimate_experimental_call(None, GetFeeEstimateExperimentalRequest { verbose }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetCurrentBlockColor => {
                if argv.is_empty() {
                    return Err(Error::custom("Missing block hash argument"));
                }
                let hash = argv.remove(0);
                let hash = RpcHash::from_hex(hash.as_str())?;
                let result = rpc.get_current_block_color_call(None, GetCurrentBlockColorRequest { hash }).await?;
                self.println(&ctx, result);
            }
            // ENX-CHANGE-BURN:
            RpcApiOps::GetTotalBurnedAmount => {
                let result = rpc.get_total_burned_amount_call(None, GetTotalBurnedAmountRequest {}).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetBurnRecordsByRange => {
                if argv.is_empty() {
                    return Err(Error::custom("Expected start and end range"));
                }
                let start_daa = argv[0].parse()?;
                let end_daa = argv[1].parse()?;
                let result = rpc.get_burn_records_by_range_call(None, GetBurnRecordsByRangeRequest { start_daa, end_daa }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetBurnRecordByUtxoId => {
                if argv.is_empty() {
                    return Err(Error::custom("Expected UTXO ID"));
                }
                let utxo_id = argv[0].clone();
                let result = rpc.get_burn_record_by_utxo_id_call(None, GetBurnRecordByUtxoIdRequest { utxo_id }).await?;
                self.println(&ctx, result);
            }
            RpcApiOps::GetCurrentBurnFee => {
                let current_block_dag_info_count = rpc.get_block_dag_info_call(None, GetBlockDagInfoRequest {}).await?;
                let result = rpc.get_current_burn_fee_call(None, GetCurrentBurnFeeRequest { current_daa_score: current_block_dag_info_count.virtual_daa_score }).await?;
                self.println(&ctx, result);
            }
            _ => {
                tprintln!(ctx, "rpc method exists but is not supported by the cli: '{op_str}'\r\n");
                return Ok(());
            }
        }

        let prefix = Regex::new(r"(?i)^\s*rpc\s+\S+\s+").unwrap();
        let _req = prefix.replace(cmd, "").trim().to_string();

        Ok(())
    }

    async fn display_help(self: Arc<Self>, ctx: Arc<EntropyXCli>, _argv: Vec<String>) -> Result<()> {
        // RpcApiOps that do not contain docs are not displayed
        let help = RpcApiOps::into_iter()
            .filter_map(|op| op.rustdoc().is_not_empty().then_some((op.as_str().to_case(Case::Kebab).to_string(), op.rustdoc())))
            .collect::<Vec<(_, _)>>();

        ctx.term().help(&help, None)?;

        tprintln!(ctx);
        tprintln!(ctx, "Please note that not all listed RPC methods are currently implemented");
        tprintln!(ctx);

        Ok(())
    }
}
