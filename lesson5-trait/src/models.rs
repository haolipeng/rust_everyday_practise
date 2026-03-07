use crate::traits::{Greet, Age};

pub struct Chinese { pub name: String }
pub struct American { pub name: String }

impl Greet for Chinese {
    fn hello(&self) -> String {
        format!("你好！我是{}", self.name)
    }
}

impl Age for Chinese {
    fn age(&self) -> u32 {
        25
    }
}

impl Greet for American {
    fn hello(&self) -> String {
        format!("Hi! I'm {}", self.name)
    }
}
