use crate::constants::{
    MAXIMUM_NODES, MAXIMUM_SUBSCRIPTIONS_PER_PLAN, MAXIMUM_SUBSCRIPTIONS_PER_USER,
    MAXIMUM_SUBSCRIPTION_PLANS, MAXIMUM_SUBSCRIPTION_PLAN_PER_AUTHOR,
};
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Subscriber {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub authority: Pubkey,
    pub subscriber_payment_account: Pubkey,
    #[max_len(MAXIMUM_SUBSCRIPTIONS_PER_USER)]
    pub subscription_accounts: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct Subscription {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub subscriber: Pubkey,
    pub subscription_plan: Pubkey,
    pub is_active: bool,
    pub is_cancelled: bool,
    pub cancellation_reason: i8,

    pub last_payment_timestamp: i64,
    pub next_payment_timestamp: i64,
}

#[account]
#[derive(InitSpace)]
pub struct SubscriptionPlan {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    #[max_len(100)]
    pub plan_name: String,
    pub subscription_plan_author: Pubkey,
    pub subscription_plan_payment_account: Pubkey,
    pub amount: i64,
    pub frequency: i64,
    pub is_active: bool,
    pub fee_percentage: i8,
    #[max_len(MAXIMUM_SUBSCRIPTIONS_PER_PLAN)]
    pub subscription_accounts: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct SubscriptionPlanAuthor {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub authority: Pubkey,
    #[max_len(MAXIMUM_SUBSCRIPTION_PLAN_PER_AUTHOR)]
    pub subscription_plan_accounts: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct Protocol {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub authority: Pubkey,
    #[max_len(MAXIMUM_SUBSCRIPTION_PLANS)]
    pub subscription_plan_accounts: Vec<Pubkey>,
    #[max_len(MAXIMUM_NODES)]
    pub registered_nodes: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct ProtocolSigner {
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Node {
    pub bump: u8,
    pub is_registered: bool,
    pub authority: Pubkey,
    pub node_payment_wallet: Pubkey,
    pub node_payment_account: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct solcast_contr {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub authority: Pubkey,
    #[max_len(MAX)]
    pub creator_accounts: Vec<Pubkey>,
}
impl Grantive {
    pub fn space() -> usize {
        8 + // discriminator
        1 + // bump
        1 + // has_already_been_initialized
        PUBKEY_SIZE + // authority
        4 + (PUBKEY_SIZE * MAXIMUM_CREATOR_ACCOUNTS) // creator_accounts
    }
}

#[account]
pub struct Creator {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub authority: Pubkey,
    pub name: String,
    pub data_id: String,
    pub subscription_plan: Pubkey,

    // This contains PubKeys of all the posts the creator has made
    pub posts: Vec<Pubkey>,
    pub last_post_index: i64
}
impl Creator {
    pub fn space(name: &str, data_id: &str) -> usize {
        8 + // discriminator
        1 + // bump
        1 + // has_already_been_initialized
        PUBKEY_SIZE + // authority
        4 + name.len() + //name
        4 + data_id.len() + //data_id
        PUBKEY_SIZE + // subscription_plan
        4 + (PUBKEY_SIZE * MAXIMUM_POSTS_PER_CREATOR) + // posts
        8 // last_post_index
    }
}

#[account]
pub struct CreatorPost {
    pub bump: u8,
    pub index: i64,
    pub has_already_been_initialized: bool,
    pub creator: Pubkey,
    pub title: String,
    pub content_data: String,
    pub published_on: i64,

    // if the post is only accessible by subscriber
    pub subscriber_only: bool,
}
impl CreatorPost {
    pub fn space(title: &str, content_data: &str) -> usize {
        8 + // discriminator
        1 + // bump
        8 + // index
        1 + // has_already_been_initialized
        PUBKEY_SIZE + // creator
        4 + title.len() + //title
        4 + content_data.len() + //content_data
        8 + // published_on
        1 // subscriber_only
    }
}