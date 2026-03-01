from manim import *

# config.frame_y_radius = 4.0
config.background_color = "#333333"
# config.pixel_height = 1080
# config.pixel_width = 1080
# config.fps = 60
# print(config)

class ManimCE(Scene):
    def construct(self):
        tiger = SVGMobject("./assets/Ghostscript_Tiger.svg", height=8.0)
        self.add(tiger)
        self.wait(1)