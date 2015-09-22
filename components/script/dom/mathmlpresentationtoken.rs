/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::MathMLPresentationTokenBinding;
use dom::bindings::codegen::InheritTypes::MathMLPresentationElementCast;
use dom::bindings::codegen::InheritTypes::MathMLPresentationTokenDerived;
use dom::bindings::js::Root;
use dom::document::Document;
use dom::element::ElementTypeId;
use dom::eventtarget::{EventTarget, EventTargetTypeId};
use dom::mathmlelement::MathMLElementTypeId;
use dom::mathmlpresentationelement::{MathMLPresentationElement, MathMLPresentationElementTypeId};
use dom::node::{Node, NodeTypeId};
use dom::virtualmethods::VirtualMethods;
use util::str::DOMString;

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
                            MathMLPresentationElementTypeId::MathMLPresentationToken))))
    }
}

impl MathMLPresentationToken {
    fn new_inherited(localName: DOMString,
                     prefix: Option<DOMString>,
                     document: &Document) -> MathMLPresentationToken {
        MathMLPresentationToken {
            mathmlpresentationelement: MathMLPresentationElement::new_inherited(
                MathMLPresentationElementTypeId::MathMLPresentationToken,
                localName, prefix, document)
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<MathMLPresentationToken> {
        let element = MathMLPresentationToken::new_inherited(localName, prefix, document);
        Node::reflect_node(box element, document, MathMLPresentationTokenBinding::Wrap)
    }
}

impl VirtualMethods for MathMLPresentationToken {
    fn super_type<'b>(&'b self) -> Option<&'b VirtualMethods> {
        let mathmlpresentationelement: &MathMLPresentationElement =
            MathMLPresentationElementCast::from_ref(self);
        Some(mathmlpresentationelement as &VirtualMethods)
    }
}
