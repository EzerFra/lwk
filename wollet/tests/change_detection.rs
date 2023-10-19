use std::{str::FromStr, time::Duration};

use elements::bitcoin::bip32::DerivationPath;
use jade::{mutex_jade::MutexJade, protocol::JadeState, serialport, Jade};
use signer::Signer;

use crate::test_session::{generate_slip77, setup, TestWollet};

#[test]
#[ignore = "requires hardware jade: initialized with localtest network, connected via usb/serial"]
fn jade_send_lbtc_detect_change() {
    let network = jade::Network::LocaltestLiquid;

    let ports = serialport::available_ports().unwrap();
    assert!(!ports.is_empty());
    let path = &ports[0].port_name;
    let port = serialport::new(path, 115_200)
        .timeout(Duration::from_secs(60))
        .open()
        .unwrap();

    let jade = Jade::new(port.into(), network);
    let mut jade = MutexJade::new(jade);

    let mut jade_state = jade.get_mut().unwrap().version_info().unwrap().jade_state;
    assert_ne!(jade_state, JadeState::Uninit);
    assert_ne!(jade_state, JadeState::Unsaved);
    if jade_state == JadeState::Locked {
        jade.unlock().unwrap();
        jade_state = jade.get_mut().unwrap().version_info().unwrap().jade_state;
    }
    assert_eq!(jade_state, JadeState::Ready);
    let signers = [&Signer::Jade(&jade)];

    send_lbtc_detect_change(&signers);

    // refuse the tx on the jade to keep the session logged
    jade.get_mut().unwrap().logout().unwrap();
}

fn send_lbtc_detect_change(signers: &[&Signer]) {
    let path = "84h/1h/0h";
    let master_node = signers[0].xpub().unwrap();
    let fingerprint = master_node.fingerprint();
    let xpub = signers[0]
        .derive_xpub(&DerivationPath::from_str(&format!("m/{path}")).unwrap())
        .unwrap();

    let slip77_key = generate_slip77();

    // m / purpose' / coin_type' / account' / change / address_index
    let desc_str = format!("ct(slip77({slip77_key}),elwpkh([{fingerprint}/{path}]{xpub}/1/*))");

    let server = setup();

    let mut wallet = TestWollet::new(&server.electrs.electrum_url, &desc_str);

    wallet.fund_btc(&server);

    let node_address = server.node_getnewaddress();
    wallet.send_btc(&signers, None, Some((node_address, 10_000)));
}
