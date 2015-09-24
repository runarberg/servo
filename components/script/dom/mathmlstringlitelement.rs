/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::MathMLStringLitElementBinding;
use dom::bindings::codegen::Bindings::MathMLStringLitElementBinding::MathMLStringLitElementMethods;
use dom::bindings::codegen::InheritTypes::MathMLStringLitElementDerived;
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
pub struct MathMLStringLitElement {
    mathmlpresentationtoken: MathMLPresentationToken
}

impl MathMLStringLitElementDerived for EventTarget {
    fn is_mathmlstringlitelement(&self) -> bool {
        *self.type_id() ==
            EventTargetTypeId::Node(
                NodeTypeId::Element(
                    ElementTypeId::MathMLElement(
                        MathMLElementTypeId::MathMLPresentationElement(
                            MathMLPresentationElementTypeId::MathMLPresentationToken(
                                MathMLPresentationTokenTypeId::MathMLStringLitElement)))))
    }
}

impl MathMLStringLitElement {
    fn new_inherited(localName: DOMString,
                     prefix: Option<DOMString>,
                     document: &Document) -> MathMLStringLitElement {
        MathMLStringLitElement {
            mathmlpresentationtoken: MathMLPresentationToken::new_inherited(
                MathMLPresentationTokenTypeId::MathMLStringLitElement,
                localName, prefix,document)
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<MathMLStringLitElement> {
        let element = MathMLStringLitElement::new_inherited(localName, prefix, document);
        Node::reflect_node(box element, document, MathMLStringLitElementBinding::Wrap)
    }
}

impl MathMLStringLitElementMethods for MathMLStringLitElement {
    // http://www.w3.org/TR/MathML/chapter3.html#id.3.2.8.2
    make_getter!(Lquote);
    make_setter!(SetLquote, "lquote");
    make_getter!(Rquote);
    make_setter!(SetRquote, "rqoute");
}
