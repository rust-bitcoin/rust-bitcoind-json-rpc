// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of `bitcoind v0.17.1`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements bitcoind JSON-RPC API method `createwallet`
#[macro_export]
macro_rules! impl_client_v17__createwallet {
    () => {
        impl Client {
            pub fn create_wallet(&self, wallet: &str) -> Result<CreateWallet> {
                self.call("createwallet", &[wallet.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `unloadwallet`
#[macro_export]
macro_rules! impl_client_v17__unloadwallet {
    () => {
        impl Client {
            pub fn unload_wallet(&self, wallet: &str) -> Result<()> {
                self.call("unloadwallet", &[wallet.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `loadwallet`
#[macro_export]
macro_rules! impl_client_v17__loadwallet {
    () => {
        impl Client {
            pub fn load_wallet(&self, wallet: &str) -> Result<LoadWallet> {
                self.call("loadwallet", &[wallet.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getbalance`
#[macro_export]
macro_rules! impl_client_v17__getbalance {
    () => {
        impl Client {
            pub fn get_balance(&self) -> Result<GetBalance> { self.call("getbalance", &[]) }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getnewaddress`
#[macro_export]
macro_rules! impl_client_v17__getnewaddress {
    () => {
        impl Client {
            /// Gets a new address from `bitcoind` and parses it assuming its correct.
            pub fn new_address(&self) -> Result<bitcoin::Address> {
                use core::str::FromStr;

                let json = self.get_new_address()?;
                let address = bitcoin::Address::from_str(&json.0)
                    .expect("assume the address is valid")
                    .assume_checked(); // Assume bitcoind will return an invalid address for the network its on.
                Ok(address)
            }

            /// Gets a new address from `bitcoind` and parses it assuming its correct.
            pub fn new_address_with_type(&self, ty: AddressType) -> Result<bitcoin::Address> {
                use core::str::FromStr;

                let json = self.get_new_address_with_type(ty)?;
                let address = bitcoin::Address::from_str(&json.0)
                    .expect("assume the address is valid")
                    .assume_checked(); // Assume bitcoind will return an invalid address for the network its on.
                Ok(address)
            }

            pub fn get_new_address(&self) -> Result<GetNewAddress> {
                self.call("getnewaddress", &[])
            }

            pub fn get_new_address_with_type(&self, ty: AddressType) -> Result<GetNewAddress> {
                self.call("getnewaddress", &["".into(), into_json(ty)?])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `sendtoaddress`
#[macro_export]
macro_rules! impl_client_v17__sendtoaddress {
    () => {
        impl Client {
            pub fn send_to_address(
                &self,
                address: &Address<NetworkChecked>,
                amount: Amount,
            ) -> Result<bitcoin::Txid> {
                let mut args = [address.to_string().into(), into_json(amount.to_btc())?];
                self.call("sendtoaddress", handle_defaults(&mut args, &["".into(), "".into()]))
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `gettransaction`
#[macro_export]
macro_rules! impl_client_v17__gettransaction {
    () => {
        impl Client {
            pub fn get_transaction(&self, txid: Txid) -> Result<GetTransaction> {
                self.call("gettransaction", &[into_json(txid)?])
            }
        }
    };
}
