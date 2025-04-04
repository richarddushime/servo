/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::default::Default;

use dom_struct::dom_struct;
use js::rust::HandleObject;

use crate::dom::bindings::codegen::Bindings::FocusEventBinding;
use crate::dom::bindings::codegen::Bindings::FocusEventBinding::FocusEventMethods;
use crate::dom::bindings::codegen::Bindings::UIEventBinding::UIEventMethods;
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::reflect_dom_object_with_proto;
use crate::dom::bindings::root::{DomRoot, MutNullableDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::eventtarget::EventTarget;
use crate::dom::uievent::UIEvent;
use crate::dom::window::Window;
use crate::script_runtime::CanGc;

#[dom_struct]
pub(crate) struct FocusEvent {
    uievent: UIEvent,
    related_target: MutNullableDom<EventTarget>,
}

impl FocusEvent {
    fn new_inherited() -> FocusEvent {
        FocusEvent {
            uievent: UIEvent::new_inherited(),
            related_target: Default::default(),
        }
    }

    pub(crate) fn new_uninitialized(window: &Window, can_gc: CanGc) -> DomRoot<FocusEvent> {
        Self::new_uninitialized_with_proto(window, None, can_gc)
    }

    pub(crate) fn new_uninitialized_with_proto(
        window: &Window,
        proto: Option<HandleObject>,
        can_gc: CanGc,
    ) -> DomRoot<FocusEvent> {
        reflect_dom_object_with_proto(Box::new(FocusEvent::new_inherited()), window, proto, can_gc)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        window: &Window,
        type_: DOMString,
        can_bubble: EventBubbles,
        cancelable: EventCancelable,
        view: Option<&Window>,
        detail: i32,
        related_target: Option<&EventTarget>,
        can_gc: CanGc,
    ) -> DomRoot<FocusEvent> {
        Self::new_with_proto(
            window,
            None,
            type_,
            can_bubble,
            cancelable,
            view,
            detail,
            related_target,
            can_gc,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new_with_proto(
        window: &Window,
        proto: Option<HandleObject>,
        type_: DOMString,
        can_bubble: EventBubbles,
        cancelable: EventCancelable,
        view: Option<&Window>,
        detail: i32,
        related_target: Option<&EventTarget>,
        can_gc: CanGc,
    ) -> DomRoot<FocusEvent> {
        let ev = FocusEvent::new_uninitialized_with_proto(window, proto, can_gc);
        ev.upcast::<UIEvent>().InitUIEvent(
            type_,
            bool::from(can_bubble),
            bool::from(cancelable),
            view,
            detail,
        );
        ev.related_target.set(related_target);
        ev
    }
}

impl FocusEventMethods<crate::DomTypeHolder> for FocusEvent {
    // https://w3c.github.io/uievents/#dom-focusevent-focusevent
    fn Constructor(
        window: &Window,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        type_: DOMString,
        init: &FocusEventBinding::FocusEventInit,
    ) -> Fallible<DomRoot<FocusEvent>> {
        let bubbles = EventBubbles::from(init.parent.parent.bubbles);
        let cancelable = EventCancelable::from(init.parent.parent.cancelable);
        let event = FocusEvent::new_with_proto(
            window,
            proto,
            type_,
            bubbles,
            cancelable,
            init.parent.view.as_deref(),
            init.parent.detail,
            init.relatedTarget.as_deref(),
            can_gc,
        );
        event
            .upcast::<Event>()
            .set_composed(init.parent.parent.composed);
        Ok(event)
    }

    // https://w3c.github.io/uievents/#widl-FocusEvent-relatedTarget
    fn GetRelatedTarget(&self) -> Option<DomRoot<EventTarget>> {
        self.related_target.get()
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.uievent.IsTrusted()
    }
}
