use std::path::PathBuf;

use anchor_client::solana_sdk::pubkey::Pubkey;
use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long, default_value = "~/.config/jet-multisig.toml")]
    pub config: String,

    #[clap(subcommand)]
    pub job: Job,
}

#[derive(Clap)]
pub enum Job {
    New(CreateMultisig),
    AddDelegates(Delegates),
    RemoveDelegates(Delegates),
    Approve(Transaction),
    Execute(Transaction),
    Get,
    GetTransaction(Key),
    InspectProposal(Key),
    ProposeUpgrade(ProposeUpgrade),
    ProposeEdit(Edit),
    ProposeSetMarketFlags(MarketFlags),
    ProposeMintTokens(TokenAction),
    ProposeTransferTokens(TokenAction),
    ProposeCustodyGenerateTokenMint(GenerateTokens),
    ProposeCustodyTransferTokens(TokenAction),
}

#[derive(Clap, Debug)]
pub struct CreateMultisig {
    pub threshold: u64,
    #[clap(required = true)]
    pub owners: Vec<Pubkey>,
}

#[derive(Clap, Debug)]
pub struct Delegates {
    pub delegates: Vec<Pubkey>,
}

#[derive(Clap, Debug)]
pub struct Edit {
    #[clap(long)]
    pub threshold: Option<u64>,
    #[clap(long)]
    pub owners: Option<Vec<Pubkey>>,
}

#[derive(Clap)]
pub struct ProposeUpgrade {
    pub program: Pubkey,
    pub buffer: Pubkey,
}

#[derive(Clap)]
pub struct TokenAction {
    #[clap(long, short)]
    pub source: Pubkey,

    #[clap(long, short)]
    pub target: Pubkey,

    #[clap(long, short)]
    pub amount: u64,
}

#[derive(Clap)]
pub struct Transaction {
    pub transaction: Pubkey,
}

#[derive(Clap)]
pub struct Key {
    pub key: Pubkey,
}

#[derive(Clap)]
pub struct GenerateTokens {
    #[clap(long, short = 'k')]
    pub mint_key: PathBuf,
}

#[derive(Clap)]
pub struct MarketFlags {
    pub market: Pubkey,

    #[clap(long, short = 'b')]
    pub halt_borrows: bool,

    #[clap(long, short = 'r')]
    pub halt_repays: bool,

    #[clap(long, short = 'd')]
    pub halt_deposits: bool,
}
