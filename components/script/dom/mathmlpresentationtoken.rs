/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::MathMLPresentationTokenBinding;
use dom::bindings::codegen::Bindings::MathMLPresentationTokenBinding::MathMLPresentationTokenMethods;
use dom::bindings::codegen::Bindings::NodeBinding::NodeMethods;
use dom::bindings::codegen::InheritTypes::MathMLPresentationElementCast;
use dom::bindings::codegen::InheritTypes::MathMLPresentationTokenDerived;
use dom::bindings::codegen::InheritTypes::NodeCast;
use dom::bindings::js::Root;
use dom::document::Document;
use dom::element::ElementTypeId;
use dom::eventtarget::{EventTarget, EventTargetTypeId};
use dom::mathmlelement::MathMLElementTypeId;
use dom::mathmlpresentationelement::{MathMLPresentationElement, MathMLPresentationElementTypeId};
use dom::nodelist::NodeList;
use dom::node::{Node, NodeTypeId};
use dom::virtualmethods::VirtualMethods;

use util::str::DOMString;

use std::intrinsics;

#[dom_struct]
pub struct MathMLPresentationToken {
    mathmlpresentationelement: MathMLPresentationElement
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
                tag_name, prefix, document)
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
}

impl VirtualMethods for MathMLPresentationToken {
    fn super_type<'b>(&'b self) -> Option<&'b VirtualMethods> {
        let mathmlpresentationelement: &MathMLPresentationElement =
            MathMLPresentationElementCast::from_ref(self);
        Some(mathmlpresentationelement as &VirtualMethods)
    }
}
