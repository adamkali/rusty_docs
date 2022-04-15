use crate::generate::generate_tsx as gtsx;
use serde::{Deserialize, Serialize};

/**
    FuncTypeParam

    The main body of the FuncType. It can describe what each of
    the params do.

    # Attributes

    * `name` The name of the attribute,
    * `type_of` describing the type of the param. If the param can have
            more than one type it would be best if the documentation
            would look something like: String | Int. So in the
            file provided by the user to RustyDocs it the value of the
            param would look like \"type_of\": \" String | Int \",
    * `explanation` This is where one would explain what the param could
            actually do when it should be String or whin it should be int.
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FuncTypeParam {
    name: String,
    type_of: String,
    explanation: String,
}

impl FuncTypeParam {
    pub fn new() -> FuncTypeParam {
        FuncTypeParam {
            name: "Param".to_string(),
            type_of: "String".to_string(),
            explanation: "TODO".to_string(),
        }
    }
}

impl gtsx::GenerateTsx for FuncTypeParam {
    fn generate(&self) -> String {
        gtsx::divwrap(
            "".to_string(),
            gtsx::tag_wrap(
                "p".to_string(),
                "".to_string(),
                gtsx::tag_wrap("b".to_string(), "".to_string(), self.name.to_string()),
            ) + &gtsx::tag_wrap("em".to_string(), "".to_string(), self.type_of.to_string())
                + &": ".to_string()
                + &self.explanation.to_string(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Params {
    P(FuncTypeParam),
    Ps(Vec<FuncTypeParam>)
}

/** FuncType

# Attributes

* `name` The name of the the documentation given in the chosen documentation.json
* `params` This can be a  Vec<FuncTypeParam> like
*   [
*      {
*          "name": "foo",
*          "type_of": "String",
*          "explanation": "does a bar"
*      },
*      {
*          name": "baz",
*          "type_of": "Int",
*          "explanation": "Does a <b>buzz</b>"
*      }
*   ]
* `returns` This shows the user what the method is supposed to return.
* `explanation` This tells the user what the function does.
* `errors` This tells the user what possible errors that this function can throw .
*/
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FuncType {
    pub name: String,
    pub params: Params,
    pub returns: String,
    pub explanation: String,
    pub errors: String,
}

impl FuncType {
    pub fn new() -> FuncType {
        FuncType {
            name: "function".to_string(),
            params: Params::Ps(vec![FuncTypeParam::new()]),
            returns: "[New Return]".to_string(),
            explanation: "".to_string(),
            errors: "[New Errors]".to_string(),
        }
    }
}

impl gtsx::GenerateTsx for FuncType {
    fn generate(&self) -> String {
        let mut params_string: String = "".to_string();
        match &self.params {
            Params::P(p) => { params_string += &p.generate() }
            Params::Ps(p) => {
                for i in 0..p.len() {
                    params_string += &p[i].generate();
                }
            }
        }
        gtsx::divwrap(
            "".to_string(),
            gtsx::tag_wrap("h2".to_string(), "".to_string(), self.name.to_string())
                + &gtsx::divwrap("".to_string(), params_string)
                + &gtsx::tag_wrap(
                    "p".to_string(),
                    "".to_string(),
                    format!("Explanation: {}", self.explanation.to_string()),
                )
                + &gtsx::tag_wrap(
                    "p".to_string(),
                    "".to_string(),
                    format!("Errors: {}", self.errors.to_string()),
                ),
        )
    }
}
