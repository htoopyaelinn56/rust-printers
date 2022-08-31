use self::operation_builder::OperationBuilder;

mod encode;
mod structs;
mod operation_builder;

pub mod constants;
pub mod operation;

pub struct Ipp {
    // uri: &'static str,
    // version: constants::IppVersion,
    pub operation: operation_builder::OperationBuilder,
}

impl Ipp {
    pub fn v2(uri: String) -> Ipp {
        return Ipp {
            // uri,
            // version: constants::IppVersion::V2,
            operation: OperationBuilder {
                ipp_uri: uri,
                ipp_version: constants::IppVersion::V2,
            },
        }
    }
}