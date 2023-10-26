import dataclasses
from typing import Optional
import pyautogui
import win32gui

@dataclasses.dataclass
class Point:
    x: Optional[int]
    y: Optional[int]

@dataclasses.dataclass
class Route:
    src: Point
    dest: Point

routes: list[Route] = [
    Route(Point(2559, None), Point(-1920, None)),
    Route(Point(-1920, None), Point(2559, None)),
    
]

def is_pos_in_route(route: Route, pos: Point) -> bool:
    return (route.src.x == pos.x or not route.src.x) and \
    (route.src.y == pos.y or not route.src.y)

def move_to_route(route: Route, pos: Point) -> None:
    dest_x = route.dest.x if route.dest.x else pos.x
    dest_y = route.dest.y if route.dest.y else pos.y
    pyautogui.moveTo((dest_x, dest_y))


def main() -> None:
    pyautogui.FAILSAFE = False
    while True:
        pos = Point(*pyautogui.position())
        print(pos)
        for route in routes:
            if is_pos_in_route(route, pos):
                move_to_route(route, pos)
        
if __name__ == '__main__':
    main() 