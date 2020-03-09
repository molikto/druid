// Copyright 2020 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Demonstrates alignment of children in the flex container.

use druid::widget::{
    Button, Checkbox, CrossAxisAlignment, Flex, Label, MainAxisAlignment, ProgressBar, RadioGroup,
    SizedBox, Slider, Stepper, Switch, TextBox, WidgetExt,
};
use druid::{
    AppLauncher, BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle,
    LifeCycleCtx, LocalizedString, PaintCtx, PlatformError, Size, UnitPoint, UpdateCtx, Widget,
    WidgetId, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct AppState {
    demo_state: DemoState,
    params: Params,
}

#[derive(Clone, Data, Lens)]
struct DemoState {
    pub input_text: String,
    pub enabled: bool,
    volume: f64,
}

#[derive(Clone, Data, Lens)]
struct Params {
    axis: FlexType,
    cross_alignment: CrossAxisAlignment,
    main_alignment: MainAxisAlignment,
    debug_layout: bool,
    fix_minor_axis: bool,
    fix_major_axis: bool,
}

#[derive(Clone, Copy, PartialEq, Data)]
enum FlexType {
    Row,
    Column,
}

/// builds a child Flex widget from some paramaters.
struct Rebuilder {
    inner: Box<dyn Widget<AppState>>,
}

impl Rebuilder {
    fn new() -> Rebuilder {
        Rebuilder {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
        self.inner = build_widget(&data.params);
    }
}

impl Widget<AppState> for Rebuilder {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        if !old_data.params.same(&data.params) {
            self.rebuild_inner(data);
            ctx.children_changed();
        } else {
            self.inner.update(ctx, old_data, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.inner.paint(paint_ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}

fn labeled_checkbox(text: &str) -> impl Widget<bool> {
    Flex::row()
        .with_child(Checkbox::new().padding((0., 0., 8., 0.)), 0.)
        .with_child(Label::new(text), 0.)
}

fn make_control_row() -> impl Widget<AppState> {
    Flex::row()
        .with_child(
            Flex::column()
                .with_child(Label::new("Type:").padding(5.0), 0.)
                .with_child(
                    RadioGroup::new(vec![("Row", FlexType::Row), ("Column", FlexType::Column)])
                        .lens(Params::axis),
                    0.,
                ),
            0.0,
        )
        .with_child(
            Flex::column()
                .with_child(Label::new("CrossAxisAlignment:").padding(5.0), 0.)
                .with_child(
                    RadioGroup::new(vec![
                        ("Start", CrossAxisAlignment::Start),
                        ("Center", CrossAxisAlignment::Center),
                        ("End", CrossAxisAlignment::End),
                    ])
                    .lens(Params::cross_alignment),
                    0.,
                ),
            0.0,
        )
        .with_child(
            Flex::column()
                .with_child(Label::new("MainAxisAlignment:").padding(5.0), 0.)
                .with_child(
                    RadioGroup::new(vec![
                        ("Start", MainAxisAlignment::Start),
                        ("Center", MainAxisAlignment::Center),
                        ("End", MainAxisAlignment::End),
                        ("SpaceBetween", MainAxisAlignment::SpaceBetween),
                        ("SpaceEvenly", MainAxisAlignment::SpaceEvenly),
                        ("SpaceAround", MainAxisAlignment::SpaceAround),
                    ])
                    .lens(Params::main_alignment),
                    0.,
                ),
            0.0,
        )
        .with_child(
            Flex::column()
                .with_child(Label::new("Misc:").padding(5.0), 0.)
                .with_child(
                    labeled_checkbox("Debug layout").lens(Params::debug_layout),
                    0.0,
                )
                .with_child(SizedBox::empty().height(10.), 0.0)
                .with_child(
                    labeled_checkbox("Fix minor axis size").lens(Params::fix_minor_axis),
                    0.,
                )
                .with_child(SizedBox::empty().height(10.), 0.0)
                .with_child(
                    labeled_checkbox("Fix major axis size").lens(Params::fix_major_axis),
                    0.,
                )
                .padding(5.0),
            0.0,
        )
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0)
        .lens(AppState::params)
}

fn build_widget(state: &Params) -> Box<dyn Widget<AppState>> {
    let flex = match state.axis {
        FlexType::Column => Flex::column(),
        FlexType::Row => Flex::row(),
    }
    .cross_axis_alignment(state.cross_alignment)
    .main_axis_alignment(state.main_alignment);

    let flex = flex
        .with_child(TextBox::new().lens(DemoState::input_text), 0.)
        .with_child(
            Button::new("Clear", |_ctx, data: &mut DemoState, _env| {
                data.input_text.clear();
                data.enabled = false;
                data.volume = 0.0;
            }),
            0.,
        )
        .with_child(
            Label::new(|data: &DemoState, _: &Env| data.input_text.clone()),
            0.,
        )
        .with_child(Checkbox::new().lens(DemoState::enabled), 0.)
        .with_child(Slider::new().lens(DemoState::volume), 0.)
        .with_child(ProgressBar::new().lens(DemoState::volume), 0.)
        .with_child(
            Stepper::new()
                .min(0.0)
                .max(1.0)
                .step(0.1)
                .lens(DemoState::volume),
            0.0,
        )
        .with_child(Switch::new().lens(DemoState::enabled), 0.)
        .background(Color::rgba8(0, 0, 0xFF, 0x30))
        .lens(AppState::demo_state);

    let mut flex = SizedBox::new(flex);
    if state.fix_minor_axis {
        match state.axis {
            FlexType::Row => flex = flex.height(200.),
            FlexType::Column => flex = flex.width(200.),
        }
    }

    if state.fix_major_axis {
        match state.axis {
            FlexType::Row => flex = flex.width(600.),
            FlexType::Column => flex = flex.height(300.),
        }
    }

    if state.debug_layout {
        flex.debug_paint_layout().boxed()
    } else {
        flex.boxed()
    }
}

fn make_ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(make_control_row(), 0.0)
        .with_child(SizedBox::empty().height(20.), 0.0)
        .with_child(Rebuilder::new(), 0.0)
        .align_vertical(UnitPoint::TOP_LEFT)
        .padding(10.0)
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(make_ui)
        .window_size((600., 600.00))
        .title(LocalizedString::new("Flex Container Options"));

    let demo_state = DemoState {
        input_text: "hello".into(),
        enabled: false,
        volume: 0.0,
    };

    let params = Params {
        axis: FlexType::Row,
        cross_alignment: CrossAxisAlignment::Center,
        main_alignment: MainAxisAlignment::Start,
        debug_layout: false,
        fix_minor_axis: false,
        fix_major_axis: false,
    };

    let data = AppState { demo_state, params };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)?;
    Ok(())
}