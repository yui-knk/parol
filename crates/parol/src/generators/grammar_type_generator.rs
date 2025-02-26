use crate::analysis::lookahead_dfa::ProductionIndex;
use crate::generators::NamingHelper as NmHlp;
use crate::grammar::ProductionAttribute;
use crate::{Pr, Symbol, Terminal};
use anyhow::{anyhow, bail, Result};
use parol_runtime::log::trace;
use std::collections::{BTreeMap, HashSet};
use std::fmt::{Debug, Display, Error, Formatter};

use crate::{grammar::SymbolAttribute, Cfg, GrammarConfig};

use super::generate_terminal_name;
use super::symbol_table::{
    Function, FunctionBuilder, MetaSymbolKind, SymbolId, SymbolTable, TypeEntrails,
};
use super::symbol_table_facade::{InstanceFacade, SymbolFacade, TypeFacade};

///
/// Type information for a given grammar
///
#[derive(Debug, Default)]
pub struct GrammarTypeInfo {
    /// All symbols are managed by the symbol table
    pub(crate) symbol_table: SymbolTable,

    /// Calculated types of non-terminals
    pub(crate) non_terminal_types: BTreeMap<String, SymbolId>,

    pub(crate) user_action_trait_id: Option<SymbolId>,
    pub(crate) adapter_grammar_struct_id: Option<SymbolId>,
    pub(crate) action_caller_trait_id: Option<SymbolId>,

    // Function in the user action trait
    // For each non-terminal a function. Sorted by order of appearance in the user grammar
    pub(crate) user_actions: Vec<(String, SymbolId)>,

    // Functions in the inner actions trait
    pub(crate) adapter_actions: BTreeMap<ProductionIndex, SymbolId>,

    // Output types of productions
    pub(crate) production_types: BTreeMap<ProductionIndex, SymbolId>,

    /// The type completely comprising the whole structural information that could be generated by
    /// the given expanded grammar.
    /// It is a type of enum kind.
    /// We use this as ASTType for the generated source.
    pub(crate) ast_enum_type: SymbolId,

    /// Indicates if the auto generation mode is active
    pub(crate) auto_generate: bool,

    /// Helper
    terminals: Vec<String>,
    terminal_names: Vec<String>,

    // Contains non-terminals that should be represented as vectors in the AST Enum type
    vector_typed_non_terminals: HashSet<String>,

    // Contains non-terminals that should be represented as options in the AST Enum type
    option_typed_non_terminals: HashSet<String>,
}

impl GrammarTypeInfo {
    /// Create a new item
    /// Initializes the inner data structures.
    pub fn try_new(grammar_type_name: &str) -> Result<Self> {
        let mut me = Self::default();
        me.symbol_table = SymbolTable::new();

        // Insert the fix UserActionsTrait into the global scope
        me.action_caller_trait_id = Some(
            me.symbol_table
                .insert_global_type("UserActionsTrait", TypeEntrails::Trait)?,
        );

        // Insert the Semantic Actions Trait into the global scope
        me.user_action_trait_id = Some(me.symbol_table.insert_global_type(
            &format!(
                "{}GrammarTrait",
                NmHlp::to_upper_camel_case(grammar_type_name)
            ),
            TypeEntrails::Trait,
        )?);

        // Insert the fix 'init' function into the user action trait to avoid name clashes with a
        // possible non-terminal 'Init'
        me.symbol_table.insert_type(
            me.user_action_trait_id.unwrap(),
            "init",
            TypeEntrails::Function(Function::default()),
        )?;

        // Insert the fix <GrammarName>GrammarAuto struct into the global scope
        me.adapter_grammar_struct_id = Some(me.symbol_table.insert_global_type(
            &format!(
                "{}GrammarAuto",
                NmHlp::to_upper_camel_case(grammar_type_name)
            ),
            TypeEntrails::Struct,
        )?);

        for n in ["new", "push", "pop", "trace_item_stack", "on_comment"] {
            me.symbol_table.insert_type(
                me.adapter_grammar_struct_id.unwrap(),
                n,
                TypeEntrails::Function(Function::default()),
            )?;
        }

        // Insert the fix Token type the global scope, simply to avoid name clashes
        me.symbol_table
            .insert_global_type("Token", TypeEntrails::Token)?;
        Ok(me)
    }

    /// Set the auto-generate mode
    /// Internally it adjust the used flags on the arguments of the actions.
    /// The arguments keep their used state only if auto generation is active.
    pub(crate) fn set_auto_generate(&mut self, auto_generate: bool) -> Result<()> {
        self.auto_generate = auto_generate;
        self.adjust_arguments_used(auto_generate)
    }

    pub(crate) fn add_user_action(&mut self, non_terminal: &str) -> Result<SymbolId> {
        let action_fn = self.symbol_table.insert_type(
            self.user_action_trait_id.unwrap(),
            non_terminal,
            TypeEntrails::Function(
                FunctionBuilder::default()
                    .non_terminal(non_terminal.to_string())
                    .build()
                    .unwrap(),
            ),
        )?;
        self.user_actions
            .push((non_terminal.to_string(), action_fn));
        Ok(action_fn)
    }

    pub(crate) fn get_user_action(&self, non_terminal: &str) -> Result<SymbolId> {
        self.user_actions
            .iter()
            .find(|(nt, _)| nt == non_terminal)
            .map(|(_, fn_id)| *fn_id)
            .ok_or_else(|| {
                anyhow!(
                    "There should be a user action for non-terminal '{}'!",
                    non_terminal
                )
            })
    }

    fn adjust_arguments_used(&mut self, used: bool) -> Result<()> {
        for action_id in self.adapter_actions.values() {
            let arguments_scope = self.symbol_table.symbol_as_type(*action_id).member_scope();
            let args = self.symbol_table.scope(arguments_scope).symbols.clone();
            for arg in args {
                self.symbol_table.set_instance_used(arg, used);
            }
        }
        Ok(())
    }

    /// Add non-terminal type
    fn add_non_terminal_type(&mut self, non_terminal: &str, nt_type: SymbolId) -> Result<()> {
        self.non_terminal_types
            .insert(non_terminal.to_owned(), nt_type)
            .map_or_else(
                || {
                    trace!("Setting type for non-terminal {}", non_terminal);
                    Ok(())
                },
                |_| {
                    Err(anyhow!(
                        "Type for non-terminal {} already specified",
                        non_terminal
                    ))
                },
            )
    }

    ///
    /// Build the type information from the given grammar.
    ///
    pub fn build(&mut self, grammar_config: &GrammarConfig) -> Result<()> {
        let cfg = &grammar_config.cfg;
        self.terminals = cfg
            .get_ordered_terminals()
            .iter()
            .map(|(t, k, _)| k.expand(t))
            .collect::<Vec<String>>();

        self.terminal_names = self.terminals.iter().fold(Vec::new(), |mut acc, e| {
            let n = generate_terminal_name(e, None, cfg);
            acc.push(n);
            acc
        });

        self.create_initial_non_terminal_types(&grammar_config.cfg)?;
        self.deduce_actions(grammar_config)?;
        self.finish_non_terminal_types(&grammar_config.cfg)?;
        self.generate_ast_enum_type()?;
        self.symbol_table.propagate_lifetimes();
        Ok(())
    }

    ///
    /// Returns a vector of actions matching the given non-terminal n
    ///
    fn matching_actions(&self, n: &str) -> Vec<SymbolId> {
        self.adapter_actions
            .iter()
            .filter(|(_, a)| match &self.symbol_table.symbol(**a).kind() {
                super::symbol_table::SymbolKind::Type(t) => match &t.entrails {
                    TypeEntrails::Function(f) => f.non_terminal == n,
                    _ => panic!("Expecting a function!"),
                },
                _ => panic!("Expecting a type!"),
            })
            .map(|(_, s)| *s)
            .collect::<Vec<SymbolId>>()
    }

    fn create_initial_non_terminal_types(&mut self, cfg: &Cfg) -> Result<()> {
        for nt in cfg.get_non_terminal_set() {
            let alternatives = cfg.matching_productions(&nt);
            if alternatives.is_empty() {
                continue;
            }
            if let Ok(nt_type) = self.create_initial_non_terminal_type(&nt, alternatives) {
                self.add_non_terminal_type(&nt, nt_type)?;
            }
        }
        Ok(())
    }

    fn create_initial_non_terminal_type(
        &mut self,
        non_terminal: &str,
        alternatives: Vec<(usize, &Pr)>,
    ) -> Result<SymbolId> {
        if alternatives.len() == 2 {
            let semantics = alternatives.iter().fold(
                Vec::new(),
                |mut res: Vec<ProductionAttribute>, (_, p)| {
                    res.push(p.2);
                    res
                },
            );
            if semantics[0] == ProductionAttribute::AddToCollection
                || semantics[0] == ProductionAttribute::CollectionStart
                || semantics[0] == ProductionAttribute::OptionalNone
                || semantics[0] == ProductionAttribute::OptionalSome
            {
                return self
                    .symbol_table
                    .insert_global_type(non_terminal, TypeEntrails::Struct);
            }
        }
        match alternatives.len() {
            // Productions can be optimized away, when they have duplicates!
            0 => bail!("Not supported!"),
            // Only one production for this non-terminal: we create an empty Struct
            1 => self
                .symbol_table
                .insert_global_type(non_terminal, TypeEntrails::Struct),
            // Otherwise: we generate an empty Enum
            _ => self
                .symbol_table
                .insert_global_type(non_terminal, TypeEntrails::Enum),
        }
    }

    fn finish_non_terminal_types(&mut self, cfg: &Cfg) -> Result<()> {
        for nt in cfg.get_non_terminal_set() {
            self.finish_non_terminal_type(&nt, cfg)?;
        }
        Ok(())
    }

    fn arguments(&self, action_id: SymbolId) -> Result<Vec<SymbolId>> {
        let action_scope = self.symbol_table.symbol_as_type(action_id).member_scope();
        Ok(self.symbol_table.scope(action_scope).symbols.clone())
    }

    fn finish_non_terminal_type(&mut self, nt: &str, cfg: &Cfg) -> Result<()> {
        let mut vector_typed_non_terminal_opt = None;
        let mut option_typed_non_terminal_opt = None;

        let actions = self.matching_actions(nt).iter().try_fold(
            Vec::new(),
            |mut res: Vec<(SymbolId, ProductionAttribute)>, a| {
                self.symbol_table.function_type_semantic(*a).map(|t| {
                    res.push((*a, t));
                    res
                })
            },
        )?;

        if actions.len() == 1 {
            let arguments = self.arguments(actions[0].0)?;
            let non_terminal_type = *self.non_terminal_types.get(nt).unwrap();
            // Copy the arguments as struct members
            self.arguments_to_struct_members(&arguments, non_terminal_type)?;
        } else if actions.len() == 2
            && (actions[0].1 == ProductionAttribute::AddToCollection
                || actions[0].1 == ProductionAttribute::CollectionStart)
        {
            let primary_action = match (&actions[0].1, &actions[1].1) {
                (ProductionAttribute::AddToCollection, ProductionAttribute::CollectionStart) => {
                    actions[0].0
                }
                (ProductionAttribute::CollectionStart, ProductionAttribute::AddToCollection) => {
                    actions[1].0
                }
                _ => bail!("Unexpected combination of production attributes"),
            };
            let mut arguments = self.arguments(primary_action)?;
            arguments.pop(); // Remove the recursive part. Vec is wrapped outside.
            vector_typed_non_terminal_opt = Some(nt.to_string());
            let non_terminal_type = *self.non_terminal_types.get(nt).unwrap();
            // Copy the arguments as struct members
            self.arguments_to_struct_members(&arguments, non_terminal_type)?;
        } else if actions.len() == 2
            && (actions[0].1 == ProductionAttribute::OptionalNone
                || actions[0].1 == ProductionAttribute::OptionalSome)
        {
            let primary_action = match (&actions[0].1, &actions[1].1) {
                (ProductionAttribute::OptionalSome, ProductionAttribute::OptionalNone) => {
                    actions[0].0
                }
                (ProductionAttribute::OptionalNone, ProductionAttribute::OptionalSome) => {
                    actions[1].0
                }
                _ => bail!("Unexpected combination of production attributes"),
            };
            let arguments = self.arguments(primary_action)?;
            option_typed_non_terminal_opt = Some(nt.to_string());
            let non_terminal_type = *self.non_terminal_types.get(nt).unwrap();
            // Copy the arguments as struct members
            self.arguments_to_struct_members(&arguments, non_terminal_type)?;
        } else {
            // This is the "enum case". We generate an enum variant for each production with a name
            // built from the right-hand side of the corresponding production.
            let non_terminal_type = *self.non_terminal_types.get(nt).unwrap();
            for (action_id, _) in actions {
                let function = self.symbol_table.symbol_as_function(action_id)?;
                let variant_name = self.generate_production_rhs_name(function.prod_num, cfg);
                let entrails = TypeEntrails::EnumVariant(
                    *self.production_types.get(&function.prod_num).unwrap(),
                );
                self.symbol_table
                    .insert_type(non_terminal_type, &variant_name, entrails)?;
            }
        }

        if let Some(vector_typed_non_terminal) = vector_typed_non_terminal_opt {
            self.vector_typed_non_terminals
                .insert(vector_typed_non_terminal);
        }

        if let Some(option_typed_non_terminal) = option_typed_non_terminal_opt {
            self.option_typed_non_terminals
                .insert(option_typed_non_terminal);
        }

        Ok(())
    }

    fn deduce_actions(&mut self, grammar_config: &GrammarConfig) -> Result<()> {
        let scanner_state_resolver = grammar_config.get_scanner_state_resolver();
        let user_type_resolver = grammar_config.get_user_type_resolver();

        for (i, pr) in grammar_config.cfg.pr.iter().enumerate() {
            let rel_idx = grammar_config
                .cfg
                .get_alternation_index_of_production(i)
                .unwrap();

            let alts = grammar_config.cfg.get_alternations_count(i).unwrap();

            let function_entrails = FunctionBuilder::default()
                .non_terminal(pr.get_n())
                .prod_num(i)
                .rel_idx(rel_idx)
                .alts(alts)
                .prod_string(pr.format(&scanner_state_resolver, &user_type_resolver)?)
                .sem(pr.2)
                .build()
                .unwrap();

            let type_name = if alts == 1 {
                NmHlp::to_lower_snake_case(pr.get_n_str())
            } else {
                NmHlp::to_lower_snake_case(&format!("{}_{}", pr.get_n_str(), rel_idx))
            };

            let function_id = self.symbol_table.insert_type(
                self.adapter_grammar_struct_id.unwrap(),
                &type_name,
                TypeEntrails::Function(function_entrails),
            )?;

            self.build_arguments(grammar_config, function_id)?;

            self.adapter_actions.insert(i, function_id);

            self.build_production_type(function_id, i, &grammar_config.cfg)?;
        }
        Ok(())
    }

    fn get_terminal_index(&self, tr: &str) -> usize {
        self.terminals.iter().position(|t| *t == tr).unwrap()
    }

    /// Generates a member name from a symbol that stems from a production's right-hand side
    /// The second string in the returned tuple is used as description, here the terminal's content.
    pub fn generate_member_name(&self, s: &Symbol) -> (String, String) {
        match s {
            Symbol::N(n, ..) => (NmHlp::to_lower_snake_case(n), String::default()),
            Symbol::T(Terminal::Trm(t, k, ..)) => {
                let terminal_name = &self.terminal_names[self.get_terminal_index(&k.expand(t))];
                (NmHlp::to_lower_snake_case(terminal_name), t.to_string())
            }
            _ => panic!("Invalid symbol type {}", s),
        }
    }

    /// Convenience function
    pub fn generate_member_names(&self, rhs: &[Symbol]) -> Vec<(String, String)> {
        rhs.iter()
            .filter(|s| s.is_n() || s.is_t())
            .map(|s| self.generate_member_name(s))
            .collect::<Vec<(String, String)>>()
    }

    fn build_arguments(
        &mut self,
        grammar_config: &GrammarConfig,
        function_id: SymbolId,
    ) -> Result<()> {
        let entrails = self
            .symbol_table
            .symbol_as_type(function_id)
            .entrails()
            .clone();
        if let TypeEntrails::Function(function_entrails) = entrails {
            let prod = &grammar_config.cfg[function_entrails.prod_num];
            let mut types = prod
                .get_r()
                .iter()
                .filter(|s| s.is_t() || s.is_n())
                .try_fold(Vec::new(), |mut acc, s| {
                    self.deduce_type_of_symbol(s).map(|t| {
                        acc.push((t, s.attribute()));
                        acc
                    })
                })?;

            if function_entrails.sem == ProductionAttribute::AddToCollection {
                let ref_mut_last_type = &mut types.last_mut().unwrap().0;
                *ref_mut_last_type = match &ref_mut_last_type {
                    TypeEntrails::Box(r) => TypeEntrails::Vec(*r),
                    _ => bail!("Unexpected last symbol in production with AddToCollection"),
                };
            }

            let result = self
                .generate_member_names(prod.get_r())
                .iter()
                .zip(types.drain(..))
                .try_for_each(|((n, r), (t, a))| {
                    // Tokens are taken from the parameter list per definition.
                    let mut used =
                        matches!(t, TypeEntrails::Token) && a != SymbolAttribute::Clipped;
                    let type_id = if let TypeEntrails::UserDefinedType(k, ref u) = t {
                        if k == MetaSymbolKind::Token {
                            used = true;
                            self.symbol_table
                                .get_or_create_scoped_user_defined_type(k, u)?
                        } else {
                            self.symbol_table.get_or_create_type(
                                SymbolTable::UNNAMED_TYPE,
                                SymbolTable::GLOBAL_SCOPE,
                                t,
                            )?
                        }
                    } else {
                        self.symbol_table.get_or_create_type(
                            SymbolTable::UNNAMED_TYPE,
                            SymbolTable::GLOBAL_SCOPE,
                            t,
                        )?
                    };
                    self.symbol_table
                        .insert_instance(function_id, n, type_id, used, a, r.to_string())
                        .map(|_| Ok(()))?
                });
            result
        } else {
            bail!("No function!")
        }
    }

    fn deduce_type_of_symbol(&mut self, symbol: &Symbol) -> Result<TypeEntrails> {
        match symbol {
            Symbol::T(Terminal::Trm(_, _, _, a, u)) => {
                if *a == SymbolAttribute::Clipped {
                    Ok(TypeEntrails::Clipped(MetaSymbolKind::Token))
                } else if let Some(ref user_defined_type) = u {
                    Ok(TypeEntrails::UserDefinedType(
                        MetaSymbolKind::Token,
                        user_defined_type.clone(),
                    ))
                } else {
                    Ok(TypeEntrails::Token)
                }
            }
            Symbol::N(n, a, u) => {
                let inner_type = self.non_terminal_types.get(n).unwrap();
                if let Some(ref user_defined_type) = u {
                    Ok(TypeEntrails::UserDefinedType(
                        MetaSymbolKind::NonTerminal(*inner_type),
                        user_defined_type.clone(),
                    ))
                } else {
                    match a {
                        SymbolAttribute::None => Ok(TypeEntrails::Box(*inner_type)),
                        SymbolAttribute::RepetitionAnchor => Ok(TypeEntrails::Vec(*inner_type)),
                        SymbolAttribute::Option => Ok(TypeEntrails::Option(*inner_type)),
                        SymbolAttribute::Clipped => Ok(TypeEntrails::Clipped(
                            MetaSymbolKind::NonTerminal(SymbolId::invalid_id()),
                        )),
                    }
                }
            }
            _ => Err(anyhow!("Unexpected symbol kind: {}", symbol)),
        }
    }

    fn build_production_type(
        &mut self,
        function_id: SymbolId,
        prod_num: ProductionIndex,
        cfg: &Cfg,
    ) -> Result<()> {
        let non_terminal = self
            .symbol_table
            .symbol_as_function(function_id)?
            .non_terminal;
        let rhs_name = self.generate_production_rhs_name(prod_num, cfg);
        let struct_name = NmHlp::to_upper_camel_case(&format!("{}_{}", non_terminal, rhs_name));
        let production_type = self
            .symbol_table
            .insert_global_type(&struct_name, TypeEntrails::Struct)?;

        let arguments = self.arguments(function_id)?;
        // Copy the arguments as struct members
        self.arguments_to_struct_members(&arguments, production_type)?;
        self.production_types.insert(prod_num, production_type);
        Ok(())
    }

    /// Copy the arguments as struct members
    fn arguments_to_struct_members(
        &mut self,
        arguments: &[SymbolId],
        production_type: SymbolId,
    ) -> Result<()> {
        for arg in arguments {
            let inst_name = self.symbol_table.symbol(*arg).name().to_string();
            let (type_id, description, sem) = {
                let inst = self.symbol_table.symbol_as_instance(*arg);
                (inst.type_id(), inst.description(), inst.sem())
            };
            self.symbol_table.insert_instance(
                production_type,
                &inst_name,
                type_id,
                true,
                sem,
                description,
            )?;
        }
        Ok(())
    }

    fn generate_ast_enum_type(&mut self) -> Result<()> {
        self.ast_enum_type = self
            .symbol_table
            .insert_global_type("ASTType", TypeEntrails::Enum)?;

        let variants = self
            .non_terminal_types
            .iter()
            .fold(Vec::new(), |mut acc, nt| {
                let inner_type = if self.vector_typed_non_terminals.contains(nt.0) {
                    self.symbol_table
                        .get_or_create_type(
                            SymbolTable::UNNAMED_TYPE,
                            SymbolTable::GLOBAL_SCOPE,
                            TypeEntrails::Vec(*nt.1),
                        )
                        .unwrap()
                } else if self.option_typed_non_terminals.contains(nt.0) {
                    self.symbol_table
                        .get_or_create_type(
                            SymbolTable::UNNAMED_TYPE,
                            SymbolTable::GLOBAL_SCOPE,
                            TypeEntrails::Option(*nt.1),
                        )
                        .unwrap()
                } else {
                    *nt.1
                };

                acc.push((nt.0.to_string(), TypeEntrails::EnumVariant(inner_type)));
                acc
            });

        for (n, e) in variants {
            self.symbol_table.insert_type(self.ast_enum_type, &n, e)?;
        }

        Ok(())
    }

    // Generates an enum variant name for the given production from its right-hand side. If the
    // production has an empty RHS we simple name this enum variant "<NonTerminal>Empty".
    fn generate_production_rhs_name(&self, prod_num: usize, cfg: &Cfg) -> String {
        let pr = &cfg[prod_num];
        let lhs = pr.get_r();
        if lhs.is_empty() {
            format!("{}Empty", NmHlp::to_upper_camel_case(pr.get_n_str()))
        } else {
            lhs.iter().fold(String::new(), |mut acc, s| {
                match s {
                    Symbol::N(n, _, _) => acc.push_str(&NmHlp::to_upper_camel_case(n)),
                    Symbol::T(Terminal::Trm(t, k, ..)) => {
                        acc.push_str(&NmHlp::to_upper_camel_case(
                            &self.terminal_names[self.get_terminal_index(&k.expand(t))],
                        ))
                    }
                    _ => (),
                }
                acc
            })
        }
    }

    /// Returns a reference to the inner symbol table.
    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }
}

impl Display for GrammarTypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        writeln!(f, "// Symbol table:")?;
        writeln!(f, "{}", self.symbol_table)?;
        writeln!(f, "// Production types:")?;
        for (p, i) in &self.production_types {
            writeln!(
                f,
                "Prod: {}: {} /* {} */",
                p,
                i,
                self.symbol_table.symbol(*i).name()
            )?;
        }
        writeln!(f, "// Non-terminal types:")?;
        for (n, i) in &self.non_terminal_types {
            writeln!(
                f,
                "{}: {} /* {} */",
                n,
                i,
                self.symbol_table.symbol(*i).name()
            )?;
        }
        writeln!(f, "// User actions:")?;
        for (n, i) in &self.user_actions {
            writeln!(
                f,
                "{}: {} /* {} */",
                n,
                i,
                self.symbol_table.symbol(*i).name()
            )?;
        }
        writeln!(f, "// Adapter actions:")?;
        for (p, i) in &self.adapter_actions {
            writeln!(
                f,
                "Prod: {}: {} /* {} */",
                p,
                i,
                self.symbol_table.symbol(*i).name()
            )?;
        }
        writeln!(f, "// Vector non-terminals:")?;
        for nt in &self.vector_typed_non_terminals {
            writeln!(f, "{}", nt)?;
        }
        writeln!(f, "// Option non-terminals:")?;
        for nt in &self.option_typed_non_terminals {
            writeln!(f, "{}", nt)?;
        }
        Ok(())
    }
}
