use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait OwnerModule: crate::storage::StorageModule {
    #[only_owner]
    #[endpoint]
    fn open_contract(&self) {
        self.is_open().set(true)
    }

    #[only_owner]
    #[endpoint]
    fn close_contract(&self) {
        self.is_open().clear()
    }

    #[only_owner]
    #[endpoint]
    fn set_deadline_and_activation_timestamp(&self, deadline: u64, activation_timestamp: u64) {
        require!(
            activation_timestamp < deadline,
            "activation timestamp cannot be after deadline"
        );

        self.deadline().set(deadline);
        self.activation_timestamp().set(activation_timestamp);
    }

    #[only_owner]
    #[endpoint]
    fn set_max_ping_amount(&self, max_ping_amount: BigUint) {
        self.max_ping_amount().set(max_ping_amount)
    }

    #[only_owner]
    #[endpoint]
    fn set_max_funds(&self, max_funds: BigUint) {
        self.max_funds().set(max_funds)
    }
}
