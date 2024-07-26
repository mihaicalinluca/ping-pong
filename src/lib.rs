#![no_std]

use multiversx_sc::imports::*;

mod events;
mod owner;
mod storage;

/// An ping-pong contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait PingPong: owner::OwnerModule + storage::StorageModule + events::EventsModule {
    #[init]
    fn init(
        &self,
        max_ping_amount: BigUint,
        deadline: u64,
        activation_timestamp: u64,
        max_funds: BigUint,
        is_open: bool,
    ) {
        self.max_ping_amount().set(max_ping_amount);
        self.deadline().set(deadline);
        self.activation_timestamp().set(activation_timestamp);
        self.max_funds().set(max_funds);
        self.is_open().set(is_open);
    }

    #[upgrade]
    fn upgrade(
        &self,
        max_ping_amount: BigUint,
        deadline: u64,
        activation_timestamp: u64,
        max_funds: BigUint,
        is_open: bool,
    ) {
        self.init(
            max_ping_amount,
            deadline,
            activation_timestamp,
            max_funds,
            is_open,
        )
    }

    #[payable("EGLD")] 
    #[endpoint]
    fn ping(&self) {
        require!(self.is_open().get(), "maintenance");

        let egld_amount = self.call_value().egld_value().clone_value();

        require!(
            &egld_amount <= &self.max_ping_amount().get(),
            "the egld_amount must be lower or equal to the max ping amount"
        );

        require!(
            &self
                .blockchain()
                .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0)
                + &egld_amount
                <= self.max_funds().get(),
            "contract is full"
        );

        let block_timestamp = self.blockchain().get_block_timestamp();
        require!(
            self.activation_timestamp().get() <= block_timestamp,
            "smart contract not active yet"
        );

        require!(
            block_timestamp <= self.deadline().get(),
            "cannot ping anymore, deadline has passed"
        );

        let caller = self.blockchain().get_caller();
        self.emit_ping_event(&caller, block_timestamp, &egld_amount);
        self.ping_amount(&caller).set(egld_amount);

        // here in order to avoid a clone for caller, all previous actions will be reverted if failed
        require!(self.all_users().insert(caller), "caller already pinged");
    }

    #[endpoint]
    fn pong(&self) {
        require!(self.is_open().get(), "maintenance");

        let block_timestamp = self.blockchain().get_block_timestamp();

        require!(
            block_timestamp > self.deadline().get(),
            "cannot pong yet, deadline has not passed"
        );

        let caller = self.blockchain().get_caller();
        require!(
            self.all_users().swap_remove(&caller),
            "caller has not pinged"
        );

        let ping_amount = self.ping_amount(&caller).get();
        self.tx().to(&caller).egld(&ping_amount).transfer();

        self.emit_pong_event(&caller, block_timestamp, &ping_amount);
    }
}
