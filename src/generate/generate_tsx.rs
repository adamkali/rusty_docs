pub trait GenerateTsx {
    fn generate(&self) -> String;
}

pub fn wrapper(function_name: String, complete: String) -> String {
    return format!(
        r#"
    import React from "react";
    export default function {}(): JSX.Element {{
        return(
            <>
                {}
            </>
        )
    }}
    "#,
        function_name, complete
    )
    .to_string();
}

pub fn title(head: String, classname: String) -> String {
    return format!(
        r#"
        <div>
            <h1 className={{"{}"}}>{}</h1>
        </div>
    "#,
        classname, head
    );
}

pub fn divwrap(classname: String, code: String) -> String {
    return format!(
        r#"
        <div className={{"{}"}}>
            {}
        </div>
    "#,
        classname, code
    )
    .to_string();
}

pub fn tag_wrap(tag: String, classname: String, value: String) -> String {
    return format!(
        r#"
        <{} className={{"{}"}}>{}</{}>
    "#,
        tag, classname, value, tag
    );
}

pub fn add_css_tag(classname: String, classbody: String) -> String {
    return format!(
        r#"
    .{}{{
    {}
    }}
    "#,
        classname, classbody
    );
}
