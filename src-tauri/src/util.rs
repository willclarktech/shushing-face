pub fn to_hex_string(bytes: &[u8]) -> String {
	bytes
		.map(|byte| format!("{:02x}", byte))
		.collect::<String>()
}
