// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use id_tree::Tree;

use miette::{miette, Result};
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType, UserActionsTrait};

use crate::scanner_states_grammar::ScannerStatesGrammar;
use std::path::Path;

///
/// The `ScannerStatesGrammarTrait` trait is automatically generated for the
/// given grammar.
/// All functions have default implementations.
///
pub trait ScannerStatesGrammarTrait {
    ///
    /// Implement this method if you need the provided information
    ///
    fn init(&mut self, _file_name: &Path) {}

    /// Semantic action for production 0:
    ///
    /// Start: StartList /* Vec */;
    ///
    fn start_0(
        &mut self,
        _start_list_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 1:
    ///
    /// StartList: Content StartList; // Vec<T>::Push
    ///
    fn start_list_1(
        &mut self,
        _content_0: &ParseTreeStackEntry,
        _start_list_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 2:
    ///
    /// StartList: ; // Vec<T>::New
    ///
    fn start_list_2(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 3:
    ///
    /// Content: Identifier;
    ///
    fn content_3(
        &mut self,
        _identifier_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 4:
    ///
    /// Content: StringDelimiter %push(String) StringContent StringDelimiter %pop();
    ///
    fn content_4(
        &mut self,
        _string_delimiter_0: &ParseTreeStackEntry,
        _string_content_2: &ParseTreeStackEntry,
        _string_delimiter_3: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 5:
    ///
    /// StringContent: StringElement StringContent;
    ///
    fn string_content_5(
        &mut self,
        _string_element_0: &ParseTreeStackEntry,
        _string_content_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 6:
    ///
    /// StringContent: ;
    ///
    fn string_content_6(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 7:
    ///
    /// StringElement: Escaped;
    ///
    fn string_element_7(
        &mut self,
        _escaped_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 8:
    ///
    /// StringElement: EscapedLineEnd;
    ///
    fn string_element_8(
        &mut self,
        _escaped_line_end_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 9:
    ///
    /// StringElement: NoneQuote;
    ///
    fn string_element_9(
        &mut self,
        _none_quote_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 10:
    ///
    /// Identifier: "[a-zA-Z_]\w*";
    ///
    fn identifier_10(
        &mut self,
        _identifier_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 11:
    ///
    /// Escaped: <String>"\u{5c}[\u{22}\u{5c}bfnt]";
    ///
    fn escaped_11(
        &mut self,
        _escaped_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 12:
    ///
    /// EscapedLineEnd: <String>"\u{5c}[\s^\n\r]*\r?\n";
    ///
    fn escaped_line_end_12(
        &mut self,
        _escaped_line_end_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 13:
    ///
    /// NoneQuote: <String>"[^\u{22}\u{5c}]+";
    ///
    fn none_quote_13(
        &mut self,
        _none_quote_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 14:
    ///
    /// StringDelimiter: <INITIAL, String>"\u{22}";
    ///
    fn string_delimiter_14(
        &mut self,
        _string_delimiter_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }
}

impl UserActionsTrait for ScannerStatesGrammar {
    ///
    /// Initialize the user with additional information.
    /// This function is called by the parser before parsing starts.
    /// Is is used to transport necessary data from parser to user.
    ///
    fn init(&mut self, _file_name: &Path) {}

    ///
    /// This function is implemented automatically for the user's item ScannerStatesGrammar.
    ///
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[ParseTreeStackEntry],
        parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        match prod_num {
            0 => self.start_0(&children[0], parse_tree),
            1 => self.start_list_1(&children[0], &children[1], parse_tree),
            2 => self.start_list_2(parse_tree),
            3 => self.content_3(&children[0], parse_tree),
            4 => self.content_4(&children[0], &children[1], &children[2], parse_tree),
            5 => self.string_content_5(&children[0], &children[1], parse_tree),
            6 => self.string_content_6(parse_tree),
            7 => self.string_element_7(&children[0], parse_tree),
            8 => self.string_element_8(&children[0], parse_tree),
            9 => self.string_element_9(&children[0], parse_tree),
            10 => self.identifier_10(&children[0], parse_tree),
            11 => self.escaped_11(&children[0], parse_tree),
            12 => self.escaped_line_end_12(&children[0], parse_tree),
            13 => self.none_quote_13(&children[0], parse_tree),
            14 => self.string_delimiter_14(&children[0], parse_tree),
            _ => Err(miette!("Unhandled production number: {}", prod_num)),
        }
    }
}
