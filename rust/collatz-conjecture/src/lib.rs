pub fn collatz(mut n: u64) -> Option<u64> {
    for steps in 0.. {
        match n {
            0 => break, 
            1 => return Some(steps), 
            even if even % 2 == 0 => n = n / 2, 
            _ => n = n.checked_mul(3)?.checked_add(1)?, 
        }
    }

    None
}
