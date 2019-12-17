import matplotlib.pyplot as plt

def segment(xa, ya, xb, yb):
    return ((xa, ya), (xb, yb))

def plot_segments(plot, segments):
    for s in segments:
        a, b = s
        xa, ya = a
        xb, yb = b
        plot.plot([xa, xb], [ya, yb], c="g")

def read_from(filename):
    segments = []
    with open(filename, "r") as f:
        while True:
            numbers = list(map(float, f.readline().split()))
            if numbers == []:
                break
            segments.append(segment(*numbers))

    return segments

_, plts = plt.subplots(2, 1)

plot_segments(plts[0], read_from("input"))
plot_segments(plts[1], read_from("output"))
plt.show()
