// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::id_tree::Tree;
{{#auto_generate?}}use parol_runtime::parol_macros::{pop_item, pop_and_reverse_item};{{/auto_generate}}
{{#auto_generate?}}use parol_runtime::lexer::Token;{{/auto_generate}}
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType, UserActionsTrait};
{{#auto_generate?}}use parol_runtime::log::trace;
{{/auto_generate}}use parol_runtime::miette::{miette, {{#auto_generate?}}bail, IntoDiagnostic, {{/auto_generate}}Result};
use crate::{{module_name}}::{{user_type_name}};{{^ast_type_has_lifetime?}}
use std::marker::PhantomData;{{/ast_type_has_lifetime}}
use parol_runtime::derive_builder::Builder;

{{#auto_generate?}}
/// Semantic actions trait generated for the user grammar
/// All functions have default implementations.
pub trait {{user_type_name}}Trait{{#ast_type_has_lifetime?}}<'t>{{/ast_type_has_lifetime}} {
    {{{user_trait_functions}}}
}

// -------------------------------------------------------------------------------------------------
//
// Output Types of productions deduced from the structure of the transformed grammar
//

{{{production_output_types}}}

// -------------------------------------------------------------------------------------------------
//
// Types of non-terminals deduced from the structure of the transformed grammar
//

{{{non_terminal_types}}}


// -------------------------------------------------------------------------------------------------

{{{ast_type_decl}}}

/// Auto-implemented adapter grammar
///
/// The lifetime parameter `'t` refers to the lifetime of the scanned text.
/// The lifetime parameter `'u` refers to the lifetime of user grammar object.
///
#[allow(dead_code)]
pub struct {{{user_type_name}}}Auto<'t, 'u> where 't: 'u {
    // Mutable reference of the actual user grammar to be able to call the semantic actions on it
    user_grammar: &'u mut dyn {{user_type_name}}Trait{{#ast_type_has_lifetime?}}<'t>{{/ast_type_has_lifetime}},
    // Stack to construct the AST on it
    item_stack: Vec<ASTType{{#ast_type_has_lifetime?}}<'t>{{/ast_type_has_lifetime}}>,{{^ast_type_has_lifetime?}}
    // Just to hold the lifetime generated by parol
    phantom: PhantomData<&'t str>,{{/ast_type_has_lifetime}}
}
{{/auto_generate}}

{{#auto_generate?}}
///
/// The `{{{user_type_name}}}Auto` impl is automatically generated for the
/// given grammar.
///
impl<'t, 'u> {{{user_type_name}}}Auto<'t, 'u> {
    pub fn new(user_grammar: &'u mut dyn {{user_type_name}}Trait{{#ast_type_has_lifetime?}}<'t>{{/ast_type_has_lifetime}}) -> Self {
        Self {
            user_grammar,
            item_stack: Vec::new(),{{^ast_type_has_lifetime?}}
            phantom: PhantomData::default(),{{/ast_type_has_lifetime}}
        }
    }

    #[allow(dead_code)]
    fn push(&mut self, item: ASTType{{#ast_type_has_lifetime?}}<'t>{{/ast_type_has_lifetime}}, context: &str) {
        trace!("push    {}: {:?}", context, item);
        self.item_stack.push(item)
    }

    #[allow(dead_code)]
    fn pop(&mut self, context: &str) -> Option<ASTType{{#ast_type_has_lifetime?}}<'t>{{/ast_type_has_lifetime}}> {
        if !self.item_stack.is_empty() {
            let item = self.item_stack.pop();
            if let Some(ref item) = item {
                trace!("pop     {}: {:?}", context, item);
            }
            item
        } else {
            None
        }
    }

    #[allow(dead_code)]
    // Use this function for debugging purposes:
    // trace!("{}", self.trace_item_stack(context));
    fn trace_item_stack(&self, context: &str) -> std::string::String {
        format!(
            "Item stack at {}:\n{}",
            context,
            self.item_stack
                .iter()
                .rev()
                .map(|s| format!("  {:?}", s))
                .collect::<Vec<std::string::String>>()
                .join("\n")
        )
    }

{{/auto_generate}}
{{^auto_generate?}}
///
/// The `{{{user_type_name}}}Trait` trait is automatically generated for the
/// given grammar.
/// All functions have default implementations.
///
pub trait {{{user_type_name}}}Trait {
{{/auto_generate}}

    {{{trait_functions}}}
}

{{#auto_generate?}}impl<'t> UserActionsTrait<'t> for {{{user_type_name}}}Auto<'t, '_> { {{/auto_generate}}
{{^auto_generate?}}impl UserActionsTrait<'_> for {{{user_type_name}}} { {{/auto_generate}}
    ///
    /// This function is implemented automatically for the user's item {{{user_type_name}}}.
    ///
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[ParseTreeStackEntry{{#auto_generate?}}<'t>{{/auto_generate}}],
        parse_tree: &Tree<ParseTreeType{{#auto_generate?}}<'t>{{/auto_generate}}>) -> Result<()> {
        match prod_num {
{{{trait_caller}}}            _ => Err(miette!("Unhandled production number: {}", prod_num)),
        }
    }
}
