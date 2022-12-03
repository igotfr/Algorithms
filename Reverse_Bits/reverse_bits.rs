impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        // Iterative 1
        let mut res = 0;
        for i in 0..32 {
            res <<= 1;
            res |= (x >> i) & 1;
        }
        return res

        // Iterative 2
        /*let (mut res, mut x) = (0u32, x);
        for _ in 0..32 {
            res = (res << 1) | (x & 1);
            x >>= 1;
        }
        return res*/

        // Iterative 3
        /*let n = 31;
        let mut ans = 0;
        for i in 0..32 {
            let bit = (x >> i) & 1;
            let v = bit << (n - i);
            ans += v;
        }
        return ans*/

        // Functional
        //return x.reverse_bits()
    }

    /*pub fn reverse_bits(mut x: u32) -> u32 {
        // Iterative
        for i in 0..16 {
            if x >> i & 0x1 != x >> 31 - i & 0x1 {
                x ^= 1 << i;
                x ^= 1 << 31 - i;
            }
        }
        return x
    }*/
}
