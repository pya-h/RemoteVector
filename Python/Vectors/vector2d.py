from vectornd import VectorND

class Vector2D(VectorND):
    def __init__(self, x, y):
        super(Vector2D, self).__init__(x, y)

if __name__ == "__main__":
    v = Vector2D(1, 2)
    u = Vector2D(3, 2)

    print(v * u, v + u, v - u, v @ u, v / 5, len(u))