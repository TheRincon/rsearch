const DUST_LEVEL: usize = 20;  // original was 20
const DUST_WORD: usize = 3;
const DUST_WINDOW: usize = 64; // original is 64
const DUST_WINDOW_2: usize = DUST_WINDOW >> 1;
const WORD_COUNT: usize = 1 << ( DUST_WORD << 1 );
static BIT_MASK: usize = WORD_COUNT - 1;

fn wo(len: usize, m: &mut [u8], beg: &mut usize, end: &mut usize) -> usize {

    let l1 = len - DUST_WORD + 1 - 5;
    if l1 < 0 { return 0 };
    let chrmap = [0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  1,  0,  0,  0,  2,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  3,  3,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  1,  0,  0,  0,  2,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  3,  3,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0];

    let (mut bestv, mut besti, mut bestj) = (0usize,0usize,0usize);
    let mut words = [0; 64];
    let mut word = 0;

    for j in 0..len {
        word <<= 2;
        word |= chrmap[m[j] as usize];
        words[j] = word & BIT_MASK;
    }

    for i in 0..l1 {
        let mut counts = [0; 64];
        let mut sum = 0;

        for j in 2 .. len - i {
            word = words[i + j];
            let mut c = counts[word];
            if c != 0 {
                sum += c;
                let mut v = 10 * sum / j;
                if v > bestv
                    {
                        bestv = v;
                        besti = i;
                        bestj = j;
                    }
            }
            counts[word] += 1;
        }
    }
    let x = besti;
    let y = bestj;
    *beg = x;
    *end = x + y;
    return bestv;
}

pub fn dust(m: &mut [u8], hardmask: bool) {

    let len = m.len();
    let (mut a, mut b) = (0usize, 0usize);
    let mut s: Vec<u8> = vec![87u8; len];
    s.clone_from_slice(&m);
    if !hardmask {
        /* convert sequence to upper case unless hardmask in effect */
        // for i in 0..len {
            // m[i] = toupper(m[i])
        // };
        // m[len] = 0;  // what is this?
    }
    let mut i = 0;
    while i < len {
        let l = if len > i + DUST_WINDOW { DUST_WINDOW } else { len-i };
        let v = wo(l, &mut s[i..], &mut a, &mut b);
        if v > DUST_LEVEL {
            if hardmask {
                for j in (a+i)..(b + i + 1 ) {
                    m[j] = 78u8;
                }
            } else {
                for j in (a + i)..(b + i + 1) {
                    match m[j] {
                        65u8 | 97u8 => m[j] = 97u8,
                        67u8 | 99u8 => m[j] = 99u8,
                        71u8 | 103u8 => m[j] = 103u8,
                        84u8 | 116u8 => m[j] = 116u8,
                        _ => m[j] = 122u8
                    }
                }
            }
            if b < DUST_WINDOW_2 { i += DUST_WINDOW_2 - b };
        }
        i += DUST_WINDOW_2;
    }
}