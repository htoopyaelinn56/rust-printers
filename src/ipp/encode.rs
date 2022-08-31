use rand::Rng;

use super::{operation::Operation, constants::IppDelimiterTag};


pub fn encode(operation: &Operation) -> Vec<u8> {
    let mut data = vec![];

    // version
    data.push(*(&2) as u8);
    data.push(0);
    data.push(0);
    
    // operation-id
    data.push(*(&operation.id) as u8);

    // request-id
    let request_id = rand::thread_rng().gen_range(10000000_u32..99999999_u32);
    data.extend(request_id.to_be_bytes());

    // attribute-group
    for (tag, attributes) in &operation.attribute_group {

        // begin-attribute-group-tag
        data.push(*(tag) as u8);

        // attribute
        for attr in attributes {

            let name = &mut attr.name.as_bytes().to_vec();
            let value = &mut attr.value.as_bytes().to_vec();

            // value-tag
            data.push(*(&attr.tag) as u8);
            data.push(0);
            
            // name-length
            data.push(name.len() as u8);
            
            // name
            data.append(name);
            data.push(0);
            
            // value-length
            data.push(value.len() as u8);
            // value
            data.append(value);
        }
    }

    // end-of-attributes-tag
    data.push(*(&IppDelimiterTag::EndOfAttributes) as u8);

    // data
    data.append(operation.data.to_vec().as_mut());

    return data.clone();

}