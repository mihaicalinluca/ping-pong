use imports::{MxscPath, TestAddress};
use multiversx_sc_scenario::*;
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const CODE_PATH: MxscPath = MxscPath::new("ping-pong.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // blockchain.account(OWNER_ADDRESS);
    blockchain.register_contract(
        "mxsc::output/ping-pong.mxsc.json",
        ping_pong::ContractBuilder,
    );
    blockchain
}

#[test]
fn ping_pong_rs() {
    let mut world = world();
    world.account(OWNER_ADDRESS);

    world.run("scenarios/trace1.scen.json");
}
