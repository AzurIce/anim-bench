import numpy as np
from janim.imports import *


class Janim(Timeline):
    def construct(self):
        # Create 10x10 grid of pi symbols
        w, h = 10, 10
        pis = Group()

        # Calculate individual pi size to match Ranim
        # Total height = TAU - 0.25, with 10 pis and 9 buffs of 0.2
        # 10 * pi_height + 9 * 0.2 = TAU - 0.25
        # pi_height = (TAU - 0.25 - 1.8) / 10
        pi_height = (TAU - 0.25 - 9 * 0.2) / 10

        for i in range(h):
            for j in range(w):
                pi_tex = TypstMath(r"pi")
                pi_tex.points.set_height(pi_height)
                pis.add(pi_tex)

        # Arrange in grid
        pis.points.arrange_in_grid(h, w, buff=0.2).to_center()

        # Animation sequence
        self.show(pis)
        self.forward(1.0)

        # Complex exponential transformation
        def complex_exp_func(p):
            z = complex(p[0], p[1])
            w = np.exp(z)
            p[0] = w.real
            p[1] = w.imag
            return p

        self.play(pis.anim.points.apply_point_fn(complex_exp_func), duration=5.0)
        self.forward(1.0)

        # Wave transformation
        def wave_func(p):
            x, y = p[0], p[1]
            p[0] = x + 0.5 * np.sin(y)
            p[1] = y + 0.5 * np.cos(x)
            return p

        self.play(pis.anim.points.apply_point_fn(wave_func), duration=5.0)
        self.forward(1.0)
