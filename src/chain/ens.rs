use alloy::primitives::{address, Address, Bytes, FixedBytes};
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::rpc::types::TransactionRequest;
use alloy::sol_types::SolValue;

/// ENS Registry 컨트랙트 주소
const ENS_REGISTRY: Address = address!("00000000000C2E074eC69A0dFb2997BA6C7d2e1e");

/// 주소가 ENS 이름인지 확인 (.eth로 끝나는지)
pub fn is_ens_name(input: &str) -> bool {
    input.ends_with(".eth")
}

/// ENS 이름을 namehash로 변환
fn namehash(name: &str) -> FixedBytes<32> {
    let mut node = [0u8; 32];
    for label in name.rsplit('.') {
        let label_hash = alloy::primitives::keccak256(label.as_bytes());
        let mut combined = [0u8; 64];
        combined[..32].copy_from_slice(&node);
        combined[32..].copy_from_slice(label_hash.as_slice());
        node = *alloy::primitives::keccak256(&combined);
    }
    FixedBytes::from(node)
}

/// ENS 이름을 주소로 해석 (Ethereum Mainnet)
pub async fn resolve_ens(name: &str) -> Option<String> {
    if !is_ens_name(name) {
        return None;
    }

    let ws = WsConnect::new("wss://eth.drpc.org");
    let provider = ProviderBuilder::new().connect_ws(ws).await.ok()?;

    let node = namehash(name);

    // 1. ENS Registry에서 resolver 주소 조회
    // resolver(bytes32 node) → address
    let mut calldata = vec![0x01, 0x78, 0xb8, 0xbf]; // resolver(bytes32)
    calldata.extend_from_slice(node.abi_encode().as_slice());

    let tx = TransactionRequest::default()
        .to(ENS_REGISTRY)
        .input(calldata.into());

    let result: Bytes = provider.call(tx).await.ok()?;
    if result.len() < 32 {
        return None;
    }

    let resolver_addr = Address::from_slice(&result[12..32]);
    if resolver_addr.is_zero() {
        return None;
    }

    // 2. Resolver에서 addr(bytes32 node) 호출
    let mut calldata2 = vec![0x3b, 0x3b, 0x57, 0xde]; // addr(bytes32)
    calldata2.extend_from_slice(node.abi_encode().as_slice());

    let tx2 = TransactionRequest::default()
        .to(resolver_addr)
        .input(calldata2.into());

    let result2: Bytes = provider.call(tx2).await.ok()?;
    if result2.len() < 32 {
        return None;
    }

    let addr = Address::from_slice(&result2[12..32]);
    if addr.is_zero() {
        return None;
    }

    Some(format!("{:#x}", addr))
}
