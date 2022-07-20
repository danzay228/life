# import subprocess
# import sys
# from typing import TextIO


# window: TextIO = None


def render(frame, new_window=False, refresh=False):
    image = [''.join(['██' if element else '  ' for element in row]) for row in frame]
    image = '\n'.join(image)

    # global window
    # if window is None:
    #     window = TextIO() if new_window else sys.stdout
    # if refresh:
    #     print('\f')  # version 1 - doesn't clear buffer
    # # window.writelines(image)
    # window.write(image)
    # window.flush()

    print(image)
