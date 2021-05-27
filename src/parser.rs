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
struct PartialParse<'a> {
   pub term: Term,
   pub rest: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
   Generic { msg: String },
}

type ParseResult<'a> = Result<PartialParse<'a>, ParseError>;

pub fn parse(_input: &str) -> Result<Term, ParseError> {
   return Ok(Term::Nil);
}

fn parse_nil<'a>(input: &'a str) -> ParseResult<'a> {
   let input = skip_whitespace(input);
   if input.starts_with("nil") {
      return Ok(PartialParse {
         term: Term::Nil,
         rest: &input[3..],
      });
   }

   Err(ParseError::Generic {
      msg: "not a nil".to_string(),
   })
}

fn skip_one_of(chars: &str) -> Box<dyn for<'a> Fn(&'a str) -> ParseResult<'a>> {
   let chars = String::from(chars);
   Box::new(move |input: & str| {
      if chars.contains(&input[0..1]) {
         return ParseResult::Ok(PartialParse {
            term: Term::Empty,
            rest: &input[1..],
         });
      }

      ParseResult::Err(ParseError::Generic {
         msg: "no match".to_string(),
      })
   })
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
      assert_eq!(result.is_ok(), true);
      assert_eq!(result.unwrap(), PartialParse{ term: Term::Nil, rest: " "});
   }
}
