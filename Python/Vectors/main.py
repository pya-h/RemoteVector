from vector2d import Vector2D
   
if __name__ == '__main__':
    vectors = []
    print("Enter your vectors in each line or press enter to end getting input.\n" + \
        "Input Format: x y\n-----------------------------")
    while True:
        v = input(f"{len(vectors) + 1}) ")
        if not v:
            break
        v = v.split()
        try:
            vectors.append(Vector2D(float(v[0]), float(v[1])))
        except:
            print("ERROR: Wrong Input!")
    two_len_vectors = filter(lambda v: len(v) == 2, vectors)
    print("Vectors with the length of 2 are:")
    for v in two_len_vectors:
        print(v)
    print("---------------------------------")
    try:
        indexes = input(f'Now Enter the index of two vectors you want to do mathmatical operations on them, seperated by space (1-{len(vectors)})\n')
        i, j = [int(x) - 1 for x in indexes.split()]
        if i >= 0 and i < len(vectors) and j >= 0 and j < len(vectors):
            print(vectors[i], " + ", vectors[j], " = ", vectors[i] + vectors[j])
            print(vectors[i], " - ", vectors[j], " = ", vectors[i] - vectors[j])
            print(vectors[i], " * ", vectors[j], " = ", vectors[i] * vectors[j])
            print(vectors[i], " @ ", vectors[j], " = ", vectors[i] @ vectors[j])
            print(vectors[i], " / len(v) = ", vectors[i] / len(vectors[i]))
            print(vectors[i], " / ", vectors[j], " = ", vectors[i] / vectors[j])
        else:
            raise ValueError("Indexes you provided are not valid!")
    except Exception as ex:
        print("ERROR:", ex)