use super::{
    DeclareTransaction, DeclareTransactionContent, DeclareTransactionV0,
    DeclareTransactionV0Content, DeclareTransactionV1, DeclareTransactionV1Content,
    DeclareTransactionV2, DeclareTransactionV2Content, DeclareTransactionV3,
    DeclareTransactionV3Content, DeployAccountTransaction, DeployAccountTransactionContent,
    DeployAccountTransactionV1, DeployAccountTransactionV1Content, DeployAccountTransactionV3,
    DeployAccountTransactionV3Content, DeployTransaction, DeployTransactionContent,
    InvokeTransaction, InvokeTransactionContent, InvokeTransactionV0, InvokeTransactionV0Content,
    InvokeTransactionV1, InvokeTransactionV1Content, InvokeTransactionV3,
    InvokeTransactionV3Content, L1HandlerTransaction, L1HandlerTransactionContent,
    LegacyContractAbiEntry, LegacyFunctionAbiType, Transaction, TransactionContent,
    contract::legacy::{
        RawLegacyAbiEntry, RawLegacyConstructor, RawLegacyEvent, RawLegacyFunction,
        RawLegacyL1Handler, RawLegacyMember, RawLegacyStruct,
    },
};

impl From<LegacyContractAbiEntry> for RawLegacyAbiEntry {
    fn from(value: LegacyContractAbiEntry) -> Self {
        match value {
            LegacyContractAbiEntry::Function(inner) => match inner.r#type {
                LegacyFunctionAbiType::Function => Self::Function(RawLegacyFunction {
                    inputs: inner.inputs,
                    name: inner.name,
                    outputs: inner.outputs,
                    state_mutability: inner.state_mutability,
                }),
                LegacyFunctionAbiType::L1Handler => Self::L1Handler(RawLegacyL1Handler {
                    inputs: inner.inputs,
                    name: inner.name,
                    outputs: inner.outputs,
                }),
                LegacyFunctionAbiType::Constructor => Self::Constructor(RawLegacyConstructor {
                    inputs: inner.inputs,
                    name: inner.name,
                    outputs: inner.outputs,
                }),
            },
            LegacyContractAbiEntry::Event(inner) => Self::Event(RawLegacyEvent {
                data: inner.data,
                keys: inner.keys,
                name: inner.name,
            }),
            LegacyContractAbiEntry::Struct(inner) => Self::Struct(RawLegacyStruct {
                members: inner
                    .members
                    .into_iter()
                    .map(|item| RawLegacyMember {
                        name: item.name,
                        offset: item.offset.unwrap_or(0),
                        r#type: item.r#type,
                    })
                    .collect(),
                name: inner.name,
                size: inner.size,
            }),
        }
    }
}

impl From<Transaction> for TransactionContent {
    fn from(value: Transaction) -> Self {
        match value {
            Transaction::Invoke(inner) => Self::Invoke(inner.into()),
            Transaction::L1Handler(inner) => Self::L1Handler(inner.into()),
            Transaction::Declare(inner) => Self::Declare(inner.into()),
            Transaction::Deploy(inner) => Self::Deploy(inner.into()),
            Transaction::DeployAccount(inner) => Self::DeployAccount(inner.into()),
        }
    }
}

impl From<InvokeTransaction> for InvokeTransactionContent {
    fn from(value: InvokeTransaction) -> Self {
        match value {
            InvokeTransaction::V0(inner) => Self::V0(inner.into()),
            InvokeTransaction::V1(inner) => Self::V1(inner.into()),
            InvokeTransaction::V3(inner) => Self::V3(inner.into()),
        }
    }
}

// NOTE: Removed in 0.10.1 - L1HandlerTransactionContent is now the same as L1HandlerTransaction
// impl From<L1HandlerTransaction> for L1HandlerTransactionContent {
//     fn from(value: L1HandlerTransaction) -> Self { ... }
// }

impl From<DeclareTransaction> for DeclareTransactionContent {
    fn from(value: DeclareTransaction) -> Self {
        match value {
            DeclareTransaction::V0(inner) => Self::V0(inner.into()),
            DeclareTransaction::V1(inner) => Self::V1(inner.into()),
            DeclareTransaction::V2(inner) => Self::V2(inner.into()),
            DeclareTransaction::V3(inner) => Self::V3(inner.into()),
        }
    }
}

// NOTE: Removed in 0.10.1 - DeployTransactionContent is now the same as DeployTransaction
// impl From<DeployTransaction> for DeployTransactionContent { ... }

impl From<DeployAccountTransaction> for DeployAccountTransactionContent {
    fn from(value: DeployAccountTransaction) -> Self {
        match value {
            DeployAccountTransaction::V1(inner) => Self::V1(inner.into()),
            DeployAccountTransaction::V3(inner) => Self::V3(inner.into()),
        }
    }
}

// NOTE: The following From implementations are removed in 0.10.1 because
// *Content types are now aliases to the main transaction types (they include transaction_hash).
// The Content types were previously used for transactions without the hash field.
// In 0.10.1, all transaction types include transaction_hash directly.

// impl From<InvokeTransactionV0> for InvokeTransactionV0Content { ... }
// impl From<InvokeTransactionV1> for InvokeTransactionV1Content { ... }
// impl From<InvokeTransactionV3> for InvokeTransactionV3Content { ... }
// impl From<DeclareTransactionV0> for DeclareTransactionV0Content { ... }
// impl From<DeclareTransactionV1> for DeclareTransactionV1Content { ... }
// impl From<DeclareTransactionV2> for DeclareTransactionV2Content { ... }
// impl From<DeclareTransactionV3> for DeclareTransactionV3Content { ... }
// impl From<DeployAccountTransactionV1> for DeployAccountTransactionV1Content { ... }
// impl From<DeployAccountTransactionV3> for DeployAccountTransactionV3Content { ... }
