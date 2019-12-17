import matplotlib.pyplot as plt

def segment(xa, ya, xb, yb):
    return ((xa, ya), (xb, yb))

def plot_segments(segments):
    for s in segments:
        a, b = s
        xa, ya = a
        xb, yb = b
        plt.plot([xa, xb], [ya, yb])

def read_from(filename):
    segments = []
    with open(filename, "r") as f:
        while True:
            numbers = list(map(float, f.readline().split()))
            if numbers == []:
                break
            segments.append(segment(*numbers))

    return segments

plot_segments(read_from("input"))
plt.show()

plot_segments(read_from("output"))
plt.show()
