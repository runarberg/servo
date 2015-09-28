/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::attr::Attr;
use dom::bindings::codegen::Bindings::MathMLPresentationTokenBinding;
use dom::bindings::codegen::Bindings::MathMLPresentationTokenBinding::MathMLPresentationTokenMethods;
use dom::bindings::codegen::Bindings::NodeBinding::NodeMethods;
use dom::bindings::codegen::InheritTypes::MathMLPresentationElementCast;
use dom::bindings::codegen::InheritTypes::MathMLPresentationTokenDerived;
use dom::bindings::codegen::InheritTypes::{ElementCast, NodeCast};
use dom::bindings::js::Root;
use dom::document::Document;
use dom::element::{AttributeMutation, ElementTypeId};
use dom::eventtarget::{EventTarget, EventTargetTypeId};
use dom::mathmlelement::MathMLElementTypeId;
use dom::mathmlpresentationelement::{MathMLPresentationElement, MathMLPresentationElementTypeId};
use dom::nodelist::NodeList;
use dom::node::{Node, NodeTypeId};
use dom::virtualmethods::VirtualMethods;

use string_cache::Atom;
use util::str::DOMString;

use std::borrow::ToOwned;
use std::cell::Cell;
use std::intrinsics;

#[derive(JSTraceable, PartialEq, Copy, Clone)]
#[allow(dead_code)]
#[derive(HeapSizeOf)]
enum MathVariant {
    Normal,
    Bold,
    Italic,
    BoldItalic,
    // DoubleStruck,
    // BoldFraktur,
    // Script,
    // BoldScript,
    // Fraktur,
    // SansSerif,
    // BoldSansSerif,
    // SansSerifItalic,
    // SansSerifBoldItalic,
    // Monospace,
    // Initial,
    // Tailed,
    // Looped,
    // Stretched,
}

#[dom_struct]
pub struct MathMLPresentationToken {
    mathmlpresentationelement: MathMLPresentationElement,

    mathvariant: Cell<MathVariant>,
}

impl MathMLPresentationTokenDerived for EventTarget {
    fn is_mathmlpresentationtoken(&self) -> bool {
        *self.type_id() ==
            EventTargetTypeId::Node(
                NodeTypeId::Element(
                    ElementTypeId::MathMLElement(
                        MathMLElementTypeId::MathMLPresentationElement(
                            MathMLPresentationElementTypeId::MathMLPresentationToken(
                                MathMLPresentationTokenTypeId::MathMLPresentationToken)))))
    }
}

impl MathMLPresentationToken {
    pub fn new_inherited(type_id: MathMLPresentationTokenTypeId,
                     tag_name: DOMString,
                     prefix: Option<DOMString>,
                     document: &Document) -> MathMLPresentationToken {
        MathMLPresentationToken {
            mathmlpresentationelement: MathMLPresentationElement::new_inherited(
                MathMLPresentationElementTypeId::MathMLPresentationToken(type_id),
                tag_name, prefix, document),
            mathvariant: Cell::new(MathVariant::Normal),
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<MathMLPresentationToken> {
        let element = MathMLPresentationToken::new_inherited(
            MathMLPresentationTokenTypeId::MathMLPresentationToken,
            localName, prefix, document);
        Node::reflect_node(box element, document, MathMLPresentationTokenBinding::Wrap)
    }

    fn default_math_variant(&self) -> &str {
        let node = NodeCast::from_ref(self);

        if node.NodeName() == "mi" {
            // Mathvariant defalts to 'normal' except for single
            // character 'mi' elements.
            match node.GetTextContent().map(|s| s.chars().count()) {
                Some(1) => "italic",
                _ => "normal",
            }
        } else {
            "normal"
        }
    }
}

#[derive(JSTraceable, Copy, Clone, Debug, HeapSizeOf)]
pub enum MathMLPresentationTokenTypeId {
    MathMLPresentationToken,

    MathMLOperatorElement,
    MathMLStringLitElement,
}

impl PartialEq for MathMLPresentationTokenTypeId {
    #[allow(unsafe_code)]
    fn eq(&self, other: &MathMLPresentationTokenTypeId) -> bool {
        unsafe {
            intrinsics::discriminant_value(self) == intrinsics::discriminant_value(other)
        }
    }
}

impl MathMLPresentationTokenMethods for MathMLPresentationToken {
    // XXX: Should return Root<MathMLNodeList>
    // http://www.w3.org/TR/MathML2/appendixd.html#id.D.1.3.2
    fn Contents(&self) -> Root<NodeList> {
        let node = NodeCast::from_ref(self);
        node.ChildNodes()
    }

    // http://www.w3.org/TR/MathML3/chapter3.html#presm.commatt
    fn Mathvariant(&self) -> DOMString {
        let element = ElementCast::from_ref(self);
        let default = self.default_math_variant();
        let val = element.get_string_attribute(&Atom::from_slice("mathvariant"));
        // https://html.spec.whatwg.org/multipage/#attr-fs-method
        match &*val {
            "normal" | "bold" | "italic" | "bold-italic" => val,
            _ => default.to_owned()
        }
    }
    make_setter!(SetMathvariant, "mathvariant");
}

impl VirtualMethods for MathMLPresentationToken {
    fn super_type<'b>(&'b self) -> Option<&'b VirtualMethods> {
        let mathmlpresentationelement: &MathMLPresentationElement =
            MathMLPresentationElementCast::from_ref(self);
        Some(mathmlpresentationelement as &VirtualMethods)
    }

    fn attribute_mutated(&self, attr: &Attr, mutation: AttributeMutation) {
        self.super_type().unwrap().attribute_mutated(attr, mutation);
        match attr.local_name() {
            &atom!(mathvariant) => {
                match mutation {
                    AttributeMutation::Set(_) => {
                        let value = match &**attr.value() {
                            "bold" => MathVariant::Bold,
                            "italic" => MathVariant::Italic,
                            "bold-italic" => MathVariant::BoldItalic,
                            _ => MathVariant::Normal,
                        };
                        self.mathvariant.set(value);
                    },
                    AttributeMutation::Removed => {
                        self.mathvariant.set(MathVariant::Normal);
                    }
                }
            },
            _ => {}
        }
    }
}
