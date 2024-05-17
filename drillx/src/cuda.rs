extern "C" {
    pub fn hash(challenge: *const u8, nonce: *const u8, out: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_gpu() {
        let challenge = [255; 32];
        let nonce = [2; 8];
        let mut out = [0; 16];
        let timer = Instant::now();
        unsafe {
            hash(challenge.as_ptr(), nonce.as_ptr(), out.as_mut_ptr());
            println!("Out: {:?}", out);
        }
        println!("Did 65536 hashx in {} ms", timer.elapsed().as_millis());
        assert_eq!(42, u128::from_le_bytes(out));
    }
}
