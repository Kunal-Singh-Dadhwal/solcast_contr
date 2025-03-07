use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use crate::instructions::*;

declare_id!("HH4iTFM56nf5zKPw3LrPZm5EJriyxYwEKMc3eacfqcJr");

#[program]
pub mod solcast_contr {
    use super::*;

    pub fn initialize(ctx: Context<InitializeProtocol>) -> Result<()> {
        instructions::initialize_protocol::handler(ctx)
    }

    pub fn subscribe(ctx: Context<Subscribe>, how_many_cycles: i64) -> Result<()> {
        instructions::subscribe::handler(ctx, how_many_cycles)
    }

    pub fn unsubscribe(ctx: Context<Unsubscribe>) -> Result<()> {
        instructions::unsubscribe::handler(ctx)
    }

    pub fn create_subscription_plan(
        ctx: Context<CreateSubscriptionPlan>,
        plan_name: String,
        subscription_amount: i64,
        frequency: i64,
        fee_percentage: i8,
    ) -> Result<()> {
        instructions::create_subscription_plan::handler(
            ctx,
            plan_name,
            subscription_amount,
            frequency,
            fee_percentage,
        )
    }

    pub fn close_subscription_plan(ctx: Context<CloseSubscriptionPlan>) -> Result<()> {
        instructions::close_subscription_plan::handler(ctx)
    }

    pub fn trigger_payment(ctx: Context<TriggerPayment>) -> Result<()> {
        instructions::trigger_payment::handler(ctx)
    }

    pub fn register_node(ctx: Context<RegisterNode>) -> Result<()> {
        instructions::register_node::handler(ctx)
    }
}
