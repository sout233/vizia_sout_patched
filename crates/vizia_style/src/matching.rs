#![allow(unused_variables)]
pub use selectors::SelectorList;

pub use selectors::{
    context::{MatchingContext, MatchingMode, QuirksMode},
    matching::{matches_selector, matches_selector_list},
    Element,
};

#[cfg(test)]
mod test {
    use std::{
        collections::{HashMap, HashSet},
        hash::Hash,
    };

    use cssparser::*;
    use selectors::{
        context::{
            MatchingContext, MatchingForInvalidation, MatchingMode, NeedsSelectorFlags, QuirksMode,
            SelectorCaches,
        },
        matching::matches_selector_list,
        OpaqueElement, SelectorList,
    };

    use crate::{CustomParseError, ParserOptions, SelectorIdent, SelectorParser, Selectors};

    fn parse(input: &str) -> Result<SelectorList<Selectors>, ParseError<'_, CustomParseError<'_>>> {
        let mut parser_input = ParserInput::new(input);
        let mut parser = Parser::new(&mut parser_input);
        let options = ParserOptions::default();
        SelectorList::parse(
            &SelectorParser { options: &options },
            &mut parser,
            selectors::parser::ParseRelative::No,
        )
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Entity(u32);

    use bitflags::bitflags;

    bitflags! {
        /// A bitflag of possible pseudoclasses.
        #[derive(Debug, Clone, Copy)]
        pub struct PseudoClass: u8 {
            const HOVER = 1;
            const OVER = 1 << 1;
            const ACTIVE = 1 << 2;
            const FOCUS = 1 << 3;
            const DISABLED = 1 << 4;
            const CHECKED = 1 << 5;
        }
    }

    #[derive(Debug)]
    pub struct Store {
        element: HashMap<Entity, String>,
        classes: HashMap<Entity, HashSet<String>>,
        pseudo_class: HashMap<Entity, PseudoClass>,
    }

    #[derive(Debug, Clone)]
    pub struct Node<'s> {
        entity: Entity,
        store: &'s Store,
    }

    impl<'s> selectors::Element for Node<'s> {
        type Impl = Selectors;

        fn opaque(&self) -> selectors::OpaqueElement {
            OpaqueElement::new(self)
        }

        fn is_html_slot_element(&self) -> bool {
            false
        }

        fn parent_node_is_shadow_root(&self) -> bool {
            false
        }

        fn containing_shadow_host(&self) -> Option<Self> {
            None
        }

        fn parent_element(&self) -> Option<Self> {
            None
        }

        fn prev_sibling_element(&self) -> Option<Self> {
            None
        }

        fn next_sibling_element(&self) -> Option<Self> {
            None
        }

        fn is_empty(&self) -> bool {
            true
        }

        fn is_root(&self) -> bool {
            false
        }

        fn is_html_element_in_html_document(&self) -> bool {
            false
        }

        fn has_local_name(&self, local_name: &SelectorIdent) -> bool {
            if let Some(element) = self.store.element.get(&self.entity) {
                return element == &local_name.0;
            }

            false
        }

        fn has_namespace(
            &self,
            ns: &<Self::Impl as selectors::SelectorImpl>::BorrowedNamespaceUrl,
        ) -> bool {
            false
        }

        fn is_part(&self, name: &<Self::Impl as selectors::SelectorImpl>::Identifier) -> bool {
            false
        }

        fn imported_part(
            &self,
            name: &<Self::Impl as selectors::SelectorImpl>::Identifier,
        ) -> Option<<Self::Impl as selectors::SelectorImpl>::Identifier> {
            None
        }

        fn is_pseudo_element(&self) -> bool {
            false
        }

        fn is_same_type(&self, other: &Self) -> bool {
            self.store.element.get(&self.entity) == other.store.element.get(&self.entity)
        }

        fn is_link(&self) -> bool {
            false
        }

        fn has_id(
            &self,
            id: &<Self::Impl as selectors::SelectorImpl>::Identifier,
            case_sensitivity: selectors::attr::CaseSensitivity,
        ) -> bool {
            false
        }

        fn has_class(
            &self,
            name: &<Self::Impl as selectors::SelectorImpl>::Identifier,
            case_sensitivity: selectors::attr::CaseSensitivity,
        ) -> bool {
            if let Some(classes) = self.store.classes.get(&self.entity) {
                return classes.contains(&name.0);
            }

            false
        }

        fn attr_matches(
            &self,
            ns: &selectors::attr::NamespaceConstraint<
                &<Self::Impl as selectors::SelectorImpl>::NamespaceUrl,
            >,
            local_name: &<Self::Impl as selectors::SelectorImpl>::LocalName,
            operation: &selectors::attr::AttrSelectorOperation<
                &<Self::Impl as selectors::SelectorImpl>::AttrValue,
            >,
        ) -> bool {
            false
        }

        fn match_pseudo_element(
            &self,
            pe: &<Self::Impl as selectors::SelectorImpl>::PseudoElement,
            context: &mut selectors::context::MatchingContext<'_, Self::Impl>,
        ) -> bool {
            false
        }

        fn match_non_ts_pseudo_class(
            &self,
            pc: &<Self::Impl as selectors::SelectorImpl>::NonTSPseudoClass,
            context: &mut selectors::context::MatchingContext<'_, Self::Impl>,
        ) -> bool {
            if let Some(psudeo_class_flag) = self.store.pseudo_class.get(&self.entity) {
                match pc {
                    crate::PseudoClass::Hover => psudeo_class_flag.contains(PseudoClass::HOVER),
                    crate::PseudoClass::Active => todo!(),
                    crate::PseudoClass::Focus => todo!(),
                    crate::PseudoClass::FocusVisible => todo!(),
                    crate::PseudoClass::FocusWithin => todo!(),
                    crate::PseudoClass::Enabled => todo!(),
                    crate::PseudoClass::Disabled => todo!(),
                    crate::PseudoClass::ReadOnly => todo!(),
                    crate::PseudoClass::ReadWrite => todo!(),
                    crate::PseudoClass::PlaceholderShown => todo!(),
                    crate::PseudoClass::Default => todo!(),
                    crate::PseudoClass::Checked => todo!(),
                    crate::PseudoClass::Indeterminate => todo!(),
                    crate::PseudoClass::Blank => todo!(),
                    crate::PseudoClass::Valid => todo!(),
                    crate::PseudoClass::Invalid => todo!(),
                    crate::PseudoClass::InRange => todo!(),
                    crate::PseudoClass::OutOfRange => todo!(),
                    crate::PseudoClass::Required => todo!(),
                    crate::PseudoClass::Optional => todo!(),
                    crate::PseudoClass::UserValid => todo!(),
                    crate::PseudoClass::UserInvalid => todo!(),
                    crate::PseudoClass::Lang(_) => todo!(),
                    crate::PseudoClass::Dir(_) => todo!(),
                    crate::PseudoClass::Custom(_) => todo!(),
                    _ => todo!(),
                }
            } else {
                false
            }
        }

        fn first_element_child(&self) -> Option<Self> {
            None
        }

        fn apply_selector_flags(&self, flags: selectors::matching::ElementSelectorFlags) {
            todo!()
        }

        fn has_custom_state(
            &self,
            name: &<Self::Impl as selectors::SelectorImpl>::Identifier,
        ) -> bool {
            false
        }

        fn add_element_unique_hashes(&self, filter: &mut selectors::bloom::BloomFilter) -> bool {
            false
        }
    }

    #[test]
    fn asterisk_match() {
        let mut store = Store {
            element: HashMap::new(),
            classes: HashMap::new(),
            pseudo_class: HashMap::new(),
        };

        let root = Entity(0);
        let child = Entity(1);

        store.element.insert(root, String::from("window"));
        store.element.insert(child, String::from("button"));

        let root_node = Node { entity: root, store: &store };

        let child_node = Node { entity: child, store: &store };

        if let Ok(selector_list) = parse("*") {
            let mut cache = SelectorCaches::default();
            let mut context = MatchingContext::new(
                MatchingMode::Normal,
                None,
                &mut cache,
                QuirksMode::NoQuirks,
                NeedsSelectorFlags::No,
                MatchingForInvalidation::No,
            );

            let result = matches_selector_list(&selector_list, &root_node, &mut context);
        }
    }

    #[test]
    fn element_match() {
        let mut store = Store {
            element: HashMap::new(),
            classes: HashMap::new(),
            pseudo_class: HashMap::new(),
        };

        let root = Entity(0);
        let child = Entity(1);

        store.element.insert(root, String::from("window"));
        store.element.insert(child, String::from("button"));

        let root_node = Node { entity: root, store: &store };

        let child_node = Node { entity: child, store: &store };

        if let Ok(selector_list) = parse("window") {
            let mut cache = SelectorCaches::default();
            let mut context = MatchingContext::new(
                MatchingMode::Normal,
                None,
                &mut cache,
                QuirksMode::NoQuirks,
                NeedsSelectorFlags::No,
                MatchingForInvalidation::No,
            );

            let result = matches_selector_list(&selector_list, &root_node, &mut context);

            assert!(result);

            let result = matches_selector_list(&selector_list, &child_node, &mut context);

            assert!(!result);
        }
    }

    #[test]
    fn class_match() {
        let mut store = Store {
            element: HashMap::new(),
            classes: HashMap::new(),
            pseudo_class: HashMap::new(),
        };

        let root = Entity(0);
        let child = Entity(1);

        store.classes.insert(root, HashSet::new());

        if let Some(classes) = store.classes.get_mut(&root) {
            classes.insert(String::from("foo"));
            classes.insert(String::from("bar"));
        }

        store.classes.insert(child, HashSet::new());

        if let Some(classes) = store.classes.get_mut(&child) {
            classes.insert(String::from("bar"));
        }

        let root_node = Node { entity: root, store: &store };

        let child_node = Node { entity: child, store: &store };

        if let Ok(selector_list) = parse(".foo") {
            let mut cache = SelectorCaches::default();
            let mut context = MatchingContext::new(
                MatchingMode::Normal,
                None,
                &mut cache,
                QuirksMode::NoQuirks,
                NeedsSelectorFlags::No,
                MatchingForInvalidation::No,
            );

            let result = matches_selector_list(&selector_list, &root_node, &mut context);
            assert!(result);

            let result = matches_selector_list(&selector_list, &child_node, &mut context);
            assert!(!result);
        }

        if let Ok(selector_list) = parse(".bar") {
            let mut cache = SelectorCaches::default();
            let mut context = MatchingContext::new(
                MatchingMode::Normal,
                None,
                &mut cache,
                QuirksMode::NoQuirks,
                NeedsSelectorFlags::No,
                MatchingForInvalidation::No,
            );

            let result = matches_selector_list(&selector_list, &root_node, &mut context);
            assert!(result);

            let result = matches_selector_list(&selector_list, &child_node, &mut context);
            assert!(result);
        }

        if let Ok(selector_list) = parse(".foo.bar") {
            let mut cache = SelectorCaches::default();
            let mut context = MatchingContext::new(
                MatchingMode::Normal,
                None,
                &mut cache,
                QuirksMode::NoQuirks,
                NeedsSelectorFlags::No,
                MatchingForInvalidation::No,
            );

            let result = matches_selector_list(&selector_list, &root_node, &mut context);
            assert!(result);

            let result = matches_selector_list(&selector_list, &child_node, &mut context);
            assert!(!result);
        }

        if let Ok(selector_list) = parse(".foo, .bar") {
            let mut cache = SelectorCaches::default();
            let mut context = MatchingContext::new(
                MatchingMode::Normal,
                None,
                &mut cache,
                QuirksMode::NoQuirks,
                NeedsSelectorFlags::No,
                MatchingForInvalidation::No,
            );

            let result = matches_selector_list(&selector_list, &root_node, &mut context);
            assert!(result);

            let result = matches_selector_list(&selector_list, &child_node, &mut context);
            assert!(result);
        }
    }

    #[test]
    fn pseudoclass_match() {
        let mut store = Store {
            element: HashMap::new(),
            classes: HashMap::new(),
            pseudo_class: HashMap::new(),
        };

        let root = Entity(0);
        let child = Entity(1);

        store.element.insert(root, String::from("window"));
        store.pseudo_class.insert(root, PseudoClass::empty());

        if let Some(pseudoclass) = store.pseudo_class.get_mut(&root) {
            pseudoclass.set(PseudoClass::HOVER, true);
        }

        store.element.insert(child, String::from("child"));

        let root_node = Node { entity: root, store: &store };

        let child_node = Node { entity: child, store: &store };

        if let Ok(selector_list) = parse("window:hover") {
            let mut cache = SelectorCaches::default();
            let mut context = MatchingContext::new(
                MatchingMode::Normal,
                None,
                &mut cache,
                QuirksMode::NoQuirks,
                NeedsSelectorFlags::No,
                MatchingForInvalidation::No,
            );

            let result = matches_selector_list(&selector_list, &root_node, &mut context);
            assert!(result);

            let result = matches_selector_list(&selector_list, &child_node, &mut context);
            assert!(!result);
        }
    }
}
