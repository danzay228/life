import time
import json
import life


def loop(init_frame, timeout=0.1):
    # for frame in life.FrameGenerator(init_frame):
    for frame in life.frame_generator(init_frame):
        life.render(frame)
        time.sleep(timeout)


def main():
    images = json.load(open("/home/dz/Documents/Life_Game/patterns_examples.json"))
    # example = images["Oscillators"]["Penta-decathlon"]
    example = images["Oscillators"]["Pulsar"]
    # example = images["Spaceships"]["Glider"]
    # example = images["Spaceships"]["HWSS"]
    loop(example)


if __name__ == '__main__':
   main()