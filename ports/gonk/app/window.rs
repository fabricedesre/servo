/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A windowing implementation using OpenGLES.

use NestedEventLoopListener;
use compositing::compositor_task::{mod, CompositorProxy, CompositorReceiver};
use compositing::windowing::{WindowEvent, WindowMethods};
use compositing::windowing::{IdleWindowEvent, ResizeWindowEvent, LoadUrlWindowEvent};
use compositing::windowing::{MouseWindowEventClass,  MouseWindowMoveEventClass, ScrollWindowEvent};
use compositing::windowing::{ZoomWindowEvent, PinchZoomWindowEvent, NavigationWindowEvent};
use compositing::windowing::{FinishedWindowEvent, QuitWindowEvent, MouseWindowClickEvent};
use compositing::windowing::{MouseWindowMouseDownEvent, MouseWindowMouseUpEvent};
use compositing::windowing::{RefreshWindowEvent, Forward, Back};
use geom::point::{Point2D, TypedPoint2D};
use geom::scale_factor::ScaleFactor;
use geom::size::TypedSize2D;
use layers::geometry::DevicePixel;
use layers::platform::surface::NativeGraphicsMetadata;
use libc::c_int;
use msg::compositor_msg::{IdleRenderState, RenderState, RenderingRenderState};
use msg::compositor_msg::{FinishedLoading, Blank, Loading, PerformingLayout, ReadyState};
use std::cell::{Cell, RefCell};
use std::comm::Receiver;
use std::rc::Rc;
use time::{mod, Timespec};
use util::geometry::ScreenPx;

/// The type of a window.
pub struct Window {
    /*glfw: glfw::Glfw,

    glfw_window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,*/

    event_queue: RefCell<Vec<WindowEvent>>,

    //mouse_down_button: Cell<Option<glfw::MouseButton>>,
    mouse_down_point: Cell<Point2D<c_int>>,

    ready_state: Cell<ReadyState>,
    render_state: Cell<RenderState>,

    last_title_set_time: Cell<Timespec>,
}

impl Window {
    /// Creates a new window.
    pub fn new() -> Rc<Window> {
        // Create our window object.
        let window = Window {
            mouse_down_point: Cell::new(Point2D(0 as c_int, 0)),

            event_queue: RefCell::new(vec!()),

            ready_state: Cell::new(Blank),
            render_state: Cell::new(IdleRenderState),

            last_title_set_time: Cell::new(Timespec::new(0, 0)),
        };

        let wrapped_window = Rc::new(window);

        wrapped_window
    }

    fn update_window_title(&self) {
    }

    pub fn wait_events(&self) -> WindowEvent {
        {
            let mut event_queue = self.event_queue.borrow_mut();
            if !event_queue.is_empty() {
                return event_queue.remove(0).unwrap();
            }
        }

        /*self.glfw.wait_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            self.handle_window_event(&self.glfw_window, event);
        }

        if self.glfw_window.should_close() {
            QuitWindowEvent
        } else {
            self.event_queue.borrow_mut().remove(0).unwrap_or(IdleWindowEvent)
        }*/
        self.event_queue.borrow_mut().remove(0).unwrap_or(IdleWindowEvent)
    }

    pub unsafe fn set_nested_event_loop_listener(
            &self,
            listener: *mut NestedEventLoopListener + 'static) {
        /*self.glfw_window.set_refresh_polling(false);
        glfw::ffi::glfwSetWindowRefreshCallback(self.glfw_window.ptr, Some(on_refresh));
        glfw::ffi::glfwSetFramebufferSizeCallback(self.glfw_window.ptr, Some(on_framebuffer_size));
        g_nested_event_loop_listener = Some(listener)*/
    }

    pub unsafe fn remove_nested_event_loop_listener(&self) {
        /*glfw::ffi::glfwSetWindowRefreshCallback(self.glfw_window.ptr, None);
        glfw::ffi::glfwSetFramebufferSizeCallback(self.glfw_window.ptr, None);
        self.glfw_window.set_refresh_polling(true);
        g_nested_event_loop_listener = None*/
    }
}

struct GonkCompositorProxy {
    sender: Sender<compositor_task::Msg>,
}

impl CompositorProxy for GonkCompositorProxy {
    fn send(&mut self, msg: compositor_task::Msg) {
        // Send a message and kick the OS event loop awake.
        self.sender.send(msg);
        //glfw::Glfw::post_empty_event()
    }
    fn clone_compositor_proxy(&self) -> Box<CompositorProxy+Send> {
        box GonkCompositorProxy {
            sender: self.sender.clone(),
        } as Box<CompositorProxy+Send>
    }
}

impl WindowMethods for Window {
    /// Returns the size of the window in hardware pixels.
    fn framebuffer_size(&self) -> TypedSize2D<DevicePixel, uint> {
        let (width, height) = (480i, 800i);
        TypedSize2D(width as uint, height as uint)
    }

    /// Returns the size of the window in density-independent "px" units.
    fn size(&self) -> TypedSize2D<ScreenPx, f32> {
        let (width, height) = (480f32, 800f32);
        TypedSize2D(width as f32, height as f32)
    }

    /// Presents the window to the screen (perhaps by page flipping).
    fn present(&self) {
        //self.glfw_window.swap_buffers();
    }

    /// Sets the ready state.
    fn set_ready_state(&self, ready_state: ReadyState) {
        self.ready_state.set(ready_state);
        self.update_window_title()
    }

    /// Sets the render state.
    fn set_render_state(&self, render_state: RenderState) {
        if self.ready_state.get() == FinishedLoading &&
            self.render_state.get() == RenderingRenderState &&
            render_state == IdleRenderState {
            // page loaded
            self.event_queue.borrow_mut().push(FinishedWindowEvent);
        }

        self.render_state.set(render_state);
        self.update_window_title()
    }

    fn hidpi_factor(&self) -> ScaleFactor<ScreenPx, DevicePixel, f32> {
        let backing_size = self.framebuffer_size().width.get();
        let window_size = self.size().width.get();
        ScaleFactor((backing_size as f32) / window_size)
    }

    fn native_metadata(&self) -> NativeGraphicsMetadata {
        use egl::egl::GetCurrentDisplay;
        NativeGraphicsMetadata {
            display: GetCurrentDisplay(),
        }
    }

    fn create_compositor_channel(_: &Option<Rc<Window>>)
                                 -> (Box<CompositorProxy+Send>, Box<CompositorReceiver>) {
        let (sender, receiver) = channel();
        (box GonkCompositorProxy {
             sender: sender,
         } as Box<CompositorProxy+Send>,
         box receiver as Box<CompositorReceiver>)
    }
}
