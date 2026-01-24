
class VectorND:
    @staticmethod
    def Zero(dimension):
        return VectorND(*([0] * dimension))
    
    def __init__(self, *components):
        if not len(components):
            raise ValueError("Vector components must be specified!")
        self.components = components
        self.n = len(self.components)
        # implement unknown components?
        
    def __add__(self, v): 
        if isinstance(v, VectorND):
            if self.n != v.n:
                raise ValueError("Vectors must have same dimensions!")
            comps = [0] * self.n
            for i in range(self.n):
                comps[i] = self.components[i] + v.components[i]
            return VectorND(*comps)
        raise ValueError("Both operands must be vectors!")
    
    def __sub__(self, v):
        if isinstance(v, VectorND):
            if self.n != v.n:
                raise ValueError("Vectors must have same dimensions!")
            comps = [0] * self.n
            for i in range(self.n):
                comps[i] = self.components[i] - v.components[i]
            return VectorND(*comps)
        raise ValueError("Both operands must be vectors!")

    def __mul__(self, v):
        if isinstance(v, VectorND):
            if self.n != v.n:
                raise ValueError("Vectors must have same dimensions!")
            comps = [0] * self.n
            for i in range(self.n):
                comps[i] = self.components[i] * v.components[i]
            return VectorND(*comps)
        elif type(v) == str or int or float:
            try:
                x = float(v) if type(v) == str else v
                comps = [x] * self.n
                for i in range(self.n):
                    comps[i] *= self.components[i]
                return VectorND(*comps)
            except:
                raise ValueError("One of the operands is neither a vector or a number!")
        else:
            raise ValueError("One of the operands is neither a vector or a number!")
    
    def __truediv__(self, v):
        if type(v) == str or int or float:
            try:
                x = float(v) if type(v) == str else v
                if x == 0:
                    raise ValueError("Cannot devide vector by Zero!")
                comps = [0] * self.n
                for i in range(self.n):
                    comps[i] = self.components[i] / x
                return VectorND(*comps)
            except:
                pass
        
        raise ValueError("Second operand of the / operator must be numeric!")
    
    def is_complex(self):
        # vector containing vector
        for vi in self.components:
            if isinstance(vi, VectorND):
                return True
        return False
    
    def __matmul__(self, v):
        if isinstance(v, VectorND):
            if self.n != v.n:
                raise ValueError("Vectors must have same dimensions!")

            if self.is_complex() or v.is_complex():
                dot = [0] * self.n
                for i in range(self.n):
                    dot[i] = self.components[i] @ v.components[i]
                return VectorND(*dot) 
            else:
                dot = 0
                for i in range(self.n):
                    dot += self.components[i] * v.components[i]
                return dot
        raise ValueError("Both @ operands must be vectors")
    
    def __str__(self):
        s = '('
        for i, x in enumerate(self.components):
            s += str(x) + (', ' if i != self.n - 1 else ')')
        return s

    def __len__(self):
        amplitude = 0
        for x in self.components:
            amplitude += x ** 2
        return int(amplitude ** 0.5)

if __name__ == "__main__":
    v = VectorND(1, 2, 0, 4)
    u = VectorND(3, 2, 1, 2)
    
    print(v * u, v + u, v - u, v @ u, v / 5, len(u))