// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketDescription` type,
//   enforcing that the description is not empty and is not longer than 500 characters.
//   Implement the traits required to make the tests pass too.
use thiserror;

#[derive(Clone, Debug, PartialEq)]
pub struct TicketDescription(String);

#[derive(thiserror::Error, Clone, Debug)]
pub enum InvalidDescription{
    #[error("The description cannot be empty")]
    EmptyDescription,
    #[error("The description cannot be longer than 500 characters")]
    TooLongDescription
}

impl TryFrom<String> for TicketDescription{
    type Error = InvalidDescription;
    
    fn try_from(s: String) -> Result<TicketDescription, Self::Error>{
        if s.is_empty(){
            return Err(InvalidDescription::EmptyDescription)
        }
        if s.len() > 500{
            return Err(InvalidDescription::TooLongDescription)
        }
        Ok(Self(s))
    }
}

impl TryFrom<&str> for TicketDescription{
    type Error = InvalidDescription;

    fn try_from(s: &str) -> Result<TicketDescription, Self::Error>{
        TicketDescription::try_from(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let description = TicketDescription::try_from("A description".to_string()).unwrap();
        assert_eq!(description.0, "A description");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketDescription::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The description cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let description = "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.".to_string();
        let err = TicketDescription::try_from(description).unwrap_err();
        assert_eq!(
            err.to_string(),
            "The description cannot be longer than 500 characters"
        );
    }

    #[test]
    fn test_try_from_str() {
        let description = TicketDescription::try_from("A description").unwrap();
        assert_eq!(description.0, "A description");
    }
}
