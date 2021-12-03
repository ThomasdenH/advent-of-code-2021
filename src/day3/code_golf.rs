pub fn z(a: &str) -> u32 {
    let (mut b, mut c, mut g, mut e, mut u) = ([0; 13], 0, 0, 0, 4096);
    for a in a.bytes() {
        c += 1;
        b[c % 13] += a as usize & 1;
    }
    for a in b.iter_mut() {
        if 26 * *a > c + 1 {
            g |= u;
        }
        u >>= 1;
        e |= u;
    }
    g * (e ^ g)
}