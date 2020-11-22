struct Prime {
    pos: u32,
    num: u32,
}

pub fn nth(n: u32) -> u32 {
    let mut prime = Prime {
        pos: 0,
        num: 2,
    };
    if n > 0 {
        let mut i: u32 = 2;
        loop {
            let mut is_prime = true;

            if i % 2 == 0 {
                is_prime = false;
            } else {
                let mut j: u32 = 3;
                while j < i / 2 {
                    if i % j == 0 {
                        is_prime = false;
                        break;
                    }
                    j += 2;
                }
            }
            if is_prime {
                prime.pos += 1;
                prime.num = i.clone();
                if prime.pos == n {
                    break;
                }
            }
            i += 1;
        }
    }
    prime.num
}
