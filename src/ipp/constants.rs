#[derive(Debug, Copy, Clone)]
pub enum IppVersion {
    V2 = 2
}

#[derive(Debug, Copy, Clone)]
pub enum IppOperationId {
    PrintJob = 0x02,
    GetPrinterAttributes = 0x0b,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum IppDelimiterTag {
    EndOfAttributes = 0x03,
    OperationAttributes = 0x01,
}

#[derive(Debug, Copy, Clone)]
pub enum IppValueTag {
    Uri = 0x45,
    Charset = 0x47,
    NaturalLang = 0x48,
    OctetString = 0x30,
    MimeMediaType = 0x49,
    NameWithoutLang = 0x42,
}