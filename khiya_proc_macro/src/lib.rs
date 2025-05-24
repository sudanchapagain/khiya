use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use proc_macro::TokenStream as ProcTokenStream;

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "हो" | "ho" => "true",
        "होइन" | "hoina" => "false",
        "प्रक्रिया" | "prakriya" => "fn",
        "परिभाषा" | "paribhasa" => "let",
        "यदि" | "yedi" => "if",
        "वा" | "wa" => "else",
        "रचना" | "rachana" => "struct",
        "विकल्प" | "bikalpa" => "enum",
        "सार्वजनिक" | "sarbajanik" | "sar" => "pub",
        "खण्ड" | "khand" => "mod",
        "प्रयोग" | "prayog" => "use",
        "विच्छेद" | "vichhed" => "break",
        "जारी" | "jari" => "continue",
        "फर्क" | "pharka" => "return",
        "आफू" | "aafu" => "self",
        "माथिल्लो" | "mathillo" | "mat" => "super",
        "स्थिर" | "sthir" => "static",
        "निश्चित" | "nischit" => "const",
        "मुख्य" | "mukhya" => "main",
        "जबसम्म" | "jabasamma" => "while",
        "परिवर्तनशील" | "parivartanshil" | "par" => "mut",
        "छाप" | "chap" => "print",
        "छापपङ्क्ति" | "chappankti" => "println", // chap + pankti. need a good replacement
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn nepali(input: ProcTokenStream) -> ProcTokenStream {
    let input: TokenStream = input.into();
    let mut returned = Vec::new();
    replace_stream(input, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out.into()
}
