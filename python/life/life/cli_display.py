import os


def render(frame, refresh=True):
    image = [''.join(['██' if element else '  ' for element in row]) for row in frame]
    image = '\n'.join(image)

    if refresh:
        os.system('clear')

    print(image)
