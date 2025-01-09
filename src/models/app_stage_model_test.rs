
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // use crate::models::app_stage_model_test::add;
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
    #[test]
    fn test_add_two() {
        assert_eq!(add(2, 2), 4);
    }
}