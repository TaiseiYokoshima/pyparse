pub type BP = f32;
pub mod bp {
    use super::BP;
    use crate::units::Operator;

    pub const ADD: (BP, BP) = (1.0, 1.1);
    pub const SUB: (BP, BP) = (1.0, 1.1);
    pub const MUL: (BP, BP) = (2.0, 2.1);
    pub const DIV: (BP, BP) = (2.0, 2.1);

    pub const NEG: (BP, BP) = (10.0, 10.1);
    pub const NOT: (BP, BP) = (10.0, 10.1);

    pub const START: (BP, BP) = (0.0, 0.0);
    pub const NEWLINE: (BP, BP) = (0.0, 0.0);

    pub fn get_infix(op: &Operator) -> (BP, BP) {
        match op {
            Operator::Add => ADD,
            Operator::Sub => SUB,
            Operator::Mul => MUL,
            Operator::Div => DIV,
            _ => panic!("you gave the wrong one"),
        }
    }

    pub fn binds_left(left: BP, operator: &Operator) -> bool {
        left > get_infix(operator).0
    }
}
