from janim.imports import *

N = 20

class Janim(Timeline):
    def construct(self):
        buff = 0.1
        size = 8.0 / N - buff

        matrix = Square(size) * (N * N)
        matrix.points.arrange_in_grid(N, N, buff=buff)

        self.show(matrix)
        self.forward(1.0)