/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::MathMLMathElementBinding;
use dom::bindings::codegen::InheritTypes::MathMLMathElementDerived;
use dom::bindings::js::Root;
use dom::document::Document;
use dom::element::ElementTypeId;
use dom::eventtarget::{EventTarget, EventTargetTypeId};
use dom::mathmlelement::{MathMLElement, MathMLElementTypeId};
use dom::node::{Node, NodeTypeId};
use util::str::DOMString;

#[dom_struct]
pub struct MathMLMathElement {
    mathmlelement: MathMLElement
}

impl MathMLMathElementDerived for EventTarget {
    fn is_mathmlmathelement(&self) -> bool {
        *self.type_id() ==
            EventTargetTypeId::Node(
                NodeTypeId::Element(ElementTypeId::MathMLElement(MathMLElementTypeId::MathMLMathElement)))
    }
}

impl MathMLMathElement {
    fn new_inherited(localName: DOMString,
                     prefix: Option<DOMString>,
                     document: &Document) -> MathMLMathElement {
        MathMLMathElement {
            mathmlelement: MathMLElement::new_inherited(MathMLElementTypeId::MathMLMathElement,
                                                        localName,
                                                        prefix,
                                                        document)
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<MathMLMathElement> {
        let element = MathMLMathElement::new_inherited(localName, prefix, document);
        Node::reflect_node(box element, document, MathMLMathElementBinding::Wrap)
    }
}
