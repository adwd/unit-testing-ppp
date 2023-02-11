pub mod ch_02;
pub mod ch_03;
pub mod ch_05;
pub mod ch_06;
pub mod ch_06_audit_log;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
