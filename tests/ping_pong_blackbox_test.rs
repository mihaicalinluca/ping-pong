use multiversx_sc_scenario::imports::*;

use ping_pong::*;
mod ping_pong_proxy;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const SECOND_USER: TestAddress = TestAddress::new("second-user");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("ping-pong");
const CODE_PATH: MxscPath = MxscPath::new("output/ping-pong.mxsc.json");
const MAX_PING_AMOUNT: u64 = 1_000;
const DEADLINE: u64 = 10_000;
const ACTIVATION_TIMESTAMP: u64 = 100;
const MAX_FUNDS: u64 = 500;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.account(OWNER_ADDRESS).balance(100);
    blockchain.account(SECOND_USER).balance(1000);

    blockchain.register_contract(CODE_PATH, ping_pong::ContractBuilder);
    blockchain
}

#[test]
fn ping_pong_blackbox() {
    let mut world = world();

    world.start_trace();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(ping_pong_proxy::PingPongProxy)
        .init(
            BigUint::from(MAX_PING_AMOUNT),
            DEADLINE,
            ACTIVATION_TIMESTAMP,
            BigUint::from(MAX_FUNDS),
            false,
        )
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world.write_scenario_trace("scenarios/trace1.scen.json");
}

#[test]
fn ping_pong_blackbox_timestamp() {
    let mut world = world();

    world.start_trace();

    world.current_block().block_timestamp(10);

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(ping_pong_proxy::PingPongProxy)
        .init(
            BigUint::from(MAX_PING_AMOUNT),
            DEADLINE,
            ACTIVATION_TIMESTAMP,
            BigUint::from(MAX_FUNDS),
            true,
        )
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(SECOND_USER)
        .to(SC_ADDRESS)
        .typed(ping_pong_proxy::PingPongProxy)
        .ping()
        .egld(BigUint::from(10u64))
        .returns(ExpectError(4u64, "smart contract not active yet"))
        .run();

    world.current_block().block_timestamp(101);

    world
        .tx()
        .from(SECOND_USER)
        .to(SC_ADDRESS)
        .typed(ping_pong_proxy::PingPongProxy)
        .ping()
        .egld(BigUint::from(10u64))
        .returns(ExpectError(0u64, ""))
        .run();

    let users = world
        .query()
        .to(SC_ADDRESS)
        .typed(ping_pong_proxy::PingPongProxy)
        .all_users()
        .returns(ReturnsResult)
        .run();
    let vec_of_users = users.to_vec();
    assert!(vec_of_users.contains(&SECOND_USER.to_managed_address()));

    let ping_amount = world
        .query()
        .to(SC_ADDRESS)
        .typed(ping_pong_proxy::PingPongProxy)
        .ping_amount(&SECOND_USER.to_managed_address())
        .returns(ReturnsResult)
        .run();

    assert_eq!(ping_amount, BigUint::from(10u64));

    world.write_scenario_trace("scenarios/trace2.scen.json");
}
