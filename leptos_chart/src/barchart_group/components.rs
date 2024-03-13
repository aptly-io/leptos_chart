use crate::{
    axes::{XAxis, YAxis},
    core::SvgChart,
};
use leptos::{component, view, IntoView};
use theta_chart::{color::Color, coord, series::Series};

/// Component LineChart for leptos
///
/// # Examples
///
/// ## Cargo.toml
///
/// ```toml
/// [dependencies]
/// leptos = {version = "0.6"}
/// leptos_chart = {version = "0.2.0", features = ["BarChartGroup"]}
/// ```
///
/// ## Component
/// ```ignore
/// use leptos::*;
/// use leptos_chart::*;
///
/// #[component]
/// pub fn App() -> impl IntoView {
/// let chart = CartesianGroup::new()
///     .set_view(840, 640, 3, 50, 50, 20)
///     .add_data(
///         Series::from(vec!["A", "B", "C"]),
///         Series::from(vec![0.7, 1.5, 1.9]),
///     )
///     .add_data(
///         Series::from(vec!["A", "B", "C"]),
///         Series::from(vec![0.3, 0.5, 0.9]),
///     );
///     
///     let color = Color::from("#ff0000");
///     let shift_degrees = 180.0;
///
///     view!{
///         // color and shift_degrees are options
///         <BarChartGroup chart=chart color=color shift_degrees=shift_degrees />
///     }
/// }
/// ```
/// ## Set view for LineChart
/// ```ignore
///     ...
///     .set_view(820, 620, 3, 100, 100, 20);
///     ...
/// ```
/// ## Arguments
/// - `width` : The width of SGV
/// - `height` : The height of SGV
/// - `position_origin` : Positions for origin of chart xOy
/// - `height_x_axis` : Height x_axis
/// - `width_y_axis` : Width y_axis
/// - `margin` : Margin for actual chart
///
/// ## About position_axes
///
/// - Top Left: 0
/// - Top Right: 1
/// - Bottom Right: 2
/// - Bottom Left: 3
///
#[allow(non_snake_case)]
#[component]
pub fn BarChartGroup(
    chart: coord::CartesianGroup,
    #[prop(default = Color::default())] color: Color,
    #[prop(default = 70.)] shift_degrees: f32,
) -> impl IntoView {
    let cview = chart.get_view();

    // For Chart
    let rec_chart = cview.get_rec_chart();
    let translate_chart = format!(
        "translate({},{})",
        rec_chart.get_origin().get_x(),
        rec_chart.get_origin().get_y()
    );

    // For x-axis
    let rec_xa = cview.get_rec_x_axis();
    let translate_xa = format!(
        "translate({},{})",
        rec_xa.get_origin().get_x(),
        rec_xa.get_origin().get_y()
    );
    let series_x_group = chart.get_ax_group();
    let axes_x = series_x_group.gen_axes();

    // For y-axis
    let rec_ya = cview.get_rec_y_axis();
    let translate_ya = format!(
        "translate({},{})",
        rec_ya.get_origin().get_x(),
        rec_ya.get_origin().get_y()
    );
    let series_y_group = chart.get_ay_group();

    let axes_y = series_y_group.gen_axes();

    // For chart
    let data = chart.get_data();
    let mut xseries: Vec<Series> = vec![];
    let mut yseries: Vec<Series> = vec![];
    for tup in data {
        xseries.push(tup.0);
        yseries.push(tup.1);
    }

    let mut x_is_label = true;
    match xseries[0] {
        Series::Label(_) => (),
        _ => x_is_label = false,
    }

    view! {
      <SvgChart cview=cview>
        <g class="axes">
          <g class="x-axis" transform=translate_xa>
            <XAxis region=rec_xa axes=axes_x/>
          </g>
          <g class="y-axis" transform=translate_ya>
            <YAxis region=rec_ya axes=axes_y/>
          </g>
        </g>
        <g class="inner-chart" transform=translate_chart>
          // For draw region of chart

          {#[cfg(feature = "debug")]
          {
              let vector = rec_chart.get_vector();
              let path = format!(
                  "M {},{} l {},{} l {},{} l {},{} Z",
                  0,
                  0,
                  vector.get_x(),
                  0,
                  0,
                  vector.get_y(),
                  -vector.get_x(),
                  0,
              );
              view! {
                <circle id="origin" cx="0" cy="0" r="3"></circle>
                <line
                  x1="0"
                  y1="0"
                  x2=vector.get_x()
                  y2=vector.get_y()
                  style="stroke:#00ff0033;stroke-width:2"
                ></line>
                <path id="region" d=path fill="#00ff0033"></path>
              }
          }}

          {
              let vector = rec_chart.get_vector();
              if x_is_label {
                  let len = xseries.len();
                  let position = 0.9 / len as f64;
                  let len_group = series_x_group.get_count();
                  xseries
                      .into_iter()
                      .enumerate()
                      .map(|(index, series_x)| {
                          let color = color.shift_hue_degrees_index(shift_degrees, index);
                          let xstick = series_x.to_stick();
                          let ystick = yseries[index].to_stick();
                          let width_col = series_x_group.scale(position) * vector.get_x();
                          let style = format!(
                              "stroke:{};stroke-width:{}",
                              color.to_string_hex(),
                              width_col.abs() as u64,
                          );
                          let interval = vector.get_x() / len_group as f64;
                          xstick
                              .into_iter()
                              .enumerate()
                              .map(|(indexi, data)| {
                                  let label = data.label;
                                  let x: f64 = ((series_x_group.scale_index(label.clone()) as f64
                                      / (len_group as f64)) as f64) * vector.get_x()
                                      + (position * index as f64 + position / 2. + 0.05) * interval;
                                  let y: f64 = series_y_group.scale(ystick[indexi].value)
                                      * vector.get_y();
                                  view! {
                                    // len as f64;

                                    <line x1=x y1="0" x2=x y2=y style=style.clone()></line>
                                  }
                              })
                              .collect::<Vec<_>>()
                      })
                      .collect::<Vec<_>>()
              } else {
                  let len = yseries.len();
                  let position = 0.9 / len as f64;
                  let len_group = series_y_group.get_count();
                  yseries
                      .into_iter()
                      .enumerate()
                      .map(|(index, series_y)| {
                          let color = color.shift_hue_degrees_index(shift_degrees, index);
                          let xstick = xseries[index].to_stick();
                          let ystick = series_y.to_stick();
                          let width_col = series_y_group.scale(position) * vector.get_y();
                          let style = format!(
                              "stroke:{};stroke-width:{}",
                              color.to_string_hex(),
                              width_col.abs() as u64,
                          );
                          let interval = vector.get_y() / len_group as f64;
                          ystick
                              .into_iter()
                              .enumerate()
                              .map(|(indexi, data)| {
                                  let label = data.label;
                                  let x: f64 = series_x_group.scale(xstick[indexi].value)
                                      * vector.get_x();
                                  let y: f64 = ((series_y_group.scale_index(label.clone()) as f64
                                      / (len_group as f64)) as f64) * vector.get_y()
                                      + (position * index as f64 + position / 2. + 0.05) * interval;
                                  view! {
                                    // len as f64;

                                    // len as f64;

                                    // len as f64;

                                    // len as f64;

                                    <line x1="0" y1=y x2=x y2=y style=style.clone()></line>
                                  }
                              })
                              .collect::<Vec<_>>()
                      })
                      .collect::<Vec<_>>()
              }
          }

        </g>

      </SvgChart>
    }
}
