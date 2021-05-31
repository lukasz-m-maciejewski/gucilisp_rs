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

type Parser = fn(&str) -> ParseResult;

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

type BoxedParser = Box<dyn for<'a> Fn(&'a str) -> ParseResult<'a>>;

fn skip_one_of(chars: &str) -> BoxedParser {
   let chars = String::from(chars);
   Box::new(move |input: &str| {
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

#[cfg(test)]
mod skip_one_of_tests {
   use super::*;

   #[test]
   fn successful_skip() {
      let result = skip_one_of("abc")("bgh");
      assert!(result.is_ok());
      assert_eq!(
         result.unwrap(),
         PartialParse {
            term: Term::Empty,
            rest: "gh"
         }
      );
   }
}

fn kleene_star(op: BoxedParser) -> BoxedParser {
   Box::new(move |input: &str| {
      let mut rest_of_input = input;
      loop {
         let result = op(rest_of_input);
         match result {
            Ok(partial) => {
               rest_of_input = partial.rest;
            }
            Err(_) => break,
         }
      }

      ParseResult::Ok(PartialParse {
         term: Term::Empty,
         rest: rest_of_input,
      })
   })
}

#[cfg(test)]
mod kleene_star_test {
   use super::*;

   #[test]
   fn kleene_star_success() {
      let result = kleene_star(skip_one_of("abc"))("ababccefg");
      assert!(result.is_ok());
      assert_eq!(
         result.unwrap(),
         PartialParse {
            term: Term::Empty,
            rest: "efg"
         }
      );
   }

   #[test]
   fn kleene_star_unchanged_when_no_matches() {
      let result = kleene_star(skip_one_of("abc"))("ezefg");
      assert!(result.is_ok());
      assert_eq!(
         result.unwrap(),
         PartialParse {
            term: Term::Empty,
            rest: "ezefg"
         }
      );
   }
}

fn skip_whitespace(input: &str) -> &str {
   kleene_star(skip_one_of(" \t\n"))(input).unwrap().rest
}

#[cfg(test)]
mod skip_whitespace_test {
   use super::*;
   #[test]
   fn success() {
      let result = skip_whitespace("   abc");
      assert_eq!(result, "abc");
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn parse_nil_success() {
      let result = parse_nil("nil ");
      assert_eq!(result.is_ok(), true);
      assert_eq!(
         result.unwrap(),
         PartialParse {
            term: Term::Nil,
            rest: " "
         }
      );
   }
}
