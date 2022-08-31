use std::{collections::HashMap};

use super::{
    constants::{IppDelimiterTag, IppOperationId, IppValueTag, IppVersion},
    structs::{Attribute}, operation::Operation,
};

pub struct OperationBuilder {
    pub ipp_uri: String,
    pub ipp_version: IppVersion,
}

impl OperationBuilder {

    pub fn get_printer_attributes(&self) -> Operation {
        return Operation {
            id: IppOperationId::GetPrinterAttributes,
            uri: self.ipp_uri.clone(),
            attribute_group: HashMap::from([(
                IppDelimiterTag::OperationAttributes,
                vec![
                    Attribute {
                        name: "attributes-charset",
                        value: "utf-8".to_string(),
                        tag: IppValueTag::Charset,
                    },
                    Attribute {
                        name: "attributes-natural-language",
                        value: "en".to_string(),
                        tag: IppValueTag::NaturalLang,
                    },
                    Attribute {
                        name: "printer-uri",
                        value: self.ipp_uri.to_string(),
                        tag: IppValueTag::Uri,
                    },
                ],
            )]),
            data: vec![],
        };
    }

    pub fn print_job(
        &self,
        data: Vec<u8>,
        job_name: Option<&str>,
        user_name: Option<&str>,
        document_format: Option<&str>,
    ) -> Operation {
        return Operation {
            id: IppOperationId::PrintJob,
            uri: self.ipp_uri.clone(),
            data,
            attribute_group: HashMap::from([(
                IppDelimiterTag::OperationAttributes,
                vec![
                    Attribute {
                        name: "attributes-charset",
                        value: "utf-8".to_string(),
                        tag: IppValueTag::Charset,
                    },
                    Attribute {
                        name: "attributes-natural-language",
                        value: "en".to_string(),
                        tag: IppValueTag::NaturalLang,
                    },
                    Attribute {
                        name: "printer-uri",
                        value: self.ipp_uri.to_string(),
                        tag: IppValueTag::Uri,
                    },
                    Attribute {
                        name: "requesting-user-name",
                        value: if user_name.is_some() {
                            user_name.unwrap().to_string()
                        } else {
                            "".to_string()
                        },
                        tag: IppValueTag::NameWithoutLang,
                    },
                    Attribute {
                        name: "job-name",
                        value: if job_name.is_some() {
                            job_name.unwrap().to_string()
                        } else {
                            "".to_string()
                        },
                        tag: IppValueTag::NameWithoutLang,
                    },
                    Attribute {
                        name: "document-format",
                        value: if document_format.is_some() {
                            document_format.unwrap().to_string()
                        } else {
                            "application/octet-stream".to_string()
                        },
                        tag: IppValueTag::MimeMediaType,
                    },
                ],
            )]),
        };
    }
}
