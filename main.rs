use nannou::prelude::*;
fn main() {
    nannou::app(model).update(update).run();
}
struct Model{
    _window:window::Id,
    phase: f32,
}
fn model(app: &App) -> Model {
    let _window:WindowId=app.new_window().view(view).build().unwrap();
    Model{_window,phase:0.0}
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.phase=model.phase+0.1;
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let win = app.window_rect();
    draw.line()
        .start(pt2(win.left(), 0.0))
        .end(pt2(win.right(), 0.0))
        .weight(2.0) 
        .color(BLACK);
        draw.line()
        .start(pt2(0.0,win.top()))
        .end(pt2(0.0, win.bottom()))
        .weight(2.0) 
        .color(BLACK);
    let y=(model.phase).sin()*30.0;
    let points = (0..200).map(|i| {
        let x = i as f32 ;          
        let point = pt2(x/10.0+5.0, (x/10.0+model.phase).sin() +5.0) * 30.0; 
        (point, RED)
      });
      draw.polyline()
          .weight(3.0)
          .points_colored(points);
       draw.ellipse().x_y(150.0,y+150.0).radius(4.0).color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
