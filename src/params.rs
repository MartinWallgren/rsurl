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

pub fn parse(items: Vec<&str>) -> Result<Vec<Param>, Box<dyn std::error::Error>> {
    let mut params = Vec::new();

    for item in items {
        match parse_param(item) {
            Ok(p) => params.push(p),
            Err(e) => return Err(e),
        };
    }

    Ok(params)
}

pub fn parse_param(param: &str) -> Result<Param, Box<dyn std::error::Error>> {
    let mut key = String::new();
    let mut param_type: Option<ParamType> = None;
    let mut chars = param.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '=' {
            if let Some(&'=') = chars.peek() {
                param_type = Some(ParamType::QUERY);
                chars.next();
            } else {
                param_type = Some(ParamType::DATA);
            }
            break;
        }
        if c == ':' {
            param_type = Some(ParamType::HEADER);
            break;
        }
        key.push(c);
    }

    if param_type.is_none() {
        return Err("Unable to parse item.".into());
    }

    let value: String = chars.collect();
    Ok(Param {
        key: key.trim().to_owned(),
        value: value.trim().to_owned(),
        param_type: param_type.unwrap(),
    })
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
        assert_eq!(parse_param(arg).unwrap(), expected);
    }

    #[test]
    fn test_parse_header_2() {
        let arg = "key:multiple=divider:ignored";
        let expected = Param {
            key: String::from("key"),
            value: String::from("multiple=divider:ignored"),
            param_type: ParamType::HEADER,
        };
        assert_eq!(parse_param(arg).unwrap(), expected);
    }

    #[test]
    fn test_parse_data() {
        let arg = "key=val";
        let expected = Param {
            key: String::from("key"),
            value: String::from("val"),
            param_type: ParamType::DATA,
        };
        assert_eq!(parse_param(arg).unwrap(), expected);
    }

    #[test]
    fn test_parse_data_trim() {
        let arg = "key = val";
        let expected = Param {
            key: String::from("key"),
            value: String::from("val"),
            param_type: ParamType::DATA,
        };
        assert_eq!(parse_param(arg).unwrap(), expected);
    }

    #[test]
    fn test_parse_query_param_trim() {
        let arg = "key == val";
        let expected = Param {
            key: String::from("key"),
            value: String::from("val"),
            param_type: ParamType::QUERY,
        };
        assert_eq!(parse_param(arg).unwrap(), expected);
    }

    #[test]
    fn test_parse_query_param() {
        let arg = "key==val";
        let expected = Param {
            key: String::from("key"),
            value: String::from("val"),
            param_type: ParamType::QUERY,
        };
        assert_eq!(parse_param(arg).unwrap(), expected);
    }

    #[test]
    fn test_parse_empty() {
        let arg = "";
        assert!(
            parse_param(arg).is_err(),
            "Empty param should result in error"
        );
    }

    #[test]
    fn test_parse_missing() {
        let arg = "paramwithouthdivider";
        assert!(
            parse_param(arg).is_err(),
            "Missing divider should result in error"
        );
    }
}
