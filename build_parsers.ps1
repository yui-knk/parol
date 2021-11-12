cargo run --bin parol -- -f ./src/parser/parol-grammar.par -e ./src/parser/parol-grammar-exp.par -p ./src/parser/parol_parser.rs -a ./src/parser/parol_grammar_trait.rs -t ParolGrammar -m parser::parol_grammar
cargo run --bin parol -- -f ./examples/calc/calc.par -e ./examples/calc/calc-exp.par -p ./examples/calc/calc_parser.rs -a ./examples/calc/calc_grammar_trait.rs -t CalcGrammar -m calc_grammar
# cargo run --bin parol -- -f ./examples/json/json.par -e ./examples/json/json-exp.par -p ./examples/json/json_parser.rs -a ./examples/json/json_grammar_trait.rs -t JsonGrammar -m json_grammar
cargo run --bin parol -- -f ./examples/list/list.par -e ./examples/list/list-exp.par -p ./examples/list/list_parser.rs -a ./examples/list/list_grammar_trait.rs -t ListGrammar -m list_grammar
cargo run --bin parol -- -f ./examples/oberon_0/oberon_0.par -e ./examples/oberon_0/oberon_0-exp.par -p ./examples/oberon_0/oberon_0_parser.rs -a ./examples/oberon_0/oberon_0_grammar_trait.rs -t Oberon0Grammar -m oberon_0_grammar


cargo run --bin parol -- -f .\examples\scanner_states\scanner_states.par -e ./examples/scanner_states/scanner_states-exp.par -p ./examples/scanner_states/scanner_states_parser.rs -a ./examples/scanner_states/scanner_states_grammar_trait.rs -t ScannerStatesGrammar -m scanner_states_grammar
