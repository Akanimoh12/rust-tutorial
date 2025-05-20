
    pub fn add(a: u64, b:u64) ->  u64 {
        a + b
    }

    #[cfg(test)]

    mod test {

        use std::result;

        use super::main;

        #[test]
        fn test_add() {

            let a: u64 = 100;
            let b: u64 = 50;
            let result = add(a, b);
            assert_eq!(result, 150);
        }
    }