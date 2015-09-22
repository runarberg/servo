/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::MathMLPresentationElementBinding;
use dom::bindings::codegen::InheritTypes::{MathMLElementCast, MathMLPresentationElementDerived};
use dom::bindings::js::Root;
use dom::bindings::utils::Reflectable;
use dom::document::Document;
use dom::element::ElementTypeId;
use dom::eventtarget::{EventTarget, EventTargetTypeId};
use dom::mathmlelement::{MathMLElement, MathMLElementTypeId};
use dom::node::{Node, NodeTypeId};
use dom::virtualmethods::VirtualMethods;

use util::str::DOMString;

use std::intrinsics;

#[dom_struct]
pub struct MathMLPresentationElement {
    mathmlelement: MathMLElement
}

impl PartialEq for MathMLPresentationElement {
    fn eq(&self, other: &MathMLPresentationElement) -> bool {
        self as *const MathMLPresentationElement == &*other
    }
}

impl MathMLPresentationElementDerived for EventTarget {
    fn is_mathmlpresentationelement(&self) -> bool {
        match *self.type_id() {
            EventTargetTypeId::Node(NodeTypeId::Element(ElementTypeId::MathMLElement(
                MathMLElementTypeId::MathMLPresentationElement(_)))) => true,
            _ => false
        }
    }
}

impl MathMLPresentationElement {
    pub fn new_inherited(type_id: MathMLPresentationElementTypeId,
                         tag_name: DOMString,
                         prefix: Option<DOMString>,
                         document: &Document) -> MathMLPresentationElement {
        MathMLPresentationElement {
            mathmlelement: MathMLElement::new_inherited(
                MathMLElementTypeId::MathMLPresentationElement(type_id),
                tag_name, prefix, document)
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<MathMLPresentationElement> {
        let element = MathMLPresentationElement::new_inherited(
            MathMLPresentationElementTypeId::MathMLPresentationElement,
            localName, prefix, document);
        Node::reflect_node(box element, document, MathMLPresentationElementBinding::Wrap)
    }
}

#[derive(JSTraceable, Copy, Clone, Debug, HeapSizeOf)]
pub enum MathMLPresentationElementTypeId {
    MathMLPresentationElement,

    MathMLPresentationToken,
}

impl PartialEq for MathMLPresentationElementTypeId {
    #[allow(unsafe_code)]
    fn eq(&self, other: &MathMLPresentationElementTypeId) -> bool {
        unsafe {
            intrinsics::discriminant_value(self) == intrinsics::discriminant_value(other)
        }
    }
}

impl VirtualMethods for MathMLPresentationElement {
    fn super_type<'b>(&'b self) -> Option<&'b VirtualMethods> {
        let mathmlelement: &MathMLElement = MathMLElementCast::from_ref(self);
        Some(mathmlelement as &VirtualMethods)
    }
}
