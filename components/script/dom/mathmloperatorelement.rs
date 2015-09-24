/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::MathMLOperatorElementBinding;
use dom::bindings::codegen::InheritTypes::MathMLOperatorElementDerived;
use dom::bindings::js::Root;
use dom::document::Document;
use dom::element::ElementTypeId;
use dom::eventtarget::{EventTarget, EventTargetTypeId};
use dom::mathmlelement::MathMLElementTypeId;
use dom::mathmlpresentationelement::MathMLPresentationElementTypeId;
use dom::mathmlpresentationtoken::{MathMLPresentationToken, MathMLPresentationTokenTypeId};
use dom::node::{Node, NodeTypeId};
use util::str::DOMString;

#[dom_struct]
pub struct MathMLOperatorElement {
    mathmlpresentationtoken: MathMLPresentationToken
}

impl MathMLOperatorElementDerived for EventTarget {
    fn is_mathmloperatorelement(&self) -> bool {
        *self.type_id() ==
            EventTargetTypeId::Node(
                NodeTypeId::Element(
                    ElementTypeId::MathMLElement(
                        MathMLElementTypeId::MathMLPresentationElement(
                            MathMLPresentationElementTypeId::MathMLPresentationToken(
                                MathMLPresentationTokenTypeId::MathMLOperatorElement)))))
    }
}

impl MathMLOperatorElement {
    fn new_inherited(localName: DOMString,
                     prefix: Option<DOMString>,
                     document: &Document) -> MathMLOperatorElement {
        MathMLOperatorElement {
            mathmlpresentationtoken: MathMLPresentationToken::new_inherited(
                MathMLPresentationTokenTypeId::MathMLOperatorElement,
                localName, prefix, document)
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<MathMLOperatorElement> {
        let element = MathMLOperatorElement::new_inherited(localName, prefix, document);
        Node::reflect_node(box element, document, MathMLOperatorElementBinding::Wrap)
    }
}
