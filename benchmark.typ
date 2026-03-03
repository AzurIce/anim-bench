#import "@preview/lilaq:0.3.0" as lq

#set page(flipped: true)

#let engines = ("Manim", "JAnim", "Ranim", "ManimCE")

#let engine_colors = (
  "Manim": rgb("#58C4DD"),
  "JAnim": rgb("#E07F7F"),
  "Ranim": rgb("#83C167"),
  "ManimCE": rgb("#F5A623"),
)

#let data_availability = (
  "0.1.1": (
    "windows": ("static_square", "static_tiger", "transform_square"),
    "linux": ("static_square", "static_tiger", "transform_square"),
    "macos": ("static_square", "static_tiger", "transform_square"),
  ),
  "main": (
    "macos": ("static_square", "static_tiger", "transform_square", "animating_pi"),
  ),
)

#let scene_inputs = (
  "static_square": (5, 10, 20, 40),
  "static_tiger": none,
  "transform_square": (5, 10, 20, 40),
  "animating_pi": none,
)

#let versions = data_availability.keys()

#let platforms = {
  let all = ()
  for (_, p) in data_availability {
    for (platform, _) in p {
      if platform not in all { all.push(platform) }
    }
  }
  all
}

#let has_data = (version, platform, scene) => {
  version in data_availability and platform in data_availability.at(version) and scene in data_availability.at(version).at(platform)
}

#let get_data = (version, platform, scene, engine_id, input) => {
  if not has_data(version, platform, scene) { return none }
  let path = "assets/data/" + version + "/" + platform + "/criterion/" + scene + "/" + engine_id + "/" + str(input) + "/new/estimates.json"
  json(path)
}

#show lq.selector(lq.legend): set table(columns: 4)

#let bar_width = 0.2

#let make_bars = (version, platform, scene) => {
  let offset_base = -(engines.len() - 1) * bar_width / 2
  engines.enumerate().map(((i, engine)) => {
    let data = get_data(version, platform, scene, lower(engine), 0)
    if data != none {
      lq.bar(
        (1,),
        (data.mean.point_estimate / 1e9,),
        label: [#engine],
        fill: engine_colors.at(engine),
        width: bar_width,
        offset: offset_base + i * bar_width,
      )
    } else {
      []
    }
  })
}

#let make_plot_multi = (version, platform, scene, inputs) => {
  engines.map(engine => {
    let values = inputs.map(n => {
      let d = get_data(version, platform, scene, lower(engine), n)
      if d != none { d.mean.point_estimate / 1e9 } else { none }
    })
    if values.any(v => v != none) {
      lq.plot(inputs, values, label: [#engine], color: engine_colors.at(engine))
    } else {
      []
    }
  })
}

#let perfchart = (version, platform, scene, show_legend: false) => {
  if not has_data(version, platform, scene) {
    return box(
      width: 100%,
      height: 3.5cm,
      fill: rgb(245, 245, 245),
      radius: 4pt,
      align(center + horizon)[#text(rgb(160, 160, 160))[数据暂缺]]
    )
  }
  let inputs = scene_inputs.at(scene)
  let plots = if inputs != none {
    make_plot_multi(version, platform, scene, inputs)
  } else {
    make_bars(version, platform, scene)
  }
  lq.diagram(
    height: 3.5cm,
    ylabel: [耗时 (s)],
    xlabel: if inputs != none { [N] },
    legend: (position: top + right, radius: 2pt, inset: 0.2em),
    xaxis: if inputs == none { (ticks: none, subticks: none) },
    margin: (left: 10%, bottom: if inputs != none { 15% } else { 8% }),
    ..plots,
  )
}

#let platform_label = (platform) => {
  if platform == "windows" { [Windows]
  } else if platform == "linux" { [Linux]
  } else if platform == "macos" { [macOS]
  } else { [#platform]
  }
}

#let scene_page = (scene_id) => {
  pagebreak()
  heading(level: 1, [#scene_id])

  let available_versions = versions.filter(v =>
    platforms.any(p => has_data(v, p, scene_id))
  )
  let available_platforms = platforms.filter(p =>
    available_versions.any(v => has_data(v, p, scene_id))
  )

  if available_versions.len() == 0 {
    align(center)[#text(rgb(128, 128, 128))[暂无数据]]
    return
  }

  let header_row = ([], ..available_versions.map(v => align(center)[#strong[#v]]))
  let data_rows = available_platforms.map(platform => {
    (
      align(right + horizon)[#strong[#platform_label(platform)]],
      ..available_versions.map(v => perfchart(v, platform, scene_id))
    )
  })

  grid(
    columns: (auto,) + (1fr,) * available_versions.len(),
    column-gutter: 0.8em,
    row-gutter: 0.6em,
    ..header_row,
    ..data_rows.flatten(),
  )
}

#scene_page("static_square")
#scene_page("static_tiger")
#scene_page("transform_square")
#scene_page("animating_pi")
