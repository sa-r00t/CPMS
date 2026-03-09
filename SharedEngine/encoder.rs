fn main() {
    const MAGIC: [u8; 2] = [0x4D, 0x53];
    const VERSION: u8 = 1;  
    const TYPE: u8 = 1;

    let mut text = String::new();
    
    std::io::stdin().read_line(&mut text).unwrap();
    let text = text.trim().to_string();
    let mut frame: Vec<u8> = Vec::new(); 

    frame.extend_from_slice(&MAGIC);
    frame.push(VERSION);
    frame.push(TYPE);
    frame.extend_from_slice(text.as_bytes());  

    let hex_string: Vec<String> = frame.iter().map(|b| format!("{:02X}", b)).collect();
    println!("{}", hex_string.join(" "));   
}
