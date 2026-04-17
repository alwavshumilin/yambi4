const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode_base64(input: Vec<u8>) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    let mut index = 0usize;

    while index < input.len() {
        let byte0 = input[index];
        let byte1 = if index + 1 < input.len() {
            input[index + 1]
        } else {
            0
        };
        let byte2 = if index + 2 < input.len() {
            input[index + 2]
        } else {
            0
        };

        let triple = ((byte0 as u32) << 16) | ((byte1 as u32) << 8) | byte2 as u32;

        let char0 = TABLE[((triple >> 18) & 0x3f) as usize] as char;
        let char1 = TABLE[((triple >> 12) & 0x3f) as usize] as char;
        let char2 = TABLE[((triple >> 6) & 0x3f) as usize] as char;
        let char3 = TABLE[(triple & 0x3f) as usize] as char;

        output.push(char0);
        output.push(char1);

        if index + 1 < input.len() {
            output.push(char2);
        } else {
            output.push('=');
        }

        if index + 2 < input.len() {
            output.push(char3);
        } else {
            output.push('=');
        }

        index += 3;
    }

    output
}
