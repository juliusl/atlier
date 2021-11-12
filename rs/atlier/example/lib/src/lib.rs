#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use atlier::Transition;

#[derive(Transition, Debug)]
#[output(sum, i32)]
#[output(display, String)]
pub struct Add {
    lhs: i32,
    rhs: i32, 
}

impl AddOutputs for Add {
    fn sum(lhs: i32, rhs: i32) -> Option<i32> {
        Some(lhs+rhs)
    }
    fn display(lhs: i32, rhs: i32) -> Option<String> {
        Some(format!("{} + {} = {}", lhs, rhs, lhs+rhs))
    }
}
