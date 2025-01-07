#!/usr/bin/env python3
import math
from typing import Optional
import numpy as np
import cv2

dst_shape = (296, 256)
dst_diameter = 256 # short diameter
src_diameter = 256
src_side_len = src_diameter / math.sqrt(30)
dst_side_len = dst_diameter / math.sqrt(3)

src_img = cv2.transpose(cv2.imread("water_texture_1.jpeg", cv2.IMREAD_COLOR))
dst_img = np.zeros((dst_shape[0], dst_shape[1], 4), np.uint8)

src_center_coord = (src_img.shape[0] / 2, src_img.shape[1] / 2)
dst_center_coord = (dst_side_len, dst_diameter / 2)

def in_hexagon(pos: np.ndarray, center: np.ndarray, diameter: float) -> Optional[tuple[float, float, float]]:
    radius = diameter / 2
    p: np.array = pos - center
    # p1 is the signed distance to top side
    p1: float = p[1]
    # p2 is the distance to bottom-left side
    # project p to (sqrt(3)/2, -1/2)
    p2: float = np.dot(p, np.array((math.sqrt(3) / 2, -0.5)))
    # p3 is the distance to bottom-right side
    # project p to (-sqrt(3)/2, -1/2)
    p3: float = np.dot(p, np.array((-math.sqrt(3) / 2, -0.5)))
    if p1 >= -radius and p1 <= radius and p2 >= -radius and p2 <= radius and p3 >= -radius and p3 <= radius:
        return (p1, p2, p3)
    else:
        return None

def bilinear_sample(img: np.ndarray, pos: tuple[float, float]) -> np.ndarray:
    pos_sat: np.ndarray = np.array(pos).clip((0, 0), (img.shape[0], img.shape[1]))
    pos_floor: np.ndarray = np.floor(pos_sat).clip((0, 0), (img.shape[0] - 2, img.shape[1] - 2)).astype(np.uint)
    pos_ceil: np.ndarray = pos_floor + 1
    p00 = img[pos_floor[0], pos_floor[1]]
    p01 = img[pos_floor[0], pos_ceil[1]]
    p10 = img[pos_ceil[0] , pos_floor[1]]
    p11 = img[pos_ceil[0] , pos_ceil[1]]
    x, y = pos_sat - pos_floor
    return (1 - x) * (1 - y) * p00 + (1 - x) * y * p01 + x * (1 - y) * p10 + x * y * p11

def interpolate(x0: np.ndarray, x1: np.ndarray, t: float) -> np.ndarray:
    t = max(0, (t - 0.5) * 2)
    return (1 - t) * x0 + t * x1


for y in range(0, dst_shape[1]):
    for x in range(0, dst_shape[0]):
        
        # filter out points
        coord = (x, y)
        uv = in_hexagon(np.array(coord), np.array(dst_center_coord), dst_diameter)
        if uv is None:
            continue
        
        src_coord = (np.array(coord) - np.array(dst_center_coord)) / dst_diameter * src_diameter + np.array(src_center_coord)
        
        src_coord_center = (src_coord[0], src_coord[1])
        src_coord_top = (src_coord[0], src_coord[1] - src_diameter)
        src_coord_bottom = (src_coord[0], src_coord[1] + src_diameter)
        src_coord_topleft = (src_coord[0] - src_diameter / 2 * math.sqrt(3), src_coord[1] - src_diameter / 2)
        src_coord_topright = (src_coord[0] + src_diameter / 2 * math.sqrt(3), src_coord[1] - src_diameter / 2)
        src_coord_bottomleft = (src_coord[0] - src_diameter / 2 * math.sqrt(3), src_coord[1] + src_diameter / 2)
        src_coord_bottomright = (src_coord[0] + src_diameter / 2 * math.sqrt(3), src_coord[1] + src_diameter / 2)
        
        sample_center = bilinear_sample(src_img, src_coord_center)
        sample_top = bilinear_sample(src_img, src_coord_top)
        sample_bottom = bilinear_sample(src_img, src_coord_bottom)
        sample_topleft = bilinear_sample(src_img, src_coord_topleft)
        sample_topright = bilinear_sample(src_img, src_coord_topright)
        sample_bottomleft = bilinear_sample(src_img, src_coord_bottomleft)
        sample_bottomright = bilinear_sample(src_img, src_coord_bottomright)
        
        t = np.array(uv) / dst_diameter + 0.5
        t1 = [max(0, (i - 0.5) * 2) for i in t] + [max(0, (0.5 - i) * 2) for i in t]
        t = [3 - sum(t1), *t1]
        """
        f(-1, y, 1-y) = f(1, y-1, -y) = sample()
        f(1-z, -1, z) = f(-z, 1, z-1)
        f(x , 1-x, -1) = f(x-1, -x, 1)
        """
        
        # dst_img[coord] = (*sample_center, 255)
        dst_img[coord] = (*((t[0] * sample_center + t[1] * sample_top + t[2] * sample_bottomleft + t[3] * sample_bottomright + t[4] * sample_bottom + t[5] * sample_topright + t[6] * sample_topleft) / 3), 255)
        

cv2.imwrite("water_tile_1.png", cv2.transpose(dst_img))