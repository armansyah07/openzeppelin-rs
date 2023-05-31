pub use erc1820_implementer::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod erc1820_implementer {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"interfaceHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"canImplementInterfaceForAddress\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static ERC1820IMPLEMENTER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        1,
        20,
        128,
        97,
        0,
        32,
        96,
        0,
        57,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        96,
        15,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        96,
        40,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        36,
        156,
        179,
        250,
        20,
        96,
        45,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        60,
        96,
        56,
        54,
        96,
        4,
        96,
        164,
        86,
        91,
        96,
        78,
        86,
        91,
        96,
        64,
        81,
        144,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        0,
        130,
        129,
        82,
        96,
        32,
        129,
        129,
        82,
        96,
        64,
        128,
        131,
        32,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        133,
        22,
        132,
        82,
        144,
        145,
        82,
        129,
        32,
        84,
        96,
        255,
        22,
        96,
        123,
        87,
        96,
        0,
        96,
        157,
        86,
        91,
        127,
        162,
        239,
        70,
        0,
        215,
        66,
        2,
        45,
        83,
        45,
        71,
        71,
        203,
        53,
        71,
        71,
        70,
        103,
        214,
        241,
        56,
        4,
        144,
        37,
        19,
        178,
        236,
        1,
        200,
        72,
        244,
        180,
        91,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        128,
        96,
        64,
        131,
        133,
        3,
        18,
        21,
        96,
        182,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
        53,
        145,
        80,
        96,
        32,
        131,
        1,
        53,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        129,
        22,
        129,
        20,
        96,
        211,
        87,
        96,
        0,
        128,
        253,
        91,
        128,
        145,
        80,
        80,
        146,
        80,
        146,
        144,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        165,
        151,
        157,
        46,
        239,
        137,
        22,
        254,
        231,
        41,
        210,
        69,
        248,
        135,
        208,
        172,
        171,
        79,
        236,
        137,
        121,
        197,
        55,
        244,
        210,
        99,
        49,
        162,
        39,
        151,
        70,
        25,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        13,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static ERC1820IMPLEMENTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        96,
        15,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        96,
        40,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        36,
        156,
        179,
        250,
        20,
        96,
        45,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        60,
        96,
        56,
        54,
        96,
        4,
        96,
        164,
        86,
        91,
        96,
        78,
        86,
        91,
        96,
        64,
        81,
        144,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        0,
        130,
        129,
        82,
        96,
        32,
        129,
        129,
        82,
        96,
        64,
        128,
        131,
        32,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        133,
        22,
        132,
        82,
        144,
        145,
        82,
        129,
        32,
        84,
        96,
        255,
        22,
        96,
        123,
        87,
        96,
        0,
        96,
        157,
        86,
        91,
        127,
        162,
        239,
        70,
        0,
        215,
        66,
        2,
        45,
        83,
        45,
        71,
        71,
        203,
        53,
        71,
        71,
        70,
        103,
        214,
        241,
        56,
        4,
        144,
        37,
        19,
        178,
        236,
        1,
        200,
        72,
        244,
        180,
        91,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        128,
        96,
        64,
        131,
        133,
        3,
        18,
        21,
        96,
        182,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
        53,
        145,
        80,
        96,
        32,
        131,
        1,
        53,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        129,
        22,
        129,
        20,
        96,
        211,
        87,
        96,
        0,
        128,
        253,
        91,
        128,
        145,
        80,
        80,
        146,
        80,
        146,
        144,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        165,
        151,
        157,
        46,
        239,
        137,
        22,
        254,
        231,
        41,
        210,
        69,
        248,
        135,
        208,
        172,
        171,
        79,
        236,
        137,
        121,
        197,
        55,
        244,
        210,
        99,
        49,
        162,
        39,
        151,
        70,
        25,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        13,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static ERC1820IMPLEMENTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ERC1820Implementer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC1820Implementer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC1820Implementer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC1820Implementer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC1820Implementer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ERC1820Implementer)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC1820Implementer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC1820IMPLEMENTER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                ERC1820IMPLEMENTER_ABI.clone(),
                ERC1820IMPLEMENTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `canImplementInterfaceForAddress` (0x249cb3fa) function
        pub fn can_implement_interface_for_address(
            &self,
            interface_hash: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 156, 179, 250], (interface_hash, account))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC1820Implementer<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `canImplementInterfaceForAddress` function with signature `canImplementInterfaceForAddress(bytes32,address)` and selector `0x249cb3fa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "canImplementInterfaceForAddress",
        abi = "canImplementInterfaceForAddress(bytes32,address)"
    )]
    pub struct CanImplementInterfaceForAddressCall {
        pub interface_hash: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `canImplementInterfaceForAddress` function with signature `canImplementInterfaceForAddress(bytes32,address)` and selector `0x249cb3fa`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CanImplementInterfaceForAddressReturn(pub [u8; 32]);
}
