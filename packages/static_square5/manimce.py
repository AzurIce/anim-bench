from manim import *

# config.frame_y_radius = 4.0
config.background_color = "#333333"
# config.pixel_height = 1080
# config.pixel_width = 1080
# config.fps = 60
# print(config)

N = 5

class ManimCE(Scene):
    def construct(self):
        buff = 0.1
        size = 8.0 / N - buff
        
        unit = size + buff
        start_coord = np.array([-4.0, -4.0, 0.0]) + np.array([unit, unit, 0.0]) / 2
        matrix = VGroup()
        coords = [start_coord + RIGHT * unit * j + UP * unit * i for j in range(N) for i in range(N)]
        for coord in coords:
            square = Square(size)
            square.move_to(coord)
            matrix.add(square)
        self.add(matrix)
        self.wait(1)