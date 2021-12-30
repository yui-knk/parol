// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use crate::json_grammar::JsonGrammar;
use id_tree::Tree;
use miette::{miette, Result};
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType, UserActionsTrait};

///
/// The `JsonGrammarTrait` trait is automatically generated for the
/// given grammar.
/// All functions have default implementations.
///
pub trait JsonGrammarTrait {
    ///
    /// Implement this method if you need the provided information
    ///
    fn init(&mut self, _file_name: &std::path::Path) {}

    /// Semantic action for production 0:
    ///
    /// Json: Value;
    ///
    fn json_0(
        &mut self,
        _value_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 1:
    ///
    /// Object: "\{" ObjectSuffix;
    ///
    fn object_1(
        &mut self,
        _l_brace_0: &ParseTreeStackEntry,
        _object_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 2:
    ///
    /// ObjectSuffix: Pair ObjectList "\}";
    ///
    fn object_suffix_2(
        &mut self,
        _pair_0: &ParseTreeStackEntry,
        _object_list_1: &ParseTreeStackEntry,
        _r_brace_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 3:
    ///
    /// ObjectSuffix: "\}";
    ///
    fn object_suffix_3(
        &mut self,
        _r_brace_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 4:
    ///
    /// ObjectList: "," Pair ObjectList;
    ///
    fn object_list_4(
        &mut self,
        _comma_0: &ParseTreeStackEntry,
        _pair_1: &ParseTreeStackEntry,
        _object_list_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 5:
    ///
    /// ObjectList: ;
    ///
    fn object_list_5(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 6:
    ///
    /// Pair: String ":" Value;
    ///
    fn pair_6(
        &mut self,
        _string_0: &ParseTreeStackEntry,
        _colon_1: &ParseTreeStackEntry,
        _value_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 7:
    ///
    /// Array: "\[" ArraySuffix;
    ///
    fn array_7(
        &mut self,
        _l_bracket_0: &ParseTreeStackEntry,
        _array_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 8:
    ///
    /// ArraySuffix: Value ArrayList "\]";
    ///
    fn array_suffix_8(
        &mut self,
        _value_0: &ParseTreeStackEntry,
        _array_list_1: &ParseTreeStackEntry,
        _r_bracket_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 9:
    ///
    /// ArraySuffix: "\]";
    ///
    fn array_suffix_9(
        &mut self,
        _r_bracket_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 10:
    ///
    /// ArrayList: "," Value ArrayList;
    ///
    fn array_list_10(
        &mut self,
        _comma_0: &ParseTreeStackEntry,
        _value_1: &ParseTreeStackEntry,
        _array_list_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 11:
    ///
    /// ArrayList: ;
    ///
    fn array_list_11(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 12:
    ///
    /// Value: String;
    ///
    fn value_12(
        &mut self,
        _string_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 13:
    ///
    /// Value: Number;
    ///
    fn value_13(
        &mut self,
        _number_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 14:
    ///
    /// Value: Object;
    ///
    fn value_14(
        &mut self,
        _object_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 15:
    ///
    /// Value: Array;
    ///
    fn value_15(
        &mut self,
        _array_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 16:
    ///
    /// Value: "true";
    ///
    fn value_16(
        &mut self,
        _true_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 17:
    ///
    /// Value: "false";
    ///
    fn value_17(
        &mut self,
        _false_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 18:
    ///
    /// Value: "null";
    ///
    fn value_18(
        &mut self,
        _null_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 19:
    ///
    /// String: "\u{0022}(\\[\u{0022}\\/bfnrt]|u[0-9a-fA-F]{4}|[^\u{0022}\\\u0000-\u001F])*\u{0022}";
    ///
    fn string_19(
        &mut self,
        _string_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 20:
    ///
    /// Number: "-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][-+]?(0|[1-9][0-9]*)?)?";
    ///
    fn number_20(
        &mut self,
        _number_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }
}

impl UserActionsTrait for JsonGrammar {
    ///
    /// Initialize the user with additional information.
    /// This function is called by the parser before parsing starts.
    /// Is is used to transport necessary data from parser to user.
    ///
    fn init(&mut self, file_name: &std::path::Path) {
        JsonGrammarTrait::init(self, file_name);
    }

    ///
    /// This function is implemented automatically for the user's item JsonGrammar.
    ///
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[ParseTreeStackEntry],
        parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        match prod_num {
            0 => self.json_0(&children[0], parse_tree),

            1 => self.object_1(&children[0], &children[1], parse_tree),

            2 => self.object_suffix_2(&children[0], &children[1], &children[2], parse_tree),

            3 => self.object_suffix_3(&children[0], parse_tree),

            4 => self.object_list_4(&children[0], &children[1], &children[2], parse_tree),

            5 => self.object_list_5(parse_tree),

            6 => self.pair_6(&children[0], &children[1], &children[2], parse_tree),

            7 => self.array_7(&children[0], &children[1], parse_tree),

            8 => self.array_suffix_8(&children[0], &children[1], &children[2], parse_tree),

            9 => self.array_suffix_9(&children[0], parse_tree),

            10 => self.array_list_10(&children[0], &children[1], &children[2], parse_tree),

            11 => self.array_list_11(parse_tree),

            12 => self.value_12(&children[0], parse_tree),

            13 => self.value_13(&children[0], parse_tree),

            14 => self.value_14(&children[0], parse_tree),

            15 => self.value_15(&children[0], parse_tree),

            16 => self.value_16(&children[0], parse_tree),

            17 => self.value_17(&children[0], parse_tree),

            18 => self.value_18(&children[0], parse_tree),

            19 => self.string_19(&children[0], parse_tree),

            20 => self.number_20(&children[0], parse_tree),

            _ => Err(miette!("Unhandled production number: {}", prod_num)),
        }
    }
}
