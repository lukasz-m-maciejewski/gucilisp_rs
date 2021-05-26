#[derive(Debug, PartialEq)]
pub enum Term {
   Empty,
   Nil,
   Number { v: i32 },
   GLString,
   Boolean,
   Cons { car: Box<Term>, cdr: Box<Term> },
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
   Generic { msg: String },
}

pub fn parse(input: &str) -> (&str, Result<Term, ParseError>) {
   return (input, Ok(Term::Nil));
}

pub fn parse_nil(input: &str) -> (&str, Result<Term, ParseError>) {
   let input = skip_whitespace(input);
   if input.starts_with("nil") {
      return (
         &input[3..],
         Ok(Term::Nil)
      );
   } 

   return (
      input,
      Err(ParseError::Generic {
         msg: String::from("not a nil"),
      }),
   );
}

fn skip_whitespace(input: &str) -> &str {
   return input;
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn parse_nil_success() {
      let result = parse_nil("nil ");
      assert_eq!(result.0, " ");
      assert_eq!(result.1.is_ok(), true);
      assert_eq!(result.1.unwrap(), Term::Nil);
   }
}