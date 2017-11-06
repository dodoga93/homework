extern crate rand;

use std::result;
use std::io;
use self::rand::Rng;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
/// An element of the stack. May be either integer or boolean.
pub enum Elt {
    Int(i32),
    Bool(bool),
}

#[derive(Debug)]
/// An RPN calculator error.
pub enum Error {
    /// Tried to pop from an empty stack.
    Underflow,
    /// Tried to operate on invalid types.
    Type,
    /// Unable to parse the input.
    Syntax,
    /// Some IO error occurred.
    IO(io::Error),
    /// The user quit the program (with `quit`).
    Quit,
}

#[derive(Debug)]
/// Types of RPN calculator operations.
pub enum Op {
    /// Adds two numbers.
    Add,
    /// Checks equality of two values.
    Eq,
    /// Negates a value: pop x, push ~x.
    Neg,
    /// Swaps two values.
    Swap,
    /// Computes a random number: pop x, push random number in [0, x).
    Rand,
    /// Quit the calculator.
    Quit,
}

// TODO: Stack.
pub struct Stack {
    elems: Vec<Elt>,
}

// TODO: Result.
pub type Result<T> = result::Result<T, Error>;

impl Stack {
    /// Creates a new Stack
    pub fn new() -> Stack {
        Stack{ elems: vec![] }
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, val: Elt) -> Result<()> {
        Ok(self.elems.push(val))
    }

    /// Tries to pop a value off of the stack.
    pub fn pop(&mut self) -> Result<Elt> {
        self.elems.pop().ok_or(Error::Underflow)
    }

    /// Tries to evaluate an operator using values on the stack.
    pub fn eval(&mut self, op: Op) -> Result<()> {
        match op {
            Op::Add => self.add(),
            Op::Eq => self.eq(),
            Op::Neg => self.neg(),
            Op::Swap => self.swap(),
            Op::Rand => self.rand(),
            Op::Quit => Err(Error::Quit),
        }
    }

    fn is_int(val: Elt) -> Result<i32> {
        if let Elt::Int(i) = val {
            Ok(i)
        } else {
            Err(Error::Type)
        }
    }

    fn is_bool(val:Elt) -> Result<bool> {
        if let Elt::Bool(b) = val {
            Ok(b)
        } else {
            Err(Error::Type)
        }
    }

    fn pop_and_check_int(&mut self) -> Result<i32> {
        self.pop().and_then( |res| { Stack::is_int(res) } )
    }

    fn pop_and_check_bool(&mut self) -> Result<bool> {
        self.pop().and_then( |res| { Stack::is_bool(res) } )
    }
    
    fn add(&mut self) -> Result<()> {
        self.pop_and_check_int().and_then(
            |first:i32| {
                self.pop_and_check_int().and_then(
                    |second:i32| {
                        self.push(Elt::Int(first+second))
                    }
                )}
        )
    }

    fn neg(&mut self) -> Result<()> {
        self.pop().and_then(
            |val:Elt| {
                match val {
                    Elt::Int(i) => self.push(Elt::Int(-i)),
                    Elt::Bool(b) => self.push(Elt::Bool(!b)),
                }
            }
        )
    }

    fn eq(&mut self) -> Result<()> {
        self.pop().and_then(
            |val:Elt| {
                match val {
                    Elt::Int(first) => self.pop_and_check_int().and_then(
                        |second| self.push(Elt::Bool(first == second))
                    ),
                    Elt::Bool(first) => self.pop_and_check_bool().and_then(
                        |second| self.push(Elt::Bool(first == second))
                    ), 
                }
            }
        )
    }

    fn swap(&mut self) -> Result<()> {
        self.pop().and_then(
            |first| {
                self.pop().and_then(
                    |second| {
                        self.push(first).and_then(
                            |_| self.push(second)
                        )
                    }
                )
            }
        )
    }

    fn rand(&mut self) -> Result<()> {
        self.pop_and_check_int().and_then(
            |upper_bound| {
                self.push(Elt::Int(
                    rand::thread_rng().gen_range::<i32>(0, upper_bound)
                ))
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_empty1() {
        let mut s = Stack::new();

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_pop_empty2() {
        let mut s = Stack::new();
        s.push(Elt::Int(0)).unwrap();

        let res = s.pop();
        assert!(res.is_ok());

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_add1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Add).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(2));
    }

    #[test]
    fn test_eval_add2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_add3() {
        let mut s = Stack::new();
        s.push(Elt::Bool(true)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_eq1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Eq).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_eq2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Eq);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_neg1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(-1));
    }

    #[test]
    fn test_eval_neg2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_swap1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        assert!(s.eval(Op::Swap).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(1));
        assert_eq!(s.pop().unwrap(), Elt::Bool(false));

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_swap2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Swap);
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_rand1() {
        let mut s = Stack::new();
        let i = 20;
        s.push(Elt::Int(i)).unwrap();

        assert!(s.eval(Op::Rand).is_ok());

        let rand_val = s.pop().unwrap();
        assert!(rand_val >= Elt::Int(0));
        assert!(rand_val < Elt::Int(i));
    }

    #[test]
    fn test_eval_rand2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Rand);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_quit() {
        let mut s = Stack::new();

        let res = s.eval(Op::Quit);
        assert!(res.is_err());
        if let Err(Error::Quit) = res { } else { assert!(false); }
    }
}
