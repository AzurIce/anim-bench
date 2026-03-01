from janim.imports import *

class Janim(Timeline):
    def construct(self):
        tiger = SVGItem("./assets/Ghostscript_Tiger.svg", height=8.0)
        self.show(tiger)
        self.forward(1.0)