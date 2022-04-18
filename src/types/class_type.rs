use crate::types::doc_type::{FuncType, FuncTypeParam, Params};
use crate::generate::generate_tsx as gtsx;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Methods {
    M(FuncType),
    Ms(Vec<FuncType>)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ClassType {
    class_name: String,
    description: String,
    class_variables: Params,
    methods: Methods
}

impl gtsx::GenerateTsx for ClassType {
    fn generate(&self) {
        let methods_string = "";
        match self.methods {
            M(m) => { methods_string += &m.generate(); }
            Ms(m) => {
                for i in 0..m {
                    methods_string += &m.generate();
                }
            }
        }
        let params_string = "";
        match self.params {
            P(p) => { params_string += &p.generate(); }
            Ps(p) => {
                for i in 0..p.len() {
                    params_string += &p.generate();
                }
            }
        }
        return gtsx::divwrap("",
            &gtsx::tag_wrap("b", "", self.class_name )
            + &gtsx::tag_wrap("em","",self.description)
            + &params_string + &methods_string
        )
    }
}
