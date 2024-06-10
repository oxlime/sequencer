use indexmap::indexmap;
use starknet_api::{
    block_hash::block_hash_calculator::TransactionOutputForHash,
    core::{ClassHash, CompiledClassHash, ContractAddress, EthAddress, Nonce, PatriciaKey},
    state::{StorageKey, ThinStateDiff},
    transaction::{
        Event, EventContent, EventData, EventKey, Fee, GasVector, L2ToL1Payload, MessageToL1,
        RevertedTransactionExecutionStatus, TransactionExecutionStatus,
    },
};
use starknet_types_core::felt::Felt;

pub(crate) fn get_transaction_output_for_hash(
    execution_status: &TransactionExecutionStatus,
) -> TransactionOutputForHash {
    let expected_execution_status = match execution_status {
        TransactionExecutionStatus::Succeeded => TransactionExecutionStatus::Succeeded,
        TransactionExecutionStatus::Reverted(_) => {
            TransactionExecutionStatus::Reverted(RevertedTransactionExecutionStatus {
                revert_reason: "reason".to_owned(),
            })
        }
    };
    TransactionOutputForHash {
        actual_fee: Fee(0),
        events: vec![Event {
            from_address: ContractAddress(PatriciaKey::from(2_u128)),
            content: EventContent {
                keys: vec![EventKey(Felt::from_bytes_be_slice(&[1_u8]))],
                data: EventData(vec![Felt::from_bytes_be_slice(&[0_u8])]),
            },
        }],
        execution_status: expected_execution_status,
        gas_consumed: GasVector {
            l1_gas: 0,
            l1_data_gas: 64,
        },
        messages_sent: vec![MessageToL1 {
            from_address: ContractAddress(PatriciaKey::from(2_u128)),
            to_address: EthAddress::try_from(Felt::from_bytes_be_slice(&[1_u8]))
                .expect("to_address"),
            payload: L2ToL1Payload(vec![Felt::from_bytes_be_slice(&[0_u8])]),
        }],
    }
}

pub(crate) fn get_thin_state_diff() -> ThinStateDiff {
    ThinStateDiff {
        deployed_contracts: indexmap! {
            ContractAddress::from(1_u128) => ClassHash(Felt::from_bytes_be_slice(&[2_u8]))
        },
        storage_diffs: indexmap! {
            ContractAddress::from(7_u128) => indexmap! {
                StorageKey::from(8_u128) => Felt::from_bytes_be_slice(&[9_u8]),
            },
        },
        declared_classes: indexmap! {
            ClassHash(Felt::from_bytes_be_slice(&[13_u8])) =>
                CompiledClassHash(Felt::from_bytes_be_slice(&[14_u8]))
        },
        deprecated_declared_classes: vec![
            ClassHash(Felt::from_bytes_be_slice(&[16_u8])),
            ClassHash(Felt::from_bytes_be_slice(&[15_u8])),
        ],
        nonces: indexmap! {
            ContractAddress::from(3_u128) => Nonce(Felt::from_bytes_be_slice(&[4_u8])),
        },
        replaced_classes: indexmap! {},
    }
}
