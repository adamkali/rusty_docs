use crate::generate::generate_tsx as gtsx;
use serde::{Deserialize, Serialize};

/**
    ErrorTypeCause

    This type deals with entries that could cause an ErrorType provided
    in the filed provided.

    # Attributes

    * `name` The name given to the cause of the ErrorTypeCause.
    * `cause` A cause as to why the ErrorType id happening. If none is
        known then it would be condusive to collaboration to use tags like
        <em>FIXME</em> or <em>TODO</em>.
    * `A Fix to this either on the client side or on development side.`
        If none is known then it would be condusive to collaboration to
        use tags like <em>FIXME</em> or <em>TODO</em>.

*/
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ErrorTypeCause {
    pub name: String,
    pub cause: String,
    pub fix: String,
}

impl ErrorTypeCause {
    pub fn new() -> ErrorTypeCause {
        return ErrorTypeCause {
            name: "New Cause".to_string(),
            cause: "<em>FIXME</em>".to_string(),
            fix: "<em>TODO</em>".to_string(),
        };
    }
}

impl gtsx::GenerateTsx for ErrorTypeCause {
    fn generate(&self) -> String {
        return gtsx::divwrap(
            "".to_string(),
            gtsx::tag_wrap("b".to_string(), "".to_string(), self.name.to_string())
                + &gtsx::tag_wrap("p".to_string(), "".to_string(), self.cause.to_string())
                + &gtsx::tag_wrap("p".to_string(), "".to_string(), self.fix.to_string()),
        );
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Causes {
    C(ErrorTypeCause),
    Cs(Vec<ErrorTypeCause>),
}

/**
    ErrorType

    This Type deals with entries that are specified in the json
    as "type": "error". This primarly should be used not denote a
    runtime or compile time error, but for error codes that a user
    might run into. Primarily for finding out what why they are getting
    error for customer service.

    # Attributes

    * `name` Whatever the name of the of the error you want to identify.
    * `causes` Vec<ErrorTypeCause> That could be responsible for why the
            Error is coming up.
    * `explanation` Explains what is happening and if it is being handled.
            Could be benifitial to add tags like <em>TODO</em> to further
            document the code base.
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ErrorType {
    pub name: String,
    pub causes: Causes,
    pub explanation: String,
}

impl ErrorType {
    pub fn new() -> ErrorType {
        return ErrorType {
            name: "Oh No!".to_string(),
            causes: Causes::Cs(vec![ErrorTypeCause::new()]),
            explanation: "Joe needs to implement changes".to_string(),
        };
    }
}

impl gtsx::GenerateTsx for ErrorType {
    fn generate(&self) -> String {
        let mut causes: String = "".to_string();
        match &self.causes {
            Causes::C(c) => { causes += &c.generate(); }
            Causes::Cs(cs) => {
                for i in 0..cs.len() {
                    causes += &cs[i].generate();
                }
            }
        }
        return gtsx::divwrap(
            "".to_string(),
            gtsx::tag_wrap("h3".to_string(), "".to_string(), self.name.to_string())
                + &causes
                + &gtsx::tag_wrap(
                    "p".to_string(),
                    "".to_string(),
                    format!("Explanation: {}", self.explanation.to_string()),
                ),
        );
    }
}
