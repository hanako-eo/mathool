mod expr;

peg::parser! {
    grammar mathool_lang() for str {
        // whitespaces
        rule ws() = quiet!{[' ' | '\n' | '\t']*}

        rule number() -> i64
            = n:$("-"? ['0'..='9']+) {?
                n.parse().or(Err("i64"))
            }

        rule symbol() -> String
            = symbol:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) {
                symbol.to_owned()
            }

        rule function_call() -> (String, Vec<expr::Expr>)
            = name:symbol() "(" ws() exprs:(expr() ** (ws() "," ws())) ws() ")" {
                (name, exprs)
            }

        rule function_definition() -> (String, Vec<String>)
            = name:symbol() ws() ":" ws() "("? ws() symbols:(symbol() ** (ws() "," ws())) ws() ")"? ws() "->" ws() body:expr() {
                (name, symbols)
            } /
            name:symbol() "(" ws() symbols:(symbol() ** (ws() "," ws())) ws() ")" ws() "=" ws() body:expr() {
                (name, symbols)
            }

        pub rule expr() -> expr::Expr
            = precedence!{
                x:(@) ws() "+" ws() y:@ { expr::Expr::Add(Box::new(x), Box::new(y)) }
                x:(@) ws() "-" ws() y:@ { expr::Expr::Sub(Box::new(x), Box::new(y)) }
                --
                x:(@) ws() "*"? ws() y:@ { expr::Expr::Mul(Box::new(x), Box::new(y)) }
                x:(@) ws() "/" ws() y:@ { expr::Expr::Div(Box::new(x), Box::new(y)) }
                --
                x:(@) ws() "^" ws() y:@ { expr::Expr::Pow(Box::new(x), Box::new(y)) }
                --
                "(" e:expr() ")" { e }
                "|" e:expr() "|" { expr::Expr::Abs(Box::new(e)) }
                s:symbol() { expr::Expr::Symbol(s) }
                n:number() { expr::Expr::Number(n) }
                f:function_call() { expr::Expr::FunctionCall(f.0, f.1) }
                f:function_definition() { expr::Expr::FunctionDefinition(f.0, f.1) }
            }
    }
}
