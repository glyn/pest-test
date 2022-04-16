/*
 * Copyright 2020 VMware, Inc.
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */

use serde_json::Value;

/// A path is a tree of selector nodes.
///
/// For example, the JSONPath `$.foo.bar` yields this AST:
///
/// ```text
///              ^
///             / \
///            ^   \___ DotName("bar")
///           / \
/// Root ___ /   \___ DotName("foo")
/// ```
///
/// A more complicated example: `$.foo[1,2]["bar"]`:
///
/// ```text
///                 ^
///                / \
///               ^   \___ Union
///              / \            \
///             /   \___ Union   \___ [Name("bar")]
///            /              \
///           ^                \___ [Index(1), Index(2)]
///          / \
/// Root ___/   \___ DotName("foo")
/// ```
///
/// Selectors are left associative, thus `$.foo[1,2]["bar"]` behaves
/// like (pseudocode) `(($.foo)[1,2])["bar"]`; thus the root of the resulting
/// tree is actually the right-most selector (the last one to be applied).
///
/// The Path::Root AST node is called "root" because that's the
/// name of the node in the JSONPath grammar. It represents the source of
/// the json value stream which gets operated upon by Selector nodes.
/// This is why despite being called "root", this node doesn't lie at the root
/// of the AST tree.
#[derive(Debug)]
pub enum Path {
    Root,
    Sel(Box<Path>, Selector),
}

type Iter<'a> = Box<dyn Iterator<Item = &'a Value> + 'a>;

impl Path {
    pub fn find<'a>(&'a self, input: &'a Value) -> Iter<'a> {
        match self {
            Path::Root => Box::new(std::iter::once(input)),
            Path::Sel(_left, _sel) => panic!("not implemented!"),
        }
    }
}

#[derive(Debug)]
pub enum Selector {
    Matcher,
}
