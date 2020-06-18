//! This crate shows how the parameterized crate can be used when testing. As example, the first
//! day of [`Advent of Code 2015`] is implemented below.
//!
//! [`Advent of Code 2015`]: https://adventofcode.com/2015/day/1

/// The parameterized macro can be included in two main ways (in Rust 2018):
/// The first way is by including the macro directly with `use parameterized::parameterized` (here
/// the first `parameterized` is the crate, while the second is the identifier of the attribute macro
/// itself. If you would like to shorten the identifier you can alias the name by doing
/// `use parameterized::parameterized as alternative_name` instead.
/// The second way is by using the macro_use + extern crate definition as used before Rust 2018:
/// `#[cfg(test)] #[macro_use] extern crate parameterized;` This method has the advantage that now
/// the identifier is already in scope for all test modules, which is convenient if the macro is used
/// in many different modules.
#[cfg(test)]
#[macro_use]
extern crate parameterized;

/// Now let's help Santa! We have to help him navigate a large apartment building. He has received
/// instructions on how to navigate the building. The instruction `(` stands for 'go one floor up`
/// and `)` stands for 'go one floor down`.
///
/// This is part 1 of the Advent of Code 2015, day 1.
pub fn go_to_apartment_level(instructions: &str) -> Result<isize, NorthPoleError> {
    instructions
        .chars()
        .try_fold(0isize, |acc, instruction| match instruction {
            '(' => Ok(acc + 1),
            ')' => Ok(acc - 1),
            instr => Err(NorthPoleError::InvalidInstruction(instr)),
        })
}

#[derive(Debug, PartialEq)]
pub enum NorthPoleError {
    InvalidInstruction(char),
}

/// The sub-module containing our test cases, because we specified `#[cfg(test)] #[macro_use] extern crate parameterized;`
/// at the top-level scope of our module, the `parameterized` macro will be in scope within this module.
#[cfg(test)]
mod part1 {
    use crate::{go_to_apartment_level, NorthPoleError};

    /// Let's define a test for the first part using the regular `#[test]` attribute.
    #[test]
    fn a_regular_test_for_comparison() {
        let level = go_to_apartment_level("()");
        assert_eq!(level.unwrap(), 0isize);
    }

    /// And now, let us define a parameterized test which will test all given examples. For good
    /// measure, we'll also add the empty case where we'll assume, no instructions means the ground
    /// floor.
    #[parameterized(
        input = {
            "(())",
            "()()",
            "(((",
            "(()(()(",
            "))(((((",
            "())",
            "))(",
            ")))",
            ")())())",
            ""
        },
        expected_level = {
            0,
            0,
            3,
            3,
            3,
            -1,
            -1,
            -3,
            -3,
            0
        },
    )]
    fn go_to_apartment_level_test(input: &str, expected_level: isize) {
        let level = go_to_apartment_level(input);
        assert_eq!(level.unwrap(), expected_level);
    }

    #[parameterized(
        input = {
            " ",                                // a space
            "()()() ()()",                      // a space surround by valid tokens
            "\t",                               // a tab
            "()()()\0",                         // nul terminated string
            "⬆"                                 // an up-pointing arrow
        },
        expected_error = {
            NorthPoleError::InvalidInstruction(' '),
            NorthPoleError::InvalidInstruction(' '),
            NorthPoleError::InvalidInstruction('\t'),
            NorthPoleError::InvalidInstruction('\0'),
            NorthPoleError::InvalidInstruction('⬆'),
        },
    )]
    fn go_to_apartment_level_invalid_token(input: &str, expected_error: NorthPoleError) {
        let level = go_to_apartment_level(input);
        assert_eq!(level.unwrap_err(), expected_error);
    }
}

/// In the second part we have to find the first followed instruction where Santa entered the basement (at level -1).
/// The first instruction is numbered 1. This time around, we won't error for an invalid token as instruction.
/// Instead we'll assume it means we can't follow the instruction and thus not find the basement.
pub fn entered_basement_at_instruction(instructions: &str) -> Option<usize> {
    instructions
        .chars()
        .scan(0isize, |state, instruction| match instruction {
            '(' => {
                *state += 1;
                Some(*state)
            }
            ')' => {
                *state -= 1;
                Some(*state)
            }
            _ => None,
        }) // calculate at what floor Santa would be if he would still be following the instructions
        .enumerate()
        .find(|(_i, floor)| *floor == -1) // find first time Santa would've been in the basement
        .map(|(i, _floor)| i + 1) // first instruction is numbered 1
}

#[cfg(test)]
mod part2 {
    use crate::entered_basement_at_instruction;

    #[parameterized(
        input = {
            ")",
            "())",
            "((())))",
            ")()",
            "()())"
        },
        first_positive_instruction = {
            1,
            3,
            7,
            1,
            5,
        },
    )]
    fn entered_basement_at_instruction_test(input: &str, first_positive_instruction: usize) {
        let instruction = entered_basement_at_instruction(input);

        assert_eq!(instruction.unwrap(), first_positive_instruction);
    }

    #[parameterized(
    input = {
        "(",
        "(())",
        "",
    })]
    fn never_entered_basement(input: &str) {
        let instruction = entered_basement_at_instruction(input);

        assert!(instruction.is_none());
    }
}
