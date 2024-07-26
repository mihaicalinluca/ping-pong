#[multiversx_sc::module]
pub trait EventsModule {
    #[event("pingEvent")]
    fn emit_ping_event(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] timestamp: u64,
        amount: &BigUint,
    );

    #[event("pongEvent")]
    fn emit_pong_event(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] timestamp: u64,
        amount: &BigUint,
    );
}
