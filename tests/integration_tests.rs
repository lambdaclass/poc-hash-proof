// These tests assume ECB. They should be updated to test CBC.
#[cfg(test)]
mod tests {
    use ark_relations::r1cs::ConstraintSystem;
    use poc_encryption_proof::{
        aes::{add_round_key, mix_columns, shift_rows, substitute_bytes},
        encrypt, synthesize_keys,
    };
    use simpleworks::gadgets::ConstraintF;

    #[test]
    fn test_one_round_aes_encryption() {
        let cs = ConstraintSystem::<ConstraintF>::new_ref();
        let plaintext: [u8; 16] = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];
        let key: [u8; 16] = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];

        let expected_start_of_round = [
            0x19, 0x3d, 0xe3, 0xbe, 0xa0, 0xf4, 0xe2, 0x2b, 0x9a, 0xc6, 0x8d, 0x2a, 0xe9, 0xf8,
            0x48, 0x08,
        ];
        let expected_after_substituting_bytes = [
            0xd4, 0x27, 0x11, 0xae, 0xe0, 0xbf, 0x98, 0xf1, 0xb8, 0xb4, 0x5d, 0xe5, 0x1e, 0x41,
            0x52, 0x30,
        ];
        let expected_after_shift_rows = [
            0xd4, 0xbf, 0x5d, 0x30, 0xe0, 0xb4, 0x52, 0xae, 0xb8, 0x41, 0x11, 0xf1, 0x1e, 0x27,
            0x98, 0xe5,
        ];
        let expected_after_mix_columns = [
            0x04, 0x66, 0x81, 0xe5, 0xe0, 0xcb, 0x19, 0x9a, 0x48, 0xf8, 0xd3, 0x7a, 0x28, 0x06,
            0x26, 0x4c,
        ];

        // TODO: Replace expected_start_of_next_round with the code commented
        // above this when key expansion is implemented.
        // let expected_start_of_next_round = [
        //     0xa4, 0x9c, 0x7f, 0xf2, 0x68, 0x9f, 0x35, 0x2b, 0x6b, 0x5b, 0xea, 0x43, 0x02, 0x6a,
        //     0x50, 0x49,
        // ];
        let expected_start_of_next_round = [
            0x2f, 0x18, 0x94, 0xf3, 0xc8, 0x65, 0xcb, 0x3c, 0xe3, 0x0f, 0xc6, 0xf2, 0x21, 0xc9,
            0x69, 0x70,
        ];

        let start_of_round = add_round_key(&plaintext, &key);
        assert_eq!(
            start_of_round, expected_start_of_round,
            "Start of round is incorrect"
        );

        // Round 1
        let after_substituting_bytes = substitute_bytes(&expected_start_of_round, &cs).unwrap();
        assert_eq!(
            after_substituting_bytes, expected_after_substituting_bytes,
            "Substitution is incorrect"
        );
        let after_shift_rows = shift_rows(&after_substituting_bytes, &cs).unwrap();
        assert_eq!(
            after_shift_rows, expected_after_shift_rows,
            "Shift rows is incorrect"
        );
        let after_mix_columns = mix_columns(&after_shift_rows).unwrap();
        assert_eq!(
            after_mix_columns, expected_after_mix_columns,
            "Mix columns is incorrect"
        );
        // We should use the round key instead of the secret key here when
        // key expansion is implemented.
        let start_of_next_round = add_round_key(&after_mix_columns, &key);
        assert_eq!(
            start_of_next_round, expected_start_of_next_round,
            "Start of next round is incorrect"
        );
    }

    #[test]
    fn test_encrypt_a_16_bytes_plaintext() {
        let plaintext: [u8; 16] = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];
        let key: [u8; 16] = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];
        let (proving_key, _verifying_key) = synthesize_keys(plaintext.len()).unwrap();
        let expected_ciphertext = [
            0x1c, 0x0b, 0xfd, 0x7c, 0xb7, 0x32, 0x04, 0x6b, 0x11, 0xd1, 0xaa, 0xa9, 0x51, 0x31,
            0x07, 0x2c,
        ];

        // TODO: Assert against proof.
        let (ciphertext, _proof) = encrypt(&plaintext, &key, proving_key).unwrap();

        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn test_one_round_aes_encryption_of_a_64_bytes_plaintext() {
        let plaintext: [u8; 64] = [
            // input Start of Round from Appendix B - Cipher Example
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34, // 1 Start of Round from Appendix B - Cipher Example
            0xa4, 0x9c, 0x7f, 0xf2, 0x68, 0x9f, 0x35, 0x2b, 0x6b, 0x5b, 0xea, 0x43, 0x02, 0x6a,
            0x50, 0x49, // 2 Start of Round from Appendix B - Cipher Example
            0xaa, 0x8f, 0x5f, 0x03, 0x61, 0xdd, 0xe3, 0xef, 0x82, 0xd2, 0x4a, 0xd2, 0x68, 0x32,
            0x46, 0x9a, // 3 Start of Round
            0x48, 0x6c, 0x4e, 0xee, 0x67, 0x1d, 0x9d, 0x0d, 0x4d, 0xe3, 0xb1, 0x38, 0xd6, 0x5f,
            0x58, 0xe7,
        ];
        let key: [u8; 16] = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];
        let (proving_key, _verifying_key) = synthesize_keys(plaintext.len()).unwrap();
        // TODO: Every expected block should be changed after key expansion is
        // implemented.
        let expected_ciphertext = [
            28, 11, 253, 124, 183, 50, 4, 107, 17, 209, 170, 169, 81, 49, 7, 44, 96, 177, 73, 164,
            242, 134, 226, 238, 0, 176, 149, 241, 114, 129, 199, 244, 148, 83, 172, 76, 183, 206,
            121, 127, 18, 174, 115, 92, 130, 167, 162, 101, 210, 99, 164, 106, 58, 132, 98, 148,
            170, 77, 84, 91, 69, 218, 113, 201,
        ];

        // TODO: Assert against proof.
        let (ciphertext, _proof) = encrypt(&plaintext, &key, proving_key).unwrap();

        assert_eq!(ciphertext, expected_ciphertext);
    }
}
