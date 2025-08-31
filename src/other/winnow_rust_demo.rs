/**
 * 在使用winnow解析字符串时，建议去除掉原内容里的空格和换行符，有注释就要去除注释。否则解析就会变得很困难
 *  */
#[cfg(test)]
mod winnow_rust_test {
    use winnow::{
        Parser,
        error::InputError,
        token::{literal, take_till, take_until},
    };

    type PResult<'a, T> = Result<T, InputError<&'a str>>;

    #[derive(Debug, PartialEq)]
    enum RustCodeElement {
        Function(String),
        Struct(String),
        Enum(String),
        Impl(Box<RustCodeElement>),
        Trait(String),
        Mod(Box<RustCodeElement>),
        Use(String),
    }

    #[test]
    fn test() {
        let content = std::fs::read_to_string("src/other/winnow_rust.txt").unwrap();
        let mut temp = content.as_str();

        let mut element_list = Vec::new();
        while !temp.is_empty() {
            if temp.starts_with("/*") || temp.starts_with("//") {
                split_comment(&mut temp).unwrap();
            } else if temp.starts_with("use") {
                element_list.push(parse_use(&mut temp).unwrap());
            } else if temp.starts_with("impl") {
                element_list.push(parse_impl(&mut temp).unwrap());
            } else if temp.starts_with("trait") {
                element_list.push(parse_trait(&mut temp).unwrap());
            } else if temp.starts_with("pub mod") {
                element_list.push(parse_mod(&mut temp).unwrap());
            } else if temp.starts_with("pub struct") {
                element_list.push(parse_base_element(&mut temp, "struct").unwrap());
            }
        }

        for item in element_list {
            println!("==========================");
            println!("{:?}", item);
        }
        // skip_whitespace(&mut temp).unwrap();
        println!("{}", temp);
    }

    /**
     * 去空格
     */
    // fn skip_whitespace<'a>(input: &mut &'a str) -> PResult<'a, ()> {
    //     take_while(0.., |c| c == ' ' || c == '\t' || c == '\n' || c == '\r')
    //         .parse_next(input)?;
    //     Ok(())
    // }

    /**
     * 去除注释
     */
    fn split_comment<'a>(code: &mut &'a str) -> PResult<'a, ()> {
        if code.starts_with("//") {
            literal("//").parse_next(code)?;
            take_till(0.., '\n').parse_next(code)?;
            // 跳过换行符
            literal("\n").parse_next(code)?;
        } else if code.starts_with("/*") {
            literal("/*").parse_next(code)?;
            take_until(0.., "*/").parse_next(code)?;
            literal("*/").parse_next(code)?;
        }
        Ok(())
    }

    fn parse_base_element<'a>(
        code: &mut &'a str,
        element_type: &'a str,
    ) -> PResult<'a, RustCodeElement> {
        let prefix = match element_type {
            "function" => "pub fn",
            "struct" => "pub struct",
            "enum" => "pub enum",
            _ => "",
        };
        let content = if code.starts_with(prefix) {
            let fun = take_till(0.., '}').parse_next(code)?;
            println!("parse_base_element = {}", fun);
            fun.to_string()
        } else {
            "".to_string()
        };
        literal("}").parse_next(code)?;
        // 跳过换行符
        literal("\n").parse_next(code)?;
        let result = match element_type {
            "function" => RustCodeElement::Function(content),
            "struct" => RustCodeElement::Struct(content),
            "enum" => RustCodeElement::Enum(content),
            _ => RustCodeElement::Function(content),
        };
        Ok(result)
    }

    fn parse_impl<'a>(code: &mut &'a str) -> PResult<'a, RustCodeElement> {
        if code.starts_with("impl") {
            let mut impl_ = take_until(0.., "end").parse_next(code)?;
            println!("parse_impl = {}", impl_);
            return Ok(RustCodeElement::Impl(Box::new(parse_base_element(
                &mut impl_, "function",
            )?)));
        }
        literal("end").parse_next(code)?;
        // 跳过换行符
        literal("\n").parse_next(code)?;
        Ok(RustCodeElement::Impl(Box::new(RustCodeElement::Function(
            "".to_string(),
        ))))
    }

    fn parse_use<'a>(code: &mut &'a str) -> PResult<'a, RustCodeElement> {
        if code.starts_with("use") {
            let use_ = take_till(0.., '\n').parse_next(code)?;
            // 跳过换行符
            literal("\n").parse_next(code)?;
            return Ok(RustCodeElement::Use(use_.to_string()));
        }
        Ok(RustCodeElement::Use("".to_string()))
    }

    fn parse_trait<'a>(code: &mut &'a str) -> PResult<'a, RustCodeElement> {
        if code.starts_with("trait") {
            let trait_ = take_till(0.., '\n').parse_next(code)?;
            // 跳过换行符
            literal("\n").parse_next(code)?;
            return Ok(RustCodeElement::Trait(trait_.to_string()));
        }
        Ok(RustCodeElement::Trait("".to_string()))
    }

    fn parse_mod<'a>(code: &mut &'a str) -> PResult<'a, RustCodeElement> {
        if code.starts_with("mod") {
            let mod_ = take_till(0.., '}').parse_next(code)?;
            // 跳过换行符
            literal("\n").parse_next(code)?;
            return Ok(RustCodeElement::Mod(Box::new(RustCodeElement::Function(
                mod_.to_string(),
            ))));
        }
        Ok(RustCodeElement::Mod(Box::new(RustCodeElement::Function(
            "".to_string(),
        ))))
    }
}
