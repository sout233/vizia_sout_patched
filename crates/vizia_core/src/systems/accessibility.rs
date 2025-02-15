use crate::{accessibility::IntoNode, events::ViewHandler, prelude::*};
use accesskit::{Node, NodeId, Rect, Toggled, Tree, TreeUpdate};
use hashbrown::HashMap;
use vizia_storage::LayoutTreeIterator;

/// Updates node properties from view properties
/// Should be run after layout so that things like bounding box are correct.
/// This system doesn't change the structure of the accessibility tree as this is done when views are built/removed.
pub fn accessibility_system(cx: &mut Context) {
    if !cx.style.reaccess.is_empty() {
        let iterator = LayoutTreeIterator::full(&cx.tree);

        for entity in iterator {
            if !cx.style.reaccess.contains(entity) {
                continue;
            }

            let mut access_context = AccessContext {
                current: entity,
                tree: &cx.tree,
                cache: &cx.cache,
                style: &cx.style,
                text_context: &mut cx.text_context,
            };

            if let Some(node) = get_access_node(&mut access_context, &mut cx.views, entity) {
                let navigable = cx
                    .style
                    .abilities
                    .get(entity)
                    .copied()
                    .unwrap_or_default()
                    .contains(Abilities::NAVIGABLE);

                if node.node_builder.role() == Role::Unknown && !navigable {
                    continue;
                }

                let mut nodes = vec![(node.node_id(), node.node_builder)];

                // If child nodes were generated then append them to the nodes list
                if !node.children.is_empty() {
                    nodes.extend(
                        node.children
                            .into_iter()
                            .map(|child_node| (child_node.node_id(), child_node.node_builder)),
                    );
                }

                cx.tree_updates.push(Some(TreeUpdate {
                    nodes,
                    tree: None,
                    focus: if cx.window_has_focus {
                        cx.focused.accesskit_id()
                    } else {
                        NodeId(0u64)
                    },
                }));
            }

            // }
        }

        cx.style.reaccess.clear();
    }
}

pub fn initial_accessibility_system(cx: &mut Context) -> TreeUpdate {
    let iterator = LayoutTreeIterator::full(&cx.tree);

    let mut nodes = vec![];

    for entity in iterator {
        let mut access_context = AccessContext {
            current: entity,
            tree: &cx.tree,
            cache: &cx.cache,
            style: &cx.style,
            text_context: &mut cx.text_context,
        };

        if let Some(node) = get_access_node(&mut access_context, &mut cx.views, entity) {
            // let navigable = cx
            //     .style
            //     .abilities
            //     .get(entity)
            //     .copied()
            //     .unwrap_or_default()
            //     .contains(Abilities::NAVIGABLE);

            // if node.node_builder.role() == Role::Unknown && !navigable {
            //     continue;
            // }

            //let mut nodes = vec![(node.node_id(), node.node_builder)];
            nodes.push((node.node_id(), node.node_builder));

            // If child nodes were generated then append them to the nodes list
            if !node.children.is_empty() {
                nodes.extend(
                    node.children
                        .into_iter()
                        .map(|child_node| (child_node.node_id(), child_node.node_builder)),
                );
            }
        }

        // }
    }

    TreeUpdate {
        nodes,
        tree: Some(Tree::new(Entity::root().accesskit_id())),
        focus: Entity::root().accesskit_id(),
    }
}

pub(crate) fn get_access_node(
    cx: &mut AccessContext,
    views: &mut HashMap<Entity, Box<dyn ViewHandler>>,
    entity: Entity,
) -> Option<AccessNode> {
    let mut node_builder = Node::default();

    if let Some(role) = cx.style.role.get(entity) {
        node_builder.set_role(*role);
    }

    let bounds = cx.cache.get_bounds(entity);

    node_builder.set_bounds(Rect {
        x0: bounds.left() as f64,
        y0: bounds.top() as f64,
        x1: bounds.right() as f64,
        y1: bounds.bottom() as f64,
    });

    if let Some(disabled) = cx.style.disabled.get(entity).copied() {
        if disabled {
            node_builder.set_disabled();
        } else {
            node_builder.clear_disabled();
        }
    }

    let focusable = cx
        .style
        .abilities
        .get(entity)
        .map(|flags| flags.contains(Abilities::NAVIGABLE))
        .unwrap_or(false);

    if focusable {
        node_builder.add_action(Action::Focus);
    } else {
        node_builder.remove_action(Action::Focus);
    }

    if let Some(value) = cx.style.text_value.get(entity) {
        node_builder.set_value(value.clone().into_boxed_str());
    }

    // if let Some(name) = cx.style.name.get(entity) {
    //     node_builder.set_name(name.clone().into_boxed_str());
    // }

    if let Some(numeric_value) = cx.style.numeric_value.get(entity) {
        node_builder.set_numeric_value(*numeric_value);
    }

    if let Some(hidden) = cx.style.hidden.get(entity) {
        if *hidden {
            node_builder.set_hidden();
        } else {
            node_builder.clear_hidden();
        }
    }

    if let Some(live) = cx.style.live.get(entity) {
        node_builder.set_live(*live);
    }

    // if let Some(default_action_verb) = cx.style.default_action_verb.get(entity) {
    //     node_builder.set_default_action_verb(*default_action_verb);
    // }

    if let Some(labelled_by) = cx.style.labelled_by.get(entity) {
        node_builder.set_labelled_by(vec![labelled_by.accesskit_id()]);
    }

    let checkable = cx
        .style
        .abilities
        .get(entity)
        .map(|abilities| abilities.contains(Abilities::CHECKABLE))
        .unwrap_or_default();

    if checkable {
        if let Some(checked) = cx
            .style
            .pseudo_classes
            .get(entity)
            .map(|pseudoclass| pseudoclass.contains(PseudoClassFlags::CHECKED))
        {
            if checked {
                node_builder.set_toggled(Toggled::True);
            } else {
                node_builder.set_toggled(Toggled::False);
            }
        }
    }

    let mut node =
        AccessNode { node_id: entity.accesskit_id(), node_builder, children: Vec::new() };

    if let Some(view) = views.remove(&entity) {
        view.accessibility(cx, &mut node);

        views.insert(entity, view);
    }

    // Layout children
    let children =
        entity.child_iter(cx.tree).map(|entity| entity.accesskit_id()).collect::<Vec<_>>();

    // Children added by `accessibility` function
    let mut child_ids =
        node.children.iter().map(|child_node| child_node.node_id()).collect::<Vec<_>>();

    child_ids.extend(children);

    if !child_ids.is_empty() {
        node.node_builder.set_children(child_ids);
    }

    Some(node)
}
