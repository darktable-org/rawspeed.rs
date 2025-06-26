use super::*;

fn variable_length_load_naive(dest: &mut [u8], src: &[u8], src_pos: usize) {
    dest.fill(0);
    for (i, e) in dest.iter_mut().enumerate() {
        if let Some(val) = src.get(src_pos + i) {
            *e = *val;
        }
    }
}

#[test]
fn basic_test() {
    {
        let mut out: [u8; 2] = Default::default();
        let src: [u8; 3] = [1, 2, 3];
        variable_length_load_naive(&mut out, &src, 0);
        assert_eq!(out, [1, 2]);
    }
    {
        let mut out: [u8; 2] = Default::default();
        let src: [u8; 3] = [1, 2, 3];
        variable_length_load_naive(&mut out, &src, 1);
        assert_eq!(out, [2, 3]);
    }
    {
        let mut out: [u8; 2] = Default::default();
        let src: [u8; 3] = [1, 2, 3];
        variable_length_load_naive(&mut out, &src, 2);
        assert_eq!(out, [3, 0]);
    }
    {
        let mut out: [u8; 2] = Default::default();
        let src: [u8; 3] = [1, 2, 3];
        variable_length_load_naive(&mut out, &src, 3);
        assert_eq!(out, [0, 0]);
    }
    {
        let mut out: [u8; 2] = Default::default();
        let src: [u8; 3] = [1, 2, 3];
        variable_length_load_naive(&mut out, &src, 4);
        assert_eq!(out, [0, 0]);
    }
}

#[test]
fn exhaustive_test() {
    for input_length in 0..=64_usize {
        let mut res: Vec<u8> = vec![];
        for i in 0..input_length {
            res.push((1 + i).try_into().unwrap());
        }
        let src = res;
        for output_length in 0..=input_length {
            let mut out_a: Vec<u8> = vec![];
            out_a.resize(output_length, u8::MAX);
            let mut out_b: Vec<u8> = vec![];
            out_b.resize(output_length, u8::MAX);
            for src_pos in 0..=4 * input_length {
                (out_a[..]).variable_length_load(&src[..], src_pos);
                variable_length_load_naive(&mut out_b[..], &src[..], src_pos);
                assert_eq!(out_a, out_b);
            }
        }
    }
}
