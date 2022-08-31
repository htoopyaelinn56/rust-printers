pub fn decode_ipp(data: &[u8]) {
    let version = &data[0..1];
    let operation_id = &data[1..4];
    let request_id = &data[4..8];

    println!("{:?}", version);
    println!("{:?}", operation_id);
    println!("{:?}", request_id);
    // println!("{:?}", pop(request_id));
}