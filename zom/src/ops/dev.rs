use anyhow::anyhow;
use std::fs;
use zom_fe::lexer::Lexer;
use zom_fe::parser::{parse, ParserSettings, ParsingContext};

use crate::ExitStatus;

pub fn dev() -> Result<ExitStatus, anyhow::Error> {
    println!("Development command.");

    let buffer = match fs::read_to_string("example/test_main.zom") {
        Ok(src) => src,
        Err(_) => return Err(anyhow!("Error while trying to read the source file.")),
    };

    println!("buffer = {:?}", buffer);

    let mut lexer = Lexer::new(buffer.as_str(), "<dev_cmd>.zom".to_string());
    let tokens;
    match lexer.make_tokens() {
        Ok(toks) => {
            tokens = toks;
            println!("toks = {:?}\n\n", tokens);
        }
        Err(err) => return Err(anyhow!(format!("{}", err))),
    }

    let mut parse_context = ParsingContext::new("<dev_cmd>.zom".to_owned(), buffer, tokens.clone());

    let ast_result = parse(
        tokens.as_slice(),
        &[],
        &mut ParserSettings::default(),
        &mut parse_context,
    );

    match ast_result {
        Ok((ast, rest_toks)) => {
            println!("ast = {:#?}", ast);
            println!("toks_rest = {:?}", rest_toks);
        }
        Err(err) => return Err(anyhow!(format!("{}", err))),
    }

    Ok(ExitStatus::Success)
}
