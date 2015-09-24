/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// http://www.w3.org/TR/MathML2/appendixd.html#dom.StringLitElement
// http://www.w3.org/TR/MathML/chapter3.html#presm.ms
interface MathMLStringLitElement : MathMLPresentationToken {
  attribute DOMString lquote;
  attribute DOMString rquote;
};
