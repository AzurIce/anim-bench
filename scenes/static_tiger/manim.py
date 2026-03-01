from manimlib import *

class Manim(Scene):
    def construct(self):
        tiger = SVGMobject("./assets/Ghostscript_Tiger.svg", height=8)
        self.add(tiger)
        self.wait(1)