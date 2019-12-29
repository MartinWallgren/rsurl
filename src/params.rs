#[derive(PartialEq, Eq, Debug)]
pub enum ParamType {
    HEADER,
    QUERY,
    DATA,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Param {
    pub key: String,
    pub value: String,
    pub param_type: ParamType,
}

pub fn parse(param: &str) -> Result<Param, Box<dyn std::error::Error>> {
    let mut keyval = param.splitn(2, |c| c == ':' || c == '=');

    let key = keyval.next();
    let val = keyval.next();
    match [key, val] {
        [Some(k), Some(v)] => Ok(Param {
            key: k.to_owned(),
            value: v.to_owned(),
            param_type: ParamType::HEADER,
        }),
        _ => Err("Oh noes".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header() {
        let arg = "key:val";
        let expected = Param {
            key: String::from("key"),
            value: String::from("val"),
            param_type: ParamType::HEADER,
        };
        assert_eq!(parse(arg).unwrap(), expected);
    }

    #[test]
    fn test_parse_header_2() {
        let arg = "key:multiple=divider:ignored";
        let expected = Param {
            key: String::from("key"),
            value: String::from("multiple=divider:ignored"),
            param_type: ParamType::HEADER,
        };
        assert_eq!(parse(arg).unwrap(), expected);
    }

    #[test]
    fn test_parse_empty() {
        let arg = "";
        assert!(parse(arg).is_err(), "Empty param should result in error");
    }

    #[test]
    fn test_parse_missing() {
        let arg = "paramwithouthdivider";
        assert!(
            parse(arg).is_err(),
            "Missing divider should result in error"
        );
    }
}
