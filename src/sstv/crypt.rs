pub fn crc(buf: &[u8]) -> u32 {
  let mut crc_table = [0; 256];

  for n in 0..256 {
      crc_table[n as usize] = (0..8).fold(n as u32, |acc, _| {
          match acc & 1 {
              1 => 0xedb88320 ^ (acc >> 1),
              _ => acc >> 1,
          }
      });
  }
  !buf.iter().fold(!0, |acc, octet| {
      (acc >> 8) ^ crc_table[((acc & 0xff) ^ *octet as u32) as usize]
  })
}

pub fn encode_data_zlib(data: &[u8]) -> Vec<u8> {
  let compressed = deflate::deflate_bytes_zlib(data);
  return compressed;
}
