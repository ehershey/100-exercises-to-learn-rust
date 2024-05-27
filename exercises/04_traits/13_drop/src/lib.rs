// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

pub struct DropBomb {
    has_been_defused: bool,
}
impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.has_been_defused {
            panic!("Dropped bomb without being defused");
        }
    }
}

impl DropBomb {
    fn defuse(&mut self) {
        self.has_been_defused = true
    }
    fn new() -> DropBomb {
        DropBomb {
            has_been_defused: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
