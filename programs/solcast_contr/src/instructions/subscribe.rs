use std::convert::TryInto;

use crate::{constants::ANCHOR_DISCRIMINATOR_SIZE, error::ErrorCode, state::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint,
    token::{self, Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(mut)]
    pub who_subscribes: Signer<'info>,

    #[account(
        mut,
        seeds = [b"protocol_signer"],
        bump = protocol_signer.bump,
    )]
    pub protocol_signer: Box<Account<'info, ProtocolSigner>>,

    #[account(
        init_if_needed,
        payer = who_subscribes,
        seeds = [b"subscription", subscriber.key().as_ref(), subscription_plan.key().as_ref()],
        bump,
        space= ANCHOR_DISCRIMINATOR_SIZE + Subscription::INIT_SPACE,
    )]
    pub subscription: Box<Account<'info, Subscription>>,

    #[account(
        init_if_needed,
        payer = who_subscribes,
        seeds = [b"state", who_subscribes.key().as_ref()],
        bump,
        space= ANCHOR_DISCRIMINATOR_SIZE + Subscriber::INIT_SPACE,
    )]
    pub subscriber: Box<Account<'info, Subscriber>>,

    #[account(
        init_if_needed,
        payer = who_subscribes,
        associated_token::mint = mint,
        associated_token::authority = who_subscribes,
    )]
    pub subscriber_payment_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = subscription_plan.has_already_been_initialized @ ErrorCode::SubscriptionPlanNotInitialized,
        constraint = subscription_plan.is_active @ ErrorCode::SubscriptionPlanInactive,
        has_one = subscription_plan_payment_account @ErrorCode::SubscriptionPlanInvalidPaymentAccount
    )]
    pub subscription_plan: Box<Account<'info, SubscriptionPlan>>,

    #[account(
        mut,
        constraint = subscription_plan_payment_account.mint ==  mint.key() @ ErrorCode::InvalidMint
    )]
    pub subscription_plan_payment_account: Box<Account<'info, TokenAccount>>,

    #[account(address = mint::USDC @ ErrorCode::InvalidMint)]
    pub mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<Subscribe>, how_many_cycles: i64) -> Result<()> {
    let subscriber = &mut ctx.accounts.subscriber;

    if !subscriber.has_already_been_initialized {
        subscriber.has_already_been_initialized = true;
        subscriber.bump = ctx.bumps.subscriber;
        subscriber.authority = ctx.accounts.who_subscribes.key();
        subscriber.subscriber_payment_account = ctx.accounts.subscriber_payment_account.key();
        subscriber.subscription_accounts = vec![];
    } else {
        require!(
            subscriber.authority.eq(&ctx.accounts.who_subscribes.key()),
            ErrorCode::SubsscriberInvalidStateAccount
        );
        require!(
            subscriber
                .subscriber_payment_account
                .eq(&ctx.accounts.subscriber_payment_account.key()),
            ErrorCode::SubsscriberInvalidStateAccount
        );
    }

    // check if already subscribed
    let subscription = &mut ctx.accounts.subscription;
    let subscriber = &mut ctx.accounts.subscriber;
    let subscription_plan = &mut ctx.accounts.subscription_plan;
    let subscriber_payment_wallet = &mut ctx.accounts.subscriber_payment_account;

    if subscription.has_already_been_initialized {
        // user has already been interracted with this subscription before
        require!(
            subscription.is_active,
            ErrorCode::SubscriptionAlreadySubscribed
        );
    } else {
        subscription.has_already_been_initialized = true;
        subscription.bump = ctx.bumps.subscription;
        subscription.subscriber = subscriber.key();
        subscription.subscription_plan = subscription_plan.key();

        subscriber
            .subscription_accounts
            .push(subscription.key().clone());
        subscription_plan
            .subscription_accounts
            .push(subscription.key().clone());
    }

    // check if the subscriber has enough funds for the first cycle
    let balance_of_user = token::accessor::amount(&subscriber_payment_wallet.to_account_info())?;
    let required_balance = subscription_plan.amount;
    require!(
        balance_of_user >= required_balance.try_into().unwrap(),
        ErrorCode::SubscriptionNotEnoughFunds
    );

    // check for delegation and delegate
    let mut amount_to_delegate: i64 = subscription_plan.amount * how_many_cycles;
    match subscriber_payment_wallet.delegate {
        anchor_lang::solana_program::program_option::COption::None => {}
        anchor_lang::solana_program::program_option::COption::Some(delegated_account) => {
            if delegated_account.eq(&ctx.accounts.protocol_signer.key()) {
                let increment: i64 = subscriber_payment_wallet
                    .delegated_amount
                    .try_into()
                    .unwrap();
                amount_to_delegate = amount_to_delegate + increment;
            }
        }
    }

    anchor_spl::token::approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Approve {
                delegate: ctx.accounts.protocol_signer.to_account_info(),
                to: subscriber_payment_wallet.to_account_info(),
                authority: ctx.accounts.who_subscribes.to_account_info(),
            },
        ),
        amount_to_delegate.try_into().unwrap(),
    )?;

    let bump = vec![ctx.accounts.protocol_signer.bump];
    let inner_seeds = vec![b"protocol_signer".as_ref(), bump.as_ref()];
    let signer_seeds = vec![&inner_seeds[..]];

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx
                    .accounts
                    .subscriber_payment_account
                    .to_account_info()
                    .clone(),
                to: ctx
                    .accounts
                    .subscription_plan_payment_account
                    .to_account_info(),
                authority: ctx.accounts.protocol_signer.to_account_info().clone(),
            },
            &signer_seeds,
        ),
        subscription_plan.amount as u64,
    )?;

    subscription.is_active = true;
    subscription.is_cancelled = false;

    let clock = &ctx.accounts.clock;
    let current_time = clock.unix_timestamp;

    subscription.last_payment_timestamp = current_time;
    subscription.next_payment_timestamp = current_time + subscription_plan.frequency;

    Ok(())
}
