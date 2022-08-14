use libsnapper::*;

fn main() {
  let d = vec![0xde, 0xad, 0xd0, 0x0d];
  let c: &[u8] = &compress(&d);
  assert!(validate_compressed_buffer(c));
  assert!(uncompress(c) == Some(d));
}
