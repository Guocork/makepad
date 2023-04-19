
use {
    crate::{
        makepad_draw::*,
        makepad_widgets::*,
    }
};

live_design!{
    import makepad_draw::shader::std::*;
    import makepad_widgets::theme::*;
    
    DrawButton = {{DrawButton}} {
        
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            sdf.box(1, 1, self.rect_size.x - 5, self.rect_size.y - 5, 1.5);
            sdf.stroke_keep(mix(#xFFFFFF80, #x00000040, pow(self.pos.y, 0.2)), 1.0);
            sdf.fill(
                mix(
                    mix(
                        mix(#xFFFFFF18, #xFFFDDD30, pow(length((self.pos - vec2(0.5, 0.5)) * 1.2), 1.5)), // 1st value = center, 2nd value = outer edges
                        mix(#xFFFFFF40, #xFFFFFF20, pow(length((self.pos - vec2(0.5, 0.5)) * 1.2), 1.25)),
                        self.hover
                    ),
                    mix(#xFFFDDDFF, #xFFFFFF08, pow(length((self.pos - vec2(0.5, 0.5)) * 1.2), 1.25)),
                    // mix(#xFFFFFFFF, #xFFFFFF08, pow(length((self.pos - vec2(0.5, 0.5)) * 1.2), 1.25)),
                    self.active
                )
            );
            return sdf.result
        }
    }
    
    SeqButton = {{SeqButton}} {
        state: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {draw_button: {hover: 0.0}}
                }
                
                on = {
                    from: {all: Snap}
                    apply: {draw_button: {hover: 1.0}}
                }
            }
            
            active = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.15}}
                    apply: {draw_button: {active: 0.0}}
                }
                
                on = {
                    from: {all: Snap}
                    apply: {draw_button: {active: 1.0}}
                }
            }
        }
    }
    
    Sequencer = {{Sequencer}} {
        button: <SeqButton> {}
        button_size: vec2(25.0, 25.0),
        grid_x: 16,
        grid_y: 16,
        walk: {
            margin: {top: 3, right: 10, bottom: 3, left: 10},
            width: Fit,
            height: Fit
        }
    }
}

// TODO support a shared 'inputs' struct on drawshaders
#[derive(Live, LiveHook)]#[repr(C)]
struct DrawButton {
    draw_super: DrawQuad,
    active: f32,
    hover: f32,
}

#[derive(Live, LiveHook)]
pub struct SeqButton {
    draw_button: DrawButton,
    state: State,
    x: usize,
    y: usize
}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct SeqButtonId(pub LiveId);

#[derive(Live)]
#[live_design_with {widget_factory!(cx, Sequencer)}]
pub struct Sequencer {
    #[rust] area: Area,
    walk: Walk,
    button: Option<LivePtr>,
    
    grid_x: usize,
    grid_y: usize,
    
    button_size: DVec2,
    
    #[rust] buttons: ComponentMap<SeqButtonId, SeqButton>,
}

impl LiveHook for Sequencer {
    fn after_apply(&mut self, cx: &mut Cx, from: ApplyFrom, index: usize, nodes: &[LiveNode]) {
        for button in self.buttons.values_mut() {
            if let Some(index) = nodes.child_by_name(index, live_id!(button).as_field()) {
                button.apply(cx, from, index, nodes);
            }
        }
        self.area.redraw(cx);
    }
}

#[derive(Clone, WidgetAction)]
pub enum SequencerAction {
    Change,
    None
}

impl SeqButton {
    
    pub fn draw_abs(&mut self, cx: &mut Cx2d, rect: Rect) {
        self.draw_button.draw_abs(cx, rect);
    }
    
    fn set_is_active(&mut self, cx: &mut Cx, is: bool, animate: Animate) {
        self.toggle_state(cx, is, animate, id!(active.on), id!(active.off))
    }
    
    fn is_active(&self, cx: &Cx) -> bool {
        self.state.is_in_state(cx, id!(active.on))
    }
    
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        sweep_area: Area,
        dispatch_action: &mut dyn FnMut(&mut Cx, SequencerAction),
    ) {
        if self.state_handle_event(cx, event).must_redraw() {
            self.draw_button.area().redraw(cx);
        }
        match event.hits_with_options(
            cx,
            self.draw_button.area(),
            HitOptions::new().with_sweep_area(sweep_area)
        ) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animate_state(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animate_state(cx, id!(hover.off));
            }
            Hit::FingerDown(_) => {
                if self.state.is_in_state(cx, id!(active.on)) {
                    self.animate_state(cx, id!(active.off));
                    dispatch_action(cx, SequencerAction::Change);
                }
                else {
                    self.animate_state(cx, id!(active.on));
                    dispatch_action(cx, SequencerAction::Change);
                    
                }
                self.animate_state(cx, id!(hover.on));
            }
            Hit::FingerUp(se) => {
                if !se.is_sweep && se.is_over && se.device.has_hovers() {
                    self.animate_state(cx, id!(hover.on));
                }
                else {
                    self.animate_state(cx, id!(hover.off));
                }
            }
            _ => {}
        }
    }
}


impl Sequencer {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        cx.begin_turtle(walk, Layout::default());
        
        let start_pos = cx.turtle().pos(); //+ vec2(10., 10.);
        
        let rect = cx.turtle().rect();
        let sz = rect.size / dvec2(self.grid_x as f64, self.grid_y as f64);
        let button = self.button;
        for y in 0..self.grid_y {
            for x in 0..self.grid_x {
                let i = x + y * self.grid_x;
                let pos = start_pos + dvec2(x as f64 * sz.x, y as f64 * sz.y);
                let btn_id = LiveId(i as u64).into();
                let btn = self.buttons.get_or_insert(cx, btn_id, | cx | {
                    SeqButton::new_from_ptr(cx, button)
                });
                btn.x = x;
                btn.y = y;
                btn.draw_abs(cx, Rect {pos: pos, size: sz});
            }
        }
        let used = dvec2(self.grid_x as f64 * self.button_size.x, self.grid_y as f64 * self.button_size.y);
        
        cx.turtle_mut().set_used(used.x, used.y);
        
        cx.end_turtle_with_area(&mut self.area);
        self.buttons.retain_visible();
    }
    
    pub fn _set_key_focus(&self, cx: &mut Cx) {
        cx.set_key_focus(self.area);
    }
    
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, SequencerAction),
    ) {
        for button in self.buttons.values_mut() {
            button.handle_event_with(cx, event, self.area, dispatch_action);
        }
        
        match event.hits(cx, self.area) {
            Hit::KeyFocus(_) => {
            }
            Hit::KeyFocusLost(_) => {
            }
            _ => ()
        }
    }
    
    pub fn get_steps(&self, cx: &Cx) -> Vec<u32> {
        let mut steps = Vec::new();
        steps.resize(self.grid_y, 0u32);
        for (btn_id, button) in self.buttons.iter() {
            let active = button.is_active(cx);
            let i = btn_id.0.0 as usize;
            let x = i % self.grid_x;
            let y = i / self.grid_x;
            if active {steps[x] |= 1 << y};
        }
        steps
    }
    
    pub fn set_steps(&mut self, cx: &mut Cx, steps: &[u32]) {
        if steps.len() != self.grid_x {
            panic!("Steps not correct for sequencer got {} expected {}", steps.len(), self.grid_x);
        }
        for (btn_id, button) in self.buttons.iter_mut() {
            let i = btn_id.0.0 as usize;
            let x = i % self.grid_x;
            let y = i / self.grid_x;
            let bit = 1 << y;
            let active = steps[x] & bit == bit;
            button.set_is_active(cx, active, Animate::Yes);
        }
    }
    
    pub fn write_state_to_data(&self, cx: &mut Cx, nodes: &mut LiveNodeVec, path: &[LiveId]) {
        let steps = self.get_steps(cx);
        let mut array = LiveNodeVec::new();
        array.open_array(LiveId(0));
        for step in steps {
            array.push(LiveNode::from_value(LiveValue::Int64(step as i64)));
        }
        array.close();
        nodes.write_field_nodes(path, &array);
    }
}


impl Widget for Sequencer {
    fn redraw(&mut self, cx: &mut Cx) {
        self.area.redraw(cx);
    }
    
    fn widget_uid(&self) -> WidgetUid {return WidgetUid(self as *const _ as u64)}
    
    fn handle_widget_event_with(&mut self, cx: &mut Cx, event: &Event, dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem)) {
        let uid = self.widget_uid();
        self.handle_event_with(cx, event, &mut | cx, action | {
            dispatch_action(cx, WidgetActionItem::new(action.into(), uid))
        });
    }
    
    fn get_walk(&self) -> Walk {self.walk}
    
    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
    
    fn widget_to_data(&self, cx: &mut Cx, actions: &WidgetActions, nodes: &mut LiveNodeVec, path: &[LiveId]) -> bool {
        let uid = self.widget_uid();
        if actions.find_single_action(uid).is_some() {
            self.write_state_to_data(cx, nodes, path);
            true
        }
        else {
            false
        }
    }

    fn data_to_widget(&mut self, cx: &mut Cx, nodes:&[LiveNode], path: &[LiveId]){
        if let Some(mut index) = nodes.child_by_field_path(0, path) {
            let mut steps = Vec::new();
            if nodes[index].is_array() {
                index += 1;
                while !nodes[index].is_close() {
                    steps.push(nodes[index].value.as_int().unwrap_or(0) as u32);
                    index += 1;
                }
            }
            self.set_steps(cx, &steps);
        }
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct SequencerRef(WidgetRef);

impl SequencerRef {
    
    pub fn clear_grid(&self, cx: &mut Cx, actions: &mut WidgetActions) {
        if let Some(mut inner) = self.inner_mut() {
            let mut steps = inner.get_steps(cx);
            for step in &mut steps {*step = 0};
            inner.set_steps(cx, &steps);
            actions.push(WidgetActionItem::new(SequencerAction::Change.into(), inner.widget_uid()));
        }
    }
    
    pub fn grid_down(&self, cx: &mut Cx, actions: &mut WidgetActions) {
        if let Some(mut inner) = self.inner_mut() {
            let mut steps = inner.get_steps(cx);
            for step in &mut steps {
                let mut modstep = *step << 1;
                if (modstep & 1 << 16) == 1 << 16 {modstep += 1; modstep -= 1 << 16};
                *step = modstep;
            }
            inner.set_steps(cx, &steps);
            actions.push(WidgetActionItem::new(SequencerAction::Change.into(), inner.widget_uid()));
        }
    }
    
    pub fn grid_up(&self, cx: &mut Cx, actions: &mut WidgetActions) {
        if let Some(mut inner) = self.inner_mut() {
            let mut steps = inner.get_steps(cx);
            for step in &mut steps {
                let mut modstep = *step >> 1;
                if (*step & 1) == 1 {modstep += 1 << 15;}
                *step = modstep;
            }
            inner.set_steps(cx, &steps);
            actions.push(WidgetActionItem::new(SequencerAction::Change.into(), inner.widget_uid()));
        }
    }
}
