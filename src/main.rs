use druid::piet::InterpolationMode;
use druid::widget::prelude::*;
use druid::widget::{Flex, Slider};
use druid::{
    AppLauncher, Color, Data, Lens, LocalizedString, Point, Rect, Widget, WidgetExt, WindowDesc,
};

mod image;
mod integration;
mod math;

#[derive(Clone, Data, Lens)]
struct AppState {
    width: f64,
    height: f64,
    radius: f64,
    std_dev: f64,
}

struct BlurWidget;

const IM_WIDTH: usize = 256;
const IM_HEIGHT: usize = 256;

impl Widget<AppState> for BlurWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        // Let's draw a picture with Piet!

        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        let size = ctx.size();
        let rect = Rect::from_origin_size(Point::ORIGIN, size);
        //ctx.fill(rect.to_rounded_rect(data.radius), &Color::WHITE);
        let radius = data.radius.min(0.5 * data.width.min(data.height));
        let data = integration::gen_integrate(
            IM_WIDTH,
            IM_HEIGHT,
            data.width,
            data.height,
            radius,
            data.std_dev,
        );
        let image = image::make_image_one(ctx, IM_WIDTH, IM_HEIGHT, &data);
        let rect = Size::new(IM_WIDTH as f64, IM_HEIGHT as f64).to_rect();
        ctx.draw_image(&image, rect, InterpolationMode::Bilinear);
    }
}

fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .with_spacer(5.0)
        .with_child(Slider::new().with_range(0.0, 250.0).lens(AppState::width))
        .with_spacer(5.0)
        .with_child(Slider::new().with_range(0.0, 250.0).lens(AppState::height))
        .with_spacer(5.0)
        .with_child(Slider::new().with_range(0.0, 50.0).lens(AppState::radius))
        .with_spacer(5.0)
        .with_child(Slider::new().with_range(0.0, 50.0).lens(AppState::std_dev))
        .with_spacer(5.0)
        .with_flex_child(BlurWidget, 1.0)
}

fn main() {
    let data = AppState {
        width: 100.0,
        height: 80.0,
        radius: 5.0,
        std_dev: 5.0,
    };
    let main_window = WindowDesc::new(ui_builder).title(LocalizedString::new("blur toy"));
    AppLauncher::with_window(main_window).launch(data).unwrap();
}
