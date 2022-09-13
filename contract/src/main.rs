#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use core::mem;

use alloc::string::String;
use alloc::vec::Vec;

use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped,
};

pub enum MyType {
    OPSTRING { mystring: Option<String> },
    OPBOOL { mybool: Option<bool> },
}

impl ToBytes for MyType {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        match self {
            MyType::OPSTRING { mystring } => {
                buffer.insert(0, 0u8);
                buffer.extend(mystring.to_bytes()?);
            }
            MyType::OPBOOL { mybool } => {
                buffer.insert(0, 1u8);
                buffer.extend(mybool.to_bytes()?);
            }
        }
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        mem::size_of::<u8>()
            + match self {
                MyType::OPSTRING { mystring } => mystring.serialized_length(),
                MyType::OPBOOL { mybool } => mybool.serialized_length(),
            }
    }
}

impl FromBytes for MyType {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (tag, remainder) = u8::from_bytes(bytes)?;
        match tag {
            0 => {
                let (mystring, remainder) = Option::<String>::from_bytes(remainder)?;

                Ok((MyType::OPSTRING { mystring }, remainder))
            }
            1 => {
                let (mybool, remainder) = Option::<bool>::from_bytes(remainder)?;
                Ok((MyType::OPBOOL { mybool }, remainder))
            }
            _ => Err(bytesrepr::Error::Formatting),
        }
    }
}
impl CLTyped for MyType {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let mut a: Vec<MyType> = Vec::new();
    let value1: MyType = MyType::OPSTRING {
        mystring: Some(String::from("hello")),
    };
    let value2 = MyType::OPSTRING { mystring: None };
    let value3 = MyType::OPBOOL { mybool: Some(true) };

    let value4 = MyType::OPBOOL { mybool: None };

    a.push(value1);
    a.push(value2);
    a.push(value3);
    a.push(value4);

    runtime::put_key("listofoptions", storage::new_uref(a).into());
}
