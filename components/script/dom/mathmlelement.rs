/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::MathMLElementBinding;
use dom::bindings::codegen::InheritTypes::{ElementCast, MathMLElementDerived};
use dom::bindings::js::Root;
use dom::bindings::utils::Reflectable;
use dom::document::Document;
use dom::element::Element;
use dom::element::ElementTypeId;
use dom::eventtarget::{EventTarget, EventTargetTypeId};
use dom::node::{Node, NodeTypeId};
use dom::virtualmethods::VirtualMethods;

use util::str::DOMString;

use std::intrinsics;

#[dom_struct]
pub struct MathMLElement {
    element: Element
}

impl PartialEq for MathMLElement {
    fn eq(&self, other: &MathMLElement) -> bool {
        self as *const MathMLElement == &*other
    }
}

impl MathMLElementDerived for EventTarget {
    fn is_mathmlelement(&self) -> bool {
        match *self.type_id() {
            EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::MathMLElement(_))) => true,
            _ => false
        }
    }
}

impl MathMLElement {
    pub fn new_inherited(type_id: MathMLElementTypeId,
                         tag_name: DOMString,
                         prefix: Option<DOMString>,
                         document: &Document) -> MathMLElement {
        MathMLElement {
            element: Element::new_inherited(ElementTypeId::MathMLElement(type_id),
                                            tag_name,
                                            ns!(MathML),
                                            prefix, document)
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString, prefix: Option<DOMString>, document: &Document)
               -> Root<MathMLElement> {
        let element = MathMLElement::new_inherited(MathMLElementTypeId::MathMLElement,
                                                   localName, prefix, document);
        Node::reflect_node(box element, document, MathMLElementBinding::Wrap)
    }
}

#[derive(JSTraceable, Copy, Clone, Debug, HeapSizeOf)]
pub enum MathMLElementTypeId {
    MathMLElement,

    MathMLMathElement
}

impl PartialEq for MathMLElementTypeId {
    #[allow(unsafe_code)]
    fn eq(&self, other: &MathMLElementTypeId) -> bool {
        unsafe {
            intrinsics::discriminant_value(self) == intrinsics::discriminant_value(other)
        }
    }
}

impl VirtualMethods for MathMLElement {
    fn super_type<'b>(&'b self) -> Option<&'b VirtualMethods> {
        let element: &Element = ElementCast::from_ref(self);
        Some(element as &VirtualMethods)
    }
}
