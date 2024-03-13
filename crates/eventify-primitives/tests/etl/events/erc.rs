use std::str::FromStr;

use alloy_primitives::{bytes, Bytes, FixedBytes, U256};

use crate::utils::{setup_test_db, setup_test_redis, teardown_test_db};
use eventify_primitives::{
    events::{ERC1155, ERC20, ERC4626, ERC721, ERC777},
    networks::NetworkKind,
    EmitT, InsertT, LogT,
};

#[tokio::test]
async fn test_insert_and_emit_erc20_transfer() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
        "from": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "to": "0xa9d1e08c7793af67e9d92fe308d5697fb81d3e43",
        "value": "0x90e2863d00000000000000000000000000000000000000000000000000000000"
    });

    let transfer = serde_json::from_value::<ERC20::Transfer>(json).unwrap();
    transfer
        .insert(
            &pool,
            &Some(
                FixedBytes::from_str(
                    "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    transfer.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc20_approval() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
      "from": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
      "owner": "0xa9d1e08c7793af67e9d92fe308d5697fb81d3e43",
      "spender": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
      "value": "0x90e2863d"
    });

    let approve = serde_json::from_value::<ERC20::Approval>(json).unwrap();
    approve
        .insert(
            &pool,
            &Some(
                FixedBytes::from_str(
                    "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    approve.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc721_transfer() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
        "from": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "to": "0xa9d1e08c7793af67e9d92fe308d5697fb81d3e43",
        "tokenId": "0x90e2863d"
    });

    let transfer = serde_json::from_value::<ERC721::Transfer>(json).unwrap();
    transfer
        .insert(
            &pool,
            &Some(
                FixedBytes::from_str(
                    "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    transfer.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc721_approval() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
        "owner": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "approved": "0xa9d1e08c7793af67e9d92fe308d5697fb81d3e43",
        "tokenId": "0x90e2863d"
    });

    let approval = serde_json::from_value::<ERC721::Approval>(json).unwrap();
    approval
        .insert(
            &pool,
            &Some(
                FixedBytes::from_str(
                    "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    approval.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc721_approval_for_all() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
        "owner": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "operator": "0xa9d1e08c7793af67e9d92fe308d5697fb81d3e43",
        "approved": true
    });

    let approval = serde_json::from_value::<ERC721::ApprovalForAll>(json).unwrap();
    approval
        .insert(
            &pool,
            &Some(
                FixedBytes::from_str(
                    "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    approval.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc777_sent() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
      "operator": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "from": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "to": "0xa9d1e08c7793af67e9d92fe308d5697fb81d3e43",
        "amount": "0x90e2863d",
        "data": [11],
        "operatorData": [0]
    });

    let sent = serde_json::from_value::<ERC777::Sent>(json).unwrap();
    sent.insert(
        &pool,
        &Some(
            FixedBytes::from_str(
                "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
            )
            .unwrap(),
        ),
    )
    .await
    .unwrap();
    sent.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc777_minted() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
      "operator": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "to": "0xa9d1e08c7793af67e9d92fe308d5697fb81d3e43",
        "amount": "0x90e2863d",
        "data": [11],
        "operatorData": [0]
    });

    let minted = serde_json::from_value::<ERC777::Minted>(json).unwrap();
    minted
        .insert(
            &pool,
            &Some(
                FixedBytes::from_str(
                    "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    minted.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc777_burned() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
      "operator": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "from": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "amount": "0x90e2863d",
        "data": [11],
        "operatorData": [0]
    });

    let burned = serde_json::from_value::<ERC777::Burned>(json).unwrap();
    burned
        .insert(
            &pool,
            &Some(
                FixedBytes::from_str(
                    "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    burned.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc777_authorized_operator() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
      "operator": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "holder": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
    });

    let authorized = serde_json::from_value::<ERC777::AuthorizedOperator>(json).unwrap();
    authorized
        .insert(
            &pool,
            &Some(
                FixedBytes::from_str(
                    "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    authorized
        .emit(&redis, &NetworkKind::Ethereum)
        .await
        .unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc777_revoked_operator() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
      "operator": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "holder": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
    });

    let revoked = serde_json::from_value::<ERC777::RevokedOperator>(json).unwrap();
    revoked
        .insert(
            &pool,
            &Some(
                FixedBytes::from_str(
                    "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    revoked.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc1155_transfer_single() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
        "operator": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "from": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "to": "0xa9d1e08c7793af67e9d92fe308d5697fb81d3e43",
        "id": "0x90e2863d",
        "value": "0x90e2863d",
        "data": [11]
    });

    let transfer = serde_json::from_value::<ERC1155::TransferSingle>(json).unwrap();
    transfer
        .insert(
            &pool,
            &Some(
                FixedBytes::from_str(
                    "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    transfer.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc1155_transfer_batch() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
        "operator": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "from": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "to": "0xa9d1e08c7793af67e9d92fe308d5697fb81d3e43",
        "ids": ["0x90e2863d", "0x90e2863d"],
        "values": ["0x90e2863d", "0x90e2863d"],
        "data": [11]
    });

    let transfer = serde_json::from_value::<ERC1155::TransferBatch>(json).unwrap();
    transfer
        .insert(
            &pool,
            &Some(
                FixedBytes::from_str(
                    "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    transfer.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc1155_uri() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
        "value": "h]ttps://example.com",
        "id": "0x90e2863d"

    });

    let uri = serde_json::from_value::<ERC1155::URI>(json).unwrap();
    uri.insert(
        &pool,
        &Some(
            FixedBytes::from_str(
                "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
            )
            .unwrap(),
        ),
    )
    .await
    .unwrap();
    uri.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc4626_deposit() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
        "sender": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "owner": "0xa9d1e08c7793af67e9d92fe308d5697fb81d3e43",
        "assets": "0x90e2863d",
        "shares": "0x90e2863d",
    });

    let deposit = serde_json::from_value::<ERC4626::Deposit>(json).unwrap();
    deposit
        .insert(
            &pool,
            &Some(
                FixedBytes::from_str(
                    "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    deposit.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_erc4626_withdrawal() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
        "sender": "0x5d590653fe409b66d1bfaf467e5e7a6a11671141",
        "receiver": "0xa9d1e08c7793af67e9d92fe308d5697fb81d3e43",
        "owner": "0xa9d1e08c7793af67e9d92fe308d5697fb81d3e55",
        "assets": "0x90e2863d",
        "shares": "0x90e2863d",
    });

    let withdrawal = serde_json::from_value::<ERC4626::Withdraw>(json).unwrap();
    withdrawal
        .insert(
            &pool,
            &Some(
                FixedBytes::from_str(
                    "0x4f9187dc24f121ab3cbde1d98db3d87645f9747b6f6f8cd6f2e1398a4772ab80",
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    withdrawal
        .emit(&redis, &NetworkKind::Ethereum)
        .await
        .unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}
