use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(isOpen)]
    #[storage_mapper("isOpen")]
    fn is_open(&self) -> SingleValueMapper<bool>;

    #[view(getMaxPingAmount)]
    #[storage_mapper("maxPingAmount")]
    fn max_ping_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getPingAmount)]
    #[storage_mapper("pingAmount")]
    fn ping_amount(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getDeadline)]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<u64>;

    #[view(getActivationTimestamp)]
    #[storage_mapper("activationTimestamp")]
    fn activation_timestamp(&self) -> SingleValueMapper<u64>;

    #[view(getMaxFunds)]
    #[storage_mapper("maxFunds")]
    fn max_funds(&self) -> SingleValueMapper<BigUint>;

    #[view(getAllUsers)]
    #[storage_mapper("allUsers")]
    fn all_users(&self) -> UnorderedSetMapper<ManagedAddress>;
}
