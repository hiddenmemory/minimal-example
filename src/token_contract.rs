pub use tokencontract_mod::*;

#[allow(clippy::too_many_arguments)]
mod tokencontract_mod {
    #![allow(dead_code)]
    #![allow(unused_imports)]

    #[doc = "TokenContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;

    use ethers::{
        contract::{
            self as ethers_contract,
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            self as ethers_core,
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::{self as ethers_providers, Middleware},
    };

    pub static TOKENCONTRACT_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"name\",\n    \"outputs\": [{ \"name\": \"\", \"type\": \"string\" }],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      { \"name\": \"_spender\", \"type\": \"address\" },\n      { \"name\": \"_value\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"approve\",\n    \"outputs\": [{ \"name\": \"\", \"type\": \"bool\" }],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"totalSupply\",\n    \"outputs\": [{ \"name\": \"\", \"type\": \"uint256\" }],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      { \"name\": \"_from\", \"type\": \"address\" },\n      { \"name\": \"_to\", \"type\": \"address\" },\n      { \"name\": \"_value\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"transferFrom\",\n    \"outputs\": [{ \"name\": \"\", \"type\": \"bool\" }],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"decimals\",\n    \"outputs\": [{ \"name\": \"\", \"type\": \"uint8\" }],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [{ \"name\": \"_owner\", \"type\": \"address\" }],\n    \"name\": \"balanceOf\",\n    \"outputs\": [{ \"name\": \"balance\", \"type\": \"uint256\" }],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"symbol\",\n    \"outputs\": [{ \"name\": \"\", \"type\": \"string\" }],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      { \"name\": \"_to\", \"type\": \"address\" },\n      { \"name\": \"_value\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"transfer\",\n    \"outputs\": [{ \"name\": \"\", \"type\": \"bool\" }],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      { \"name\": \"_owner\", \"type\": \"address\" },\n      { \"name\": \"_spender\", \"type\": \"address\" }\n    ],\n    \"name\": \"allowance\",\n    \"outputs\": [{ \"name\": \"\", \"type\": \"uint256\" }],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  { \"payable\": true, \"stateMutability\": \"payable\", \"type\": \"fallback\" },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      { \"indexed\": true, \"name\": \"owner\", \"type\": \"address\" },\n      { \"indexed\": true, \"name\": \"spender\", \"type\": \"address\" },\n      { \"indexed\": false, \"name\": \"value\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"Approval\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      { \"indexed\": true, \"name\": \"from\", \"type\": \"address\" },\n      { \"indexed\": true, \"name\": \"to\", \"type\": \"address\" },\n      { \"indexed\": false, \"name\": \"value\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"Transfer\",\n    \"type\": \"event\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct TokenContract<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for TokenContract<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for TokenContract<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(TokenContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> TokenContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), TOKENCONTRACT_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers_core::types::Address,
            spender: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers_core::types::Address,
            value: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            owner: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers_contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers_contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            to: ethers_core::types::Address,
            value: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers_core::types::Address,
            to: ethers_core::types::Address,
            value: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers_contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers_contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, TokenContractEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers_core::types::Address,
        pub value: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers_core::types::Address,
        pub value: ethers_core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TokenContractEvents {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers_core::abi::Tokenizable for TokenContractEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::from_token(token.clone()) {
                return Ok(TokenContractEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::from_token(token.clone()) {
                return Ok(TokenContractEvents::TransferFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                TokenContractEvents::ApprovalFilter(element) => element.into_token(),
                TokenContractEvents::TransferFilter(element) => element.into_token(),
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for TokenContractEvents {}
    impl ethers_contract::EthLogDecode for TokenContractEvents {
        fn decode_log(log: &ethers_core::abi::RawLog) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(TokenContractEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(TokenContractEvents::TransferFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}
