#import "@preview/lilaq:0.3.0" as lq

#let engines = ("Manim", "JAnim", "Ranim", "ManimCE")
#let platforms = ("windows", "linux", "macos")
#let json_file = (version, platform, scene, engine, input) => {
  json("assets/data/" + version + "/" + platform + "/criterion/" + scene + "/" + engine + "/" + str(input) + "/new/estimates.json")
}

#show lq.selector(lq.legend): set table(columns: 4)
#let ns = (5, 10, 20, 40)
#let perfplot = (version, platform, scene) => {
  engines.map(engine => {
    let datas = ns.map(n => json_file(version, platform, scene, lower(engine), n))
    lq.plot(
      ns,
      datas.map(data => {
        data.mean.point_estimate / 1e9
      }),
      label: [#engine],
    )
  })
}

#let perfdiagram = (version, platform, scene, caption, id) => {

  [
    #figure(
      lq.diagram(
        height: 3.4cm,
        ylabel: [平均耗时 (s)],
        xlabel: [输入的 N 值],
        legend: (position: (13%, 0% - 4em)),
        ..perfplot(version, platform, scene),
      ),
      kind: "image",
      caption: caption,
      supplement: "pic"
    )
    #label(id)
  ]
}

#perfdiagram(
  "0.1.1",
  "windows",
  "static_square",
  [各引擎在 Windows 平台下以不同 $N$ 为输入的

    静态场景渲染耗时数据比较],
  "static-square-perf-win",
)

#v(0.5em)

// #perfdiagram(
//   "linux",
//   "static_square",
//   [各引擎在 Linux 平台下以不同 $N$ 为输入的

//     静态场景渲染耗时数据比较],
//   "static-square-perf-linux",
// )

// #v(0.5em)

#perfdiagram(
  "0.1.1",
  "macos",
  "static_square",
  [各引擎在 Mac 平台下以不同 $N$ 为输入的

    静态场景渲染耗时数据比较],
  "static-square-perf-mac",
)
