use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn ping_pong_go() {
    world().run("scenarios/trace1.scen.json");
}

#[test]
fn ping_pong_go2() {
    world().run("scenarios/trace2.scen.json");
}
