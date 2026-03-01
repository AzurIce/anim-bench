from manim import *
import numpy as np

config.background_color = "#333333"


class ManimCE(Scene):
    def construct(self):
        # Create 10x10 grid of pi symbols
        w, h = 10, 10
        pis = VGroup()

        # Calculate individual pi size to match Ranim
        # Total height = TAU - 0.25, with 10 pis and 9 buffs of 0.2
        # 10 * pi_height + 9 * 0.2 = TAU - 0.25
        # pi_height = (TAU - 0.25 - 1.8) / 10
        pi_height = (TAU - 0.25 - 9 * 0.2) / 10

        for i in range(h):
            for j in range(w):
                pi_tex = MathTex(r"\pi", color=WHITE)
                pi_tex.set_height(pi_height)
                pis.add(pi_tex)

        # Arrange in grid
        pis.arrange_in_grid(rows=h, cols=w, buff=0.2)
        pis.move_to(ORIGIN)

        # Animation sequence
        self.add(pis)
        self.wait(1)

        # Complex exponential transformation
        def complex_exp_func(point):
            z = complex(point[0], point[1])
            w = np.exp(z)
            return np.array([w.real, w.imag, point[2]])

        self.play(pis.animate.apply_function(complex_exp_func), run_time=5)
        self.wait(1)

        # Wave transformation
        def wave_func(point):
            x, y, z = point
            new_x = x + 0.5 * np.sin(y)
            new_y = y + 0.5 * np.cos(x)
            return np.array([new_x, new_y, z])

        self.play(pis.animate.apply_function(wave_func), run_time=5)
        self.wait(1)
